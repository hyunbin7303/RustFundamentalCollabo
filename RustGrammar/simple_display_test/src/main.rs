use album::{Album, Albums};
use password::Password;

mod album;
mod password;
const OKAY_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str)-> i32 {
    if !input.chars().all(|character| OKAY_CHARACTERS.contains(character)){
        panic!("fuck this");
    }
    let input = input.trim_end_matches(|x| "+- ".contains(x))
        .chars()
        .filter(|character| *character != ' ')
        .collect::<String>();
    println!("{}", input);

    let mut result_vec = vec![];
    let mut push_string = String::new();
    for character in input.chars() {
        match character {
            '+' => {
                if !push_string.is_empty(){
                    result_vec.push(push_string.clone());
                    push_string.clear();
                }
            },
            '-' => {
                if push_string.contains('-') || push_string.is_empty(){
                    push_string.push(character);
                } else {
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(character);
                }
            },
            number => {
                if push_string.contains('-'){
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(number);
                } else {
                    push_string.push(number);
                }
            }
        }
    } ;
    result_vec.push(push_string);
    let mut total = 0;
    let mut adds = true;
    let mut math_iter = result_vec.into_iter();
    while let Some(entry) = math_iter.next() {
        if entry.contains('-') {
            if entry.chars().count() % 2 == 1 {
                adds = match adds {
                    true => false,
                    false => true
                };
                continue;
            }else {
                continue;
            }
        }
        if adds == true {
            total += entry.parse::<i32>().unwrap();
        }else{
            total -= entry.parse::<i32>().unwrap();
        }
    }
    total
}


fn main() {
    let first_album = Album {
        title: "Desperado".into(),
        artist: "The Beatles".into(),
    };
    println!("{}", first_album);
    println!("Creating Own Albums struct in order to display Album in vec.");
    let albums = Albums(vec![
        first_album,
        Album {
            title: "Dark Side of the Moon".into(),
            artist: "Kevin Park".into(),
        },
    ]);

    let secure_pwd= Password("Password#1234".to_string());
    println!("Safe pwd : {}", secure_pwd);



    let value = math("1 + 1");
    println!("The result value : {}", value);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn one_plus_one_is_two(){
        assert_eq!(math("1 + 1"), 2);
    }
    #[test]
    fn one_minus_two_is_minusone(){
        assert_eq!(math("1-2"), -1);
    }

    #[test]
    fn one_minus_minus_one_is_two(){
        assert_eq!(math("1- -1"), 2);
    }

}