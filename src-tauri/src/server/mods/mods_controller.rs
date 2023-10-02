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

#[post("/mods/update/{id}")]
pub async fn mod_update(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<RequestInventoryModUpdate>,
) -> HttpResponse {
    let account_id = path.into_inner();
    if account_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid account_id");
    };

    let inventory = db
        .get_inventory(&account_id)
        .await
        .expect("Inventory not found");

    let (raw_upgrades, upgrades) = upgrade_or_degrade_mod(inventory, post_item.into_inner());

    let data = Inventory {
        raw_upgrades: Some(raw_upgrades),
        upgrades: Some(upgrades),
        ..Default::default()
    };

    let update_result = db.update_inventory(&account_id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_inventory_info = db.get_inventory(&account_id).await;

                return match updated_inventory_info {
                    Ok(inventory) => HttpResponse::Ok().json(inventory),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No inventory found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/mods/add/{id}")]
pub async fn mods_add(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<Vec<RequestInventoryMod>>,
) -> HttpResponse {
    let account_id = path.into_inner();
    if account_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid account_id");
    };

    let inventory = db
        .get_inventory(&account_id)
        .await
        .expect("Inventory not found");

    let (raw_upgrades, upgrades) = add_mods(inventory, post_item.into_inner());

    let data = Inventory {
        raw_upgrades: Some(raw_upgrades),
        upgrades: Some(upgrades),
        ..Default::default()
    };

    let update_result = db.update_inventory(&account_id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_inventory_info = db.get_inventory(&account_id).await;

                return match updated_inventory_info {
                    Ok(inventory) => HttpResponse::Ok().json(inventory),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No inventory found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/mods/remove/{id}")]
pub async fn mods_remove(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<Vec<RequestInventoryMod>>,
) -> HttpResponse {
    // HttpResponse::Ok().json("good")
    let account_id = path.into_inner();
    if account_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid account_id");
    };

    let inventory = db
        .get_inventory(&account_id)
        .await
        .expect("Inventory not found");

    let (raw_upgrades, upgrades) = remove_mods(inventory, post_item.into_inner());

    let data = Inventory {
        raw_upgrades: Some(raw_upgrades),
        upgrades: Some(upgrades),
        ..Default::default()
    };

    let update_result = db.update_inventory(&account_id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_inventory_info = db.get_inventory(&account_id).await;

                return match updated_inventory_info {
                    Ok(inventory) => HttpResponse::Ok().json(inventory),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No inventory found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
