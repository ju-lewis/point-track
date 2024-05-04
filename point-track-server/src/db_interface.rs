use serde::Serialize;
use sqlx::{
    prelude::FromRow, sqlite::{SqlitePool, SqlitePoolOptions, SqliteQueryResult},
};  
use random_string::generate;

use axum_extra::extract::cookie::CookieJar;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use std::{fs, time::Duration};

use crate::forms::RegisterBoatForm;

#[derive(Clone)]
pub struct Database {
    conn: SqlitePool
}

#[derive(FromRow)]
pub struct YachtClub {
    pub name: String
}


impl YachtClub {
    pub fn new() -> Self {
        return YachtClub {
            name: String::new()
        };
    }
}

#[derive(FromRow, Serialize, Debug)]
pub struct CoursePointName {
    name: String,
    id: i64
}

// Note: coordinates are in dddmmsss format
pub struct CoursePoint {
    name: String,
    lat: i32,
    lon: i32
}


const CHARSET: &'static str = "0123456789";


impl Database {

    pub async fn new(db_path: &str) -> Database {
        return Database { conn: SqlitePoolOptions::new()
                                .max_connections(5)
                                .acquire_timeout(Duration::from_secs(3))
                                .connect(db_path)
                                .await
                                .unwrap()
        };
    }
    
    pub async fn sign_up(&self, username: &str, password: &[u8], yacht_club: &str) -> Result<i64, &'static str>{

        // Generate salt
        let salt = SaltString::generate(&mut OsRng);

        // Hash password
        let argon2 = Argon2::default();
        let hash: String = argon2.hash_password(password, &salt).unwrap().to_string();
        
        //println!("{hash}");

        // Check if email is already registered
        let res: (u32, ) = sqlx::query_as(&format!("SELECT * FROM account WHERE username = '{username}';"))
            .fetch_optional(&self.conn)
            .await
            .unwrap_or(None) // Unwrap result
            .unwrap_or((0, )); // Unwrap Option - If no account was found (None returned), set count to 0
        
        if res.0 == 1 {
            //println!("Account exists!");
            return Err("Account already exists.");
        }

        // We know account doesn't exist, now create yacht club
        let result = sqlx::query(&format!("INSERT INTO yachtClub (name) VALUES ('{yacht_club}');")).execute(&self.conn).await;

        if !result.is_ok() {
            //println!("Signup Error!");
            return Err("Error Creating Yacht Club.");
        }
        
        // Now create account
        let yacht_club_id = result.unwrap().last_insert_rowid();
        let acc_result = sqlx::query(&format!("INSERT INTO account VALUES ({}, '{}', '{}');", yacht_club_id, username, hash)).execute(&self.conn).await;
        //println!("{acc_result:?}");
        
        match acc_result {
            Ok(x) => return Ok(x.last_insert_rowid()),      // Return account ID if successful
            Err(_) => return Err("Error creating account.") // Return error message if not
        }
    }

    pub async fn test_signup(&self) {
        let _ = self.sign_up("test_account", b"test", "Test Yacht Club").await;
    }

    
    /*
        Returns: `true` if credentials are correct, `false` if not
    */
    pub async fn validate_credentials(&self, username: &str, password: &[u8]) -> bool {

        // Pull hash corresponding to username from DB
        let maybe_credentials: Option<(i64, String, String)> = sqlx::query_as(&format!("SELECT * FROM account WHERE username = '{username}';"))
            .fetch_optional(&self.conn)
            .await.unwrap_or(None); // Unwrap result (if query succeeded)
        

        if maybe_credentials.is_none() {
            // No matching results were pulled from DB
            return false;
        }
        let credentials = maybe_credentials.unwrap();

        //println!("Credentials: {credentials:?}");

        // Extract hash from response
        let db_password_hash = PasswordHash::new(&(credentials.2));
        // Return false if there was a hashing error
        if db_password_hash.is_err() {
            //println!("Couldn't form PasswordHash from credential");
            return false;
        }

        println!("Parsed password hash: {db_password_hash:?}");
        let argon2 = Argon2::default();

        // Compare hashes
        match argon2.verify_password(password, &db_password_hash.unwrap()) {
            Ok(_) => return true,
            Err(_) => return false
        }
    }

    pub async fn create_session(&self, username: &str) -> String {

        // Create session token string
        let session_token = generate(20, CHARSET);
        
        // First get the ID
        let maybe_id = self.get_id_from_username(username).await;
        if maybe_id.is_none() {
            return String::new();
        }
        let id = maybe_id.unwrap();

        // We know the account exists, now insert session token
        let _ = sqlx::query(&format!("INSERT INTO account_session VALUES ({id}, '{session_token}');"))
            .execute(&self.conn).await;
        
        return session_token;
    }
    
    pub async fn is_session_valid(&self, jar: &CookieJar) -> bool {

        // First check if user has a valid session
        let maybe_token = jar.get("session");
        let maybe_username = jar.get("username");
            

        // Either username or token not found
        if maybe_token.is_none() || maybe_username.is_none() {
            return false;
        }
    
        let token = maybe_token.unwrap().value();
        let username = maybe_username.unwrap().value();

        // Query DB
        let valid_tokens: Vec<(String, )> = sqlx::query_as(&format!("SELECT token FROM 
                        account INNER JOIN account_session
                        ON account.yachtClubId = account_session.yachtClubId 
                        WHERE username = '{username}';"))
                        .fetch_all(&self.conn)
                        .await
                        .unwrap_or( Vec::new() );  // Unwrap result (Empty vector if query failed)
        
        //println!("token: {token}   token_list: {valid_tokens:?}");

        // If the current token is valid for the account, return true
        if valid_tokens.contains( &(token.to_string(), ) ) {
            return true;
        }
        return false;
    }

    pub async fn get_id_from_username(&self, username: &str) -> Option<i64> {
        let res: Option<(i64, )> = sqlx::query_as(&format!("SELECT yachtClubId FROM account WHERE username = '{username}'"))
            .fetch_optional(&self.conn)
            .await.unwrap();

        if res.is_none() {
            return None;
        }
        
        // Return id
        return Some(res.unwrap().0);
    }

    pub async fn get_yacht_club_info(&self, username: &str) -> YachtClub {

        // Query all yacht club information
        let maybe_yacht_club: Option<YachtClub> = sqlx::query_as(&format!("SELECT yachtClub.name FROM 
                                                                    yachtClub INNER JOIN account
                                                                    ON account.yachtClubId = yachtClub.yachtClubId
                                                                    WHERE username = '{username}' LIMIT 1;"))
        .fetch_optional(&self.conn)
        .await
        .unwrap_or(None);
        
        if maybe_yacht_club.is_none() {
            return YachtClub::new();
        }

        return maybe_yacht_club.unwrap();
    }

    pub async fn get_yacht_club_id(&self, username: &str) -> i64 {
        
        // Query all yacht club information
        let maybe_yacht_club: Option<(i64, )> = sqlx::query_as(&format!("SELECT yachtClub.yachtClubId FROM 
                                                                    yachtClub INNER JOIN account
                                                                    ON account.yachtClubId = yachtClub.yachtClubId
                                                                    WHERE username = '{username}' LIMIT 1;"))
        .fetch_optional(&self.conn)
        .await
        .unwrap_or(None);
        
        if maybe_yacht_club.is_none() {
            return -1;
        }
    
        // Return the ID
        return maybe_yacht_club.unwrap().0;
    }

    pub async fn get_all_known_points(&self) -> Vec<CoursePointName> {
        let course_points: Vec<CoursePointName> = sqlx::query_as("SELECT name, pointId as id FROM coursePoint;")
            .fetch_all(&self.conn).await.unwrap();
        return course_points;
    }

    pub async fn insert_course_points(&self, filename: &str) {

        let csv_string: String = fs::read_to_string(filename).unwrap_or("".to_string());
        
        let mut header: bool = true;

        let mut new_points: Vec<CoursePoint> = Vec::new();

        let lines = csv_string.split('\n');
        for line in lines {
            // Skip the header (so we don't try to parse strings)
            if header {
                header = false;
                continue
            };

            let elements: Vec<&str> = line.split(',').collect();
            if elements.len() != 3 {
                // Alert user the CSV is in the wrong format
                break;
            }
            // The format will now be:
            // [name, lat, lon]
            //println!("{}", (&(elements[2][0..elements[2].len()-3])).trim());
            new_points.push(CoursePoint {
                name: elements[0].to_string(),
                lat: (&(elements[1][0..elements[1].len()-1])).trim().parse().unwrap_or(0),
                lon: (&(elements[2][0..elements[2].len()-3])).trim().parse().unwrap_or(0),
            });

        }

        // We can now format and insert all of the processed points
        let mut query: String = "INSERT INTO coursePoint (name, latitude, longitude) VALUES ".to_string();
        for (i, point) in new_points.iter().enumerate() {
            query.push_str(&format!("(\"{}\", {}, {})", point.name, point.lat, point.lon));

            // Add comma if we're adding another point
            if i < new_points.len()-1 {
                query.push_str(",\n");
            }
        }
        query.push_str(";\n");
        
        println!("{}", query);
        let _ = sqlx::query(&query).execute(&self.conn).await;
    }

    pub async fn does_race_exist(&self, username: &str, date: i64) -> Option<i64> {
        let res: Option<(i64, i64, i64)>  = sqlx::query_as(&format!("SELECT * FROM yachtClub INNER JOIN account
                                                        ON yachtClub.yachtClubId = account.yachtClubId
                                                        INNER JOIN race ON race.yachtClubId = account.yachtClubId
                                                        WHERE account.username = '{}' AND race.raceDate = {};", username, date))
            .fetch_optional(&self.conn)
            .await.unwrap_or(None);
        
        if res.is_none() {
            return None;
        }

        // Return race ID
        let race_id = res.unwrap().0;
        return Some(race_id);
    }

    pub async fn create_race(&self, date: i64, yacht_club_id: i64) -> i64 {
        let res: Result<SqliteQueryResult, sqlx::Error>= sqlx::query(&format!(
            "INESRT INTO race (raceDate, yachtClubId) VALUES
            ({date}, {yacht_club_id});"))
        .execute(&self.conn).await;
        
        if res.is_err() {
            return -1;
        }

        return res.unwrap().last_insert_rowid();
    }

    pub async fn register_boat_in_race(&self, race_id: i64, boat_id: i64, nominated_speed: i64) {
        let _res = sqlx::query(&format!("INSERT INTO boatRace (boatId, raceId, nominatedSpeed)
                    VALUES ({boat_id}, {race_id}, {nominated_speed});"))
        .execute(&self.conn).await;
    }

    pub async fn get_registered_boats_in_race(&self, date: i64) -> Vec<String> {
        let boat_stats: Vec<(String, String, String)> = 
            match sqlx::query_as(&format!(
                "SELECT name, skipper, navigator FROM
                boat NATURAL JOIN boatRace NATURAL JOIN race
                WHERE raceDate = {date}"))
                .fetch_all(&self.conn).await {
                    Ok(stats) => stats,
                    Err(_) => Vec::new()
            };

        let mut boats: Vec<String> = Vec::new();
        
        // Add all boats to the vector
        for boat in boat_stats.iter() {
        boats.push(format!("{} | {}, {}", 
            boat.0, boat.1, boat.2));
        }

        return boats;
    }

    pub async fn boat_id_lookup(&self, username: &str, comp_num: i64) -> Option<i64> {
        let maybe_id: Option<(i64, )> = sqlx::query_as(&format!("SELECT boatId FROM boat 
                        NATURAL JOIN yachtClub NATURAL JOIN account
                        WHERE account.username='{username}' AND boat.compNumber = {comp_num};"))
                        .fetch_optional(&self.conn)
                        .await
                        .unwrap_or(None);

        if maybe_id.is_none() {
            return None;
        }
        return Some(maybe_id.unwrap().0);
    }

    pub async fn register_new_boat(&self, boat_form: RegisterBoatForm, yacht_club_id: i64) -> Option<i64> {
        let res = sqlx::query(&format!("INSERT INTO boat (compNumber, name, skipper, navigator, yachtClubId)
                    VALUES ({}, {}, {}, {}, {});",
                    boat_form.comp_number, boat_form.name, boat_form.skipper,
                    boat_form.navigator, yacht_club_id))
        .execute(&self.conn).await;
        
        return match res {
            Ok(result) => Some(result.last_insert_rowid()),
            Err(_) => None
        };
    }

    pub fn compute_race_results(&self) {
        todo!();
    }
    
}
