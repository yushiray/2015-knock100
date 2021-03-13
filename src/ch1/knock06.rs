use crate::ch1::knock05::character_n_gram;
use std::collections::HashSet;

pub fn set() -> HashSet<String> {
   let text_x = "paraparaparadise";
   let text_y = "paragraph";
   let x = character_n_gram(text_x, 2).into_iter().collect::<HashSet<String>>();
   let y = character_n_gram(text_y, 2).into_iter().collect::<HashSet<String>>();
   &x | &y
}