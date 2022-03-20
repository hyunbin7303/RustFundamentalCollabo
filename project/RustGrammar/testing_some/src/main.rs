struct Name {
    first_name: String,
    middle_name: Option<String>, // middle_name can be empty
    last_name: String,
}
use std::env;
fn get_full_name(fname: &str, lname: &str, mname: Option<&str>) -> String { // middle name can be empty
  match mname {
    Some(n) => format!("{} {} {}", fname, n, lname),
    None => format!("{} {}", fname, lname),
  }
}
  

fn take_fifth(value:Vec<i32>) ->Option<i32> {
  if value.len() <4 {
    None
  }
  else{
    Some(value[4])
  }
}

fn handle_option(my_options: Vec<Option<i32>>){
  for item in my_options {
    match item {
      Some(number) => println!("Got a {}", number),
      None => println!("The vec is too short")
    }
  }
} 


struct StructTest{
  a: String,
  b: Option<String>,
  c: Option<String>,
  d: Option<i32>,
  e: Option<i64>,
  f: Option<f32>
}

// impl StructTest {
//   fn new(a:String, b: String, c: String, d: i32, e: i64, f: f32) -> StructTest {
//     a= "HI".to_string();
//     b= b.to_string();
//     c = c.to_string():
//   };
// }






 fn main() {
  println!("{}", get_full_name("Galileo", "Galilei", None));
  println!("{}", get_full_name("Leonardo", "Vinci", Some("Da")));

  let new_vec = vec![1,2];
  let big_vec = vec![1,2,3,4,5,6];
  let mut option_vec = Vec::new();
  option_vec.push(take_fifth(new_vec));
  option_vec.push(take_fifth(big_vec));
  handle_option(option_vec);

  let my_num = 20;
  let single_ref = &my_num;
  let double_ref = &&single_ref;
  let triple_ref = &&&double_ref;

  println!("Number displays: {}", triple_ref);

}


// &str is a simple string. 
// let my_var = "Hello" -> you create a &str. which is very fast.println!

// String is a pointer, with data on the heap.println!
