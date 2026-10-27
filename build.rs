use std::{env, path::PathBuf, process::Command};
fn build(source: &str, compiler: &str, packages: &[&str], output: &str) {
    println!("cargo:rerun-if-changed={source}");
    let flags = Command::new("pkg-config")
        .arg("--cflags")
        .arg("--libs")
        .args(packages)
        .output()
        .expect("pkg-config is required");
    assert!(
        flags.status.success(),
        "Install native Qt6 / GTK4 / libadwaita / json-glib development packages"
    );
    let dest = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join(output);
    let mut cmd = Command::new(compiler);
    cmd.arg("-I").arg(env::var_os("OUT_DIR").unwrap());
    cmd.args(["-O2", "-fPIC", source, "-o"]).arg(dest);
    if compiler == "c++" {
        cmd.arg("-std=c++17");
    }
    cmd.args(String::from_utf8(flags.stdout).unwrap().split_whitespace());
    assert!(
        cmd.status()
            .expect("native frontend compiler failed")
            .success(),
        "Native {output} build failed"
    );
}
fn main() {
    catalogs();
    build("native/qt.cpp", "c++", &["Qt6Widgets"], "frontend-qt");
    build(
        "native/gnome.c",
        "cc",
        &["gtk4", "libadwaita-1", "json-glib-1.0"],
        "frontend-gnome",
    );
}

fn catalogs() {
    let en = std::fs::read_to_string("resources/i18n/en.json").unwrap();
    std::fs::write(
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("i18n_fallback.h"),
        format!("static const char APPIMAGE_EN_JSON[] = {:?};\n", en),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=resources/i18n");
    let mut files: Vec<_> = std::fs::read_dir("resources/i18n")
        .expect("Translation directory missing")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort();
    let contents = files
        .iter()
        .map(|p| {
            println!("cargo:rerun-if-changed={}", p.display());
            format!(
                "\"{}\":{}",
                p.file_stem().unwrap().to_string_lossy(),
                std::fs::read_to_string(p).unwrap()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    std::fs::write(
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("catalogs.json"),
        format!("{{{contents}}}"),
    )
    .unwrap();
}
