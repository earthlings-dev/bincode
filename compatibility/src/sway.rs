// Credits to Sway in the Rust Programming Language

use rand::RngExt;
use serde::{Deserialize, Serialize};

#[test]
pub fn test() {
    let mut rng = rand::rng();
    for _ in 0..1000 {
        crate::test_same(random(&mut rng));
    }
}

fn random(rng: &mut impl RngExt) -> FTXresponse<Trade> {
    if rng.random_bool(0.5) {
        FTXresponse::Result(FTXresponseSuccess {
            result: Trade::random(rng),
            success: rng.random_bool(0.5),
        })
    } else {
        FTXresponse::Error(FTXresponseFailure {
            success: rng.random_bool(0.5),
            error: crate::gen_string(rng),
        })
    }
}

#[derive(bincode::Encode, bincode::Decode, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum FTXresponse<T> {
    Result(FTXresponseSuccess<T>),
    Error(FTXresponseFailure),
}

#[derive(
    bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq,
)]
pub struct FTXresponseSuccess<T> {
    pub success: bool,
    pub result: T,
}

#[derive(bincode::Encode, bincode::Decode, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FTXresponseFailure {
    pub success: bool,
    pub error: String,
}

#[derive(bincode::Encode, bincode::Decode, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(bincode::Encode, bincode::Decode, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub liquidation: bool,
    pub price: f64,
    pub side: TradeSide,
    pub size: f64,
    pub time: String,
}

impl Trade {
    fn random(rng: &mut impl RngExt) -> Self {
        Self {
            id: rng.random(),
            liquidation: rng.random_bool(0.5),
            price: rng.random(),
            side: if rng.random_bool(0.5) {
                TradeSide::Buy
            } else {
                TradeSide::Sell
            },
            size: rng.random(),
            time: crate::gen_string(rng),
        }
    }
}
