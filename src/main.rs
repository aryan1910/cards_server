use csv::Reader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

fn read_translations() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let file = File::open("src/data.csv")?;

    let mut rdr = Reader::from_reader(file);

    let mut translations = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let english = record.get(0).unwrap().to_string();
        let german = record.get(1).unwrap().to_string();
        println!("English: {}, German: {}", english, german);
        translations.push((english, german));
    }
    Ok(translations)
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

fn main() -> Result<(), Box<dyn Error>> {
    let translations = read_map()?;
    for (id, (english, german)) in translations {
        println!("{} {} {}", id, english, german)
    }
    Ok(())
}
