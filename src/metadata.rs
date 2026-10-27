//! Resolve AppImage icons without mounting or running the application.
//!
//! The optional squashfs-tools process only lists or cats bounded image entries;
//! it never extracts paths to disk. Only validated icon bytes reach our cache.
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom, Write},
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant, UNIX_EPOCH},
};

const ICON_LIMIT: usize = 4 * 1024 * 1024;
const TEXT_LIMIT: usize = 256 * 1024;
const PROCESS_TIMEOUT: Duration = Duration::from_secs(3);
const IMAGE_TIMEOUT: Duration = Duration::from_secs(8);
const ELF_SEARCH_LIMIT: usize = 1024 * 1024;

/// Read identity/version from the embedded desktop entry without executing it.
pub fn embedded_desktop(path: &Path) -> Option<std::collections::BTreeMap<String, String>> {
    let metadata = fs::metadata(path).ok()?;
    let key = hex::encode(Sha256::digest(
        format!(
            "{}:{}:{:?}",
            path.display(),
            metadata.len(),
            metadata.modified().ok()?
        )
        .as_bytes(),
    ));
    let cache = dirs::cache_dir()?.join("appimage-mannger/metadata");
    let cached = cache.join(format!("{key}.json"));
    if let Ok(bytes) = fs::read(&cached) {
        return serde_json::from_slice(&bytes).ok();
    }
    let offset = squashfs_offset(path)?;
    let listing = squashfs_command(path, offset, &["-ll", "-max-depth", "1"], &[], TEXT_LIMIT)?;
    for line in std::str::from_utf8(&listing).ok()?.lines().take(128) {
        let Some((_, entry)) = line.split_once(" squashfs-root/") else {
            continue;
        };
        if !entry.ends_with(".desktop") || entry.contains(" -> ") {
            continue;
        }
        let bytes = cat_entry(path, offset, &safe_relative_path(entry)?, TEXT_LIMIT)?;
        let mut values = std::collections::BTreeMap::new();
        let mut active = false;
        for line in std::str::from_utf8(&bytes).ok()?.lines() {
            if line.starts_with('[') {
                active = line == "[Desktop Entry]";
            } else if active && let Some((key, value)) = line.split_once('=') {
                values.insert(key.to_owned(), value.to_owned());
            }
        }
        fs::create_dir_all(&cache).ok()?;
        let mut temp = tempfile::NamedTempFile::new_in(&cache).ok()?;
        temp.write_all(&serde_json::to_vec(&values).ok()?).ok()?;
        temp.persist(cached).ok()?;
        return Some(values);
    }
    None
}

/// Resolve a desktop icon first, then the AppImage's embedded icon.
///
/// Call once during background scanning, not from the render method. `None`
/// means no usable icon or missing squashfs-tools; callers keep their fallback.
pub fn resolve_icon(path: &Path, desktop_file: Option<&Path>) -> Option<PathBuf> {
    let roots = icon_roots();
    let cache = dirs::cache_dir()?.join("appimage-mannger/icons");
    resolve_icon_with(path, desktop_file, &roots, &cache)
}

fn resolve_icon_with(
    path: &Path,
    desktop_file: Option<&Path>,
    theme_roots: &[PathBuf],
    cache: &Path,
) -> Option<PathBuf> {
    if let Some(name) = desktop_file.and_then(read_desktop_icon)
        && let Some(icon) = find_theme_icon(&name, theme_roots)
    {
        return Some(icon);
    }
    let metadata = fs::metadata(path).ok()?;
    if !metadata.is_file() || fs::symlink_metadata(path).ok()?.file_type().is_symlink() {
        return None;
    }
    let mut hasher = Sha256::new();
    hasher.update(path.canonicalize().ok()?.as_os_str().as_encoded_bytes());
    hasher.update(metadata.len().to_le_bytes());
    hasher.update(
        metadata
            .modified()
            .ok()?
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_nanos()
            .to_le_bytes(),
    );
    let key = hex::encode(hasher.finalize());
    for extension in ["png", "svg"] {
        let cached = cache.join(format!("{key}.{extension}"));
        if usable_icon_file(&cached) {
            return Some(cached);
        }
    }
    let offset = squashfs_offset(path)?;
    let bytes = embedded_icon(path, offset)?;
    let extension = icon_extension(&bytes)?;
    fs::create_dir_all(cache).ok()?;
    let destination = cache.join(format!("{key}.{extension}"));
    let mut temp = tempfile::NamedTempFile::new_in(cache).ok()?;
    temp.write_all(&bytes).ok()?;
    temp.as_file().sync_all().ok()?;
    temp.persist(&destination).ok()?;
    Some(destination)
}

fn icon_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(data) = dirs::data_dir() {
        roots.push(data.join("icons/hicolor"));
        roots.push(data.join("pixmaps"));
    }
    if let Some(home) = dirs::home_dir() {
        roots.push(home.join(".icons/hicolor"));
    }
    let system =
        std::env::var_os("XDG_DATA_DIRS").unwrap_or_else(|| "/usr/local/share:/usr/share".into());
    for data in std::env::split_paths(&system) {
        roots.push(data.join("icons/hicolor"));
        roots.push(data.join("pixmaps"));
    }
    roots
}

fn read_desktop_icon(path: &Path) -> Option<String> {
    let metadata = fs::metadata(path).ok()?;
    if !metadata.is_file() || metadata.len() > TEXT_LIMIT as u64 {
        return None;
    }
    let text = fs::read_to_string(path).ok()?;
    desktop_icon(&text)
}
fn desktop_icon(text: &str) -> Option<String> {
    let mut entry = false;
    for line in text.lines() {
        if line.starts_with('[') {
            entry = line.trim() == "[Desktop Entry]";
        } else if entry && let Some(value) = line.strip_prefix("Icon=") {
            let value = value.trim();
            if !value.is_empty() && !value.chars().any(char::is_control) {
                return Some(value.to_string());
            }
        }
    }
    None
}
fn find_theme_icon(name: &str, roots: &[PathBuf]) -> Option<PathBuf> {
    let absolute = Path::new(name);
    if absolute.is_absolute() {
        return usable_icon_file(absolute).then(|| absolute.to_path_buf());
    }
    if name.contains('/') || name.contains('\\') || name.chars().any(char::is_control) {
        return None;
    }
    for root in roots {
        for sub in [
            "512x512/apps",
            "256x256/apps",
            "scalable/apps",
            "128x128/apps",
            "96x96/apps",
            "64x64/apps",
            "48x48/apps",
            "32x32/apps",
            "",
        ] {
            for extension in ["png", "svg"] {
                let filename = if name.ends_with(&format!(".{extension}")) {
                    name.to_string()
                } else {
                    format!("{name}.{extension}")
                };
                let candidate = root.join(sub).join(filename);
                if usable_icon_file(&candidate) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}
fn usable_icon_file(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() || metadata.len() > ICON_LIMIT as u64 {
        return false;
    }
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    icon_extension(&bytes).is_some()
}
pub(crate) fn icon_extension(bytes: &[u8]) -> Option<&'static str> {
    if bytes.len() > ICON_LIMIT || bytes.is_empty() {
        return None;
    }
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") && bytes.len() >= 24 {
        let width = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
        let height = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
        return (width > 0 && height > 0 && width <= 8192 && height <= 8192).then_some("png");
    }
    if bytes.len() <= 1024 * 1024 {
        let text = std::str::from_utf8(bytes)
            .ok()?
            .trim_start_matches('\u{feff}')
            .trim();
        let lower = text.to_ascii_lowercase();
        let compact: String = lower.chars().filter(|c| !c.is_ascii_whitespace()).collect();
        if lower.contains("<svg")
            && ![
                "<!doctype",
                "<!entity",
                "<script",
                "<foreignobject",
                "<image",
                "@import",
                "<?xml-stylesheet",
            ]
            .iter()
            .any(|s| lower.contains(s))
            && !compact.contains("href=")
            && !lower
                .split("url(")
                .skip(1)
                .any(|s| !s.trim_start().starts_with('#'))
        {
            return Some("svg");
        }
    }
    None
}

/// Locate the appended filesystem from ELF runtime boundaries, examining at most
/// one MiB of padding rather than scanning the entire AppImage.
fn squashfs_offset(path: &Path) -> Option<u64> {
    let mut file = File::open(path).ok()?;
    let length = file.metadata().ok()?.len();
    let mut header = [0u8; 64];
    file.read_exact(&mut header).ok()?;
    if &header[..4] != b"\x7fELF" || &header[8..11] != b"AI\x02" || header[5] != 1 {
        return None;
    }
    let is64 = match header[4] {
        2 => true,
        1 => false,
        _ => return None,
    };
    let (shoff, shsize, shcount, phoff, phsize, phcount) = if is64 {
        (
            u64_at(&header, 40)?,
            u16_at(&header, 58)?,
            u16_at(&header, 60)?,
            u64_at(&header, 32)?,
            u16_at(&header, 54)?,
            u16_at(&header, 56)?,
        )
    } else {
        (
            u32_at(&header, 32)?,
            u16_at(&header, 46)?,
            u16_at(&header, 48)?,
            u32_at(&header, 28)?,
            u16_at(&header, 42)?,
            u16_at(&header, 44)?,
        )
    };
    if shcount > 4096 || phcount > 1024 || shsize > 256 || phsize > 256 {
        return None;
    }
    let mut end = if is64 { 64u64 } else { 52u64 };
    if shcount > 0 {
        if shsize < if is64 { 64 } else { 40 } {
            return None;
        }
        let table_end = shoff.checked_add(shsize.checked_mul(shcount)?)?;
        if table_end > length {
            return None;
        }
        end = end.max(table_end);
        for i in 0..shcount {
            let entry = read_at(&mut file, shoff + i * shsize, shsize as usize)?;
            if u32_at(&entry, 4)? == 8 {
                continue;
            } // SHT_NOBITS has no file bytes.
            let (start, size) = if is64 {
                (u64_at(&entry, 24)?, u64_at(&entry, 32)?)
            } else {
                (u32_at(&entry, 16)?, u32_at(&entry, 20)?)
            };
            let section_end = start.checked_add(size)?;
            if section_end > length {
                return None;
            }
            end = end.max(section_end);
        }
    }
    if phcount > 0 {
        if phsize < if is64 { 56 } else { 32 } {
            return None;
        }
        let table_end = phoff.checked_add(phsize.checked_mul(phcount)?)?;
        if table_end > length {
            return None;
        }
        end = end.max(table_end);
        for i in 0..phcount {
            let entry = read_at(&mut file, phoff + i * phsize, phsize as usize)?;
            let (start, size) = if is64 {
                (u64_at(&entry, 8)?, u64_at(&entry, 32)?)
            } else {
                (u32_at(&entry, 4)?, u32_at(&entry, 16)?)
            };
            let segment_end = start.checked_add(size)?;
            if segment_end > length {
                return None;
            }
            end = end.max(segment_end);
        }
    }
    if end.checked_add(96)? > length {
        return None;
    }
    let amount = (length - end).min(ELF_SEARCH_LIMIT as u64) as usize;
    let bytes = read_at(&mut file, end, amount)?;
    for index in 0..bytes.len().saturating_sub(95) {
        if &bytes[index..index + 4] == b"hsqs" {
            let superblock = &bytes[index..index + 96];
            let block = u32_at(superblock, 12)?;
            let used = u64_at(superblock, 40)?;
            let offset = end + index as u64;
            if u16_at(superblock, 28)? == 4
                && u16_at(superblock, 30)? == 0
                && block.is_power_of_two()
                && (4096..=1024 * 1024).contains(&block)
                && used >= 96
                && offset.checked_add(used).is_some_and(|n| n <= length)
                && u32_at(superblock, 4)? > 0
            {
                return Some(offset);
            }
        }
    }
    None
}
fn read_at(file: &mut File, offset: u64, length: usize) -> Option<Vec<u8>> {
    file.seek(SeekFrom::Start(offset)).ok()?;
    let mut bytes = vec![0; length];
    file.read_exact(&mut bytes).ok()?;
    Some(bytes)
}
fn u16_at(b: &[u8], i: usize) -> Option<u64> {
    Some(u16::from_le_bytes(b.get(i..i + 2)?.try_into().ok()?) as u64)
}
fn u32_at(b: &[u8], i: usize) -> Option<u64> {
    Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?) as u64)
}
fn u64_at(b: &[u8], i: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(i..i + 8)?.try_into().ok()?))
}

fn embedded_icon(path: &Path, offset: u64) -> Option<Vec<u8>> {
    let started = Instant::now();
    if let Some(bytes) = cat_entry(path, offset, ".DirIcon", ICON_LIMIT)
        && icon_extension(&bytes).is_some()
    {
        return Some(bytes);
    }
    let listing = squashfs_command(path, offset, &["-ll", "-max-depth", "1"], &[], TEXT_LIMIT)?;
    let text = String::from_utf8(listing).ok()?;
    if let Some(line) = text
        .lines()
        .find(|l| l.contains(" squashfs-root/.DirIcon -> "))
        && let Some((_, target)) = line.split_once(" -> ")
        && let Some(target) = safe_relative_path(target)
        && let Some(bytes) = cat_entry(path, offset, &target, ICON_LIMIT)
        && icon_extension(&bytes).is_some()
    {
        return Some(bytes);
    }
    for line in text.lines().take(128) {
        if started.elapsed() >= IMAGE_TIMEOUT {
            return None;
        }
        let Some((_, entry)) = line.split_once(" squashfs-root/") else {
            continue;
        };
        if !entry.ends_with(".desktop") || entry.contains(" -> ") {
            continue;
        }
        let Some(desktop) = cat_entry(path, offset, entry, TEXT_LIMIT) else {
            continue;
        };
        let Some(name) = std::str::from_utf8(&desktop).ok().and_then(desktop_icon) else {
            continue;
        };
        for candidate in embedded_candidates(&name) {
            if started.elapsed() >= IMAGE_TIMEOUT {
                return None;
            }
            if let Some(bytes) = cat_entry(path, offset, &candidate, ICON_LIMIT)
                && icon_extension(&bytes).is_some()
            {
                return Some(bytes);
            }
        }
    }
    None
}
fn embedded_candidates(name: &str) -> Vec<String> {
    let Some(name) = safe_relative_path(name) else {
        return Vec::new();
    };
    let mut candidates = vec![name.clone()];
    let prefixes = [
        "",
        "usr/share/icons/hicolor/256x256/apps/",
        "usr/share/icons/hicolor/512x512/apps/",
        "usr/share/icons/hicolor/scalable/apps/",
        "usr/share/pixmaps/",
    ];
    for prefix in prefixes {
        for suffix in [".png", ".svg"] {
            candidates.push(format!("{prefix}{name}{suffix}"));
        }
    }
    candidates
}
fn safe_relative_path(value: &str) -> Option<String> {
    if value.is_empty() || value.chars().any(char::is_control) || value.contains('\\') {
        return None;
    }
    let mut parts = Vec::new();
    for component in Path::new(value).components() {
        match component {
            Component::Normal(part) => parts.push(part.to_str()?),
            Component::CurDir => (),
            Component::ParentDir => {
                parts.pop()?;
            }
            _ => return None,
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}
fn cat_entry(path: &Path, offset: u64, entry: &str, limit: usize) -> Option<Vec<u8>> {
    let entry = safe_relative_path(entry)?;
    squashfs_command(path, offset, &["-no-wildcards", "-cat"], &[&entry], limit)
}
fn squashfs_command(
    path: &Path,
    offset: u64,
    options: &[&str],
    entries: &[&str],
    limit: usize,
) -> Option<Vec<u8>> {
    let mut child = Command::new("unsquashfs")
        .args(["-o", &offset.to_string(), "-processors", "1"])
        .args(options)
        .arg(path)
        .args(entries)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let stdout = child.stdout.take()?;
    let reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout
            .take((limit + 1) as u64)
            .read_to_end(&mut bytes)
            .ok()?;
        (bytes.len() <= limit).then_some(bytes)
    });
    let start = Instant::now();
    let success = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status.success(),
            Ok(None) if start.elapsed() < PROCESS_TIMEOUT => {
                thread::sleep(Duration::from_millis(10))
            }
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                break false;
            }
        }
    };
    let bytes = reader.join().ok()??;
    success.then_some(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    const PNG_HEX: &str = "89504e470d0a1a0a0000000d4948445200000001000000010804000000b51c0c020000000b4944415478da6364f80f00010501012718e3660000000049454e44ae426082";
    fn png() -> Vec<u8> {
        hex::decode(PNG_HEX).unwrap()
    }
    fn image_fixture(root: &Path, name: &str, symlink: bool, desktop_only: bool) -> PathBuf {
        let appdir = root.join(format!("{name}-AppDir"));
        fs::create_dir_all(&appdir).unwrap();
        fs::write(appdir.join("fixture.png"), png()).unwrap();
        if desktop_only {
            fs::write(
                appdir.join("fixture.desktop"),
                "[Desktop Entry]\nName=Fixture\nIcon=fixture\n",
            )
            .unwrap();
        } else if symlink {
            std::os::unix::fs::symlink("fixture.png", appdir.join(".DirIcon")).unwrap();
        } else {
            fs::write(appdir.join(".DirIcon"), png()).unwrap();
        }
        let squash = root.join(format!("{name}.squashfs"));
        let status = Command::new("mksquashfs")
            .arg(&appdir)
            .arg(&squash)
            .args(["-noappend", "-processors", "1", "-quiet"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("squashfs-tools required for metadata fixtures");
        assert!(status.success());
        let mut bytes = vec![0; 64];
        bytes[..4].copy_from_slice(b"\x7fELF");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[8..11].copy_from_slice(b"AI\x02");
        bytes[18] = 62;
        bytes.extend_from_slice(&fs::read(squash).unwrap());
        let path = root.join(format!("{name}.AppImage"));
        fs::write(&path, bytes).unwrap();
        path
    }
    #[test]
    fn resolves_named_desktop_icon_from_hicolor_without_reading_appimage() {
        let root = tempfile::tempdir().unwrap();
        let icons = root.path().join("hicolor");
        let icon = icons.join("256x256/apps/test-icon.png");
        fs::create_dir_all(icon.parent().unwrap()).unwrap();
        fs::write(&icon, png()).unwrap();
        let desktop = root.path().join("fixture.desktop");
        fs::write(&desktop, "[Desktop Entry]\nIcon=test-icon\n").unwrap();
        assert_eq!(
            resolve_icon_with(
                &root.path().join("not-executed.AppImage"),
                Some(&desktop),
                &[icons],
                &root.path().join("cache")
            ),
            Some(icon)
        );
    }
    #[test]
    fn parses_real_squashfs_diricon_and_uses_cached_file() {
        let root = tempfile::tempdir().unwrap();
        let image = image_fixture(root.path(), "direct", false, false);
        let cache = root.path().join("cache");
        let path = resolve_icon_with(&image, None, &[], &cache).unwrap();
        assert_eq!(fs::read(&path).unwrap(), png());
        assert_eq!(resolve_icon_with(&image, None, &[], &cache), Some(path));
    }
    #[test]
    fn resolves_diricon_symlink_inside_real_squashfs() {
        let root = tempfile::tempdir().unwrap();
        let image = image_fixture(root.path(), "symlink", true, false);
        let path = resolve_icon_with(&image, None, &[], &root.path().join("cache")).unwrap();
        assert_eq!(fs::read(path).unwrap(), png());
    }
    #[test]
    fn resolves_embedded_desktop_icon_without_diricon() {
        let root = tempfile::tempdir().unwrap();
        let image = image_fixture(root.path(), "desktop", false, true);
        let path = resolve_icon_with(&image, None, &[], &root.path().join("cache")).unwrap();
        assert_eq!(fs::read(path).unwrap(), png());
    }
    #[test]
    fn rejects_paths_escaping_image_and_external_svg_content() {
        for path in [
            "../secret.png",
            "/etc/passwd",
            "a/../../secret.png",
            "a\nb.png",
        ] {
            assert!(safe_relative_path(path).is_none());
        }
        assert_eq!(
            safe_relative_path("icons/../fixture.png").as_deref(),
            Some("fixture.png")
        );
        assert!(icon_extension(br#"<svg><image href="file:///etc/passwd"/></svg>"#).is_none());
        assert!(
            icon_extension(br#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M0 0"/></svg>"#)
                .is_some()
        );
    }
    #[test]
    fn invalid_elf_boundaries_fail_without_running_image() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("bad.AppImage");
        let mut bytes = vec![0; 128];
        bytes[..4].copy_from_slice(b"\x7fELF");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[8..11].copy_from_slice(b"AI\x02");
        bytes[40..48].copy_from_slice(&u64::MAX.to_le_bytes());
        bytes[58..60].copy_from_slice(&64u16.to_le_bytes());
        bytes[60..62].copy_from_slice(&1u16.to_le_bytes());
        fs::write(&path, bytes).unwrap();
        assert_eq!(squashfs_offset(&path), None);
    }
}
