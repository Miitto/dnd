use dioxus::logger::tracing;

fn main() {
    dioxus::logger::initialize_default();
    tracing::info!("============================= Starting Dioxus =============================");

    dioxus::launch(ui::App);
}
