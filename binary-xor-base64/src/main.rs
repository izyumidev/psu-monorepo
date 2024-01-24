use base64::{engine::general_purpose, Engine as _};

fn main() {
    let line1 = std::fs::read_to_string("mes.txt").unwrap();
    let line2 = std::fs::read_to_string("key.txt").unwrap();

    let mes = line1.trim().as_bytes();
    let key = line2.trim().as_bytes();

    let mut res = Vec::new();

    for i in 0..mes.len() {
        res.push(mes[i] ^ key[i % key.len()]);
    }

    println!("{}", general_purpose::STANDARD.encode(&res));
}
