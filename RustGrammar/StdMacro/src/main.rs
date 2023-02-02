use include_bytes_plus::include_bytes;

fn main() {
    // let bytes = include_bytes!("spanish.in");
    // assert_eq!(bytes, b"adi\xc3\xb3s\n");
    // print!("{}", String::from_utf8_lossy(bytes));

    let bytes_u16_2 = include_bytes!("tests/include with whitespaces.in" as u16);
let bytes_u16_3 = include_bytes!("tests/include with whitespaces.in" as [u8; 48]);
let bytes_u16_4 = include_bytes!("tests/include with whitespaces.in" as [u16; 12]);
}