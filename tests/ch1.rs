extern crate knock100_2015;

#[cfg(test)]
mod ch1_tests {
    use knock100_2015::ch1::knock00::reverse;
    use knock100_2015::ch1::knock01::patcar_taxi;
    use knock100_2015::ch1::knock02::patcartaxi;
    use knock100_2015::ch1::knock03::pi_from_text;
    use knock100_2015::ch1::knock05::character_n_gram;
    use knock100_2015::ch1::knock05::word_n_gram;
    #[test]
    fn knock00() {
        assert_eq!(reverse("stressed"), "desserts".to_string());
    }
    #[test]
    fn knock01() {
        assert_eq!(patcar_taxi("パタトクカシーー"), "パトカー".to_string());
    }
    #[test]
    fn knock02() {
        assert_eq!(patcartaxi("パトカー", "タクシー"), "パタトクカシーー".to_string());
    }
    #[test]
    fn knock03() {
        assert_eq!(pi_from_text(),vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9]);
    }
    #[test]
    fn knock05() {
        assert_eq!(character_n_gram("I am an NLPer", 2),vec!["I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er"]);
        assert_eq!(word_n_gram("I am an NLPer", 2),vec![["I", "am"], ["am", "an"], ["an", "NLPer"]]);
    }
}