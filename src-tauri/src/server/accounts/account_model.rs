use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "CountryCode")]
    pub country_code: String,
    #[serde(rename = "ClientType")]
    pub client_type: String,
    #[serde(rename = "CrossPlatformAllowed")]
    pub cross_platform_allowed: bool,
    #[serde(rename = "ForceLogoutVersion")]
    pub force_logout_version: i32,
    #[serde(rename = "ConsentNeeded")]
    pub consent_needed: bool,
    #[serde(rename = "TrackedSettings")]
    pub tracked_settings: Vec<String>,
    #[serde(rename = "__v")]
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestAccountCreationData {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestAccountUpdateData {
    pub email: String,
    pub display_name: String,
}
