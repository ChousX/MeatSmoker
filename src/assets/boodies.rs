type Name = String;
use rand::{thread_rng, Rng, prelude::SliceRandom};
use serde::{Serialize, Deserialize};
use reqwest::Url;

#[derive(Deserialize, Serialize, Default)]
pub struct Boody{
    pub url: Option<Url>,
    pub title: Option<String>,
    pub adder: Option<Name>
}

#[derive(Deserialize, Serialize)]
pub struct BootieIndexer{
    boodies: Vec<Boody>,
    max: usize
}
impl BootieIndexer {
    pub fn add_boody(&mut self, path: &str, url: &str, title: Option<String>, adder: Option<Name>) -> Option<()>{
        println!("{},{}", self.boodies.len(), url);
        if self.boodies.len() >= self.max{ return None;}
        if let Ok(url) = Url::parse(url){
            let boody = Boody{
                url: Some(url),
                title,
                adder
            };
            self.boodies.push(boody);
            println!("ass added");

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
    
    fn new() -> Self{
        Self{
            boodies: vec![],
            max: 500
        }
    }

    fn load(path: &str) -> Option<Self>{
        if let  Ok(_) = std::fs::File::open(path){
            println!("ass loaded");
            let data = std::fs::read_to_string(path).expect("Unable to read file");

            let boodies: Self =  match serde_json::from_str(&data){
                Ok(t) => t,
                Err(_) => { println!("aaaaa");
                    return None;},
            };

            Some(boodies)
        } else {
            println!("bbbbbb");
            None
        }        
    }

    fn save(&self, path: &str){
        let bytes = serde_json::to_string(&self).expect("well to string is not going well");
        std::fs::write(path, bytes).expect("Unable to write file");
        println!("Ass Saved");
    }

    pub fn get_random_image(&self) -> (Option<Url>, String){
        let len = self.boodies.len();
        let id = thread_rng().gen_range(0..len);
        (self.boodies[id].url.clone(), {
            if let Some(title) = self.boodies[id].title.clone(){
                title
            } else {
                "".to_string()
            }
        })
    }

    pub fn len(&self) -> usize{
        self.boodies.len()
    }
}
impl Default for BootieIndexer{
    fn default() -> Self {
        Self::new()
    }
}