use actix_web::{web, HttpResponse, Result};
use mongodb::Database;
use mongodb::bson::doc;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use chrono::{Utc, Duration};

use crate::models::*;

pub async fn register(
    db: web::Data<Database>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let collection = db.collection::<User>("users");
    
    // Check if user already exists
    let existing_user = collection
        .find_one(doc! {"email": &req.email}, None)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    if existing_user.is_some() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "User with this email already exists"
        })));
    }
    
    // Hash password
    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // Create user
    let user = User {
        id: None,
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash,
        created_at: Utc::now(),
    };
    
    let result = collection
        .insert_one(&user, None)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let user_id = result.inserted_id.as_object_id().unwrap().to_hex();
    
    // Generate JWT token
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let claims = Claims {
        sub: user_id.clone(),
        username: user.username.clone(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            username: user.username,
            email: user.email,
        },
    }))
}

pub async fn login(
    db: web::Data<Database>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let collection = db.collection::<User>("users");
    
    // Find user by email
    let user = collection
        .find_one(doc! {"email": &req.email}, None)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let user = match user {
        Some(u) => u,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid email or password"
            })));
        }
    };
    
    // Verify password
    let valid = verify(&req.password, &user.password_hash)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    if !valid {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid email or password"
        })));
    }
    
    let user_id = user.id.unwrap().to_hex();
    
    // Generate JWT token
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let claims = Claims {
        sub: user_id.clone(),
        username: user.username.clone(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            username: user.username,
            email: user.email,
        },
    }))
}
