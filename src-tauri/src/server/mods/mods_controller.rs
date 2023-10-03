use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::{
    server::inventories::{inventory_model::Inventory, inventory_repo::InventoryRepo},
    server::mods::{
        mods_model::{RequestInventoryMod, RequestInventoryModUpdate},
        mods_service::{add_mods, remove_mods, upgrade_or_degrade_mod},
    },
};

use super::mods_model::ResponseInventoryMods;

enum ModAction {
    Update(RequestInventoryModUpdate),
    Add(Vec<RequestInventoryMod>),
    Remove(Vec<RequestInventoryMod>),
}

async fn handle_mod_action(
    db: Data<InventoryRepo>,
    path: Path<String>,
    action: ModAction,
) -> HttpResponse {
    let account_id = path.into_inner();
    if account_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid account_id");
    };

    let inventory = db
        .get_inventory(&account_id)
        .await
        .expect("Inventory not found");

    let (raw_upgrades, upgrades) = match action {
        ModAction::Update(post_item) => upgrade_or_degrade_mod(inventory, post_item),
        ModAction::Add(post_item) => add_mods(inventory, post_item),
        ModAction::Remove(post_item) => remove_mods(inventory, post_item),
    };

    let data = Inventory {
        raw_upgrades: Some(raw_upgrades.clone()),
        upgrades: Some(upgrades.clone()),
        ..Default::default()
    };

    let update_result = db.update_inventory(&account_id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let inventory_mods = ResponseInventoryMods {
                    raw_upgrades,
                    upgrades,
                };
                HttpResponse::Ok().json(inventory_mods)
            } else {
                return HttpResponse::NotFound().body("No inventory found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/mods/update/{id}")]
pub async fn mod_update(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<RequestInventoryModUpdate>,
) -> HttpResponse {
    let action = ModAction::Update(post_item.into_inner());
    handle_mod_action(db, path, action).await
}

#[post("/mods/add/{id}")]
pub async fn mods_add(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<Vec<RequestInventoryMod>>,
) -> HttpResponse {
    let action: ModAction = ModAction::Add(post_item.into_inner());
    handle_mod_action(db, path, action).await
}

#[post("/mods/remove/{id}")]
pub async fn mods_remove(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<Vec<RequestInventoryMod>>,
) -> HttpResponse {
    let action = ModAction::Remove(post_item.into_inner());
    handle_mod_action(db, path, action).await
}
