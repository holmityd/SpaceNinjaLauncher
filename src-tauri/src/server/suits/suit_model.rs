use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::server::inventories::inventory_model::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suit {
    #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<ObjectId>,
    #[serde(rename = "ItemType")]
    pub item_type: String,
    #[serde(rename = "Configs")]
    pub configs: Vec<serde_json::Value>,
    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: i32,
    #[serde(rename = "XP")]
    pub xp: i32,
    #[serde(rename = "InfestationDate")]
    pub infestation_date: Option<Date>,
    #[serde(rename = "Features")]
    pub features: Option<i32>,
    #[serde(rename = "Polarity")]
    pub polarity: Option<Vec<Polarity>>,
    #[serde(rename = "Polarized")]
    pub polarized: Option<i32>,
    #[serde(rename = "ModSlotPurchases")]
    pub mod_slot_purchases: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polarity {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Slot")]
    pub slot: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseInventorySuits {
    #[serde(rename = "Suits")]
    pub suits: Vec<Suit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestInventorySuitsRemove {
    #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<ObjectId>,
}
