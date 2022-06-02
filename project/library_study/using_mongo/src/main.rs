use mongodb::{Client, Collection, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;
use chrono::{TimeZone, Utc};
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};

const DB_NAME: &str = "mongo-rust-db";

#[derive(Clone, Debug)]
pub struct DB {
   pub client : Client,
}
impl DB {
   pub async fn init() -> Result<Self> {
      let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
      client_options.app_name = Some("booky".to_string());
      Ok(Self {
         client: Client::with_options(client_options)?,
      })
   }
}


#[derive(Serialize, Deserialize)]
struct Person {
   id : u32,
   name: String,
   test_scores: Vec<u32>,
}



fn get_collection() {
   // let coll = client.database("items").collection("in_stock");

   // for i in 0..5 {
   //    let coll_ref = coll.clone();

   //    std::thread::spawn(move || {
   //       // Perform operations with `coll_ref`. For example:
   //       coll_ref.insert_one(doc! { "x": i }, None);
   //    });
   // }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri ="mongodb://127.0.0.1:27017/";
    //   env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");`

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }

   let new_doc = doc! {
      "title": "Parasite",
      "year": 2020,
      "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
      "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
   };

   let db = client.database("learn-mongo");
   println!("Collection of Learn-mongo");
   for collection_name in db.list_collection_names(None).await? {
      println!("{}", collection_name);
   }

   let user_coll = db.collection::<Document>("user");


   let person = Person {
      id: 1,
      name: "Kevin".to_string(),
      test_scores : vec![10,20,30],
   };
   let result = user_coll.insert_one(person, None).await;



   Ok(())
}