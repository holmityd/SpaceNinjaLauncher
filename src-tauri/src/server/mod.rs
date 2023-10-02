mod handlers;

use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};
use tauri::AppHandle;

use self::handlers::example::test;

pub struct TauriAppState {
    pub app: Mutex<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(tauri_app.clone())
            .wrap(middleware::Logger::default())
            .service(test)
    })
    .bind(("127.0.0.1", 4875))?
    .run()
    .await
}
