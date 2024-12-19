use std::process::Command;

fn npx() -> Command {
    #[cfg(windows)]
    const NPM: &str = "npx.cmd";
    #[cfg(not(windows))]
    const NPM: &str = "npx";

    Command::new(NPM)
}

const BUILD_CSS_ON_BUILD: bool = false;

fn main() {
    if BUILD_CSS_ON_BUILD {
        build_css();
    }
}

fn build_css() {
    npx()
        .args([
            "tailwindcss",
            "-i",
            "./input.css",
            "-o",
            "./ui/assets/styling/tailwind.css",
        ])
        .status()
        .expect("Failed to run npx for tailwindcss");
}
