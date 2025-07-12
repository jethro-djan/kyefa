
use reqwest::{Client, StatusCode};
use serde_json::json;

use kyefa_models::{UserAccount, UserResponse};
use crate::error::{LoginError};

pub async fn login(username: &str, password: &str) -> Result<UserResponse, LoginError> {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:3050/login")
        .json(&json!({
            "username": username,
            "password": password,
        }))
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                let response_text = response.text().await.map_err(|e| 
                    LoginError::ServerError(format!("Failed to read response: {}", e))
                )?;
                
                println!("Server response: {}", response_text); 
                
                match serde_json::from_str::<UserResponse>(&response_text) {
                    Ok(user) => Ok(user),
                    Err(e) => {
                        println!("JSON parse error: {}", e);
                        Err(LoginError::ServerError(format!("Failed to parse server response: {}", e)))
                    }
                }

                // match response.json::<UserAccount>().await {
                //     Ok(user) => Ok(user),
                //     Err(_) => Err(LoginError::ServerError("Failed to parse server response.".to_string())),
                // }
            } else if response.status() == StatusCode::UNAUTHORIZED {
                Err(LoginError::InvalidCredentials("Incorrect username or password.".to_string()))
            } else if response.status() == StatusCode::NOT_FOUND {
                Err(LoginError::UserNotFound("User with that username does not exist.".to_string()))
            } else {
                Err(LoginError::ServerError(format!("Server returned an unexpected status: {}", response.status())))
            }
        }
        Err(e) => Err(LoginError::NetworkIssue(format!("Could not connect to the server: {}", e))),
    }
}
