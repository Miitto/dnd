use std::process::Command;

fn npx() -> Command {
    #[cfg(windows)]
    const NPM: &str = "npx.cmd";
    #[cfg(not(windows))]
    const NPM: &str = "npx";

    Command::new(NPM)
}

fn main() {
    npx()
        .args([
            "tailwindcss",
            "-i",
            "./input.css",
            "-o",
            "./assets/styling/tailwind.css",
        ])
        .status()
        .expect("Failed to run npx for tailwindcss");
}
