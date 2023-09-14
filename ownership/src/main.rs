fn main() {
    let word = String::from("asd lalala es aris");
    let length = first_word(&word);
    println!("{length}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
