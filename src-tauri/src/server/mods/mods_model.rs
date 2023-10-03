use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawUpgrade {
    #[serde(rename = "ItemType")]
    pub item_type: String,
    #[serde(rename = "ItemCount")]
    pub item_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestInventoryModUpdate {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    #[serde(rename = "ItemType")]
    pub item_type: Option<String>,
    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestInventoryMod {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    #[serde(rename = "ItemType")]
    pub item_type: Option<String>,
    #[serde(rename = "ItemCount")]
    pub item_count: Option<i32>,
    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseInventoryMods {
    #[serde(rename = "RawUpgrades")]
    pub raw_upgrades: Vec<RawUpgrade>,
    #[serde(rename = "Upgrades")]
    pub upgrades: Vec<Upgrade>,
}
