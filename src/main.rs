use dotenv::dotenv;
use indicatif::ProgressBar;
use mongodb::{
    bson::{doc, from_bson, Document},
    sync::{Client, Collection, Database},
};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};
#[derive(Serialize, Deserialize)]
struct Config {
    name_for_mongo: Option<String>,
}
impl Config {
    fn load() -> Self {
        let path = Path::new("config.json");
        if path.exists() {
            let config_data = fs::read_to_string(path).expect("Unable to read config file");
            serde_json::from_str(&config_data).expect("unable to parse config file")
        } else {
            Config {
                name_for_mongo: None,
            }
        }
    }
    fn save(&self) {
        let config_data =
            serde_json::to_string_pretty(self).expect("Unable to serialise the config");
        fs::write("config.json", config_data).expect("Unable to write config file");
    }
}
fn main() -> mongodb::error::Result<()> {
    let bar = ProgressBar::new_spinner().with_message("waiting for response.");
    bar.enable_steady_tick(25);
    let command = env::args().nth(1).expect("What do you want to do?");
    let mut config = Config::load();
    match command.as_str() {
        // READ
        "list" => {
            let name = get_username(&mut config);
            let database = connect_to_mongo();
            let collection: Collection<Document> = database.collection("list");
            let c  = collection.find_one(doc! {"name" : name}).run()?.expect("No one having this name found in database, we recommed adding something to your list : cargo run -- add <something..something> ");
            let d: Vec<String> = from_bson(c.get("list").expect("could not find list").clone())
                .expect("Something went wrong!");
            bar.finish_and_clear();
            if d.len() != 0 {
                println!("List Items - ");
                for (i, s) in d.iter().enumerate() {
                    println!("{} : {}", i + 1, s);
                }
            }else{
                println!("No item in the list! Try adding some items.")
            }
        }
        // Adding the user
        "name" => {
            let username = env::args()
                .nth(2)
                .expect("give it a valid name as : cargo run -- name <valid name>");
            config.name_for_mongo = Some(username.clone());
            config.save();
            let database = connect_to_mongo();
            let collection: Collection<Document> = database.collection("list");
            let c = collection.find_one(doc! {"name" : &username}).run()?;
            if c.is_some() {
                let c = c.unwrap();
                let d: Vec<String> = from_bson(c.get("list").expect("could not find list").clone())
                    .expect("Something went wrong!");
                bar.finish_and_clear();
                if d.len() != 0 {
                    println!("List Items - ");
                    for (i, s) in d.iter().enumerate() {
                        println!("{} : {}", i + 1, s);
                    }
                } else {
                    println!("No item in the list! Try adding some items.");
                }
            } else {
                let _ = collection
                    .insert_one(doc! {"name" : &username, "list" : []})
                    .run()?;
                bar.finish_and_clear();
            }
            println!("You are all set to add items to your list {}", username);
            return Ok(());
        }
        "add" => {
            let item_to_add = env::args()
                .nth(2)
                .expect("You need to pass item like : cargo run -- add \"<item>\"");
            let database = connect_to_mongo();
            let collection: Collection<Document> = database.collection("list");
            let name = get_username(&mut config);
            let _ = collection
                .find_one_and_update(
                    doc! {"name" : name},
                    doc! {"$push" : doc! {"list" : item_to_add}},
                )
                .run()?;
            bar.finish_and_clear();
            println!("New item successfully added.");
            return Ok(());
        }
        "delete" => {
            let item_to_delete = env::args().nth(2).expect("Must pass a value to delete");
            let idx = item_to_delete.parse::<i32>().expect("expected a number") - 1;
            let update_pipeline = vec![doc! {
                "$set": {
                    "list": {
                        "$concatArrays": [
                            { "$slice": ["$list", idx] },
                            { "$slice": ["$list", { "$add": [idx + 1, 0] }, { "$size": "$list" }] }
                        ]
                    }
                }
            }];
            let database = connect_to_mongo();
            let collection: Collection<Document> = database.collection("list");
            let name = get_username(&mut config);
            let _ = collection
                .update_one(doc! {"name" : name}, update_pipeline)
                .run()?;
            bar.finish_and_clear();
            println!("Item deleted successfully.");
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
fn get_username(config: &mut Config) -> &str {
    if config.name_for_mongo.is_none() {
        let username = env::args().nth(2).expect("Something went wrong");
        config.name_for_mongo = Some(username);
        config.save();
    }
    config.name_for_mongo.as_ref().unwrap()
}
