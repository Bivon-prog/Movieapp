use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

mod models;
mod db;
mod auth;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Movie {
    id: u32,
    title: String,
    year: u32,
    rating: f32,
    genre: String,
    poster: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct TmdbMovie {
    id: u32,
    title: String,
    overview: String,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    release_date: String,
    vote_average: f32,
    genre_ids: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct TmdbResponse {
    results: Vec<TmdbMovie>,
}

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "StreamBox API is running with TMDb integration"
    })))
}

fn get_genre_name(genre_id: u32) -> String {
    match genre_id {
        28 => "Action",
        12 => "Adventure",
        16 => "Animation",
        35 => "Comedy",
        80 => "Crime",
        99 => "Documentary",
        18 => "Drama",
        10751 => "Family",
        14 => "Fantasy",
        36 => "History",
        27 => "Horror",
        10402 => "Music",
        9648 => "Mystery",
        10749 => "Romance",
        878 => "Sci-Fi",
        10770 => "TV Movie",
        53 => "Thriller",
        10752 => "War",
        37 => "Western",
        _ => "Unknown",
    }
    .to_string()
}

async fn get_movies(query: web::Query<std::collections::HashMap<String, String>>) -> Result<HttpResponse> {
    let api_key = env::var("TMDB_API_KEY").unwrap_or_else(|_| "0d96054fcce69715012311c8aa326e7c".to_string());
    let base_url = env::var("TMDB_BASE_URL").unwrap_or_else(|_| "https://api.themoviedb.org/3".to_string());
    
    let page = query.get("page").and_then(|p| p.parse::<u32>().ok()).unwrap_or(1);
    let category = query.get("category").map(|s| s.as_str()).unwrap_or("trending");
    
    // Fetch movies based on category
    let url = match category {
        "popular" => format!("{}/movie/popular?api_key={}&page={}", base_url, api_key, page),
        "top_rated" => format!("{}/movie/top_rated?api_key={}&page={}", base_url, api_key, page),
        "upcoming" => format!("{}/movie/upcoming?api_key={}&page={}", base_url, api_key, page),
        "now_playing" => format!("{}/movie/now_playing?api_key={}&page={}", base_url, api_key, page),
        _ => format!("{}/trending/movie/week?api_key={}&page={}", base_url, api_key, page),
    };
    
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<TmdbResponse>().await {
                Ok(tmdb_data) => {
                    let movies: Vec<Movie> = tmdb_data.results.iter().map(|m| {
                        let year = m.release_date.split('-').next()
                            .and_then(|y| y.parse::<u32>().ok())
                            .unwrap_or(2024);
                        
                        let genre = if !m.genre_ids.is_empty() {
                            get_genre_name(m.genre_ids[0])
                        } else {
                            "Unknown".to_string()
                        };
                        
                        let poster = if let Some(path) = &m.poster_path {
                            format!("https://image.tmdb.org/t/p/w500{}", path)
                        } else {
                            "https://via.placeholder.com/500x750/1a1a1a/ffffff?text=No+Image".to_string()
                        };
                        
                        Movie {
                            id: m.id,
                            title: m.title.clone(),
                            year,
                            rating: (m.vote_average * 10.0).round() / 10.0,
                            genre,
                            poster,
                            description: m.overview.clone(),
                        }
                    }).collect();
                    
                    Ok(HttpResponse::Ok().json(json!({
                        "movies": movies,
                        "total": movies.len(),
                        "page": page,
                        "source": "TMDb API"
                    })))
                }
                Err(e) => {
                    eprintln!("Error parsing TMDb response: {}", e);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to parse movie data"
                    })))
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching from TMDb: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch movies from TMDb"
            })))
        }
    }
}

async fn search_movies(query: web::Query<std::collections::HashMap<String, String>>) -> Result<HttpResponse> {
    let search_query = query.get("q").or_else(|| query.get("query")).unwrap_or(&"".to_string()).clone();
    
    if search_query.is_empty() {
        let empty_query = web::Query(std::collections::HashMap::new());
        return get_movies(empty_query).await;
    }
    
    let api_key = env::var("TMDB_API_KEY").unwrap_or_else(|_| "0d96054fcce69715012311c8aa326e7c".to_string());
    let base_url = env::var("TMDB_BASE_URL").unwrap_or_else(|_| "https://api.themoviedb.org/3".to_string());
    
    let encoded_query = search_query.replace(" ", "%20");
    let url = format!("{}/search/movie?api_key={}&query={}", base_url, api_key, encoded_query);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<TmdbResponse>().await {
                Ok(tmdb_data) => {
                    let movies: Vec<Movie> = tmdb_data.results.iter().map(|m| {
                        let year = m.release_date.split('-').next()
                            .and_then(|y| y.parse::<u32>().ok())
                            .unwrap_or(2024);
                        
                        let genre = if !m.genre_ids.is_empty() {
                            get_genre_name(m.genre_ids[0])
                        } else {
                            "Unknown".to_string()
                        };
                        
                        let poster = if let Some(path) = &m.poster_path {
                            format!("https://image.tmdb.org/t/p/w500{}", path)
                        } else {
                            "https://via.placeholder.com/500x750/1a1a1a/ffffff?text=No+Image".to_string()
                        };
                        
                        Movie {
                            id: m.id,
                            title: m.title.clone(),
                            year,
                            rating: (m.vote_average * 10.0).round() / 10.0,
                            genre,
                            poster,
                            description: m.overview.clone(),
                        }
                    }).collect();
                    
                    Ok(HttpResponse::Ok().json(json!({
                        "movies": movies,
                        "total": movies.len(),
                        "query": search_query
                    })))
                }
                Err(e) => {
                    eprintln!("Error parsing search results: {}", e);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to parse search results"
                    })))
                }
            }
        }
        Err(e) => {
            eprintln!("Error searching TMDb: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to search movies"
            })))
        }
    }
}

async fn get_movie_by_id(path: web::Path<u32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let api_key = env::var("TMDB_API_KEY").unwrap_or_else(|_| "0d96054fcce69715012311c8aa326e7c".to_string());
    let base_url = env::var("TMDB_BASE_URL").unwrap_or_else(|_| "https://api.themoviedb.org/3".to_string());
    
    let url = format!("{}/movie/{}?api_key={}&append_to_response=videos,credits", base_url, id, api_key);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.text().await {
                Ok(text) => Ok(HttpResponse::Ok().body(text)),
                Err(e) => {
                    eprintln!("Error reading response: {}", e);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to read movie details"
                    })))
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching movie details: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch movie details"
            })))
        }
    }
}

async fn get_movie_videos(path: web::Path<u32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let api_key = env::var("TMDB_API_KEY").unwrap_or_else(|_| "0d96054fcce69715012311c8aa326e7c".to_string());
    let base_url = env::var("TMDB_BASE_URL").unwrap_or_else(|_| "https://api.themoviedb.org/3".to_string());
    
    let url = format!("{}/movie/{}/videos?api_key={}", base_url, id, api_key);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.text().await {
                Ok(text) => Ok(HttpResponse::Ok().body(text)),
                Err(e) => {
                    eprintln!("Error reading response: {}", e);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to read movie videos"
                    })))
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching movie videos: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch movie videos"
            })))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    
    println!("🎬 StreamBox Server Starting...");
    
    // Initialize MongoDB
    let database = match db::init_db().await {
        Ok(db) => {
            println!("✅ MongoDB connected");
            Some(db)
        }
        Err(e) => {
            println!("⚠️  MongoDB connection failed: {}", e);
            println!("⚠️  Running without authentication features");
            None
        }
    };
    
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let bind_address = format!("{}:{}", host, port);
    
    println!("🌐 Binding server to: http://{}", bind_address);
    println!("🔌 API path: http://{}/api", bind_address);
    println!("🎥 TMDb Integration: Active");
    println!("📡 Fetching real movie data from TMDb");
    println!("\n✨ Server ready!");
    
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            );
        
        // Add database to app data if available
        if let Some(ref db) = database {
            app = app.app_data(web::Data::new(db.clone()));
        }
        
        app.service(
                web::scope("/api")
                    .route("/health", web::get().to(health))
                    .route("/movies", web::get().to(get_movies))
                    .route("/search", web::get().to(search_movies))
                    .route("/movies/{id}", web::get().to(get_movie_by_id))
                    .route("/movies/{id}/videos", web::get().to(get_movie_videos))
                    // Auth routes
                    .route("/auth/register", web::post().to(auth::register))
                    .route("/auth/login", web::post().to(auth::login))
            )
            .service(fs::Files::new("/", "./public").index_file("index.html"))
    })
    .bind(&bind_address)?
    .run()
    .await
}
