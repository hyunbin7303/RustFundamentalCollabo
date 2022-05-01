const OKAY_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str)->  i32 {
    if !input.chars().all(|character| OKAY_CHARACTERS.contains(character)){
        panic!("fuck this");
    }
    let input = input.trim_end_matches(|x| "+- ".contains(x))
        .chars()
        .filter(|character| *character != ' ')
        .collect::<String>();
    println!("{}", input);

    1
}


fn main() {
    math("1 + 1");

}