use std::io; 

fn read_input() -> String {
    let mut plaintext = String::new();
    io::stdin()
    .read_line(&mut plaintext)
    .expect("Failed to read plaintext.");
    return plaintext

}

fn encode(plaintext: String) -> String {
    let mut ciphertext_arr: [char; 5] = ['h', 'e', 'l','l','o'];
    let mut i = 0;
    for letter in plaintext.trim().bytes() {
        let mut encoded_letter = letter + 3;
        if encoded_letter > 123 {
            encoded_letter = encoded_letter % 123 + 97;
        }
        ciphertext_arr[i] = char::from(encoded_letter);
        i = i+1
    }
    let ciphertext_str: String = ciphertext_arr.iter().collect();

    return ciphertext_str
}

fn main() {
    let plaintext = read_input();
    let ciphertext = encode(plaintext.clone());
    println!("{plaintext}:{ciphertext}");
}
