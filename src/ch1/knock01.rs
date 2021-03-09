pub fn patcar_taxi(s: &str) -> String {
    s.chars().step_by(2).collect::<String>()
}