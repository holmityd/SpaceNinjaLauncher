use std::collections::HashSet;

use mongodb::bson::oid::ObjectId;

use crate::server::inventories::inventory_model::Inventory;

use super::suit_model::{RequestInventorySuitsRemove, Suit};

pub fn add_suits(inventory: Inventory, post_suits: Vec<Suit>) -> Vec<Suit> {
    let mut suits = inventory.suits.unwrap();

    for mut item in post_suits {
        item.item_id = Some(ObjectId::new());
        suits.push(item);
    }

    suits
}

pub fn remove_suits(
    inventory: Inventory,
    post_suits: Vec<RequestInventorySuitsRemove>,
) -> Vec<Suit> {
    let mut suits = inventory.suits.unwrap();

    let items_set: HashSet<String> = post_suits
        .iter()
        .map(|item| {
            if let Some(item_id) = &item.item_id {
                item_id.to_string()
            } else {
                "".to_string()
            }
        })
        .collect();

    // Filter Suits
    suits.retain(|item| {
        if let Some(item_id) = &item.item_id {
            let item_id_string = item_id.to_string();
            !items_set.contains(&item_id_string)
        } else {
            // Keep item.item_id is None
            true
        }
    });

    suits
}

pub fn update_suit(inventory: Inventory, post_suit: Suit) -> Vec<Suit> {
    let mut suits = inventory.suits.unwrap();

    let item_index = suits
        .iter()
        .position(|i| i.item_id == post_suit.item_id)
        .expect("err");
    suits[item_index] = post_suit;

    suits
}
