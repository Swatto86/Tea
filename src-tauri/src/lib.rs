// Mobile entry point with proper error handling
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let result = tauri::Builder::default()
        .run(tauri::generate_context!());
    
    if let Err(e) = result {
        eprintln!("Fatal error running Tauri application: {}", e);
        std::process::exit(1);
    }
}
