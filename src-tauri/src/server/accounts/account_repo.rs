use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection, Database,
};

use super::account_model::{Account, RequestAccountUpdateData};

pub struct AccountRepo {
    col: Collection<Account>,
}

impl AccountRepo {
    pub async fn new(db: Database) -> Self {
        let col: Collection<Account> = db.collection("accounts");
        AccountRepo { col }
    }

    pub async fn create_account(&self, new_data: Account) -> Result<InsertOneResult, Error> {
        let data = self
            .col
            .insert_one(new_data, None)
            .await
            .ok()
            .expect("Error creating account");

        Ok(data)
    }

    pub async fn get_account(&self, id: &String) -> Result<Account, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let data = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting account's detail");

        Ok(data.unwrap())
    }

    pub async fn get_all_accounts(&self) -> Result<Vec<Account>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of accounts");
        let mut accounts: Vec<Account> = Vec::new();
        while let Some(account) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            accounts.push(account)
        }
        Ok(accounts)
    }

    pub async fn delete_account(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let account_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting account");

        Ok(account_detail)
    }

    pub async fn update_account(
        &self,
        id: &String,
        new_account: RequestAccountUpdateData,
    ) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "email": new_account.email,
                    "DisplayName": new_account.display_name,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating account");
        Ok(updated_doc)
    }
}
