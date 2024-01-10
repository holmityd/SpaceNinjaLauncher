mod accounts;
mod inventories;
mod mods;
mod suits;
use accounts::{account_controller::get_all_accounts, account_repo::AccountRepo};

use dotenv::dotenv;

use actix_cors::Cors;
use actix_web::{http, middleware, web, web::Data, App, HttpServer};

use mongodb::{Client, Database};

use inventories::{inventory_controller::get_inventory_by_user_id, inventory_repo::InventoryRepo};
use mods::mods_controller::{mod_update, mods_add, mods_remove};
use suits::suit_controller::suits_add;

use std::{io::Read, sync::Mutex, fs::File};

use tauri::AppHandle;

use self::suits::suit_controller::{suits_remove, suits_update};

pub struct TauriAppState {
    pub app: Mutex<AppHandle>,
}

const DEFAULT_MONGODB_URL: &str = "mongodb://127.0.0.1:27017/openWF";

fn read_mongodb_url_from_file(resource_path: &str) -> String {
    let mut file = match File::open(resource_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error reading the file or file not found. Using default URI.");
            return String::from(DEFAULT_MONGODB_URL);
        }
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file content.");
        return String::from(DEFAULT_MONGODB_URL);
    }

    let config: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error parsing JSON content.");
            return String::from(DEFAULT_MONGODB_URL);
        }
    };

    if let Some(url) = config.get("mongodbUrl").and_then(|u| u.as_str()) {
        return url.to_string();
    } else {
        eprintln!("'mongodbUrl' parameter not found or not a string.");
    }

    String::from(DEFAULT_MONGODB_URL)
}

pub async fn get_mongo_database() -> Database {
    dotenv().ok();
    let uri = read_mongodb_url_from_file("config.json");
    let db_name = uri.split('/').last().unwrap_or("No last part found.");
    let client = Client::with_uri_str(&uri)
        .await
        .expect("error connecting to database");
    let db: mongodb::Database = client.database(db_name);
    db
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    let db = get_mongo_database().await;

    let account_repo = AccountRepo::new(db.clone()).await;
    let inventory_repo = InventoryRepo::new(db.clone()).await;

    let account_data = Data::new(account_repo);
    let inventory_data = Data::new(inventory_repo);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::ACCEPT,
                http::header::ACCEPT_LANGUAGE,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(tauri_app.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(account_data.clone())
            .app_data(inventory_data.clone())
            .service(get_all_accounts)
            .service(get_inventory_by_user_id)
            .service(mod_update)
            .service(mods_add)
            .service(mods_remove)
            .service(suits_add)
            .service(suits_remove)
            .service(suits_update)
    })
    .bind(("127.0.0.1", 53426))?
    .run()
    .await
}
