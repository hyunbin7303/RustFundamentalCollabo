#[macro_use]
extern crate serde_derive;

use serde_json::Result;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct PersonalDetails {
    first_name: String,
    last_name: String,
    primary_address: i32
}


#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct BusinessDetails {
    name: String,
    company_role: String,
    primary_address: i32
}
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Profile {
    Personal {
        id: i32, 
        details: PersonalDetails,
    },
    Business {
        id:i32,
        details: BusinessDetails,
    },
}

fn main() -> Result<()> {
    let point = Point {x: 1, y: 2};

    let serializedObj = serde_json::to_string(&point).unwrap();
    println!("Serialized = {}", serializedObj);

    let deserializedObj: Point = serde_json::from_str(&serializedObj).unwrap();
    println!("Deserialized = {:?}", deserializedObj);

    let data = r#"
    [
      {
        "id": 1,
        "type": "personal",
        "details": {
          "firstName": "Juliano",
          "lastName": "Alves",
          "primaryAddress": 7777777
        }
      },
      {
        "id": 2,
        "type": "business",
        "details": {
          "name": "Juliano Business",
          "companyRole": "OWNER",
          "primaryAddress": 8888888
        }
      }
    ]
    "#;
    let profiles: Vec<Profile> = serde_json::from_str(data)?;
    println!("{:#?}", profiles);
    Ok(())
}
