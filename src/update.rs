//! GitHub stable release updates with bounded downloads and rollback.
use crate::manager::{self, AppInfo, Config};
use anyhow::{Context, Result, ensure};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: Option<u64>,
}
pub fn read_progress(app: &AppInfo) -> Option<DownloadProgress> {
    if fs::metadata(&app.update_progress_path).ok()?.len() > 4096 {
        return None;
    }
    serde_json::from_slice(&fs::read(&app.update_progress_path).ok()?).ok()
}
struct ProgressReporter {
    path: PathBuf,
    last_write: Instant,
}
impl ProgressReporter {
    fn new(app: &AppInfo) -> Self {
        Self {
            path: app.update_progress_path.clone(),
            last_write: Instant::now() - Duration::from_secs(1),
        }
    }
    fn report(&mut self, downloaded: u64, total: Option<u64>, force: bool) {
        if !force && self.last_write.elapsed() < Duration::from_millis(250) {
            return;
        }
        self.last_write = Instant::now();
        let write = (|| -> Result<()> {
            let dir = self.path.parent().context("Missing progress directory")?;
            fs::create_dir_all(dir)?;
            let mut temp = tempfile::NamedTempFile::new_in(dir)?;
            serde_json::to_writer(&mut temp, &DownloadProgress { downloaded, total })?;
            temp.persist(&self.path).map_err(|e| e.error)?;
            Ok(())
        })();
        if let Err(error) = write {
            log::debug!("Download progress: {error:#}");
        }
    }
}
impl Drop for ProgressReporter {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAsset {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub digest: Option<String>,
    pub version: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCheck {
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub notes: String,
    pub asset: Option<UpdateAsset>,
    pub status: String,
}
#[derive(Deserialize)]
struct Release {
    tag_name: String,
    #[serde(default)]
    html_url: String,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
    assets: Vec<ReleaseAsset>,
}
#[derive(Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
    size: u64,
    #[serde(default)]
    digest: Option<String>,
}

pub fn check_update(app: &AppInfo) -> Result<UpdateCheck> {
    let Some(repo) = &app.repository else {
        return Ok(UpdateCheck {
            current_version: app.version.clone(),
            latest_version: String::new(),
            release_url: String::new(),
            notes: String::new(),
            asset: None,
            status: "Update source is not configured; enter the application GitHub repository"
                .into(),
        });
    };
    let repo = manager::normalize_repository(repo)?;
    // Public Release pages and browser download URLs require neither gh nor tokens.
    // API is optional: it can supply exact sizes/digests, but cannot block updates.
    match check_release_page(app, &repo) {
        Ok(check) => Ok(check),
        Err(page_error) => check_update_at(
            app,
            &format!("https://api.github.com/repos/{repo}/releases/latest"),
        )
        .with_context(|| format!("Release page discovery failed: {page_error:#}")),
    }
}

fn bounded_text(response: reqwest::blocking::Response) -> Result<String> {
    let mut bytes = Vec::new();
    response
        .error_for_status()?
        .take(4 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() <= 4 * 1024 * 1024,
        "Release response is too large"
    );
    Ok(String::from_utf8(bytes)?)
}
fn check_release_page(app: &AppInfo, repo: &str) -> Result<UpdateCheck> {
    let client = client(45)?;
    let response = client
        .get(format!("https://github.com/{repo}/releases/latest"))
        .send()?
        .error_for_status()?;
    let url = response.url().clone();
    let prefix = format!("/{repo}/releases/tag/");
    let tag = url
        .path()
        .strip_prefix(&prefix)
        .context("Latest stable release did not resolve to a release tag")?;
    ensure!(
        url.host_str() == Some("github.com") && !tag.is_empty(),
        "Unexpected release redirect"
    );
    let html = bounded_text(
        client
            .get(format!(
                "https://github.com/{repo}/releases/expanded_assets/{tag}"
            ))
            .send()?,
    )?;
    let release = release_from_html(repo, tag, url.as_str(), &html)?;
    select_release(app, release)
}
fn release_from_html(repo: &str, tag: &str, release_url: &str, html: &str) -> Result<Release> {
    let links = regex::Regex::new(r#"href="([^"]+)""#)?;
    let hashes = regex::Regex::new(r"sha256:[a-fA-F0-9]{64}")?;
    let prefix = format!("/{repo}/releases/download/{tag}/");
    let mut assets = Vec::new();
    for row in html.split("</li>") {
        for link in links.captures_iter(row) {
            let href = &link[1];
            let Some(name) = href.strip_prefix(&prefix) else {
                continue;
            };
            let url = reqwest::Url::parse(&format!("https://github.com{href}"))?;
            // URL-decode only the filename; retain the publisher's encoded URL for transfer.
            let name = reqwest::Url::parse(&format!("https://local/?name={name}"))?
                .query_pairs()
                .next()
                .context("Missing asset name")?
                .1
                .into_owned();
            if !assets
                .iter()
                .any(|asset: &ReleaseAsset| asset.browser_download_url == url.as_str())
            {
                assets.push(ReleaseAsset {
                    name,
                    browser_download_url: url.to_string(),
                    size: 0,
                    digest: hashes.find(row).map(|m| m.as_str().to_owned()),
                });
            }
        }
    }
    ensure!(
        !assets.is_empty(),
        "Release page contains no downloadable assets"
    );
    Ok(Release {
        tag_name: tag.into(),
        html_url: release_url.into(),
        body: None,
        draft: false,
        prerelease: false,
        assets,
    })
}
fn client(timeout: u64) -> Result<Client> {
    Ok(Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(timeout))
        .user_agent(concat!("AppimageMannger/", env!("CARGO_PKG_VERSION")))
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.previous().len() > 8 {
                return attempt.error("Too many redirects");
            }
            if allowed_url(attempt.url().as_str()) {
                attempt.follow()
            } else {
                attempt.error("Refusing an insecure update redirect")
            }
        }))
        .build()?)
}
fn allowed_url(url: &str) -> bool {
    let Ok(url) = reqwest::Url::parse(url) else {
        return false;
    };
    if url.username() != "" || url.password().is_some() {
        return false;
    }
    if url.scheme() == "https" && url.host_str().is_some() {
        return true;
    }
    #[cfg(test)]
    if url.scheme() == "http" && matches!(url.host_str(), Some("127.0.0.1" | "localhost" | "[::1]"))
    {
        return true;
    }
    false
}
fn check_update_at(app: &AppInfo, url: &str) -> Result<UpdateCheck> {
    ensure!(allowed_url(url), "Update API must use HTTPS");
    let response = client(35)?
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .context("Cannot connect to the update server")?;
    let status = response.status();
    ensure!(
        status.is_success(),
        "Update check failed with HTTP {status}; the GitHub API may be rate limited"
    );
    let mut bytes = Vec::new();
    response.take(4 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() <= 4 * 1024 * 1024,
        "Update API response is too large"
    );
    let release: Release =
        serde_json::from_slice(&bytes).context("Invalid update server response")?;
    select_release(app, release)
}
fn select_release(app: &AppInfo, release: Release) -> Result<UpdateCheck> {
    ensure!(
        !release.draft && !release.prerelease,
        "Latest release is not a stable release"
    );
    let latest = release.tag_name.trim_start_matches('v').to_string();
    let current = app.version.trim_start_matches('v');
    let comparison = match (
        semver::Version::parse(&latest),
        semver::Version::parse(current),
    ) {
        (Ok(l), Ok(c)) => Some(l.cmp(&c)),
        _ => None,
    };
    let newer = comparison == Some(std::cmp::Ordering::Greater);
    let pattern = app.update_info.as_deref().and_then(|s| {
        let p: Vec<_> = s.split('|').collect();
        if p.first() == Some(&"gh-releases-zsync") && p.len() >= 5 {
            Some(p[4].trim_end_matches(".zsync").to_string())
        } else {
            None
        }
    });
    let mut candidates: Vec<_> = release
        .assets
        .into_iter()
        .filter(|a| {
            asset_matches(&a.name, std::env::consts::ARCH)
                && pattern.as_ref().is_none_or(|p| glob_matches(p, &a.name))
        })
        .collect();
    let best = candidates
        .iter()
        .map(|a| candidate_score(&a.name, app))
        .max()
        .unwrap_or(0);
    candidates.retain(|a| candidate_score(&a.name, app) == best);
    let ambiguous = candidates.len() > 1;
    let asset = if candidates.len() == 1 {
        candidates.pop()
    } else {
        None
    };
    let status = if comparison.is_none() {
        "Cannot compare the current version; review the release page"
    } else if !newer {
        "Application is up to date"
    } else if ambiguous {
        "Multiple matching assets found; review the release page"
    } else if asset.is_none() {
        "A newer release exists but no AppImage matches this architecture"
    } else {
        "A newer release is available and can be installed"
    }
    .to_string();
    Ok(UpdateCheck {
        current_version: app.version.clone(),
        latest_version: latest.clone(),
        release_url: release.html_url,
        notes: release.body.unwrap_or_default(),
        asset: if newer {
            asset.map(|a| UpdateAsset {
                name: a.name,
                url: a.browser_download_url,
                size: a.size,
                digest: a.digest,
                version: latest,
            })
        } else {
            None
        },
        status,
    })
}
fn candidate_score(name: &str, app: &AppInfo) -> u8 {
    let portable = app
        .path
        .to_string_lossy()
        .to_lowercase()
        .contains("portable");
    architecture_score(name, std::env::consts::ARCH) * 2
        + u8::from(name.to_lowercase().contains("portable") == portable)
}
// Standard AppImage update information uses '*' and '?' globs for asset names.
fn glob_matches(pattern: &str, value: &str) -> bool {
    let p = pattern.as_bytes();
    let v = value.as_bytes();
    let (mut i, mut j, mut star, mut retry) = (0usize, 0usize, None, 0usize);
    while j < v.len() {
        if i < p.len() && (p[i] == b'?' || p[i] == v[j]) {
            i += 1;
            j += 1;
        } else if i < p.len() && p[i] == b'*' {
            star = Some(i);
            i += 1;
            retry = j;
        } else if let Some(k) = star {
            retry += 1;
            j = retry;
            i = k + 1;
        } else {
            return false;
        }
    }
    while i < p.len() && p[i] == b'*' {
        i += 1;
    }
    i == p.len()
}
fn architecture_score(name: &str, arch: &str) -> u8 {
    let lower = name.to_ascii_lowercase();
    let aliases: &[&str] = match arch {
        "x86_64" => &["x86_64", "amd64", "x64"],
        "aarch64" => &["aarch64", "arm64"],
        "x86" => &["i386", "i686", "x86"],
        "arm" => &["armv7", "armhf"],
        _ => &[],
    };
    if aliases.iter().any(|a| lower.contains(a)) {
        2
    } else {
        1
    }
}
fn asset_matches(name: &str, arch: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    if !lower.ends_with(".appimage") {
        return false;
    }
    let all = [
        ("x86_64", &["x86_64", "amd64", "x64"][..]),
        ("aarch64", &["aarch64", "arm64"][..]),
        ("x86", &["i386", "i686"][..]),
        ("arm", &["armv7", "armhf"][..]),
    ];
    !all.iter()
        .any(|(candidate, aliases)| *candidate != arch && aliases.iter().any(|a| lower.contains(a)))
}

pub fn install_update(config: &Config, app: &AppInfo, asset: &UpdateAsset) -> Result<AppInfo> {
    let _lock = manager::lock_library(config)?;
    manager::ensure_managed_path(config, &app.path)?;
    manager::validate_appimage(&app.path)?;
    ensure!(
        asset_matches(&asset.name, std::env::consts::ARCH),
        "Update asset architecture does not match this machine"
    );
    ensure!(
        allowed_url(&asset.url),
        "Update download URL must use HTTPS"
    );
    ensure!(
        (asset.size == 0 || asset.size > 64) && asset.size <= 4 * 1024 * 1024 * 1024,
        "Invalid update asset size"
    );
    let mut download = tempfile::NamedTempFile::new_in(&config.applications_dir)?;
    let mut progress = ProgressReporter::new(app);
    progress.report(0, (asset.size > 0).then_some(asset.size), true);
    let mut response = client(1800)?
        .get(&asset.url)
        .send()
        .context("Failed to download update")?
        .error_for_status()?;
    let expected_size = if asset.size > 0 {
        Some(asset.size)
    } else {
        response.content_length()
    };
    progress.report(0, expected_size, true);
    let mut hasher = Sha256::new();
    let mut total = 0u64;
    let mut buf = [0; 64 * 1024];
    loop {
        let count = response.read(&mut buf)?;
        if count == 0 {
            break;
        }
        total += count as u64;
        ensure!(
            expected_size.is_none_or(|size| total <= size) && total <= 4 * 1024 * 1024 * 1024,
            "Downloaded asset exceeds the expected size"
        );
        hasher.update(&buf[..count]);
        download.write_all(&buf[..count])?;
        progress.report(total, expected_size, false);
    }
    progress.report(total, expected_size, true);
    ensure!(
        expected_size.is_none_or(|size| total == size),
        "Incomplete download: expected {} bytes, received {total} bytes",
        expected_size.unwrap_or(0)
    );
    let computed = hex::encode(hasher.finalize());
    if let Some(digest) = &asset.digest {
        let expected = digest
            .strip_prefix("sha256:")
            .context("Server provided an unsupported digest algorithm")?;
        ensure!(
            expected.eq_ignore_ascii_case(&computed),
            "SHA-256 verification failed; original application is unchanged"
        );
    }
    let header = manager::validate_appimage(download.path())?;
    let machine = u16::from_le_bytes([header[18], header[19]]);
    ensure!(
        (std::env::consts::ARCH == "x86_64" && machine == 62)
            || (std::env::consts::ARCH == "aarch64" && machine == 183),
        "Downloaded ELF architecture does not match this machine"
    );
    download
        .as_file()
        .set_permissions(fs::Permissions::from_mode(0o755))?;
    download.as_file().sync_all()?;
    // Make a synced rollback copy before the single atomic replacement.
    let backup = backup_path(app)?;
    fs::create_dir_all(backup.parent().unwrap())?;
    let mut backup_temp = tempfile::NamedTempFile::new_in(backup.parent().unwrap())?;
    std::io::copy(&mut File::open(&app.path)?, &mut backup_temp)?;
    backup_temp.as_file().sync_all()?;
    backup_temp.persist(&backup).map_err(|e| e.error)?;
    let launcher = manager::user_data_dir()?
        .join("applications")
        .join(format!("appimage-mannger-{}.desktop", app.id));
    let old_launcher = fs::read(&launcher).ok();
    download.persist(&app.path).map_err(|e| e.error)?;
    let result = (|| -> Result<AppInfo> {
        let mut updated = manager::inspect(config, &app.path)?;
        updated.version = asset.version.clone();
        manager::integrate(&updated)?;
        manager::save_installed_version(&updated, &computed)?;
        Ok(updated)
    })();
    if let Err(error) = result {
        // Same directory temp + rename keeps rollback atomic even when backup data is elsewhere.
        let mut restore = tempfile::NamedTempFile::new_in(&config.applications_dir)?;
        std::io::copy(&mut File::open(&backup)?, &mut restore)?;
        restore
            .as_file()
            .set_permissions(fs::Permissions::from_mode(0o755))?;
        restore.as_file().sync_all()?;
        restore.persist(&app.path).map_err(|e| e.error)?;
        if let Some(bytes) = old_launcher {
            fs::write(&launcher, bytes)?;
        } else if launcher.exists() {
            fs::remove_file(&launcher)?;
        }
        return Err(error.context("Update failed; original application was restored"));
    }
    result
}
fn backup_path(app: &AppInfo) -> Result<PathBuf> {
    Ok(manager::user_data_dir()?
        .join("appimage-mannger/backups")
        .join(format!("{}.AppImage", app.id)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeMap, net::TcpListener, thread};
    fn serve(body: Vec<u8>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut request = [0; 4096];
            let _ = socket.read(&mut request);
            write!(
                socket,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            )
            .unwrap();
            socket.write_all(&body).unwrap();
        });
        url
    }
    fn fixture() -> Vec<u8> {
        let mut b = vec![0u8; 128];
        b[..4].copy_from_slice(b"\x7fELF");
        b[4] = 2;
        b[5] = 1;
        b[8..11].copy_from_slice(b"AI\x02");
        let arch: u16 = if std::env::consts::ARCH == "aarch64" {
            183
        } else {
            62
        };
        b[18..20].copy_from_slice(&arch.to_le_bytes());
        b
    }
    #[test]
    fn rejects_wrong_arch_and_non_image_assets() {
        assert!(asset_matches("Tool-x86_64.AppImage", "x86_64"));
        assert!(!asset_matches("Tool-arm64.AppImage", "x86_64"));
        assert!(!asset_matches("Tool-x86_64.AppImage.zsync", "x86_64"));
        assert!(!asset_matches("Tool-x86_64.deb", "x86_64"));
    }
    #[test]
    fn update_information_glob_selects_only_the_named_application() {
        assert!(glob_matches(
            "Tool-*-x86_64.AppImage",
            "Tool-1.2.3-x86_64.AppImage"
        ));
        assert!(!glob_matches(
            "Tool-*-x86_64.AppImage",
            "Other-1.2.3-x86_64.AppImage"
        ));
        assert!(!glob_matches(
            "Tool-*-x86_64.AppImage",
            "Tool-1.2.3-x86_64.AppImage.zsync"
        ));
    }
    #[test]
    fn latest_release_selects_native_stable_asset() {
        let body=serde_json::json!({"tag_name":"v1.3.0","html_url":"https://github.com/a/b/releases/v1.3.0","assets":[{"name":format!("Tool-{}.AppImage",std::env::consts::ARCH),"size":128,"browser_download_url":"https://example.com/a.AppImage","digest":null}]}).to_string();
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("Tool-1.2.0.AppImage");
        fs::write(&p, fixture()).unwrap();
        let config = Config {
            applications_dir: dir.path().into(),
            repositories: BTreeMap::new(),
            ..Config::default()
        };
        let app = manager::inspect(&config, &p).unwrap();
        let check = check_update_at(&app, &serve(body.into_bytes())).unwrap();
        assert_eq!(check.latest_version, "1.3.0");
        assert!(check.asset.is_some());
    }
    #[test]
    fn public_release_html_selects_normal_native_asset_and_publisher_digest() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("Tool-1.0.0.AppImage");
        fs::write(&p, fixture()).unwrap();
        let config = Config {
            applications_dir: dir.path().into(),
            ..Config::default()
        };
        let app = manager::inspect(&config, &p).unwrap();
        let arch = std::env::consts::ARCH;
        let digest = format!("sha256:{}", "a".repeat(64));
        let html = format!(
            r#"<li><a href="/owner/tool/releases/download/v2.0.0/Tool-{arch}.AppImage">normal</a><span>{digest}</span></li><li><a href="/owner/tool/releases/download/v2.0.0/Tool-{arch}-portable.AppImage">portable</a></li><li><a href="https://evil.test/unrelated.AppImage">evil</a></li>"#
        );
        let release = release_from_html(
            "owner/tool",
            "v2.0.0",
            "https://github.com/owner/tool/releases/tag/v2.0.0",
            &html,
        )
        .unwrap();
        assert_eq!(release.assets.len(), 2);
        let check = select_release(&app, release).unwrap();
        let asset = check.asset.unwrap();
        assert_eq!(asset.name, format!("Tool-{arch}.AppImage"));
        assert_eq!(asset.digest.as_deref(), Some(digest.as_str()));
        assert_eq!(asset.size, 0);
    }
    #[test]
    fn checksum_failure_preserves_original() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("Tool-1.2.0.AppImage");
        let old = fixture();
        fs::write(&p, &old).unwrap();
        let config = Config {
            applications_dir: dir.path().into(),
            repositories: BTreeMap::new(),
            ..Config::default()
        };
        let app = manager::inspect(&config, &p).unwrap();
        let asset = UpdateAsset {
            name: format!("Tool-{}.AppImage", std::env::consts::ARCH),
            url: serve(fixture()),
            size: 128,
            digest: Some(format!("sha256:{}", "0".repeat(64))),
            version: "1.3.0".into(),
        };
        assert!(install_update(&config, &app, &asset).is_err());
        assert_eq!(fs::read(p).unwrap(), old);
        assert!(!app.update_progress_path.exists());
    }
    #[test]
    fn forbids_untrusted_transport_and_credentials() {
        assert!(!allowed_url("http://example.com/app.AppImage"));
        assert!(!allowed_url("https://u:p@example.com/a"));
        assert!(!allowed_url("file:///tmp/a"));
        assert!(allowed_url(
            "https://github.com/a/b/releases/download/v1/a.AppImage"
        ));
    }

    #[test]
    fn successful_update_keeps_rollback_and_persists_installed_version() {
        let dir = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        manager::with_test_data(data.path(), || {
            let p = dir.path().join("Tool-1.2.0.AppImage");
            let old = fixture();
            fs::write(&p, &old).unwrap();
            let config = Config {
                applications_dir: dir.path().into(),
                repositories: BTreeMap::new(),
                ..Config::default()
            };
            let app = manager::inspect(&config, &p).unwrap();
            let mut new = fixture();
            new[100] = 42;
            let asset = UpdateAsset {
                name: format!("Tool-{}.AppImage", std::env::consts::ARCH),
                url: serve(new.clone()),
                size: 0, // Release HTML omits exact size; validate Content-Length and digest.
                digest: Some(format!("sha256:{}", hex::encode(Sha256::digest(&new)))),
                version: "1.3.0".into(),
            };
            let updated = install_update(&config, &app, &asset).unwrap();
            assert_eq!(updated.version, "1.3.0");
            assert_eq!(fs::read(&p).unwrap(), new);
            assert_eq!(fs::read(backup_path(&app).unwrap()).unwrap(), old);
            assert_eq!(manager::inspect(&config, &p).unwrap().version, "1.3.0");
        });
    }
    #[test]
    fn live_download_reports_bytes_before_atomic_replacement_and_cleans_progress() {
        let dir = tempfile::tempdir().unwrap();
        let data = tempfile::tempdir().unwrap();
        manager::with_test_data(data.path(), || {
            let path = dir.path().join("Tool-1.0.0.AppImage");
            let old = fixture();
            fs::write(&path, &old).unwrap();
            let config = Config {
                applications_dir: dir.path().into(),
                ..Config::default()
            };
            let app = manager::inspect(&config, &path).unwrap();
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let url = format!("http://{}", listener.local_addr().unwrap());
            let mut bytes = fixture();
            bytes.resize(256, 42);
            let digest = hex::encode(Sha256::digest(&bytes));
            let (paused_tx, paused_rx) = std::sync::mpsc::channel();
            let (resume_tx, resume_rx) = std::sync::mpsc::channel();
            let server = thread::spawn(move || {
                let (mut socket, _) = listener.accept().unwrap();
                let mut request = [0; 4096];
                let _ = socket.read(&mut request).unwrap();
                write!(
                    socket,
                    "HTTP/1.1 200 OK\r\nContent-Length: 256\r\nConnection: close\r\n\r\n"
                )
                .unwrap();
                socket.write_all(&bytes[..128]).unwrap();
                thread::sleep(Duration::from_millis(300));
                socket.write_all(&bytes[128..192]).unwrap();
                paused_tx.send(()).unwrap();
                resume_rx.recv_timeout(Duration::from_secs(5)).unwrap();
                socket.write_all(&bytes[192..]).unwrap();
            });
            let asset = UpdateAsset {
                name: format!("Tool-{}.AppImage", std::env::consts::ARCH),
                url,
                size: 0,
                digest: Some(format!("sha256:{digest}")),
                version: "2.0.0".into(),
            };
            let worker_config = config.clone();
            let worker_app = app.clone();
            let data_path = data.path().to_path_buf();
            let download = thread::spawn(move || {
                manager::with_test_data(&data_path, || {
                    install_update(&worker_config, &worker_app, &asset)
                })
            });
            paused_rx.recv_timeout(Duration::from_secs(5)).unwrap();
            let mut progress = None;
            for _ in 0..50 {
                progress = read_progress(&app);
                if progress.as_ref().is_some_and(|p| p.downloaded >= 192) {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
            assert_eq!(
                progress,
                Some(DownloadProgress {
                    downloaded: 192,
                    total: Some(256)
                })
            );
            assert_eq!(fs::read(&path).unwrap(), old);
            resume_tx.send(()).unwrap();
            assert!(download.join().unwrap().is_ok());
            server.join().unwrap();
            assert!(!app.update_progress_path.exists());
            assert_eq!(manager::inspect(&config, &path).unwrap().version, "2.0.0");
        });
    }
    #[test]
    fn invalid_download_preserves_original() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("Tool-1.2.0.AppImage");
        let old = fixture();
        fs::write(&p, &old).unwrap();
        let config = Config {
            applications_dir: dir.path().into(),
            repositories: BTreeMap::new(),
            ..Config::default()
        };
        let app = manager::inspect(&config, &p).unwrap();
        let asset = UpdateAsset {
            name: format!("Tool-{}.AppImage", std::env::consts::ARCH),
            url: serve(vec![42; 128]),
            size: 128,
            digest: None,
            version: "1.3.0".into(),
        };
        assert!(install_update(&config, &app, &asset).is_err());
        assert_eq!(fs::read(p).unwrap(), old);
    }
}
