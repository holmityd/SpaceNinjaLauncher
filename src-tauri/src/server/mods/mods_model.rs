use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Upgrade {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: String,
}

// impl Serialize for Upgrade {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("Upgrade", 3)?;

//         // Rename the id field to "ItemId" if it exists
//         if let Some(ref id) = self.id {
//             state.serialize_field("ItemId", id)?;
//         }

//         state.serialize_field("ItemType", &self.item_type)?;
//         state.serialize_field("UpgradeFingerprint", &self.upgrade_fingerprint)?;
//         state.end()
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
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
