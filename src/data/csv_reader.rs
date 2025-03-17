use std::{collections::HashMap, error::Error, fs::File};

use csv::Reader;

use crate::models::{DataProvider, Translation};

pub struct CsvReader {
    pub file_path: String,
}

impl DataProvider for CsvReader {
    fn load_translations(&self) -> Result<HashMap<i32, Translation>, Box<dyn Error>> {
        let file = File::open(&self.file_path)?;
        let mut rdr = Reader::from_reader(file);

        let mut translations = HashMap::new();
        for (idx, result) in rdr.records().enumerate() {
            let record = result?;
            let english = record.get(0).unwrap().to_string();
            let german = record.get(1).unwrap().to_string();
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
}
