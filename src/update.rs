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
    time::Duration,
};

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
            status: "未配置更新源：请填写此应用的 GitHub 仓库".into(),
        });
    };
    let repo = manager::normalize_repository(repo)?;
    check_update_at(
        app,
        &format!("https://api.github.com/repos/{repo}/releases/latest"),
    )
}
fn client(timeout: u64) -> Result<Client> {
    Ok(Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(timeout))
        .user_agent(concat!("AppimageMannger/", env!("CARGO_PKG_VERSION")))
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.previous().len() > 8 {
                return attempt.error("重定向次数过多");
            }
            if allowed_url(attempt.url().as_str()) {
                attempt.follow()
            } else {
                attempt.error("拒绝不安全的更新重定向")
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
    ensure!(allowed_url(url), "更新 API 必须使用 HTTPS");
    let response = client(35)?
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .context("无法连接更新服务器")?;
    let status = response.status();
    ensure!(
        status.is_success(),
        "检查更新失败，服务器返回 {status}（GitHub API 可能限流）"
    );
    let mut bytes = Vec::new();
    response.take(4 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 4 * 1024 * 1024, "更新 API 响应过大");
    let release: Release = serde_json::from_slice(&bytes).context("更新服务器的响应格式无效")?;
    ensure!(
        !release.draft && !release.prerelease,
        "最新版本不是稳定版本"
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
        .map(|a| architecture_score(&a.name, std::env::consts::ARCH))
        .max()
        .unwrap_or(0);
    candidates.retain(|a| architecture_score(&a.name, std::env::consts::ARCH) == best);
    let ambiguous = candidates.len() > 1;
    let asset = if candidates.len() == 1 {
        candidates.pop()
    } else {
        None
    };
    let status = if comparison.is_none() {
        "无法比较当前版本；请核对发布页面"
    } else if !newer {
        "已是最新版本"
    } else if ambiguous {
        "存在多个适合的安装包，请在发布页面核对"
    } else if asset.is_none() {
        "存在新版本，但没有适合本机架构的 AppImage"
    } else {
        "发现新版本，可以安装更新"
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
    manager::ensure_managed_path(config, &app.path)?;
    manager::validate_appimage(&app.path)?;
    ensure!(
        asset_matches(&asset.name, std::env::consts::ARCH),
        "更新包架构与本机不匹配"
    );
    ensure!(allowed_url(&asset.url), "更新下载地址必须使用 HTTPS");
    ensure!(
        asset.size > 64 && asset.size <= 4 * 1024 * 1024 * 1024,
        "更新包大小不合法"
    );
    let mut download = tempfile::NamedTempFile::new_in(&config.applications_dir)?;
    let mut response = client(300)?
        .get(&asset.url)
        .send()
        .context("下载更新失败")?
        .error_for_status()?;
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
            total <= asset.size && total <= 4 * 1024 * 1024 * 1024,
            "下载包大小超出预期"
        );
        hasher.update(&buf[..count]);
        download.write_all(&buf[..count])?;
    }
    ensure!(
        total == asset.size,
        "下载不完整：预期 {} 字节，实际 {total} 字节",
        asset.size
    );
    let computed = hex::encode(hasher.finalize());
    if let Some(digest) = &asset.digest {
        let expected = digest
            .strip_prefix("sha256:")
            .context("服务器提供了不支持的校验算法")?;
        ensure!(
            expected.eq_ignore_ascii_case(&computed),
            "SHA-256 校验失败，原应用保持不变"
        );
    }
    let header = manager::validate_appimage(download.path())?;
    let machine = u16::from_le_bytes([header[18], header[19]]);
    ensure!(
        (std::env::consts::ARCH == "x86_64" && machine == 62)
            || (std::env::consts::ARCH == "aarch64" && machine == 183),
        "下载文件的 ELF 架构与本机不匹配"
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
        return Err(error.context("更新未完成，已恢复原应用"));
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
        };
        let app = manager::inspect(&config, &p).unwrap();
        let check = check_update_at(&app, &serve(body.into_bytes())).unwrap();
        assert_eq!(check.latest_version, "1.3.0");
        assert!(check.asset.is_some());
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
            };
            let app = manager::inspect(&config, &p).unwrap();
            let mut new = fixture();
            new[100] = 42;
            let asset = UpdateAsset {
                name: format!("Tool-{}.AppImage", std::env::consts::ARCH),
                url: serve(new.clone()),
                size: new.len() as u64,
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
    fn invalid_download_preserves_original() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("Tool-1.2.0.AppImage");
        let old = fixture();
        fs::write(&p, &old).unwrap();
        let config = Config {
            applications_dir: dir.path().into(),
            repositories: BTreeMap::new(),
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
