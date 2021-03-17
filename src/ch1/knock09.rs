use rand::seq::SliceRandom;

pub fn typoglycemia(text: &str)  -> String {
    text.split_whitespace().map(|x| {
         if x.len() > 4 {
            let (head, remaining) = x.split_at(1); 
            let (body, tail) = remaining.split_at(remaining.len() - 1);
            let mut body: Vec<_> = body.chars().collect();
            let mut rng = rand::thread_rng();
            body.shuffle(&mut rng);
            head.to_string() + &body.into_iter().collect::<String>() + tail 
        } else { 
            x.to_string()
        }
    }).fold(String::new(), |result, s| if result == "" { s } else { result + " " + &s })
}