mod commands {
    pub mod customers;
    pub mod mock_data;
    pub mod system;
    pub mod users;
}

mod core {
    pub mod db;
    pub mod error;
}

mod services {
    pub mod api_client;
    pub mod local_api;
}

use commands::{
    customers::{get_local_customers, sync_customers_from_cloud},
    mock_data::generate_mock_data,
    system::{generate_local_api_key, get_local_api_status, revoke_local_api_key},
    users::{
        get_operator_status, reset_password_with_recovery_key, save_api_token, setup_operator,
        unlock_operator,
    },
};
use core::db::init_database;

fn main() {
    init_database().expect("failed to initialize SQLite database");

    tauri::Builder::default()
        .setup(|_| {
            services::local_api::spawn_local_api_server();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_mock_data,
            sync_customers_from_cloud,
            get_local_customers,
            get_local_api_status,
            generate_local_api_key,
            revoke_local_api_key,
            get_operator_status,
            setup_operator,
            unlock_operator,
            save_api_token,
            reset_password_with_recovery_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
