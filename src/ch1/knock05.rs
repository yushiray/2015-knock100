pub fn character_n_gram(text: &str, n: usize) -> Vec<String> {
    let max = text.len() - n + 1;
    let mut result = Vec::new();
    for i in 0..max {
        result.push(text.get(i..(i+n)).unwrap().to_string());
    }
    result
}

pub fn word_n_gram(text: &str, n: usize) -> Vec<Vec<String>> {
    text.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>().windows(n).map(|x| Vec::from(x.to_vec())).collect::<Vec<Vec<String>>>()
}