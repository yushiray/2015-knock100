pub fn cipher(text: &str)  -> String {
    text.chars().map(|x| if x.is_ascii_lowercase(){ (219 - x as u8 ) as char } else {x} ).collect::<String>()
}