use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use csv::Reader;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::i32;

#[derive(Serialize, Debug)]
struct Translation {
    id: i32,
    english: String,
    german: String,
}

fn read_translations() -> Result<HashMap<i32, Translation>, Box<dyn Error>> {
    let file = File::open("src/data.csv")?;

    let mut rdr = Reader::from_reader(file);

    let mut translations = HashMap::new();
    for (idx, result) in rdr.records().enumerate() {
        let record = result?;
        let english = record.get(0).unwrap().to_string();
        let german = record.get(1).unwrap().to_string();
        println!("English: {}, German: {}", english, german);
        translations.insert(
            idx as i32,
            Translation {
                id: idx as i32,
                english,
                german,
            },
        );
    }
    Ok(translations)
}

lazy_static::lazy_static! {
    static ref TRANSLATIONS: HashMap<i32, Translation> = read_translations().expect("failed to read translations");
}

fn read_map() -> Result<HashMap<i32, (String, String)>, Box<dyn Error>> {
    let file = File::open("src/data.csv")?;

    let mut rdr = Reader::from_reader(file);

    let mut translations = HashMap::new();
    for (index, result) in rdr.records().enumerate() {
        let record = result?;
        let english = record.get(0).unwrap().to_string();
        let german = record.get(1).unwrap().to_string();
        println!("English: {}, German: {}", english, german);
        translations.insert(index as i32, (english, german));
    }
    Ok(translations)
}

#[get("/translations")]
async fn get_all_translations() -> impl Responder {
    let translations: Vec<&Translation> = TRANSLATIONS.values().collect();
    HttpResponse::Ok().json(translations)
}

#[get("/translation/random")]
async fn get_random_translation() -> impl Responder {
    let random_number = rand::thread_rng().gen_range(0..TRANSLATIONS.len() as i32);

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

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // let translations = read_map();
    // for (id, (english, german)) in translations {
    //     println!("{} {} {}", id, english, german)
    // }
    // for (_, translation) in TRANSLATIONS.iter() {
    //     println!(
    //         "{} {} {}",
    //         translation.id, translation.english, translation.german
    //     )
    // }
    HttpServer::new(|| {
        App::new()
            .service(get_all_translations)
            .service(get_translation)
            .service(get_random_translation)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
