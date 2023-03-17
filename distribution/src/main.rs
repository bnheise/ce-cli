use cargo_toml::Manifest;
use npm_package_json::Package;
use serde::Deserialize;
use std::io::BufRead;
use std::path::Path;
use std::{
    fmt::Display,
    fs,
    io::{self, BufReader, Result},
    process::{Command, Stdio},
    thread,
};

const CLI_CARGO_TOML_PATH: &str = "../app/Cargo.toml";
const PACKAGE_JSON_PATH: &str = "../npm_dist/package.json";
const MAIN_PLATFORMS_PATH: &str = "../platforms.json";
const NPM_PLATFORMS_PATH: &str = "../npm_dist/platforms.json";
const DIST_DIR: &str = "../dist";
const BUILD_DIR: &str = "../build";
const NPM_README_PATH: &str = "../npm_dist/README.md";
const MAIN_README_PATH: &str = "../README.md";

fn main() -> Result<()> {
    let version = get_current_version()?;
    let release_path_string = format!("../changelogs/{version}.md");
    let release_file_path = Path::new(&release_path_string);
    if !release_file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Release file is missing",
        ));
    }
    let raw_json = fs::read_to_string(PACKAGE_JSON_PATH).expect("Failed to load package.json");

    let package_json = serde_json::from_str::<Package>(&raw_json)?;

    validate_version(&version, &package_json)?;

    clean_previous_builds()?;
    let platforms = get_platforms()?;
    run_builds(&platforms)?;
    update_package_json_version(&version, package_json)?;
    publish_to_github(&platforms, &version, &release_path_string);
    publish_to_cargo();
    update_npm_readme();
    update_npm_platforms();
    publish_to_npm();
    Ok(())
}

fn clean_previous_builds() -> Result<()> {
    fs::remove_dir_all(DIST_DIR).ok();
    fs::remove_dir_all(BUILD_DIR).ok();
    fs::create_dir_all(DIST_DIR)?;
    fs::create_dir_all(BUILD_DIR)?;

    Ok(())
}

fn run_builds(platforms: &[PlatformtDef]) -> Result<()> {
    let mut handles = Vec::with_capacity(platforms.len());

    for platform in platforms.iter() {
        let target = platform.rust_target.to_string();
        let binary_name = platform.binary_name.to_owned();
        let handle = thread::spawn(move || {
            println!("Starting build for {target}");
            let mut child = Command::new("cross")
                .args([
                    "build",
                    "--manifest-path",
                    "../app/Cargo.toml",
                    "--release",
                    "--target",
                    &target,
                ])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap_or_else(|err| panic!("Failed to build for {target}: {err}"));
            {
                let stdout = child.stdout.as_mut().unwrap();
                let stdout_reader = BufReader::new(stdout);
                let stdout_lines = stdout_reader.lines();

                for line in stdout_lines {
                    println!("{target}: {line:?}");
                }
            }

            child.wait().expect("Build process failed to complete.");

            let output_foldername = format!("ce-cli-{target}");
            let build_dir = format!("../build/{output_foldername}");
            let target_dir = format!("../target/{target}/release/{binary_name}");
            fs::create_dir_all(&build_dir).ok();

            let output = Command::new("cp")
                .args([&target_dir, &build_dir])
                .output()
                .expect("failed to move binary from target folder to build folder");

            println!("{output:?}");
            let output_tar_name = format!("../dist/{output_foldername}.tar.gz");
            let output = Command::new("tar")
                .args(["-C", &build_dir, "-czvf", &output_tar_name, "."])
                .output()
                .expect("failed to make tarball");

            println!("{output:?}");
        });
        handles.push((platform.rust_target.to_string(), handle));
    }

    for (target, handle) in handles.into_iter() {
        match handle.join() {
            Ok(()) => (),
            Err(_) => println!("Build thread failed for {target}"),
        }
    }

    Ok(())
}

fn publish_to_npm() {
    println!("Publishing to npm...");
    let output = Command::new("npm")
        .current_dir("../npm_dist")
        .args(["publish"])
        .output()
        .expect("failed to publish to npm");
    println!("{output:?}",);
    println!("Done!");
}

fn publish_to_cargo() {
    println!("Publishing to cargo...");
    let output = Command::new("cargo")
        .current_dir("../app")
        .args(["publish", "--allow-dirty"])
        .output()
        .expect("failed to publish to cargo");
    println!("{output:?}",);
    println!("Done!");
}

fn publish_to_github(platforms: &[PlatformtDef], version: &str, release_file_path: &str) {
    let filenames = platforms
        .iter()
        .map(|platform| format!("../dist/ce-cli-{}.tar.gz", platform.rust_target))
        .collect::<Vec<String>>();

    let mut args = vec![
        "release",
        "create",
        "--title",
        version,
        version,
        "--notes-file",
        release_file_path,
    ];

    for filename in filenames.iter() {
        args.push(filename);
    }

    println!("Publishing to Github...");

    let output = Command::new("gh")
        .args(args)
        .output()
        .expect("failed to move binary from target folder to build folder");
    println!("{output:?}");
    println!("Done!");
}

fn get_current_version() -> Result<String> {
    let cargo = Manifest::from_path(CLI_CARGO_TOML_PATH)
        .map_err(|err| io::Error::new(io::ErrorKind::NotFound, err))?;

    let package = cargo.package();

    Ok(package.version().to_owned())
}

fn validate_version(version: &str, package_json: &Package) -> Result<()> {
    if package_json.version == version {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cargo.toml version is the same as Package.json. Did you forget to update the version in app/Cargo.toml?",
        ));
    }

    Ok(())
}

fn update_package_json_version(version: &str, mut package_json: Package) -> Result<()> {
    if package_json.version == version {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cargo.toml version is the same as Package.json. Did you forget to update the version in app/Cargo.toml?",
        ));
    }

    package_json.version = version.to_owned();

    let raw_json = serde_json::to_string_pretty(&package_json)?;

    fs::write(PACKAGE_JSON_PATH, raw_json)
        .expect("Failed to update distribute.sh with new version.");

    Ok(())
}

fn get_platforms() -> Result<Vec<PlatformtDef>> {
    let raw_json = fs::read_to_string(MAIN_PLATFORMS_PATH).expect("Failed to load platforms.json");
    Ok(serde_json::from_str::<Vec<PlatformtDef>>(&raw_json)?)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct PlatformtDef {
    #[serde(rename = "TYPE")]
    _type: OsType,
    #[allow(dead_code)]
    architecture: OsArchitecture,
    rust_target: RustTarget,
    binary_name: String,
}

#[derive(Debug, Deserialize)]
enum OsType {
    #[serde(rename = "Windows_NT")]
    WindowsNT,
    Linux,
    Darwin,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum OsArchitecture {
    X64,
    X32,
}

#[derive(Debug, Deserialize)]
#[allow(clippy::enum_variant_names)]
enum RustTarget {
    #[serde(rename = "x86_64-pc-windows-gnu")]
    X86_64PcWindowsGnu,
    #[serde(rename = "x86_64-unknown-linux-gnu")]
    X86_64UnknownLinuxGnu,
    #[serde(rename = "x86_64-unknown-linux-musl")]
    X86_64UnknownLinuxMusl,
    #[serde(rename = "x86_64-apple-darwin")]
    X86_64AppleDarwin,
    #[serde(rename = "x86_64-pc-windows-msvc")]
    X86_64PcWindowsMsvc,
}

impl Display for RustTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustTarget::X86_64PcWindowsGnu => write!(f, "x86_64-pc-windows-gnu"),
            RustTarget::X86_64UnknownLinuxMusl => write!(f, "x86_64-unknown-linux-musl"),
            RustTarget::X86_64AppleDarwin => write!(f, "x86_64-apple-darwin"),
            RustTarget::X86_64UnknownLinuxGnu => write!(f, "x86_64-unknown-linux-gnu"),
            RustTarget::X86_64PcWindowsMsvc => write!(f, "x86_64-pc-windows-msvc"),
        }
    }
}

fn update_npm_readme() {
    fs::copy(MAIN_README_PATH, NPM_README_PATH).expect("Failed to copy the file");
}

fn update_npm_platforms() {
    fs::copy(MAIN_PLATFORMS_PATH, NPM_PLATFORMS_PATH).expect("Failed to copy the file");
}
