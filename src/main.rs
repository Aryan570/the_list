use dotenv::dotenv;
use mongodb::{
    bson::{doc, Bson, Document},
    sync::{Client, Collection, Database},
};
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};
#[derive(Serialize,Deserialize)]
struct Config{
    name_for_mongo : Option<String>
}
impl Config{
    fn load() -> Self{
        let path = Path::new("config.json");
        if path.exists(){
            let config_data = fs::read_to_string(path).expect("Unable to read config file");
            serde_json::from_str(&config_data).expect("unable to parse config file")
        }else {
            Config{
                name_for_mongo : None
            }
        }
    }
    fn save(&self){
        let config_data = serde_json::to_string_pretty(self).expect("Unable to serialise the config");
        fs::write("config.json", config_data).expect("Unable to write config file");
    }
}
fn main() -> mongodb::error::Result<()> {
    let bar = ProgressBar::new_spinner().with_message("waiting for response.");
    bar.enable_steady_tick(25);
    let command = env::args().nth(1).expect("What do you want to do?");
    let mut config = Config::load();
    match command.as_str() {
        "list" => {
            let name = get_username(&mut config);
            let database = connect_to_mongo();
            let collection: Collection<Document> = database.collection("list");
            let c  = collection.find_one(doc! {"name" : name}).run()?.expect("No one having this name found in database, we recommed adding something to your list : cargo run -- add <something..something> ");
            let c = c.get("list").unwrap();
            bar.finish_and_clear();
            if let Bson::Array(arr) = c {
                let mut i = 1;
                println!("List items -");
                for s in arr {
                    if let Bson::String(str) = s {
                        println!("{} : {}", i, str);
                    }
                    i += 1;
                }
            }
        }
        "name" => {
            let username = env::args()
                .nth(2)
                .expect("give it a valid name as : cargo run -- name <valid name>");
            config.name_for_mongo = Some(username.clone());
            config.save();
            let database = connect_to_mongo();
            let collection: Collection<Document> = database.collection("list");
            let c = collection
                .find_one(doc! {"name" : &username})
                .run()?;
            if c.is_some() {
                let c = c.unwrap();
                let c = c.get("list").unwrap();
                bar.finish_and_clear();
                if let Bson::Array(arr) = c {
                    let mut i = 1;
                    println!("List items -");
                    for s in arr {
                        if let Bson::String(str) = s {
                            println!("{} : {}", i, str);
                        }
                        i += 1;
                    }
                }
            } else {
                let _ = collection.insert_one(doc! {"name" : &username, "list" : []}).run()?;
                bar.finish_and_clear();
            }
            println!("You are all set to add items to your list {}", username);
            return Ok(());
        }
        _ => panic!("Unknown command"),
    }
    Ok(())
}
fn connect_to_mongo() -> Database {
    dotenv().ok();
    let client_uri = env::var("MONGO_URI").expect("Can't connect to Database");
    let client = Client::with_uri_str(client_uri).expect("look something went wrong");
    client.database("Rust")
}
fn get_username(config : &mut Config) -> &str{
    if config.name_for_mongo.is_none(){
        let username = env::args().nth(2).expect("Something went wrong");
        config.name_for_mongo = Some(username);
        config.save();
    }
    config.name_for_mongo.as_ref().unwrap()
}