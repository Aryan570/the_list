use std::env;
use dotenv::dotenv;
use mongodb::{bson::{doc, Document}, sync::{Client, Database,Collection}};
fn main() -> mongodb::error::Result<()> {
    let command = env::args().nth(1).expect("What do you want to do?");
    let name = env::var("00").expect("No username found, we recommend running the command : cargo run -- name <your_name>");
    match command.as_str() {
        "list" => {
            let database = connect_to_mongo();
            let collection: Collection<Document> = database.collection("list");
            let c  = collection.find_one(doc! {"name" : name}).run()?.unwrap();
            println!("{:#?}",c);
        }
        _ => panic!("Unknown command")
    }
    Ok(())
}
fn connect_to_mongo() -> Database {
    dotenv().ok();
    let client_uri = env::var("MONGO_URI").expect("Can't connect to Database");
    let client = Client::with_uri_str(client_uri).expect("look something went wrong");
    client.database("Rust")
}