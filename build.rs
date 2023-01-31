use std::fs;

pub fn main() {
    let cargo_toml =
        fs::read_to_string("./Cargo.toml").expect("Should have been able to read Cargo.toml");

    let mut new_version = String::new();

    for line in cargo_toml.lines() {
        if line.starts_with("version =") {
            let version_parts = line.split(" = ");
            let version = version_parts.last().expect("Didn't find the version");
            new_version.push_str(version);
        }
    }

    let contents =
        fs::read_to_string("./distribute.sh").expect("Should have been able to read the file");

    let build_sh_version = format!("version=v{new_version}");
    let new_content = contents
        .lines()
        .map(|line| {
            if line.starts_with("version=") {
                build_sh_version.as_str()
            } else {
                line
            }
        })
        .collect::<Vec<&str>>()
        .join("\n");

    fs::write("./distribute.sh", new_content)
        .expect("Failed to update distribute.sh with new version.");

    let package_contents = fs::read_to_string("./npm_dist/package.json")
        .expect("Should have been able to read the file");

    let package_contents = replace_version("  \"version\": ", &new_version, package_contents);

    fs::write("./npm_dist/package.json", package_contents)
        .expect("Failed to update distribute.sh with new version.");
}

fn replace_version(matcher: &str, new_version: &str, content: String) -> String {
    let new_version = format!("{matcher}{new_version},");
    println!("{new_version }");
    content
        .lines()
        .map(|line| {
            if line.starts_with(matcher) {
                new_version.as_str()
            } else {
                line
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}
