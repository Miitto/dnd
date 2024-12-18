use dioxus::prelude::*;
#[allow(unused_imports)] // Weird behavior with the macro
use std::time::{SystemTime, UNIX_EPOCH};

#[server]
pub async fn server_log(message: String) -> Result<(), ServerFnError> {
    println!("Server: {}", message);

    Ok(())
}

#[server]
pub async fn is_alive() -> Result<u64, ServerFnError> {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time Went backwards");
    let since_epoch = since_epoch.as_secs();

    Ok(since_epoch)
}
