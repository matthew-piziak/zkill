//! Rust wrapper for the zKillboard API.

use std::collections::BTreeMap;

extern crate hyper;
use hyper::Client;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

// filter out item flag 5 for cargo
// filter out item flag 87 for drones
// filter out ship ID 670 for pods

#[derive(Debug)]
struct Loss {
    kill_id: u64,
    victim_ship_type_id: u64,
}

pub fn sound_losses() {
    use std::io::prelude::*;

    let client = Client::new();
    let endpoint = endpoint(false, true, 99000739);
    let mut response = client.get(&endpoint).send().expect("Could not read API");

    let mut response_string = String::new();
    response.read_to_string(&mut response_string).expect("Could not read response");
    let data = Json::from_str(&response_string).expect("Could not parse into JSON");
    let losses = data.as_array().expect("Could not read as array");
    for loss_json in losses {
        let loss = loss(loss_json.as_object().expect("Could not read as object"));
        println!("{:?}", loss);
    }
}

fn loss(loss: &BTreeMap<String, Json>) -> Loss {
    let kill_id = loss.get("killID")
                      .expect("Could not read kill_id")
                      .as_u64()
                      .expect("kill_id not u64");
    let victim = loss.get("victim")
                     .expect("Could not read victim")
                     .as_object()
                     .expect("victim not object");
    let victim_ship_type_id = victim.get("shipTypeID")
                                    .expect("Could not read ship_id")
                                    .as_u64()
                                    .expect("ship_id not u64");
    Loss {
        kill_id: kill_id,
        victim_ship_type_id: victim_ship_type_id,
    }
}

fn endpoint(include_kills: bool, include_losses: bool, alliance_id: usize) -> String {
    if !include_kills && !include_losses {
        panic!("No results will be returned.")
    }
    format!("https://zkillboard.com/api/allianceID/{}{}{}",
            alliance_id,
            if include_kills {
                "/kills"
            } else {
                ""
            },
            if include_losses {
                "/losses"
            } else {
                ""
            })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sound_losses() {
        sound_losses();
        assert_eq!(false, true);
    }
}
