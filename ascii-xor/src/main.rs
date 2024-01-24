fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();

    println!("Enter the message: ");
    std::io::stdin().read_line(&mut line1).unwrap();
    println!("Enter the key: ");
    std::io::stdin().read_line(&mut line2).unwrap();

    let mes_ascii = line1.trim().chars().map(|x| x as u8).collect::<Vec<u8>>();
    let key_ascii = line2.trim().chars().map(|x| x as u8).collect::<Vec<u8>>();

    for letter in mes_ascii.iter().zip(key_ascii.iter()) {
        print!("{} ", letter.0 ^ letter.1);
    }
}
