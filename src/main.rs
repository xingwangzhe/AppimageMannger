mod chrome;
mod frontends;
mod i18n;
mod manager;
mod metadata;
mod ui;
mod update;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|a| a == "--rpc") {
        println!(
            "{}",
            frontends::dispatch_json(args.get(1).map(String::as_str).unwrap_or("{}"))
        );
        return Ok(());
    }
    if args.first().is_some_and(|a| a == "--export-frontends") {
        let folder = std::path::Path::new(
            args.get(1)
                .ok_or_else(|| anyhow::anyhow!("Missing export directory"))?,
        );
        println!("{}", frontends::export_frontends(folder)?);
        return Ok(());
    }
    if args.iter().any(|a| a == "--version") {
        println!("AppimageMannger {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.iter().any(|a| a == "--help") {
        println!(
            "AppimageMannger — Ely GPUI AppImage manager\n\nUsage: appimage-mannger [--list | --smoke-test] [--directory PATH]\n\nNo arguments opens the desktop application. --list prints the managed apps as JSON.\n--smoke-test checks installation and scans without launching or modifying applications."
        );
        return Ok(());
    }
    if args.iter().any(|a| a == "--list" || a == "--smoke-test") {
        let mut config = manager::Config::load()?;
        if let Some(i) = args.iter().position(|a| a == "--directory") {
            config.applications_dir = args
                .get(i + 1)
                .ok_or_else(|| anyhow::anyhow!("--directory requires a path"))?
                .into();
        }
        let apps = manager::scan(&config)?;
        let summary: Vec<_> = apps.iter().map(|a| serde_json::json!({"name":a.name,"version":a.version,"path":a.path,"size_bytes":a.size_bytes,"repository":a.repository,"icon_path":a.icon_path})).collect();
        println!(
            "{}",
            serde_json::to_string_pretty(
                &serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"directory":config.applications_dir,"applications":summary})
            )?
        );
        return Ok(());
    }
    if !args.is_empty() {
        anyhow::bail!("Unknown argument. Use --help.");
    }
    frontends::launch_if_native(manager::Config::load().unwrap_or_default().window_style)?;
    ui::run();
    Ok(())
}
