use std::marker::Destruct;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: i32,
    pub name: String,
    pub location: String,
    pub title: String,
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self{
            id: self.id.clone(),
            title: self.title.clone(),
            name: self.name.clone(),
            location: self.location.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) where Self: Destruct {
        todo!()
    }
}