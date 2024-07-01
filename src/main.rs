use actix_web::{web, App, HttpServer, Responder};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use serde_derive::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Info {
    length: u32,
    uppercase: Option<bool>,
    lowercase: Option<bool>,
    numbers: Option<bool>,
}

fn random_string(info: web::Query<Info>) -> String {
    let mut rng: ThreadRng = thread_rng();
    let mut random_string: String = String::new();
    let mut charset: Vec<char> = Vec::new();

    if info.uppercase.unwrap_or(false) {
        charset.extend('A'..='Z');
    }

    if info.lowercase.unwrap_or(false) {
        charset.extend('a'..='z');
    }

    if info.numbers.unwrap_or(false) {
        charset.extend('0'..='9');
    }

    if charset.is_empty() {
        return random_string;
    }

    for _ in 0..info.length {
        let random_index = rng.gen_range(0..charset.len());
        random_string.push(charset[random_index]);
    }

    random_string
}

// GET: /random_string?length=10
async fn random_string_handler(info: web::Query<Info>) -> impl Responder {
    let random_string: String = random_string(info);
    web::Json(json!({ "random_string": random_string }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/random_string", web::get().to(random_string_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
