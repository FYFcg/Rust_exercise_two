fn main() {
    let chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let trans_chars: Vec<char> = chars
        .into_iter()
        .map(|c| {let next_char = (c as u8 + 1) as char; next_char})
        .collect();

    println!("{:?}", trans_chars);
}
