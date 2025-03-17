use std::{collections::HashMap, error::Error};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Translation {
    pub id: i32,
    pub english: String,
    pub german: String,
}

pub trait DataProvider {
    fn load_translations(&self) -> Result<HashMap<i32, Translation>, Box<dyn Error>>;
}
