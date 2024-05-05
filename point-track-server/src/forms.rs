use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct DateQuery {
    pub date: i64
}


// Note: All times are in seconds since the Epoch (UNIX time)
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Boat {
    pub id: i64,
    pub side: i64,
    pub time: i64
}

#[derive(Deserialize, FromRow, Serialize, Debug)]
pub struct RegisterBoatForm {
    pub comp_number: i64,
    pub name: String,
    pub skipper: String,
    pub navigator: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RaceRegisterBoat {
    pub boat: i64,
    pub race_date: i64,
    pub nom_speed: i64,
    pub points: Vec<Boat>
}

impl RaceRegisterBoat {
    pub fn new() -> Self {
        return RaceRegisterBoat {
            boat: 0,
            race_date: 0,
            nom_speed: 0,
            points: Vec::new()
        }
    }
}


