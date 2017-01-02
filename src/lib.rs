//! Rust wrapper for the zKillboard API.

use std::collections::BTreeMap;

extern crate hyper;
use hyper::Client;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

/// Represents one lost ship.
///
/// All IDs are EVE type IDs.
#[derive(Debug)]
pub struct Kill {
    /// The unique kill ID.
    pub kill_id: u64,

    /// The ship-type (hull) ID of the lost ship.
    pub victim_ship_type_id: u64,

    /// The items associated with the ship when it was destroyed.
    pub victim_items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    /// The EVE type ID of the item.
    pub type_id: u64,

    /// The item flag.
    ///
    /// Useful for telling whether an item was in cargo, for example.
    pub flag: u64,

    /// How many of the item were dropped.
    pub quantity_dropped: u64,

    /// How many of the item were destroyed.
    pub quantity_destroyed: u64,
}

/// Defines whether the zKillboard request should be for the alliance's kills,
/// losses, or both.
pub enum ZkillRequestType {
    /// Retrieve records where the alliance killed a ship.
    Kills,

    /// Retrieve records where the alliance lost a ship.
    Losses,

    /// Retrieve records where the alliance killed or lost a ship.
    Both,
}

/// The parameters for one request to zKillboard.
pub struct ZkillRequest {
    alliance_id: u64,
    request_type: ZkillRequestType,
}

/// Retrieves kills from zKillboard according to the parameters of the request.
pub fn kills(request: ZkillRequest) -> Vec<Kill> {
    use std::io::prelude::*;

    let client = Client::new();
    let mut response = client.get(&request.endpoint()).send().expect("Could not read API");
    let mut response_string = String::new();
    response.read_to_string(&mut response_string).expect("Could not read response");
    let json = Json::from_str(&response_string).expect("Could not parse into JSON");
    let kills = json.as_array().expect("Could not read as array");
    kills.iter().map(|k| kill(k.as_object().expect("Could not read as object"))).collect()
}

fn kill(kill: &BTreeMap<String, Json>) -> Kill {
    let kill_id = kill.get("killID")
        .expect("Could not read kill_id")
        .as_u64()
        .expect("kill_id not u64");
    let victim = kill.get("victim")
        .expect("Could not read victim")
        .as_object()
        .expect("victim not object");
    let victim_ship_type_id = victim.get("shipTypeID")
        .expect("Could not read ship_id")
        .as_u64()
        .expect("ship_id not u64");
    let victim_items = kill.get("items")
        .expect("Could not read items")
        .as_array()
        .expect("items not array")
        .into_iter()
        .map(|i| item(i.as_object().expect("item not object")))
        .collect();
    Kill {
        kill_id: kill_id,
        victim_ship_type_id: victim_ship_type_id,
        victim_items: victim_items,
    }
}

fn item(item: &BTreeMap<String, Json>) -> Item {
    let type_id = item.get("typeID")
        .expect("Could not read type_id")
        .as_u64()
        .expect("type_id not u64");
    let flag = item.get("flag")
        .expect("Could not read flag")
        .as_u64()
        .expect("flag not u64");
    let quantity_dropped = item.get("qtyDropped")
        .expect("Could not read quantity_dropped")
        .as_u64()
        .expect("quantity_dropped not u64");
    let quantity_destroyed = item.get("qtyDestroyed")
        .expect("Could not read quantity_destroyed")
        .as_u64()
        .expect("quantity_destroyed not u64");
    Item {
        type_id: type_id,
        flag: flag,
        quantity_dropped: quantity_dropped,
        quantity_destroyed: quantity_destroyed,
    }
}

impl ZkillRequest {
    pub fn new(alliance_id: u64, request_type: ZkillRequestType) -> Self {
        ZkillRequest {
            alliance_id: alliance_id,
            request_type: request_type,
        }
    }

    /// Returns the zKillboard API endpoint for the given request.
    pub fn endpoint(&self) -> String {
        use ZkillRequestType::*;

        format!("https://zkillboard.com/api/allianceID/{}{}",
                self.alliance_id,
                match self.request_type {
                    Kills => "/kills",
                    Losses => "/losses",
                    Both => "/kills/losses",
                })
    }
}
