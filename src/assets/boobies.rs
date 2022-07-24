use std::{path::{PathBuf , Path}, collections::VecDeque};
use chrono::Utc;
use rand::{thread_rng, Rng, prelude::SliceRandom};
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, ErrorKind};
use reqwest::Url;
use std::io::{Write};
use std::fs::OpenOptions;
type Name = String;
type TimeStamp = chrono::DateTime<Utc>;
use serde::{Serialize, Deserialize};
 

#[derive(Deserialize, Serialize, Default)]
pub struct Booby{
    pub url: Option<Url>,
    pub title: Option<String>,
    pub adder: Option<Name>
}

#[derive(Deserialize, Serialize)]
pub struct BoobieIndexer{
    boobs: Vec<Booby>,
    max: usize
}

const DEFAULT_INDEX: &str = r#"

"#;

impl BoobieIndexer{
    fn new() -> Self{
        Self{
            boobs: vec![],
            max: 500
        }
    }

    fn load(path: &str) -> Option<Self>{
        if let  Ok(output) = File::open(path){
            let data = std::fs::read_to_string(path).expect("Unable to read file");

            let boobies: Self =  match serde_json::from_str(&data){
                Ok(t) => t,
                Err(_) => {return None},
            };

            Some(boobies)
        } else {
            None
        }        
    }

    fn save(&self, path: &str){
        let bytes = serde_json::to_string(&self).expect("well to string is not going well");
        std::fs::write(path, bytes).expect("Unable to write file");
        println!("Saved");


    }

    pub fn get_random_image(&self) -> (Option<Url>, String){
        let len = self.boobs.len();
        let id = thread_rng().gen_range(0..len);
        (self.boobs[id].url.clone(), {
            if let Some(title) = self.boobs[id].title.clone(){
                title
            } else {
                "".to_string()
            }
        })
    }

    pub fn len(&self) -> usize{
        self.boobs.len()
    }

    pub fn add_booby(&mut self, path: &str, url: &str, title: Option<String>, adder: Option<Name>) -> Option<()>{
        println!("{},{}", self.boobs.len(), url);
        if self.boobs.len() >= self.max{ return None;}
        if let Ok(url) = Url::parse(url){
            let booby = Booby{
                url: Some(url),
                title,
                adder
            };
            self.boobs.push(booby);
            self.save(path);
            Some(())
        } else {
            println!("did not pars");
            None
        }
        
    }

    pub fn start_up(path: &str) -> Self{
        if let Some(loaded) = Self::load(path){
            loaded
        } else {
            Self::new()
        }
    } 
}

impl Default for BoobieIndexer{
    fn default() -> Self {
        Self::new()
    }
}