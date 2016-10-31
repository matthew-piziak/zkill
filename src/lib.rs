//! Rust wrapper for the zKillboard API.

use std::collections::BTreeMap;

extern crate hyper;
use hyper::Client;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

// filter out item flag 5 for cargo
// filter out item flag 87 for drones
// filter out ship ID 670 for pods

pub enum ZkillRequestType {
    Kills,
    Losses,
    Both,
}

pub struct ZkillRequest {
    alliance_id: u64,
    request_type: ZkillRequestType,
}

#[derive(Debug)]
pub struct Kill {
    kill_id: u64,
    victim_ship_type_id: u64,
}

pub fn kills(request: ZkillRequest) -> Vec<Kill>{
    use std::io::prelude::*;

    let client = Client::new();
    let mut response = client.get(&request.endpoint()).send().expect("Could not read API");

    let mut response_string = String::new();
    response.read_to_string(&mut response_string).expect("Could not read response");
    let data = Json::from_str(&response_string).expect("Could not parse into JSON");
    let kills_json = data.as_array().expect("Could not read as array");
    let mut kills = vec![];
    for kill_json in kills_json {
        let kill = kill(kill_json.as_object().expect("Could not read as object"));
        kills.push(kill);
    }
    kills
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
    Kill {
        kill_id: kill_id,
        victim_ship_type_id: victim_ship_type_id,
    }
}

impl ZkillRequest {
    pub fn new(alliance_id: u64, request_type: ZkillRequestType) -> Self {
        ZkillRequest {
            alliance_id: alliance_id,
            request_type: request_type,
        }
    }

    pub fn endpoint(&self) -> String {
        use ZkillRequestType::*;

        format!("https://zkillboard.com/api/allianceID/{}{}",
                self.alliance_id,
                match self.request_type {
                    Kills => "/kills",
                    Losses => "/kills",
                    Both => "/kills/losses",
                })
    }
}
