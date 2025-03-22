use crate::models::Translation;
use crate::{data::csv_reader, models::DataProvider};
use actix_web::{get, web, HttpResponse, Responder};
use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    static ref TRANSLATIONS: HashMap<i32, Translation> = {
        let provider: Box<dyn DataProvider> = Box::new(csv_reader::CsvReader {
            file_path: "src/data.csv".to_string(),
        });
        provider
            .load_translations()
            .expect("Failed to load translations")
    };
}

#[get("/translations")]
async fn get_all_translations() -> impl Responder {
    let translations: Vec<&Translation> = TRANSLATIONS.values().collect();
    HttpResponse::Ok().json(translations)
}

#[get("/translation/random")]
async fn get_random_translation() -> impl Responder {
    let random_number = rand::rng().random_range(0..TRANSLATIONS.len() as i32);

    if let Some(translation) = TRANSLATIONS.get(&random_number) {
        HttpResponse::Ok().json(translation) // Semicolon here will return ()
    } else {
        HttpResponse::NotFound().body("Not found")
    }
}

#[derive(Deserialize)]
struct TranslationRequest {
    id: i32,
}

#[derive(Serialize)]
struct TranslationError {
    error: &'static str,
}

#[get("/translation")]
async fn get_translation(query: web::Query<TranslationRequest>) -> impl Responder {
    match TRANSLATIONS.get(&query.id) {
        Some(translation) => HttpResponse::Ok().json(translation),
        None => HttpResponse::NotFound().json(TranslationError {
            error: "translation not found",
        }),
    }
}
