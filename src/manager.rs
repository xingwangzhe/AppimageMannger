//! Filesystem operations never execute an AppImage while inspecting it.
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Appearance {
    #[default]
    Light,
    Dark,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowStyle {
    #[default]
    Gpui,
    Qt,
    Gnome,
}
fn default_language() -> String {
    "system".into()
}
fn default_width() -> f32 {
    1160.0
}
fn default_height() -> f32 {
    800.0
}
fn default_sidebar_fraction() -> f32 {
    0.27
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_language")]
    pub language: String,
    pub applications_dir: PathBuf,
    #[serde(default)]
    pub repositories: BTreeMap<String, String>,
    #[serde(default)]
    pub appearance: Appearance,
    #[serde(default)]
    pub window_style: WindowStyle,
    #[serde(default)]
    pub last_selected_path: Option<PathBuf>,
    #[serde(default = "default_width")]
    pub window_width: f32,
    #[serde(default = "default_height")]
    pub window_height: f32,
    #[serde(default)]
    pub maximized: bool,
    #[serde(default = "default_sidebar_fraction")]
    pub sidebar_fraction: f32,
    #[serde(default)]
    pub directory_draft: Option<String>,
    #[serde(default)]
    pub repository_drafts: BTreeMap<String, String>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            applications_dir: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Applications"),
            repositories: BTreeMap::new(),
            language: default_language(),
            appearance: Appearance::Light,
            window_style: WindowStyle::Gpui,
            last_selected_path: None,
            window_width: default_width(),
            window_height: default_height(),
            maximized: false,
            sidebar_fraction: default_sidebar_fraction(),
            directory_draft: None,
            repository_drafts: BTreeMap::new(),
        }
    }
}
impl Config {
    pub fn load() -> Result<Self> {
        Self::load_from(&config_path()?)
    }
    fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let mut config: Self = serde_json::from_slice(&fs::read(path)?)?;
        let legacy: Vec<_> = config
            .repository_drafts
            .keys()
            .filter(|k| Path::new(k).is_absolute())
            .cloned()
            .collect();
        for key in legacy {
            if let Some(value) = config.repository_drafts.remove(&key)
                && let Some(name) = Path::new(&key).file_name()
            {
                config
                    .repository_drafts
                    .entry(name.to_string_lossy().into_owned())
                    .or_insert(value);
            }
        }
        if !config.window_width.is_finite() || !(880.0..=7680.0).contains(&config.window_width) {
            config.window_width = default_width();
        }
        if !config.window_height.is_finite() || !(580.0..=4320.0).contains(&config.window_height) {
            config.window_height = default_height();
        }
        if !config.sidebar_fraction.is_finite() {
            config.sidebar_fraction = default_sidebar_fraction();
        }
        config.sidebar_fraction = config.sidebar_fraction.clamp(0.15, 0.7);
        ensure!(
            config.applications_dir.is_absolute(),
            "Applications directory must be an absolute path"
        );
        Ok(config)
    }
    pub fn save(&self) -> Result<()> {
        self.save_to(&config_path()?)
    }
    fn save_to(&self, path: &Path) -> Result<()> {
        ensure!(
            self.applications_dir.is_absolute(),
            "Applications directory must be an absolute path"
        );
        fs::create_dir_all(path.parent().unwrap())?;
        let mut tmp = tempfile::NamedTempFile::new_in(path.parent().unwrap())?;
        tmp.write_all(&serde_json::to_vec_pretty(self)?)?;
        tmp.as_file().sync_all()?;
        tmp.persist(path).map_err(|e| e.error)?;
        Ok(())
    }
}
fn config_path() -> Result<PathBuf> {
    Ok(dirs::config_dir()
        .context("Cannot determine the configuration directory")?
        .join("appimage-mannger/config.json"))
}
fn applications_desktop_dir() -> Result<PathBuf> {
    Ok(user_data_dir()?.join("applications"))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub description: String,
    pub repository: Option<String>,
    pub update_info: Option<String>,
    pub desktop_file: Option<PathBuf>,
    pub architecture: String,
    pub icon_path: Option<PathBuf>,
    pub update_progress_path: PathBuf,
}

pub fn scan(config: &Config) -> Result<Vec<AppInfo>> {
    if !config.applications_dir.exists() {
        return Ok(vec![]);
    }
    ensure!(
        config.applications_dir.is_dir(),
        "Applications path is not a directory"
    );
    let mut apps = Vec::new();
    // A scan uses one snapshot; reopening every launcher for every image makes
    // desktop I/O grow with the product of the two directory sizes.
    let desktops = read_desktops().unwrap_or_default();
    for entry in fs::read_dir(&config.applications_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type()?.is_file() {
            continue;
        }
        if path
            .extension()
            .is_none_or(|e| !e.eq_ignore_ascii_case("AppImage"))
        {
            continue;
        }
        let target = DesktopTarget::new(&path);
        let desktop = desktops
            .iter()
            .find(|desktop| target.matches(desktop))
            .cloned()
            .unwrap_or_default();
        if let Ok(app) = inspect_with_desktop(config, &path, desktop) {
            apps.push(app);
        }
    }
    apps.sort_by_cached_key(|a| a.name.to_lowercase());
    Ok(apps)
}

pub fn inspect(config: &Config, path: &Path) -> Result<AppInfo> {
    inspect_with_desktop(config, path, find_desktop(path).unwrap_or_default())
}

fn inspect_with_desktop(
    config: &Config,
    path: &Path,
    mut desktop: DesktopMetadata,
) -> Result<AppInfo> {
    let header = validate_appimage(path)?;
    ensure!(
        !path.to_string_lossy().chars().any(char::is_control),
        "Application path contains control characters"
    );
    let filename = path
        .file_name()
        .context("Missing filename")?
        .to_string_lossy()
        .into_owned();
    let mut name = filename
        .trim_end_matches(".AppImage")
        .trim_end_matches(".appimage")
        .to_string();
    let mut version = filename_version(&name);
    if let Some(v) = &version
        && let Some(i) = name.find(v)
    {
        name = name[..i].trim_end_matches(['-', '_', ' ']).to_string();
    }
    if name.is_empty() {
        name = filename.clone();
    }
    // Embedded publisher metadata is authoritative; generated launchers may be old.
    if let Some(embedded) = crate::metadata::embedded_desktop(path) {
        desktop.values.extend(embedded);
    }
    if let Some(v) = desktop.values.get("Name") {
        name = v.clone();
    }
    version = desktop
        .values
        .get("X-AppImage-Version")
        .filter(|value| {
            version.is_none() || semver::Version::parse(value.trim_start_matches('v')).is_ok()
        })
        .cloned()
        .or(version);
    let update_info = read_update_info(path)
        .ok()
        .flatten()
        .or_else(|| desktop.values.get("X-AppImage-UpdateInformation").cloned());
    let id = stable_id(&filename);
    if let Ok(Some(installed)) = installed_version(path, &id) {
        version = Some(installed);
    }
    let repository = config
        .repositories
        .get(&filename)
        .cloned()
        .or_else(|| update_info.as_deref().and_then(repository_from_update_info))
        .or_else(|| {
            desktop
                .values
                .get("X-GitHub-Repository")
                .and_then(|s| normalize_repository(s).ok())
        });
    Ok(AppInfo {
        update_progress_path: user_data_dir()?
            .join("appimage-mannger/progress")
            .join(format!("{id}.json")),
        id,
        name,
        version: version.unwrap_or_else(|| "Unknown".into()),
        path: path.to_path_buf(),
        size_bytes: fs::metadata(path)?.len(),
        description: desktop
            .values
            .get("Comment")
            .cloned()
            .unwrap_or_else(|| "Standalone Linux AppImage application".into()),
        repository,
        update_info,
        icon_path: crate::metadata::resolve_icon(path, desktop.path.as_deref()),
        desktop_file: desktop.path,
        architecture: match u16::from_le_bytes([header[18], header[19]]) {
            62 => "x86_64",
            183 => "aarch64",
            3 => "x86",
            40 => "arm",
            _ => "Unknown",
        }
        .into(),
    })
}

pub fn set_repository(config: &mut Config, app: &AppInfo, repository: &str) -> Result<()> {
    let key = app
        .path
        .file_name()
        .context("Missing filename")?
        .to_string_lossy()
        .into_owned();
    let mut next = config.clone();
    if repository.trim().is_empty() {
        next.repositories.remove(&key);
    } else {
        next.repositories
            .insert(key, normalize_repository(repository)?);
    }
    next.save()?;
    *config = next;
    Ok(())
}

pub fn normalize_repository(value: &str) -> Result<String> {
    let v = value
        .trim()
        .trim_end_matches('/')
        .strip_suffix(".git")
        .unwrap_or(value.trim().trim_end_matches('/'));
    let v = v.strip_prefix("https://github.com/").unwrap_or(v);
    let parts: Vec<_> = v.split('/').collect();
    ensure!(
        parts.len() == 2
            && parts.iter().all(|p| !p.is_empty()
                && *p != "."
                && *p != ".."
                && p.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')),
        "Enter a GitHub owner/repository or repository HTTPS URL"
    );
    Ok(v.into())
}
fn repository_from_update_info(value: &str) -> Option<String> {
    let p: Vec<_> = value.split('|').collect();
    if p.len() >= 5 && p[0] == "gh-releases-zsync" {
        normalize_repository(&format!("{}/{}", p[1], p[2])).ok()
    } else {
        None
    }
}

pub fn launch(app: &AppInfo) -> Result<()> {
    validate_appimage(&app.path)?;
    ensure!(
        app.architecture == std::env::consts::ARCH
            || (app.architecture == "x86_64" && std::env::consts::ARCH == "x86_64"),
        "Application architecture does not match this machine"
    );
    let mut permissions = fs::metadata(&app.path)?.permissions();
    permissions.set_mode(permissions.mode() | 0o100);
    fs::set_permissions(&app.path, permissions)?;
    Command::new(&app.path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to launch application")?;
    Ok(())
}

pub fn import_app(config: &Config, source: &Path) -> Result<AppInfo> {
    validate_appimage(source)?;
    fs::create_dir_all(&config.applications_dir)?;
    let _lock = lock_library(config)?;
    let filename = source
        .file_name()
        .context("Missing source filename")?
        .to_string_lossy();
    ensure!(
        !filename.chars().any(char::is_control),
        "Filename contains control characters"
    );
    let filename = if source
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("AppImage"))
    {
        filename.into_owned()
    } else {
        format!("{filename}.AppImage")
    };
    let incoming = inspect(config, source)?;
    let mut matches: Vec<_> = scan(config)?
        .into_iter()
        .filter(|app| same_application(app, &incoming))
        .collect();
    matches.sort_by_key(|app| {
        (
            config.last_selected_path.as_ref() != Some(&app.path),
            app.repository.is_none(),
            app.path.clone(),
        )
    });
    let source_path = source.canonicalize()?;
    let in_library = source
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("appimage"))
        && source_path.parent() == Some(config.applications_dir.canonicalize()?.as_path());
    // Keep the installed path stable so launchers, selection and repository settings survive.
    let destination = if in_library {
        source_path.clone()
    } else {
        matches
            .first()
            .map(|app| app.path.clone())
            .unwrap_or_else(|| config.applications_dir.join(filename))
    };
    if fs::symlink_metadata(&destination).is_ok() {
        ensure_managed_path(config, &destination)?;
        ensure!(
            matches.iter().any(|a| a.path == destination),
            "Destination belongs to a different application"
        );
    }
    let mut temp = tempfile::NamedTempFile::new_in(&config.applications_dir)?;
    std::io::copy(&mut File::open(source)?, &mut temp)?;
    temp.as_file()
        .set_permissions(fs::Permissions::from_mode(0o755))?;
    temp.as_file().sync_all()?;
    validate_appimage(temp.path())?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut File::open(temp.path())?, &mut hasher)?;
    let digest = hex::encode(hasher.finalize());
    let mut moved = Vec::new();
    let mut replaced = false;
    let launcher = applications_desktop_dir()?.join(format!(
        "appimage-mannger-{}.desktop",
        stable_id(&destination.file_name().unwrap().to_string_lossy())
    ));
    let old_launcher = fs::read(&launcher).ok();
    let result = (|| -> Result<AppInfo> {
        for app in &matches {
            if app.path.canonicalize()? == source_path {
                continue;
            }
            ensure_managed_path(config, &app.path)?;
            // The destination's launchers continue pointing to the atomic replacement.
            if app.path != destination {
                for desktop in find_desktops(&app.path)?.into_iter().filter_map(|d| d.path) {
                    let trash = move_to_trash(&desktop)?;
                    moved.push((desktop, trash));
                }
            }
            let trash = trash_file(&app.path, app.path == destination)?;
            moved.push((app.path.clone(), trash));
        }
        if source_path != destination {
            if matches.iter().any(|app| app.path == destination) {
                temp.persist(&destination).map_err(|e| e.error)?;
            } else {
                temp.persist_noclobber(&destination).map_err(|e| e.error)?;
            }
            replaced = true;
        }
        let mut app = inspect(config, &destination)?;
        app.version = incoming.version.clone();
        integrate(&app)?;
        save_installed_version(&app, &digest)?;
        Ok(app)
    })();
    if result.is_err() {
        if replaced && !moved.iter().any(|(original, _)| original == &destination) {
            let _ = fs::remove_file(&destination);
        }
        for (original, trash) in moved.iter().rev() {
            if let Err(error) = restore_from_trash(original, trash) {
                log::error!(
                    "Import rollback failed for {}: {error:#}",
                    original.display()
                );
            }
        }
        if let Some(bytes) = old_launcher {
            let _ = fs::write(&launcher, bytes);
        } else {
            let _ = fs::remove_file(&launcher);
        }
        refresh_desktop_database();
    }
    result
}
pub(crate) fn lock_library(config: &Config) -> Result<File> {
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .custom_flags(0x20000) // Linux O_NOFOLLOW: never lock an attacker-supplied symlink.
        .mode(0o600)
        .open(config.applications_dir.join(".appimage-mannger.lock"))?;
    lock.lock()?;
    Ok(lock)
}

fn application_stem(path: &Path) -> String {
    let stem = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    // Version, architecture and build hashes are packaging details, not identity.
    let suffix = regex::Regex::new(
        r"(?i)(?:[-_ ]v?\d+(?:\.\d+)+|[-_ ](?:linux|x86_64|amd64|aarch64|arm64|x64)(?:[-_ ]|$))",
    )
    .unwrap();
    let end = suffix.find(&stem).map_or(stem.len(), |m| m.start());
    stem[..end]
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}
fn same_application(a: &AppInfo, b: &AppInfo) -> bool {
    if a.architecture != b.architecture {
        return false;
    }
    let a_meta = crate::metadata::embedded_desktop(&a.path).unwrap_or_default();
    let b_meta = crate::metadata::embedded_desktop(&b.path).unwrap_or_default();
    if let (Some(a), Some(b)) = (a_meta.get("X-AppImage-Id"), b_meta.get("X-AppImage-Id")) {
        return !a.is_empty() && a == b;
    }
    if let (Some(a), Some(b)) = (&a.repository, &b.repository)
        && a != b
    {
        return false;
    }
    let stem = application_stem(&a.path);
    !stem.is_empty() && stem == application_stem(&b.path)
        || a_meta
            .get("Name")
            .is_some_and(|name| !name.is_empty() && b_meta.get("Name") == Some(name))
}

/// Install our own desktop entry without changing entries owned by other tools.
pub fn integrate_app(config: &Config, app: &AppInfo) -> Result<()> {
    ensure_managed_path(config, &app.path)?;
    repair_registration()?;
    integrate(app)
}

pub fn self_icon_path() -> Result<PathBuf> {
    let dir = dirs::cache_dir()
        .context("Cannot determine cache directory")?
        .join("appimage-mannger/branding");
    fs::create_dir_all(&dir)?;
    let path = dir.join("appimage-mannger.png");
    let bytes = include_bytes!("../packaging/appimage-mannger.png");
    if fs::read(&path).ok().as_deref() != Some(bytes) {
        let mut temp = tempfile::NamedTempFile::new_in(&dir)?;
        temp.write_all(bytes)?;
        temp.persist(&path).map_err(|e| e.error)?;
    }
    Ok(path)
}

/// Register Open With support for installed and portable launches alike.
pub fn repair_registration() -> Result<()> {
    let dir = applications_desktop_dir()?;
    fs::create_dir_all(&dir)?;
    let engine = std::env::var_os("APPIMAGE")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute() && p.is_file())
        .unwrap_or(std::env::current_exe()?);
    let content = include_str!("../packaging/appimage-mannger.desktop")
        .replace(
            "Exec=appimage-mannger --import %F",
            &format!("Exec={} --import %F", desktop_exec(&engine)),
        )
        .replace(
            "TryExec=appimage-mannger",
            &format!("TryExec={}", desktop_value(&engine.to_string_lossy())),
        )
        .replace(
            "Icon=appimage-mannger",
            &format!(
                "Icon={}",
                desktop_value(&self_icon_path()?.to_string_lossy())
            ),
        );
    for (name, text) in [
        ("appimage-mannger.desktop", content.clone()),
        (
            "io.github.xingwangzhe.AppimageMannger.desktop",
            format!(
                "{}\nNoDisplay=true\n",
                content
                    .lines()
                    .filter(|line| !line.starts_with("MimeType="))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        ),
    ] {
        let path = dir.join(name);
        if fs::read_to_string(&path).ok().as_deref() != Some(&text) {
            let mut temp = tempfile::NamedTempFile::new_in(&dir)?;
            temp.write_all(text.as_bytes())?;
            temp.persist(path).map_err(|e| e.error)?;
        }
    }
    let mime = user_data_dir()?.join("mime");
    fs::create_dir_all(mime.join("packages"))?;
    let path = mime.join("packages/appimage-mannger.xml");
    let xml = include_str!("../packaging/appimage-mannger.xml");
    if fs::read_to_string(&path).ok().as_deref() != Some(xml) {
        fs::write(path, xml)?;
        let _ = Command::new("update-mime-database").arg(mime).status();
    }
    refresh_desktop_database();
    Ok(())
}

pub fn integrate(app: &AppInfo) -> Result<()> {
    let dir = applications_desktop_dir()?;
    fs::create_dir_all(&dir)?;
    ensure!(
        !app.path.to_string_lossy().chars().any(char::is_control),
        "Application path contains control characters"
    );
    let path = dir.join(format!("appimage-mannger-{}.desktop", app.id));
    if find_desktops(&app.path)?
        .iter()
        .any(|d| d.path.as_ref() != Some(&path))
    {
        return Ok(());
    }
    let icon = app
        .icon_path
        .as_ref()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "application-x-executable".into());
    let content = format!(
        "[Desktop Entry]\nType=Application\nName={}\nComment={}\nExec={}\nTryExec={}\nIcon={}\nTerminal=false\nCategories=Utility;\nX-AppimageMannger-Managed=true\nX-AppImage-Version={}\n",
        desktop_value(&app.name),
        desktop_value(&app.description),
        desktop_exec(&app.path),
        desktop_value(&app.path.to_string_lossy()),
        desktop_value(&icon),
        desktop_value(&app.version)
    );
    let mut temp = tempfile::NamedTempFile::new_in(&dir)?;
    temp.write_all(content.as_bytes())?;
    temp.persist(path).map_err(|e| e.error)?;
    refresh_desktop_database();
    Ok(())
}
fn desktop_value(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "")
}
fn desktop_exec(path: &Path) -> String {
    let quoted = path
        .to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('`', "\\`")
        .replace('$', "\\$")
        .replace('%', "%%");
    // Desktop strings are unescaped once before Exec argument unquoting.
    format!("\"{}\"", desktop_value(&quoted))
}
fn refresh_desktop_database() {
    if let Ok(dir) = applications_desktop_dir() {
        let _ = Command::new("update-desktop-database")
            .arg(dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

pub fn uninstall(config: &Config, app: &AppInfo) -> Result<PathBuf> {
    let _lock = lock_library(config)?;
    ensure_managed_path(config, &app.path)?;
    validate_appimage(&app.path)?;
    let mut paths: Vec<PathBuf> = find_desktops(&app.path)?
        .into_iter()
        .filter_map(|d| d.path)
        .collect();
    paths.push(app.path.clone());
    let mut moved: Vec<(PathBuf, PathBuf)> = Vec::new();
    for original in paths {
        match move_to_trash(&original) {
            Ok(destination) => moved.push((original, destination)),
            Err(error) => {
                for (original, destination) in moved.iter().rev() {
                    let _ = restore_from_trash(original, destination);
                }
                return Err(error.context(
                    "Uninstall failed; restoration of previously moved files was attempted",
                ));
            }
        }
    }
    refresh_desktop_database();
    Ok(moved.last().unwrap().1.clone())
}
fn restore_from_trash(original: &Path, trash: &Path) -> Result<()> {
    if let Err(error) = fs::rename(trash, original) {
        if error.raw_os_error() != Some(18) {
            return Err(error.into());
        }
        let mut temp = tempfile::NamedTempFile::new_in(
            original.parent().context("Missing original directory")?,
        )?;
        std::io::copy(&mut File::open(trash)?, &mut temp)?;
        temp.as_file()
            .set_permissions(fs::metadata(trash)?.permissions())?;
        temp.as_file().sync_all()?;
        temp.persist(original).map_err(|e| e.error)?;
        fs::remove_file(trash)?;
    }
    remove_trash_info(trash)
}
fn remove_trash_info(destination: &Path) -> Result<()> {
    let name = destination
        .file_name()
        .context("Missing Trash filename")?
        .to_string_lossy();
    fs::remove_file(
        destination
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("info")
            .join(format!("{name}.trashinfo")),
    )?;
    Ok(())
}
fn move_to_trash(path: &Path) -> Result<PathBuf> {
    trash_file(path, false)
}
fn trash_file(path: &Path, keep_original: bool) -> Result<PathBuf> {
    let trash = user_data_dir()?.join("Trash");
    fs::create_dir_all(trash.join("files"))?;
    fs::create_dir_all(trash.join("info"))?;
    let filename = path
        .file_name()
        .context("Missing filename")?
        .to_string_lossy();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let unique = format!("{filename}.{stamp}");
    let dest = trash.join("files").join(&unique);
    let info = trash.join("info").join(format!("{unique}.trashinfo"));
    let content = format!(
        "[Trash Info]\nPath={}\nDeletionDate={}\n",
        percent_encode_path(&path.canonicalize()?),
        trash_date()
    );
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&info)?;
    f.write_all(content.as_bytes())?;
    f.sync_all()?;
    if keep_original {
        let copy = (|| -> Result<()> {
            let mut out = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(fs::metadata(path)?.permissions().mode())
                .open(&dest)?;
            std::io::copy(&mut File::open(path)?, &mut out)?;
            out.sync_all()?;
            Ok(())
        })();
        if let Err(error) = copy {
            let _ = fs::remove_file(&info);
            let _ = fs::remove_file(&dest);
            return Err(error);
        }
        return Ok(dest);
    }
    if let Err(error) = fs::rename(path, &dest) {
        if error.raw_os_error() != Some(18) {
            let _ = fs::remove_file(info);
            return Err(error.into());
        }
        let copied = (|| -> Result<()> {
            let mut out = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o700)
                .open(&dest)?;
            std::io::copy(&mut File::open(path)?, &mut out)?;
            out.sync_all()?;
            fs::remove_file(path)?;
            Ok(())
        })();
        if let Err(error) = copied {
            let _ = fs::remove_file(&dest);
            let _ = fs::remove_file(info);
            return Err(error);
        }
    }
    Ok(dest)
}

pub(crate) fn ensure_managed_path(config: &Config, path: &Path) -> Result<()> {
    ensure!(
        !fs::symlink_metadata(path)?.file_type().is_symlink(),
        "Refusing to modify a symbolic link"
    );
    let root = config.applications_dir.canonicalize()?;
    let canonical = path.canonicalize()?;
    ensure!(
        canonical.parent() == Some(root.as_path()),
        "Application is outside the configured library"
    );
    ensure!(
        fs::metadata(&canonical)?.is_file(),
        "Application is not a regular file"
    );
    Ok(())
}

pub(crate) fn validate_appimage(path: &Path) -> Result<[u8; 64]> {
    ensure!(
        !fs::symlink_metadata(path)?.file_type().is_symlink(),
        "Refusing a symbolic link as an AppImage"
    );
    ensure!(
        fs::metadata(path)?.is_file(),
        "AppImage is not a regular file"
    );
    let mut header = [0u8; 64];
    File::open(path)?
        .read_exact(&mut header)
        .context("File is too small to be an AppImage")?;
    ensure!(
        &header[..4] == b"\x7fELF" && &header[8..10] == b"AI" && [1, 2].contains(&header[10]),
        "File is not a valid ELF AppImage"
    );
    ensure!(
        header[5] == 1 && [1, 2].contains(&header[4]),
        "Unsupported ELF format"
    );
    ensure!(fs::metadata(path)?.len() > 64, "Incomplete AppImage file");
    Ok(header)
}
fn stable_id(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))[..20].into()
}
fn filename_version(name: &str) -> Option<String> {
    name.split(['-', '_', ' ']).find_map(|part| {
        let v = part.trim_start_matches('v');
        if semver::Version::parse(v).is_ok() {
            Some(v.to_string())
        } else {
            None
        }
    })
}
#[derive(Clone, Default)]
struct DesktopMetadata {
    path: Option<PathBuf>,
    values: BTreeMap<String, String>,
}
fn find_desktop(path: &Path) -> Result<DesktopMetadata> {
    Ok(find_desktops(path)?.into_iter().next().unwrap_or_default())
}
fn find_desktops(path: &Path) -> Result<Vec<DesktopMetadata>> {
    let target = DesktopTarget::new(path);
    Ok(read_desktops()?
        .into_iter()
        .filter(|desktop| target.matches(desktop))
        .collect())
}

fn read_desktops() -> Result<Vec<DesktopMetadata>> {
    let dir = applications_desktop_dir()?;
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut found = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() || entry.path().extension().is_none_or(|e| e != "desktop")
        {
            continue;
        }
        if entry.metadata()?.len() > 128 * 1024 {
            continue;
        }
        let Ok(s) = fs::read_to_string(entry.path()) else {
            continue;
        };
        let mut values = BTreeMap::new();
        let mut in_entry = false;
        for line in s.lines() {
            if line.starts_with('[') {
                in_entry = line == "[Desktop Entry]";
                continue;
            }
            if in_entry && let Some((k, v)) = line.split_once('=') {
                values.insert(k.to_string(), v.to_string());
            }
        }
        found.push(DesktopMetadata {
            path: Some(entry.path()),
            values,
        });
    }
    Ok(found)
}
struct DesktopTarget {
    plain: String,
    quoted: String,
}
impl DesktopTarget {
    fn new(path: &Path) -> Self {
        Self {
            plain: path.to_string_lossy().into_owned(),
            quoted: desktop_exec(path),
        }
    }

    fn matches(&self, desktop: &DesktopMetadata) -> bool {
        desktop
            .values
            .get("TryExec")
            .is_some_and(|v| v.trim_matches('"') == self.plain)
            || desktop.values.get("Exec").is_some_and(|value| {
                [&self.quoted, &self.plain].iter().any(|prefix| {
                    value.strip_prefix(prefix.as_str()).is_some_and(|rest| {
                        rest.is_empty()
                            || rest.starts_with(' ')
                                && rest
                                    .split_whitespace()
                                    .all(|a| ["%f", "%F", "%u", "%U"].contains(&a))
                    })
                })
            })
    }
}

/// Read a bounded ELF section; no subprocess and no AppImage execution.
pub fn read_update_info(path: &Path) -> Result<Option<String>> {
    let header = validate_appimage(path)?;
    let mut file = File::open(path)?;
    let length = file.metadata()?.len();
    let u16_at = |b: &[u8], i| u16::from_le_bytes([b[i], b[i + 1]]) as u64;
    let u32_at = |b: &[u8], i| u32::from_le_bytes(b[i..i + 4].try_into().unwrap()) as u64;
    let u64_at = |b: &[u8], i| u64::from_le_bytes(b[i..i + 8].try_into().unwrap());
    let is64 = header[4] == 2;
    let (offset, entry_size, count, string_index) = if is64 {
        (
            u64_at(&header, 40),
            u16_at(&header, 58),
            u16_at(&header, 60),
            u16_at(&header, 62),
        )
    } else {
        (
            u32_at(&header, 32),
            u16_at(&header, 46),
            u16_at(&header, 48),
            u16_at(&header, 50),
        )
    };
    if offset == 0 || count == 0 {
        return Ok(None);
    }
    ensure!(
        entry_size >= if is64 { 64 } else { 40 }
            && entry_size <= 256
            && count <= 4096
            && string_index < count,
        "无效的 ELF 节表"
    );
    ensure!(
        offset
            .checked_add(entry_size * count)
            .is_some_and(|v| v <= length),
        "ELF 节表超出文件"
    );
    let mut table = vec![0u8; (entry_size * count) as usize];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut table)?;
    let section_range = |index: u64| {
        let s = &table[(index * entry_size) as usize..((index + 1) * entry_size) as usize];
        if is64 {
            (u64_at(s, 24), u64_at(s, 32))
        } else {
            (u32_at(s, 16), u32_at(s, 20))
        }
    };
    let (strings_offset, strings_size) = section_range(string_index);
    ensure!(
        strings_size <= 1024 * 1024
            && strings_offset
                .checked_add(strings_size)
                .is_some_and(|v| v <= length),
        "ELF 节名表无效"
    );
    let mut strings = vec![0; strings_size as usize];
    file.seek(SeekFrom::Start(strings_offset))?;
    file.read_exact(&mut strings)?;
    for i in 0..count {
        let s = &table[(i * entry_size) as usize..];
        let name_offset = u32_at(s, 0) as usize;
        if name_offset >= strings.len() {
            continue;
        }
        let name = &strings[name_offset..];
        let end = name.iter().position(|c| *c == 0).unwrap_or(name.len());
        if &name[..end] == b".upd_info" {
            let (start, size) = section_range(i);
            ensure!(
                size <= 64 * 1024 && start.checked_add(size).is_some_and(|v| v <= length),
                "更新信息节无效"
            );
            let mut data = vec![0; size as usize];
            file.seek(SeekFrom::Start(start))?;
            file.read_exact(&mut data)?;
            let s = String::from_utf8_lossy(&data)
                .trim_matches('\0')
                .trim()
                .to_string();
            return Ok(if s.is_empty() { None } else { Some(s) });
        }
    }
    Ok(None)
}
pub(crate) fn user_data_dir() -> Result<PathBuf> {
    #[cfg(test)]
    if let Some(p) = TEST_DATA_DIR.with(|p| p.borrow().clone()) {
        return Ok(p);
    }
    dirs::data_dir().context("无法确定用户数据目录")
}
#[cfg(test)]
thread_local! {static TEST_DATA_DIR:std::cell::RefCell<Option<PathBuf>>=const{std::cell::RefCell::new(None)};}
#[cfg(test)]
pub(crate) fn with_test_data<T>(path: &Path, f: impl FnOnce() -> T) -> T {
    TEST_DATA_DIR.with(|p| *p.borrow_mut() = Some(path.to_path_buf()));
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            TEST_DATA_DIR.with(|p| *p.borrow_mut() = None);
        }
    }
    let _reset = Reset;
    f()
}
#[derive(Serialize, Deserialize)]
struct InstalledVersion {
    path: PathBuf,
    version: String,
    sha256: String,
    size: u64,
    modified_nanos: u128,
}
fn version_record_path(id: &str) -> Result<PathBuf> {
    Ok(user_data_dir()?
        .join("appimage-mannger/installed")
        .join(format!("{id}.json")))
}
fn modified_nanos(path: &Path) -> Result<u128> {
    Ok(fs::metadata(path)?
        .modified()?
        .duration_since(UNIX_EPOCH)?
        .as_nanos())
}
fn installed_version(path: &Path, id: &str) -> Result<Option<String>> {
    let record = version_record_path(id)?;
    if !record.exists() {
        return Ok(None);
    }
    let r: InstalledVersion = serde_json::from_slice(&fs::read(record)?)?;
    Ok(
        if r.path == path
            && r.size == fs::metadata(path)?.len()
            && r.modified_nanos == modified_nanos(path)?
        {
            Some(r.version)
        } else {
            None
        },
    )
}
pub(crate) fn save_installed_version(app: &AppInfo, sha256: &str) -> Result<()> {
    let p = version_record_path(&app.id)?;
    fs::create_dir_all(p.parent().unwrap())?;
    let r = InstalledVersion {
        path: app.path.clone(),
        version: app.version.clone(),
        sha256: sha256.into(),
        size: fs::metadata(&app.path)?.len(),
        modified_nanos: modified_nanos(&app.path)?,
    };
    let mut temp = tempfile::NamedTempFile::new_in(p.parent().unwrap())?;
    temp.write_all(&serde_json::to_vec_pretty(&r)?)?;
    temp.as_file().sync_all()?;
    temp.persist(p).map_err(|e| e.error)?;
    Ok(())
}
fn percent_encode_path(path: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str()
        .as_bytes()
        .iter()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"/-_.~".contains(b) {
                (*b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn trash_date() -> String {
    let out = Command::new("date").arg("+%Y-%m-%dT%H:%M:%S").output();
    out.ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "1970-01-01T00:00:00".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn fixture(path: &Path) {
        let mut b = vec![0u8; 128];
        b[..4].copy_from_slice(b"\x7fELF");
        b[4] = 2;
        b[5] = 1;
        b[8..11].copy_from_slice(b"AI\x02");
        b[18] = 62;
        fs::write(path, b).unwrap();
    }

    #[test]
    fn preferences_survive_restart_and_legacy_configuration() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("config.json");
        fs::write(
            &path,
            serde_json::json!({"applications_dir":temp.path(),"repositories":{}}).to_string(),
        )
        .unwrap();
        let mut config = Config::load_from(&path).unwrap();
        assert_eq!(config.window_style, WindowStyle::Gpui);
        assert_eq!(config.appearance, Appearance::Light);
        config.appearance = Appearance::Dark;
        config.language = "de".into();
        config.window_style = WindowStyle::Gnome;
        config.window_width = 1040.0;
        config.window_height = 720.0;
        config.maximized = true;
        config.sidebar_fraction = 0.42;
        config.last_selected_path = Some(temp.path().join("Tool.AppImage"));
        config.directory_draft = Some("/tmp/a draft".into());
        config
            .repository_drafts
            .insert("Tool.AppImage".into(), "owner/project".into());
        config.save_to(&path).unwrap();
        let reloaded = Config::load_from(&path).unwrap();
        assert_eq!(reloaded.appearance, Appearance::Dark);
        assert_eq!(reloaded.language, "de");
        assert_eq!(reloaded.window_style, WindowStyle::Gnome);
        assert_eq!(reloaded.window_width, 1040.0);
        assert_eq!(reloaded.window_height, 720.0);
        assert!(reloaded.maximized);
        assert_eq!(reloaded.sidebar_fraction, 0.42);
        assert_eq!(reloaded.last_selected_path, config.last_selected_path);
        assert_eq!(reloaded.directory_draft, config.directory_draft);
        assert_eq!(reloaded.repository_drafts, config.repository_drafts);
        for style in [WindowStyle::Gpui, WindowStyle::Qt, WindowStyle::Gnome] {
            config.window_style = style;
            config.save_to(&path).unwrap();
            assert_eq!(Config::load_from(&path).unwrap().window_style, style);
        }
    }
    #[test]
    fn scans_only_valid_images_and_ignores_symlinks() {
        let d = tempfile::tempdir().unwrap();
        fixture(&d.path().join("Example-1.2.3.AppImage"));
        fs::write(d.path().join("Bad.AppImage"), "not an image").unwrap();
        std::os::unix::fs::symlink(
            d.path().join("Example-1.2.3.AppImage"),
            d.path().join("Link.AppImage"),
        )
        .unwrap();
        let c = Config {
            applications_dir: d.path().into(),
            repositories: BTreeMap::new(),
            ..Config::default()
        };
        let list = scan(&c).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].version, "1.2.3");
        assert_eq!(list[0].name, "Example");
    }
    #[test]
    fn rejects_outside_managed_root() {
        let d = tempfile::tempdir().unwrap();
        let other = tempfile::tempdir().unwrap();
        let file = other.path().join("x.AppImage");
        fixture(&file);
        let c = Config {
            applications_dir: d.path().into(),
            repositories: BTreeMap::new(),
            ..Config::default()
        };
        assert!(ensure_managed_path(&c, &file).is_err());
        assert!(file.exists());
    }
    #[test]
    fn scan_snapshot_preserves_launcher_matching_and_refreshes_changes() {
        let library = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        with_test_data(data.path(), || {
            let config = Config {
                applications_dir: library.path().into(),
                ..Config::default()
            };
            let desktop_dir = data.path().join("applications");
            fs::create_dir_all(&desktop_dir).unwrap();
            for (index, filename) in ["a %f $x.AppImage", "b space.AppImage", "c.AppImage"]
                .iter()
                .enumerate()
            {
                let path = library.path().join(filename);
                fixture(&path);
                let command = match index {
                    0 => format!("Exec={} %F %u", desktop_exec(&path)),
                    1 => format!("TryExec=\"{}\"", path.display()),
                    _ => format!("Exec={}", path.display()),
                };
                fs::write(
                    desktop_dir.join(format!("{index}.desktop")),
                    format!("[Desktop Entry]\nName=Tool {index}\n{command}\n"),
                )
                .unwrap();
            }
            let list = scan(&config).unwrap();
            assert_eq!(list.len(), 3);
            for (index, app) in list.iter().enumerate() {
                assert_eq!(app.name, format!("Tool {index}"));
                assert_eq!(
                    serde_json::to_value(app).unwrap(),
                    serde_json::to_value(inspect(&config, &app.path).unwrap()).unwrap()
                );
            }
            let path = library.path().join("c.AppImage");
            for command in [
                format!("{}extra %F", path.display()),
                format!("{} --unsafe-option", desktop_exec(&path)),
            ] {
                fs::write(
                    desktop_dir.join("2.desktop"),
                    format!("[Desktop Entry]\nName=Wrong application\nExec={command}\n"),
                )
                .unwrap();
                let list = scan(&config).unwrap();
                let app = list.iter().find(|app| app.path == path).unwrap();
                assert_eq!(app.name, "c");
                assert!(app.desktop_file.is_none());
            }
        });
    }
    #[test]
    fn normalizes_only_valid_github_repositories() {
        assert_eq!(
            normalize_repository("https://github.com/owner/repo.git").unwrap(),
            "owner/repo"
        );
        for bad in [
            "https://evil.test/x/y",
            "../repo",
            "owner/repo/releases",
            "owner/repo?token=abc",
            "",
        ] {
            assert!(normalize_repository(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn reads_empty_update_info_without_execution() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("x.AppImage");
        fixture(&p);
        assert_eq!(read_update_info(&p).unwrap(), None);
    }
    #[test]
    fn publisher_build_hash_does_not_override_comparable_filename_version() {
        let d = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        with_test_data(data.path(), || {
            let p = d.path().join("Tool-2.1.10.AppImage");
            fixture(&p);
            let desktop = applications_desktop_dir().unwrap();
            fs::create_dir_all(&desktop).unwrap();
            fs::write(
                desktop.join("tool.desktop"),
                format!(
                    "[Desktop Entry]\nName=Tool\nTryExec={}\nX-AppImage-Version=308113a\n",
                    p.display()
                ),
            )
            .unwrap();
            let config = Config {
                applications_dir: d.path().into(),
                ..Config::default()
            };
            assert_eq!(inspect(&config, &p).unwrap().version, "2.1.10");
        });
    }
    #[test]
    fn reads_bounded_elf_update_section_and_discovers_repository() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("SectionFixture-1.2.3.AppImage");
        fixture(&p);
        let mut bytes = fs::read(&p).unwrap();
        bytes.resize(640, 0);
        bytes[40..48].copy_from_slice(&128u64.to_le_bytes());
        bytes[58..60].copy_from_slice(&64u16.to_le_bytes());
        bytes[60..62].copy_from_slice(&3u16.to_le_bytes());
        bytes[62..64].copy_from_slice(&1u16.to_le_bytes());
        let names = b"\0.shstrtab\0.upd_info\0";
        bytes[320..320 + names.len()].copy_from_slice(names);
        bytes[192..196].copy_from_slice(&1u32.to_le_bytes());
        bytes[216..224].copy_from_slice(&320u64.to_le_bytes());
        bytes[224..232].copy_from_slice(&(names.len() as u64).to_le_bytes());
        bytes[256..260].copy_from_slice(&11u32.to_le_bytes());
        bytes[280..288].copy_from_slice(&400u64.to_le_bytes());
        bytes[288..296].copy_from_slice(&128u64.to_le_bytes());
        let info = b"gh-releases-zsync|owner|repository|latest|Tool-*.AppImage.zsync";
        bytes[400..400 + info.len()].copy_from_slice(info);
        fs::write(&p, &bytes).unwrap();
        let config = Config {
            applications_dir: d.path().into(),
            repositories: BTreeMap::new(),
            ..Config::default()
        };
        assert_eq!(
            inspect(&config, &p).unwrap().repository.as_deref(),
            Some("owner/repository")
        );
        bytes[288..296].copy_from_slice(&u64::MAX.to_le_bytes());
        fs::write(&p, bytes).unwrap();
        assert!(read_update_info(&p).is_err());
    }
    #[test]
    fn desktop_exec_escapes_field_codes_and_specials() {
        let s = desktop_exec(Path::new("/tmp/a %f $x`\".AppImage"));
        assert!(s.contains("%%f"));
        assert!(s.contains("\\$"));
        assert!(s.contains("\\`"));
    }

    #[test]
    fn imports_other_extensions_with_image_suffix_and_keeps_source() {
        let d = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        with_test_data(data.path(), || {
            let source = d.path().join("Tool-1.2.3.bin");
            fixture(&source);
            let config = Config {
                applications_dir: d.path().join("apps"),
                repositories: BTreeMap::new(),
                ..Config::default()
            };
            let app = import_app(&config, &source).unwrap();
            assert!(source.exists());
            assert!(app.path.to_string_lossy().ends_with(".bin.AppImage"));
            assert_eq!(scan(&config).unwrap().len(), 1);
        });
    }
    #[test]
    fn rejects_filename_control_characters() {
        let d = tempfile::tempdir().unwrap();
        let source = d.path().join("Tool\nExec=bad.AppImage");
        fixture(&source);
        let config = Config {
            applications_dir: d.path().join("apps"),
            repositories: BTreeMap::new(),
            ..Config::default()
        };
        assert!(import_app(&config, &source).is_err());
        assert!(source.exists());
    }
    #[test]
    fn import_replaces_versions_preserves_source_settings_and_restorable_backups() {
        let d = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        with_test_data(data.path(), || {
            let library = d.path().join("apps");
            fs::create_dir(&library).unwrap();
            let old = library.join("Tool-1.2.3-linux-x86_64_deadbeef.AppImage");
            let duplicate = library.join("Tool-1.3.0-x86_64.AppImage");
            let other = library.join("OtherTool-1.0.0.AppImage");
            for path in [&old, &duplicate, &other] {
                fixture(path);
            }
            let mut config = Config {
                applications_dir: library,
                ..Config::default()
            };
            config.repositories.insert(
                old.file_name().unwrap().to_string_lossy().into_owned(),
                "owner/tool".into(),
            );
            integrate(&inspect(&config, &old).unwrap()).unwrap();
            integrate(&inspect(&config, &duplicate).unwrap()).unwrap();
            let source = d.path().join("Tool-2.0.0-x86_64.AppImage");
            fixture(&source);
            let mut new = fs::read(&source).unwrap();
            new[100] = 42;
            fs::write(&source, &new).unwrap();
            let imported = import_app(&config, &source).unwrap();
            assert_eq!(imported.version, "2.0.0");
            assert_eq!(inspect(&config, &imported.path).unwrap().version, "2.0.0");
            assert_eq!(scan(&config).unwrap().len(), 2);
            assert_eq!(fs::read(&imported.path).unwrap(), new);
            assert_eq!(fs::read(&source).unwrap(), new);
            assert!(other.exists());
            assert_eq!(
                fs::read_dir(data.path().join("Trash/info"))
                    .unwrap()
                    .count(),
                3
            );
            assert_eq!(find_desktops(&imported.path).unwrap().len(), 1);
            assert_eq!(imported.repository.as_deref(), Some("owner/tool"));
            // Re-importing the same filename also replaces instead of failing.
            import_app(&config, &source).unwrap();
            assert_eq!(scan(&config).unwrap().len(), 2);
        });
    }
    #[test]
    fn failed_import_restores_previous_binary_and_launcher() {
        let d = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        with_test_data(data.path(), || {
            let config = Config {
                applications_dir: d.path().join("apps"),
                ..Config::default()
            };
            fs::create_dir(&config.applications_dir).unwrap();
            let old = config.applications_dir.join("Tool-1.0.0.AppImage");
            fixture(&old);
            let app = inspect(&config, &old).unwrap();
            integrate(&app).unwrap();
            let launcher = app
                .desktop_file
                .clone()
                .unwrap_or_else(|| find_desktop(&old).unwrap().path.unwrap());
            let original_launcher = fs::read(&launcher).unwrap();
            // Force version-record persistence to fail after the replacement and launcher write.
            fs::create_dir_all(data.path().join("appimage-mannger")).unwrap();
            fs::write(data.path().join("appimage-mannger/installed"), "blocked").unwrap();
            let source = d.path().join("Tool-2.0.0.AppImage");
            fixture(&source);
            let mut bytes = fs::read(&source).unwrap();
            bytes[100] = 77;
            fs::write(&source, bytes).unwrap();
            assert!(import_app(&config, &source).is_err());
            assert_eq!(fs::read(&old).unwrap()[100], 0);
            assert_eq!(fs::read(&launcher).unwrap(), original_launcher);
            assert!(source.exists());
        });
    }
    #[test]
    fn identity_does_not_merge_other_apps_or_architectures() {
        let d = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        with_test_data(data.path(), || {
            let a = d.path().join("Tool-1.0.0.AppImage");
            let b = d.path().join("Toolbox-2.0.0.AppImage");
            fixture(&a);
            fixture(&b);
            let config = Config {
                applications_dir: d.path().into(),
                ..Config::default()
            };
            let a = inspect(&config, &a).unwrap();
            let mut b = inspect(&config, &b).unwrap();
            assert!(!same_application(&a, &b));
            b.path = d.path().join("Tool-2.0.0.AppImage");
            b.architecture = "aarch64".into();
            assert!(!same_application(&a, &b));
        });
    }
    #[test]
    fn reuses_existing_launcher_and_trashes_all_launchers_with_restore_metadata() {
        let d = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        with_test_data(data.path(), || {
            let source = d.path().join("Tool-1.2.3.AppImage");
            fixture(&source);
            let config = Config {
                applications_dir: d.path().into(),
                repositories: BTreeMap::new(),
                ..Config::default()
            };
            let desktop = applications_desktop_dir().unwrap();
            fs::create_dir_all(&desktop).unwrap();
            let legacy = desktop.join("legacy.desktop");
            fs::write(
                &legacy,
                format!(
                    "[Desktop Entry]\nName=Tool\nExec={} %U\nIcon=old-icon\n",
                    source.display()
                ),
            )
            .unwrap();
            let app = inspect(&config, &source).unwrap();
            integrate(&app).unwrap();
            assert_eq!(fs::read_dir(&desktop).unwrap().count(), 1);
            let second = desktop.join("duplicate.desktop");
            fs::write(
                &second,
                format!("[Desktop Entry]\nName=Tool\nTryExec={}\n", source.display()),
            )
            .unwrap();
            let destination = uninstall(&config, &app).unwrap();
            assert!(!source.exists());
            assert!(destination.exists());
            assert!(!legacy.exists());
            assert!(!second.exists());
            let info = data.path().join("Trash/info");
            assert_eq!(fs::read_dir(info).unwrap().count(), 3);
            assert_eq!(fs::read(&destination).unwrap().len(), 128);
        });
    }
}
