use std::io::BufRead;
use std::path::Path;
use std::{
    fmt::Display,
    fs,
    io::{self, BufReader, Result},
    process::{Command, Stdio},
    thread,
};

use cargo_toml::Manifest;
use npm_package_json::Package;
use serde::Deserialize;

const CLI_CARGO_TOML_PATH: &str = "../app/Cargo.toml";
const PACKAGE_JSON_PATH: &str = "../npm_dist/package.json";
const PLATFORMS_PATH: &str = "../npm_dist/platforms.json";
const DIST_DIR: &str = "../dist";
const BUILD_DIR: &str = "../build";

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
    update_package_json_version(&version)?;
    clean_previous_builds()?;
    let platforms = get_platforms()?;
    run_builds(&platforms)?;
    publish_to_github(&platforms, &version, &release_path_string);
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

fn run_builds(platforms: &Vec<PlatformtDef>) -> Result<()> {
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
        .expect("failed to move binary from target folder to build folder");
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

fn update_package_json_version(version: &str) -> Result<()> {
    let raw_json = fs::read_to_string(PACKAGE_JSON_PATH).expect("Failed to load package.json");

    let mut package_json = serde_json::from_str::<Package>(&raw_json)?;

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
    let raw_json = fs::read_to_string(PLATFORMS_PATH).expect("Failed to load platforms.json");
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
    #[serde(rename = "x86_64-unknown-linux-musl")]
    X86_64UnknownLinuxMusl,
    #[serde(rename = "x86_64-apple-darwin")]
    X86_64AppleDarwin,
}

impl Display for RustTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustTarget::X86_64PcWindowsGnu => write!(f, "x86_64-pc-windows-gnu"),
            RustTarget::X86_64UnknownLinuxMusl => write!(f, "x86_64-unknown-linux-musl"),
            RustTarget::X86_64AppleDarwin => write!(f, "x86_64-apple-darwin"),
        }
    }
}
// {
//   "TYPE": "Windows_NT",
//   "ARCHITECTURE": "x64",
//   "RUST_TARGET": "x86_64-pc-windows-msvc",
//   "BINARY_NAME": "ce-cli.exe"
// },
