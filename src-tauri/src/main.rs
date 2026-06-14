mod commands {
    pub mod mock_data;
}

mod core {
    pub mod db;
}

use commands::mock_data::generate_mock_data;
use core::db::init_database;

fn main() {
    init_database().expect("failed to initialize SQLite database");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_mock_data])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
