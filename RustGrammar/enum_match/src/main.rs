use linked_list::{address, LList};
mod linked_list;


#[derive(Debug)] enum Champ { Garen, Nami, Vayne }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

pub enum UserAction {
    ProgramStart(String),
    KeyboardPress(u32),
    // MousePress { x: i64, y: i64 },
    RefreshPage
}


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
    play_game_better(vayne, Day::Monday);
    play_game_better(nami, Day::Tuesday);
    // play_game(vayne, Day::Monday);


    let mut head = LList {
        value : 1,
        next: address::Nil
    };
    head.add_node_to_the_end(2);
    head.add_node_to_the_end(3);
    head.add_node_to_the_end(4);
    head.add_node_to_the_end(5);
    head.print_out();


    let action = UserAction::KeyboardPress(50);
    // let action2 = UserAction::MousePress { x: 30, y: 40 };ㅃ영ㅇ
    match action {
        UserAction::KeyboardPress(press) => println!("Press button : {}", press),
        // UserAction::MousePress { x: 40, y: 50 } => println!("{}"),
        UserAction::ProgramStart(start) => println!("Program start : {}", start),
        UserAction::RefreshPage => println!("Refresh Page."),
    };

}
