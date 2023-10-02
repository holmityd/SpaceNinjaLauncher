use std::collections::HashSet;

use mongodb::bson::oid::ObjectId;
use serde_json::{json, Map, Value};

use crate::server::inventories::inventory_model::Inventory;

use super::mods_model::{RawUpgrade, RequestInventoryMod, RequestInventoryModUpdate, Upgrade};

pub fn add_upgrade(
    mut upgrades: Vec<Upgrade>,
    item_type: String,
    upgrade_fingerprint: String,
) -> Vec<Upgrade> {
    upgrades.push(Upgrade {
        id: Some(ObjectId::new()),
        item_type,
        upgrade_fingerprint,
    });

    upgrades
}

pub fn add_raw_upgrade(mut raw_upgrades: Vec<RawUpgrade>, item_type: String) -> Vec<RawUpgrade> {
    if let Some(item) = raw_upgrades.iter_mut().find(|i| i.item_type == item_type) {
        item.item_count += 1;
    } else {
        raw_upgrades.push(RawUpgrade {
            item_type,
            item_count: 1,
        });
    }

    raw_upgrades
}

pub fn upgrade_or_degrade_mod(
    inventory: Inventory,
    update_info: RequestInventoryModUpdate,
) -> (Vec<RawUpgrade>, Vec<Upgrade>) {
    let mut raw_upgrades = inventory.raw_upgrades.unwrap();
    let mut upgrades = inventory.upgrades.unwrap();

    let item_type = update_info.item_type.unwrap();
    let upgrade_fingerprint: String = update_info.upgrade_fingerprint.unwrap_or("".to_string());

    let lvl = serde_json::from_str::<serde_json::Value>(&upgrade_fingerprint)
        .ok()
        .and_then(|parsed| parsed["lvl"].as_i64())
        .unwrap_or(0);

    if let Some(id) = update_info.id {
        // in Upgrades
        let item_index = upgrades.iter().position(|i| i.id == Some(id)).expect("msg");
        if lvl > 0 {
            // if level>0 update mod in Uprgrades
            upgrades[item_index].upgrade_fingerprint = upgrade_fingerprint;
        } else {
            // if level<=0 remove mod from Uprgrades and add to RawUpgrades
            upgrades.remove(item_index);

            raw_upgrades = add_raw_upgrade(raw_upgrades, item_type);
        }
    } else {
        // in RawUpgrades
        if lvl > 0 {
            if let Some(item_index) = raw_upgrades.iter().position(|i| i.item_type == item_type) {
                raw_upgrades[item_index].item_count -= 1;
                if raw_upgrades[item_index].item_count <= 0 {
                    raw_upgrades.remove(item_index);
                }
            }
            upgrades = add_upgrade(upgrades, item_type, upgrade_fingerprint);
        }
    }

    (raw_upgrades, upgrades)
}

pub fn add_mods(
    inventory: Inventory,
    mods: Vec<RequestInventoryMod>,
) -> (Vec<RawUpgrade>, Vec<Upgrade>) {
    let mut raw_upgrades = inventory.raw_upgrades.unwrap();
    let mut upgrades = inventory.upgrades.unwrap();

    for item in mods {
        let item_type = item.item_type.unwrap();

        match item.upgrade_fingerprint {
            None => {
                raw_upgrades = add_raw_upgrade(raw_upgrades, item_type);
            }
            Some(upgrade_fingerprint) => {
                upgrades = add_upgrade(upgrades, item_type, upgrade_fingerprint);
            }
        }
    }

    (raw_upgrades, upgrades)
}

pub fn remove_mods(
    inventory: Inventory,
    mods: Vec<RequestInventoryMod>,
) -> (Vec<RawUpgrade>, Vec<Upgrade>) {
    let mut raw_upgrades = inventory.raw_upgrades.unwrap();
    let mut upgrades = inventory.upgrades.unwrap();

    // Convert postItems to a HashSet of serialized JSON strings
    let items_set: HashSet<String> = mods
        .iter()
        .map(|item| {
            let mut map = Map::new();
            if let Some(id) = &item.id {
                map.insert("_id".to_string(), json!(id));
            }
            if let Some(item_type) = &item.item_type {
                map.insert("ItemType".to_string(), json!(item_type));
            }
            if let Some(item_count) = &item.item_count {
                map.insert("ItemCount".to_string(), json!(item_count));
            }
            if let Some(upgrade_fingerprint) = &item.upgrade_fingerprint {
                map.insert("UpgradeFingerprint".to_string(), json!(upgrade_fingerprint));
            }
            serde_json::to_string(&Value::Object(map)).unwrap()
        })
        .collect();

    // Filter RawUpgrades
    raw_upgrades.retain(|item| {
        let serialized = serde_json::to_string(item).unwrap();
        !items_set.contains(&serialized)
    });

    // Filter Upgrades
    upgrades.retain(|item| {
        let serialized = serde_json::to_string(item).unwrap();
        !items_set.contains(&serialized)
    });

    (raw_upgrades, upgrades)
}
