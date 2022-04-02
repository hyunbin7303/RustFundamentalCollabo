#[macro_use]
extern crate serde_derive;

use serde_json::{Number, Value, Result, Map};
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

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
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
fn simple_map_test() {
  let maptesting =
    serde_json::from_str::<Map<String, Value>>(r#"
        {
          "gender":"male",
          "blah":25,
          "Kevin": "Rust Beginner",
          "Macy": "Product Manager"
        }
        "#).unwrap();
    println!("{:?}", maptesting);
}
pub fn strongly_type_json() -> Result<()> {
  let test_profile_json = {
    let text = std::fs::read_to_string("profile.json").unwrap();
    let profiles: Vec<Profile> = serde_json::from_str(&text)?;
    profiles
  };
  println!("Reading from the json file. {:#?}", test_profile_json);
  println!("---------");
  Ok(())
}

pub fn strongly_type_json2() -> Result<()> {
  let person_json = {
    let text = std::fs::read_to_string("person.json").unwrap();
    let people: Vec<Person> = serde_json::from_str(&text).unwrap();
    people
  };
  println!("Reading from the json file. {:#?}", person_json);
  Ok(())
}


fn main() -> Result<()> {
    //simple_map_test();
    // let point = Point {x: 1, y: 2};
    // let serialized_obj = serde_json::to_string(&point).unwrap();
    // println!("Serialized = {}", serialized_obj);
    // let deserialized_obj: Point = serde_json::from_str(&serialized_obj).unwrap();
    // println!("Deserialized = {:?}", deserialized_obj);



    let dic_json = {
      let text = std::fs::read_to_string("dic.json").unwrap();
      let dic = serde_json::from_str::<Map<String, Value>>(&text).unwrap();
      dic
    };
    println!("Reading from the json file. {:#?}", dic_json);
    println!("{}", dic_json["kevin1"]);
    println!("------------");

    let mut missy_diet = {
        // Load the first file into a string.
        let text = std::fs::read_to_string("test.json").unwrap();
        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    // Get the number of elements in the object 'missy_food_schedule'
    let nb_elements = missy_diet["missy_food_schedule"].as_array().unwrap().len();
    println!("{}", nb_elements);
    for index in 0..nb_elements{
        if let Value::Number(n) = &missy_diet["missy_food_schedule"][index]["quantity"] {
            // Double the quantity for each element in 'missy_food_schedule'
            missy_diet["missy_food_schedule"][index]["quantity"] =
                Value::Number(Number::from_f64(n.as_f64().unwrap() * 2.).unwrap());

        }
    }
    Ok(())
}


