struct Name {
    first_name: String,
    middle_name: Option<String>, // middle_name can be empty
    last_name: String,
}
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
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1),
  }
}
struct StructTest{
  a: String,
  b: Option<String>,
  c: Option<String>,
  d: Option<i32>,
  e: Option<i64>,
}
impl StructTest {
  fn new(a: String, b: String, c: String, d: i32, e: i64) -> StructTest {
    StructTest {
      a: String::from(a),
      b: Some(b),
      c: Some(c),
      d: Some(d),
      e: Some(e)
    }
  }
}
impl Default for StructTest {
  fn default() -> StructTest {
    StructTest {
        a: String::from("Kevin"),
        b: Some("b".to_string()),
        c: Some("c".to_string()),
        d: Some(0),
        e: Some(-4i64),
    }
  }
}
struct Person {
  job: Option<Job>,
}
#[derive(Clone, Copy)]
struct Job {
  phone_number: Option<PhoneNumber>,
}
#[derive(Clone, Copy)]
struct PhoneNumber {
  area_code: Option<u8>,
  number: u32,
}
impl Person {

  // Gets the area code of the phone number of the person's job, if it exists.
  fn work_phone_area_code(&self) -> Option<u8> {
      // This would need many nested `match` statements without the `?` operator.
      // It would take a lot more code - try writing it yourself and see which
      // is easier.
      self.job?.phone_number?.area_code
  }
}

 fn main() {
  let new_vec = vec![1,2];
  let big_vec = vec![10,20,30,40,50,60,70];
  let mut option_vec = Vec::new();
  option_vec.push(take_fifth(new_vec));
  option_vec.push(take_fifth(big_vec));
  handle_option(option_vec); // Vector Array Testing .

  let name = Name {
    first_name : String::from("Kevin"), 
    last_name : String::from("Park"),
    middle_name : Some(String::from("Middle")),
  };
  // println!("Kevin's middle name is {}",
  //   match name.middle_name {
  //       None => "No middle name!",
  //       Some(ref x) => x,
  //   } ); // remember that string literal is actually a string slice,   use ref in a pattern match to borrow a reference
 
 //as_ref() converts Option<String> to Option<&String>
  println!("Kevin's full name is {} {} {}", name.first_name, name.middle_name.as_ref().map(|m| &m[0..1]).unwrap_or(""), name.last_name);
  println!("Kevin's middle name is {}", name.middle_name.unwrap_or("No middle name.".to_owned()));
  //     alice.middle.as_ref().map(|m| &m[0..1]).unwrap_or(""), // 



  let p = Person {
    job: Some(Job {
        phone_number: Some(PhoneNumber {
            area_code: Some(61),
            number: 439222222,
        }),
    }),
  };
  p.work_phone_area_code();
  println!("{:?}",Some(p.work_phone_area_code()));

  let num = Some(20);// using map with Option.
  let another_num = num
      .map(|n| n-10) // 10
      .map(|n| n*10);// 100
  println!("{}", another_num.unwrap());  




    // let another_number = some_number
    //     .map(|n| n - 1) // => Some(8)
    //     .map(|n| n * n) // => Some(64)
    //     .and_then(|n| divide(n, 4)); // => Some(16)

    //map() is used to transform Option values. 
    // let collection = vec![Some(10), Some(20)].into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_full_name_without_middlename()
    {
      let value = get_full_name("Galileo", "Galilei", None);
      assert_eq!(value, "Galileo Galilei");
    }
    #[test]
    fn test_get_full_name_with_middlename()
    {
      let value = get_full_name("Leonardo", "Vinci", Some("Da"));
      assert_eq!(value, "Leonardo Da Vinci");
    }
    #[test]
    fn test_take_fifth_return_validNumber()
    {
      let big_vec = vec![1,2,3,4,5,6];
      let mut i :i32 = 0;
      let check  = take_fifth(big_vec);
      if check.is_some() {
        i = check.unwrap();
      }
      assert_eq!(i,5);
    }
    #[test]
    fn test_take_fifth_return_none()
    {
      let small_vec = vec![1,2,3];
      let check = take_fifth(small_vec);
      assert_eq!(check,None);
    }
    #[test]
    fn test_plus_one_pass()
    {
      let addup = plus_one(Some(30)).unwrap();
      assert_eq!(addup, 31);
    }
    #[test]
    fn test_into_iter()
    {
      let v = vec!["testing", "Value", "Kevin"];
      let mut iter = v.into_iter();

      assert_eq!(Some("testing"), iter.next());
    }
}