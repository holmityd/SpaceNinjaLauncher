use std::{fs::File, io::Read};

use mongodb::error::Error;

use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
    results::{InsertOneResult, UpdateResult},
    Collection, Database,
};

use super::inventory_model::Inventory;

pub struct InventoryRepo {
    col: Collection<Inventory>,
}

impl InventoryRepo {
    pub async fn new(db: Database) -> Self {
        let col: Collection<Inventory> = db.collection("inventories");
        InventoryRepo { col }
    }

    pub async fn create_inventory(
        &self,
        account_owner_id: ObjectId,
    ) -> Result<InsertOneResult, Error> {
        // Read the JSON data from the file
        let mut file = File::open("src/static/inventory_data.json").expect("Failed to open file");
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)
            .expect("Failed to read file");

        // Deserialize the JSON data
        let mut new_inventory =
            serde_json::from_str::<Inventory>(&json_data).expect("Error deserializing JSON");
        new_inventory.account_owner_id = Some(account_owner_id);

        // Use the 'new_inventory' variable here

        let data = self
            .col
            .insert_one(new_inventory, None)
            .await
            .ok()
            .expect("Error creating inventory");

        Ok(data)
    }

    // pub async fn get_inventory(&self, account_owner_id: &String) -> Result<Inventory, Error> {
    //     let obj_id = ObjectId::parse_str(account_owner_id).unwrap();
    //     let filter = doc! {"accountOwnerId": obj_id};
    //     let data = self
    //         .col
    //         .find_one(filter, None)
    //         .await
    //         .ok()
    //         .expect("Error getting inventory's detail");

    //     Ok(data.unwrap())
    // }

    pub async fn get_inventory(&self, account_owner_id: &String) -> Result<Inventory, Error> {
        let obj_id = ObjectId::parse_str(account_owner_id).expect("Invalid ObjectId");
        let filter = doc! {"accountOwnerId": obj_id};

        match self.col.find_one(filter, None).await {
            Ok(Some(data)) => Ok(data),
            Ok(None) => Err(Error::from(mongodb::error::ErrorKind::MissingResumeToken)),
            Err(e) => Err(e),
        }
    }

    pub async fn update_inventory(
        &self,
        account_owner_id: &String,
        new_inventory: Inventory,
    ) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(account_owner_id).unwrap();
        let filter = doc! {"accountOwnerId": obj_id};

        let bson_doc = match bson::to_bson(&new_inventory) {
            Ok(Bson::Document(doc)) => doc,
            _ => panic!("Failed to convert struct to BSON"),
        };

        let mut set_fields = bson::document::Document::new();
        for (key, value) in bson_doc {
            if value != Bson::Null {
                set_fields.insert(key, value);
            }
        }

        let new_doc = doc! {
            "$set": set_fields,
        };

        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating inventory");
        Ok(updated_doc)
    }
}
