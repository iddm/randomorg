use vergen;

fn create_build_info() {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    let mut f = File::create(&dest_path).unwrap();

    let features = env::vars()
        .filter(|t| t.0.starts_with("CARGO_FEATURE_"))
        .map(|t| t.0.replace("CARGO_FEATURE_", "").to_lowercase())
        .collect::<Vec<String>>()
        .join(", ");
    let mut string = format!(
        "/// Features the library was built with
pub fn features() -> &'static str {{\n\t\"{}\"\n}}\n\n",
        features
    );
    f.write_all(&string.into_bytes()).unwrap();
    string = format!(
        "/// Build profile information\n
pub fn profile() -> &'static str {{\n\t\"{}\"\n}}\n",
        env::var("PROFILE").unwrap()
    );
    f.write_all(&string.into_bytes()).unwrap();
}

fn main() {
    use vergen::{generate_cargo_keys, ConstantsFlags};
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);
    generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate the cargo keys!");
    create_build_info();
}
