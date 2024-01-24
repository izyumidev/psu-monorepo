use base64::{engine::general_purpose, Engine as _};

fn main() {
    let mut line = String::new();

    std::io::stdin().read_line(&mut line).unwrap();

    let mes = general_purpose::STANDARD
        .decode(line.trim().as_bytes())
        .unwrap();

    let key = std::fs::read_to_string("key.txt")
        .unwrap()
        .trim()
        .as_bytes()
        .to_vec();

    let res = mes
        .iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>();

    println!("{}", String::from_utf8(res).unwrap());
}
