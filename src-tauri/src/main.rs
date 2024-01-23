// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;
extern crate dotenv;

use std::env;
use std::fmt::format;
use std::sync::Mutex;
use dotenv::dotenv;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tauri::State;

lazy_static! {
  static ref CLIENT:reqwest::Client = reqwest::Client::builder()
    .user_agent("NNRYUserAgent")
    .build()
    .expect("Failed to build Client");
  static ref API_URL: String = {
    dotenv().ok();
    match env::var("API_URL") {
      Ok(v) => v.to_string(),
      Err(_) => String::from("https://api.mangadex.org/"),
    }
  };
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct LoginToken {
  session: String,
  refresh: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct AccountAttributes {
  username: String,
  roles: Vec<String>,
  version: u32,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct UserData {
  id: String,
  #[serde( rename = "type" )]
  account_type: String,
  attributes: AccountAttributes,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct UserDetailResponse {
  result: String,
  response: String,
  data: UserData,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct LoginResponse {
  result: String,
  token: LoginToken,
}

struct AppState {
  token: Mutex<LoginToken>
}

fn get_url(path:&str) -> String {
  [API_URL.to_string(), path.to_string()].concat()
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
async fn login<'s>(username: &str, password: &str, app_state: State<'s, AppState>) -> Result<String, String> {
  let payload = json!({
    "username": username,
    "password": password,
  });

  let response = match CLIENT
    .post(get_url("/auth/login"))
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .json(&payload)
    .send()
    .await {
    Ok(response) => response,
    Err(err) => return Err(err.to_string()),
  };

  if (response.status().as_u16()) >= 400 {
    return Err(match response.text().await {
      Ok(v) => v,
      Err(e) => return Err(e.to_string()),
    })
  }

  let json_response: LoginResponse = match response.json().await {
    Ok(json_response) => json_response,
    Err(err) => return Err(err.to_string()),
  };
  *app_state.token.lock().unwrap() = json_response.token;
  Ok("Login successfull".to_owned())
}

#[tauri::command]
async fn is_logged_in<'s>(app_state: State<'s, AppState>) -> Result<bool, String> {
  let token = app_state.token.lock().unwrap().clone();
  let response = match CLIENT
    .get(get_url("/user/me"))
    .bearer_auth(token.session)
    .send()
    .await {
    Ok(v) => v,
    Err(e) => return Err(e.to_string()),
  };

  Ok(response.status().as_u16() < 400)
}

#[tauri::command]
fn get_token<'r>(app_state: State<'r, AppState>) -> LoginToken {
  let state_lock = app_state.token.lock().unwrap();
  state_lock.clone()
}

#[tauri::command]
fn get_cd() -> String {
  std::env::current_dir().unwrap().to_str().unwrap().to_string()
}

fn main() {
  tauri::Builder::default()
    .manage(AppState{ token: Default::default() })
    .invoke_handler(tauri::generate_handler![
      greet,
      login,
      get_token,
      get_cd,
      is_logged_in,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
