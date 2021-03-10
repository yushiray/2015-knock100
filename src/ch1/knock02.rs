pub fn patcartaxi(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars())
    .map(|(c1, c2)| format!("{}{}", c1, c2))
    .collect::<String>()
}