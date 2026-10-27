//! Real toolkit processes share the Rust engine and one persistent configuration.
use crate::{
    manager::{self, Config, WindowStyle},
    update,
};
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    os::unix::{fs::PermissionsExt, process::CommandExt},
    path::PathBuf,
    process::Command,
};

pub fn dispatch_json(request: &str) -> Value {
    match dispatch(request) {
        Ok(data) => json!({"ok":true,"data":data,"error":null}),
        Err(e) => json!({"ok":false,"data":null,"error":format!("{e:#}")}),
    }
}
fn dispatch(request: &str) -> Result<Value> {
    ensure!(request.len() < 1024 * 1024, "Request is too large");
    let request: Value = serde_json::from_str(request)?;
    let action = request
        .get("action")
        .or_else(|| request.get("op"))
        .or_else(|| request.get("command"))
        .and_then(Value::as_str)
        .context("Missing action")?;
    let config = Config::load()?;
    if action == "scan" {
        return Ok(
            json!({"translations":crate::i18n::translations(&config.language),"resolved_language":crate::i18n::resolve(&config.language),"locales":crate::i18n::locale_list(),"config":config,"apps":manager::scan(&config)?,"version":env!("CARGO_PKG_VERSION")}),
        );
    }
    if action == "save_config" {
        let mut next: Config = serde_json::from_value(
            request
                .get("config")
                .context("Missing configuration")?
                .clone(),
        )?;
        ensure!(
            next.applications_dir.is_absolute(),
            "Applications directory must be an absolute path"
        );
        ensure!(
            next.window_width.is_finite() && next.window_height.is_finite(),
            "Invalid window size"
        );
        next.window_width = next.window_width.clamp(880.0, 7680.0);
        next.window_height = next.window_height.clamp(580.0, 4320.0);
        for repo in next.repositories.values_mut() {
            *repo = manager::normalize_repository(repo)?;
        }
        if next.applications_dir != config.applications_dir {
            fs::create_dir_all(&next.applications_dir)?;
        }
        next.save()?;
        return Ok(serde_json::to_value(next)?);
    }
    if action == "open_directory" {
        Command::new("xdg-open")
            .arg(&config.applications_dir)
            .spawn()?;
        return Ok(json!({"message":"Application directory opened"}));
    }
    let path = PathBuf::from(
        request
            .get("path")
            .and_then(Value::as_str)
            .context("Missing application path")?,
    );
    if action == "import" {
        return Ok(serde_json::to_value(manager::import_app(&config, &path)?)?);
    }
    manager::ensure_managed_path(&config, &path)?;
    let app = manager::inspect(&config, &path)?;
    match action {
        "launch" => {
            manager::launch(&app)?;
            Ok(json!({"message":"Application launched"}))
        }
        "uninstall" => Ok(json!({"trash_path":manager::uninstall(&config,&app)?})),
        "integrate" => {
            manager::integrate_app(&config, &app)?;
            Ok(json!({"message":"Desktop entry repaired"}))
        }
        "check_update" => Ok(serde_json::to_value(update::check_update(&app)?)?),
        "install_update" => {
            let asset = serde_json::from_value(
                request
                    .get("asset")
                    .context("Missing update asset")?
                    .clone(),
            )?;
            Ok(serde_json::to_value(update::install_update(
                &config, &app, &asset,
            )?)?)
        }
        _ => bail!("Unknown action: {action}"),
    }
}

pub fn export_frontends(folder: &std::path::Path) -> Result<Value> {
    fs::create_dir_all(folder)?;
    let frontends: [(&str, &[u8]); 2] = [
        (
            "frontend-qt",
            include_bytes!(concat!(env!("OUT_DIR"), "/frontend-qt")),
        ),
        (
            "frontend-gnome",
            include_bytes!(concat!(env!("OUT_DIR"), "/frontend-gnome")),
        ),
    ];
    for (name, bytes) in frontends {
        let path = folder.join(name);
        fs::write(&path, bytes)?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
    }
    Ok(json!({"qt":folder.join("frontend-qt"),"gnome":folder.join("frontend-gnome")}))
}

pub fn launch_if_native(style: WindowStyle) -> Result<()> {
    if style == WindowStyle::Gpui {
        return Ok(());
    }
    let (name, bytes): (&str, &[u8]) = match style {
        WindowStyle::Qt => (
            "frontend-qt",
            include_bytes!(concat!(env!("OUT_DIR"), "/frontend-qt")),
        ),
        WindowStyle::Gnome => (
            "frontend-gnome",
            include_bytes!(concat!(env!("OUT_DIR"), "/frontend-gnome")),
        ),
        WindowStyle::Gpui => return Ok(()),
    };
    let engine = std::env::current_exe()?;
    let packaged = engine
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("lib/appimage-mannger").join(name));
    if let Some(path) = packaged.filter(|p| p.is_file()) {
        let error = Command::new(path)
            .env("APPIMAGE_MAN_NGER_ENGINE", &engine)
            .exec();
        return Err(error).context("Failed to launch native toolkit frontend");
    }
    let folder = dirs::cache_dir()
        .context("Cannot determine the cache directory")?
        .join("appimage-mannger/native")
        .join(env!("CARGO_PKG_VERSION"));
    fs::create_dir_all(&folder)?;
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o700))?;
    let target = folder.join(name);
    let mut temp = tempfile::NamedTempFile::new_in(&folder)?;
    temp.write_all(bytes)?;
    temp.as_file()
        .set_permissions(fs::Permissions::from_mode(0o700))?;
    temp.as_file().sync_all()?;
    temp.persist(&target).map_err(|e| e.error)?;
    let error = Command::new(target)
        .env("APPIMAGE_MAN_NGER_ENGINE", std::env::current_exe()?)
        .exec();
    Err(error).context("Failed to launch native toolkit frontend")
}
