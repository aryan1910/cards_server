use crate::models::Translation;
use crate::{data::csv_reader, models::DataProvider};
use actix_web::{get, web, HttpResponse, Responder};
use lazy_static::lazy_static;
use rand::Rng;
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

#[get("/translation")]
async fn get_translation(query: web::Query<HashMap<String, String>>) -> impl Responder {
    if let Some(id_str) = query.get("id") {
        if let Ok(id) = id_str.parse::<i32>() {
            if let Some(translation) = TRANSLATIONS.get(&id) {
                return HttpResponse::Ok().json(translation);
            } else {
                HttpResponse::NotFound().body("translation not found")
            }
        } else {
            HttpResponse::NotFound().body("invalid id")
        }
    } else {
        HttpResponse::NotFound().body("id not found")
    }
}
