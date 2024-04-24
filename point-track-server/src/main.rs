// Standard imports
use std::{fs, string};

// Project Module Imports
mod forms;
mod db_interface;
mod control_panel;
mod results;

// Server imports
use axum::{
    extract::{Form, FromRef, State}, 
    response::{IntoResponse, Redirect, Response}, 
    routing::{get, post}, 
    Router,
};
use axum_extra::{extract::{cookie::{Cookie, Key, PrivateCookieJar}, CookieJar}, response::Html};
use db_interface::{CoursePoint, Database, YachtClub};
use forms::LoginForm;
use results::RaceResult;
use serde::Deserialize;

use tower_http::services::{ServeDir, ServeFile};

// Templating imports
use tera::Tera;

// DB imports

#[derive(Clone)]
struct AppState {
    pub key: Key,
    db: Database 
}
impl FromRef<AppState> for Key {
    fn from_ref(input: &AppState) -> Self {
        return input.key.clone();
    }
}
impl FromRef<AppState> for Database {
    fn from_ref(input: &AppState) -> Self {
        return input.db.clone();
    }
}

#[tokio::main]
async fn main() {

    // Create application state
    let state = AppState{
        key: Key::generate(),
        db: Database::new("./point-track.db").await // Database connection pool
    };

    // Create app
    let app = Router::new()
        .route("/", get(get_login))         // Index routes to login for now
        .route("/login", get(get_login).post(login_user))
        .route("/signup", get(get_signup))
        .route("/home", get(get_control_panel))

        .route("/poll-results", get(get_result_updates))
        .nest_service("/css", ServeDir::new("css"))
        .nest_service("/js", ServeDir::new("js"))
        .with_state(state);

    
    // Create tokio tcp listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    // Serve app with tokio listener
    axum::serve(listener, app).await.unwrap();
}


/*
    Basic login page
*/
async fn get_login() -> Response {


    return Html(fs::read_to_string("templates/login.html").unwrap()).into_response();
}

/* 
    Handle login form
*/
async fn login_user(State(db): State<Database>, jar: CookieJar, Form(login): Form<LoginForm>) -> (CookieJar, Redirect) {

    let res: bool = db.validate_credentials(&login.username, login.password.as_bytes()).await;
    if !res {
        // Invalid credentials, redirect user to login page
        return (jar, Redirect::to("/login"));
    }
    
    // Create session
    let token: String = db.create_session(&login.username).await;

    // Set cookie in user session and log user in
    return (jar.add(Cookie::new("session", token)).add(Cookie::new("username", login.username)), Redirect::to("/home"));
}

async fn get_signup(State(db): State<Database>) -> Html<String> {
    
    // Create test account
    db.test_signup().await;

    return Html("temp".to_string());
}

async fn get_control_panel(jar: CookieJar, State(db): State<Database>) -> (CookieJar, Response) {

    // Check if session is valid
    let valid_session = db.is_session_valid(&jar).await;
    if !valid_session {
        return (jar, Redirect::to("/login").into_response());
    }

    // Get yacht club info (we know the account exists)
    let backup_cookie =  Cookie::new("username", "");
    let username = jar.get("username").unwrap_or(&backup_cookie).value();
    let yacht_club: YachtClub = db.get_yacht_club_info(username).await;

    // Get all recorded course points
    let course_points: Vec<CoursePoint> = db.get_all_known_points().await;
    
    // Create templating context
    let tera = Tera::new("./templates/*").unwrap();
    let mut context = tera::Context::new();

    // Insert yacht club name
    context.insert("yacht_club", &yacht_club.name);

    // Insert course points
    context.insert("possible_points", &course_points);


    let rendered: String = tera.render("control-panel.html", &context).unwrap_or(String::new());
    return (jar, Html(rendered).into_response());
}


/* Computes any updates in the race results and renders the result table */
async fn get_result_updates() -> Html<String> {
        
    // Create templating context
    let tera = Tera::new("./templates/*").unwrap();
    let mut context = tera::Context::new();

    let mut race_results: Vec<RaceResult> = Vec::new();

    // Push test data
    for _ in 0..8 {
        race_results.push(RaceResult::test()); 
    }

    // Insert all current results into race_results
    context.insert("race_results", &race_results);

    let rendered_table: String = tera.render("result-table.html", &context).unwrap_or(String::new());
    return Html(rendered_table);
}

