use serde_json::json;

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::{
    server::accounts::{
        account_model::{Account, RequestAccountCreationData, RequestAccountUpdateData},
        account_repo::AccountRepo,
    },
    server::inventories::inventory_repo::InventoryRepo,
};

#[post("/account")]
pub async fn create_account(
    data_account: Data<AccountRepo>,
    data_inventory: Data<InventoryRepo>,
    data: Json<RequestAccountCreationData>,
) -> HttpResponse {
    let new_account = Account {
        id: None,
        email: data.email.clone(),
        password: data.password.clone(),
        display_name: data.display_name.clone(),
        country_code: "EN".to_owned(),
        client_type: "".to_owned(),
        cross_platform_allowed: true,
        force_logout_version: 0,
        consent_needed: false,
        tracked_settings: [].to_vec(),
        version: 0,
    };

    match data_account.create_account(new_account).await {
        Ok(inserted_account) => {
            if let Some(account_id) = inserted_account.inserted_id.as_object_id() {
                if let Err(err) = data_inventory.create_inventory(account_id).await {
                    return HttpResponse::InternalServerError().body(err.to_string());
                }
                return HttpResponse::Ok().json(inserted_account);
            }
            HttpResponse::InternalServerError().body("Failed to retrieve account ID")
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/account/{id}")]
pub async fn get_account(db: Data<AccountRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let account_detail = db.get_account(&id).await;

    match account_detail {
        Ok(account) => {
            let response_json = json!({
                "id": account.id.map(|id| id.to_string()),
                "email": account.email,
                "display_name": account.display_name,
            });

            HttpResponse::Ok().json(response_json)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/accounts")]
pub async fn get_all_accounts(db: Data<AccountRepo>) -> HttpResponse {
    let accounts = db.get_all_accounts().await;

    match accounts {
        Ok(accounts) => {
            let response_accounts: Vec<_> = accounts
                .into_iter()
                .map(|account| {
                    // serde_json::to_value(&account).unwrap()
                    json!({
                        "id": account.id.map(|id| id.to_string()),
                        "email": account.email,
                        "display_name": account.display_name,
                    })
                })
                .collect();

            HttpResponse::Ok().json(response_accounts)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/account/{id}")]
pub async fn delete_account(db: Data<AccountRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_account(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Account successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Account with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/account/{id}")]
pub async fn update_account(
    db: Data<AccountRepo>,
    path: Path<String>,
    data: Json<RequestAccountUpdateData>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = RequestAccountUpdateData {
        email: data.email.clone(),
        display_name: data.display_name.clone(),
    };

    let update_result = db.update_account(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_account_info = db.get_account(&id).await;

                return match updated_account_info {
                    Ok(account) => HttpResponse::Ok().json(account),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No account found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
