extern crate knock100_2015;

#[cfg(test)]
mod ch1_tests {
    use knock100_2015::ch1::knock00::reverse;
    use knock100_2015::ch1::knock01::patcar_taxi;
    use knock100_2015::ch1::knock02::patcartaxi;
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
}