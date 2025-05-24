use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src");

    let css = Command::new("npx")
        .arg("@tailwindcss/cli")
        .arg("-i")
        .arg("./src/tailwind.css")
        .arg("-o")
        .arg("assets/main.css")
        .arg("--minify")
        .output()
        .expect("Failed to execute tailwindcss");

    if !css.status.success() {
        panic!(
            "Failed to execute tailwindcss\n{}",
            String::from_utf8_lossy(&css.stderr)
        );
    }

    let js = Command::new("npx")
        .arg("rspack")
        .arg("--entry")
        .arg("./src/index.js")
        .arg("-o")
        .arg("assets")
        .output()
        .expect("Failed to execute rspack");

    if !js.status.success() {
        panic!(
            "Failed to execute rspack\n{}",
            String::from_utf8_lossy(&js.stderr)
        );
    }
}
