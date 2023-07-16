use rand::Rng;
use rand::distributions::{Distribution, Standard, Alphanumeric};
fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    let result = (0..10).flat_map(|_| {
        let vec: Vec<String> = vec!["abc".into(), "BCDEF".into(), "UserInformation".into()];
        vec.into_iter() // what is the purpose of this? 
    }).collect::<Vec<_>>(); // using Flat mapp and into Iter for testing purpose.
    println!("{:?}", result);

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
    .map(|_| {
    let idx = rng.gen_range(0..CHARSET.len());
    CHARSET[idx] as char
    })
    .collect();

    println!("{:?}", password);
    println!("Random string : {}", get_random_string(5));

    let v: Vec<f32> = Standard.sample_iter(&mut rng).take(10).collect();
    println!("{:?}", v);

    let s: String = Alphanumeric.sample_iter(&mut rng).take(10).map(char::from).collect();
    println!("{:?}", s);


}

fn get_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(len)
        .collect()
}