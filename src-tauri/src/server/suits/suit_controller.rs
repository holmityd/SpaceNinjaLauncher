use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::server::{
    inventories::{inventory_model::Inventory, inventory_repo::InventoryRepo},
    suits::{
        suit_model::{RequestInventorySuitsRemove, ResponseInventorySuits, Suit},
        suit_service::{add_suits, remove_suits, update_suit},
    },
};

// #[post("/suits/add/{id}")]
// pub async fn suits_add(
//     db: Data<InventoryRepo>,
//     path: Path<String>,
//     post_item: Json<Vec<Suit>>,
// ) -> HttpResponse {
//     let account_id = path.into_inner();
//     if account_id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid account_id");
//     };

//     let inventory = db
//         .get_inventory(&account_id)
//         .await
//         .expect("Inventory not found");

//     let suits = add_suits(inventory, post_item.into_inner());

//     let data = Inventory {
//         suits: Some(suits.clone()),
//         ..Default::default()
//     };

//     let update_result = db.update_inventory(&account_id, data).await;

//     match update_result {
//         Ok(update) => {
//             if update.matched_count == 1 {
//                 let inventory_suits = ResponseInventorySuits { suits };
//                 HttpResponse::Ok().json(inventory_suits)
//             } else {
//                 return HttpResponse::NotFound().body("No inventory found with specified ID");
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[post("/suits/remove/{id}")]
// pub async fn suits_remove(
//     db: Data<InventoryRepo>,
//     path: Path<String>,
//     post_item: Json<Vec<RequestInventorySuitsRemove>>,
// ) -> HttpResponse {
//     let account_id = path.into_inner();
//     if account_id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid account_id");
//     };

//     let inventory = db
//         .get_inventory(&account_id)
//         .await
//         .expect("Inventory not found");

//     let suits = remove_suits(inventory, post_item.into_inner());

//     let data = Inventory {
//         suits: Some(suits.clone()),
//         ..Default::default()
//     };

//     let update_result = db.update_inventory(&account_id, data).await;

//     match update_result {
//         Ok(update) => {
//             if update.matched_count == 1 {
//                 let inventory_suits = ResponseInventorySuits { suits };
//                 HttpResponse::Ok().json(inventory_suits)
//             } else {
//                 return HttpResponse::NotFound().body("No inventory found with specified ID");
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[post("/suits/update/{id}")]
// pub async fn suit_update(
//     db: Data<InventoryRepo>,
//     path: Path<String>,
//     post_item: Json<Suit>,
// ) -> HttpResponse {
//     let account_id = path.into_inner();
//     if account_id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid account_id");
//     };

//     let inventory = db
//         .get_inventory(&account_id)
//         .await
//         .expect("Inventory not found");

//     let suits = update_suit(inventory, post_item.into_inner());

//     let data = Inventory {
//         suits: Some(suits.clone()),
//         ..Default::default()
//     };

//     let update_result = db.update_inventory(&account_id, data).await;

//     match update_result {
//         Ok(update) => {
//             if update.matched_count == 1 {
//                 let inventory_suits = ResponseInventorySuits { suits };
//                 HttpResponse::Ok().json(inventory_suits)
//             } else {
//                 return HttpResponse::NotFound().body("No inventory found with specified ID");
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

enum SuitAction {
    Update(Suit),
    Add(Vec<Suit>),
    Remove(Vec<RequestInventorySuitsRemove>),
}

async fn handle_mod_action(
    db: Data<InventoryRepo>,
    path: Path<String>,
    action: SuitAction,
) -> HttpResponse {
    let account_id = path.into_inner();
    if account_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid account_id");
    };

    let inventory = db
        .get_inventory(&account_id)
        .await
        .expect("Inventory not found");

    let suits = match action {
        SuitAction::Update(post_item) => update_suit(inventory, post_item),
        SuitAction::Add(post_item) => add_suits(inventory, post_item),
        SuitAction::Remove(post_item) => remove_suits(inventory, post_item),
    };

    let data = Inventory {
        suits: Some(suits.clone()),
        ..Default::default()
    };

    let update_result = db.update_inventory(&account_id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let inventory_suits = ResponseInventorySuits { suits };
                HttpResponse::Ok().json(inventory_suits)
            } else {
                return HttpResponse::NotFound().body("No inventory found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/suits/update/{id}")]
pub async fn suits_update(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<Suit>,
) -> HttpResponse {
    let action = SuitAction::Update(post_item.into_inner());
    handle_mod_action(db, path, action).await
}

#[post("/suits/add/{id}")]
pub async fn suits_add(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<Vec<Suit>>,
) -> HttpResponse {
    let action = SuitAction::Add(post_item.into_inner());
    handle_mod_action(db, path, action).await
}

#[post("/suits/remove/{id}")]
pub async fn suits_remove(
    db: Data<InventoryRepo>,
    path: Path<String>,
    post_item: Json<Vec<RequestInventorySuitsRemove>>,
) -> HttpResponse {
    let action = SuitAction::Remove(post_item.into_inner());
    handle_mod_action(db, path, action).await
}
