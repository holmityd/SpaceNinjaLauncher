use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

use crate::server::inventories::inventory_repo::InventoryRepo;

#[get("/inventory/{id}")]
pub async fn get_inventory_by_user_id(db: Data<InventoryRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let inventory_detail = db.get_inventory(&id).await;

    match inventory_detail {
        Ok(inventory) => HttpResponse::Ok().json(inventory),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
