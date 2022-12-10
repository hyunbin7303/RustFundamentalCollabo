#[derive(Debug)] enum Champ { Garen, Nami, Vayne }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }
#[derive(Debug)]
enum GamePlayOption {

}
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeChampColorMessage(i32, i32, i32); // tuple struct
fn skin_own(champ: Champ) -> Option<Champ> {
    match champ {
        Champ::Nami => None,
        _ => Some(champ),
    }
}
fn champ_own(champ: Champ) -> Option<Champ> {
    match champ {
        Champ::Garen => None,
        _ => Some(champ),
    }
}
fn playable_champ(champ : Champ) -> Option<Champ> {
    match champ_own(champ) {
        None => None,
        Some(champ) => match skin_own(champ) {
            None => None,
            Some(champ) => Some(champ),
        }
    }
}
fn playable_champ_better(champ: Champ) -> Option<Champ> {
    champ_own(champ).and_then(skin_own)
}
fn play_game(champ: Champ, day: Day) {
    match playable_champ(champ) {
        Some(champ) => println!("Yay! On {:?} we get to play {:?}.", day, champ),
        None => println!("Oh no. We don't get to play on {:?}?", day),
    }
}
fn play_game_better(champ: Champ, day: Day) {
    match playable_champ_better(champ) {
        Some(champ) => println!("Yay! On {:?} we get to play {:?}.", day, champ),
        None => println!("Oh no. We don't get to play on {:?}?", day),
    }
}
// Use case : User only gonna play games when he owns champ and skin at the same time? 
fn main() {

    let (garen, nami, vayne) = (Champ::Garen, Champ::Nami, Champ::Vayne);
    // play_game(vayne, Day::Monday);
    play_game_better(vayne, Day::Monday);
}
