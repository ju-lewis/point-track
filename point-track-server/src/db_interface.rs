use sqlx::{
    prelude::FromRow, sqlite::{SqlitePool, SqlitePoolOptions}
};  
use random_string::generate;

use axum_extra::extract::cookie::{CookieJar, Cookie};
use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use std::{intrinsics::r#try, time::Duration};

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
        
        println!("{hash}");

        // Check if email is already registered
        let res: (u32, ) = sqlx::query_as(&format!("SELECT * FROM account WHERE username = '{username}';"))
            .fetch_optional(&self.conn)
            .await
            .unwrap_or(None) // Unwrap result
            .unwrap_or((0, )); // Unwrap Option - If no account was found (None returned), set count to 0
        
        if res.0 == 1 {
            println!("Account exists!");
            return Err("Account already exists.");
        }

        // We know account doesn't exist, now create yacht club
        let result = sqlx::query(&format!("INSERT INTO yachtClub (name) VALUES ('{yacht_club}');")).execute(&self.conn).await;

        if !result.is_ok() {
            println!("Signup Error!");
            return Err("Error Creating Yacht Club.");
        }
        
        // Now create account
        let yacht_club_id = result.unwrap().last_insert_rowid();
        let acc_result = sqlx::query(&format!("INSERT INTO account VALUES ({}, '{}', '{}');", yacht_club_id, username, hash)).execute(&self.conn).await;
        println!("{acc_result:?}");
        
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

        println!("Credentials: {credentials:?}");

        // Extract hash from response
        let db_password_hash = PasswordHash::new(&(credentials.2));
        // Return false if there was a hashing error
        if db_password_hash.is_err() {
            println!("Couldn't form PasswordHash from credential");
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
        
        println!("token: {token}   token_list: {valid_tokens:?}");

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

    pub fn compute_race_results(&self) {
        todo!();
    }
    
}
