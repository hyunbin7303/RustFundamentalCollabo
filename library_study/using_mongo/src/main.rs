use mongodb::{Client, Collection, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;
use chrono::{TimeZone, Utc};
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

const DB_NAME: &str = "mongo-rust-db";

// Serialized Macro? 
#[derive(Serialize, Deserialize)]
struct Person {
   id : u32,
   name: String,
   test_scores: Vec<u32>,
   inventory: Vec<String>
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let client_uri ="mongodb://127.0.0.1:27017/";

   let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
   let client = Client::with_options(options)?;
   println!("Listing all Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }

   let db = client.database("learn-mongo");
   println!("Collection of Learn-mongo");
   for collection_name in db.list_collection_names(None).await? {
      println!("{}", collection_name);
   }

   let collection = db.collection::<Person>("people");
   let docs = vec![
      Person {
         id: 1,
         name: "Kevin".to_string(),
         test_scores : vec![10,20,30],
         inventory: vec!["Inventory1".to_string(),"Inventory2".to_string()]
      },
      Person {
         id: 2,
         name: "Hyunbin".to_string(),
         test_scores : vec![3,4,5],
         inventory: vec!["Hyunbin1".to_string(),"Hyunbin2".to_string()]
      },
      Person {
         id: 3,
         name: "Macy".to_string(),
         test_scores : vec![5,6,7],
         inventory: vec!["Snack".to_string(),"Snack2".to_string()]
      },
      Person {
         id: 4,
         name: "Macy".to_string(),
         test_scores : vec![5,6,7],
         inventory: vec!["Snack".to_string(),"Snack2".to_string()]
      }
   ];



   collection.insert_many(docs, None).await?;
   
   
   
   let cursor = collection.find(doc! { "name": "Macy"}, None).await?;
   // for result in cursor {
   //    println!()
   // }
   Ok(())
}