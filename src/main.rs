use dioxus::logger::tracing;

fn main() {
    dioxus::logger::initialize_default();
    tracing::info!("============================= Starting Dioxus =============================");

    #[cfg(feature = "desktop")]
    fn launch_app() {
        use dioxus::desktop::WindowBuilder;

        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::default()
                    .with_menu(None)
                    .with_window(WindowBuilder::new().with_title("DnD 5e")),
            )
            .launch(ui::App);
    }

    #[cfg(not(feature = "desktop"))]
    fn launch_app() {
        dioxus::launch(ui::App);
    }

    launch_app();
}
