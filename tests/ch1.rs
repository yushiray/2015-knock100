extern crate knock100_2015;

#[cfg(test)]
mod ch1_tests {
    use knock100_2015::ch1::knock00::reverse;
    use knock100_2015::ch1::knock01::patcar_taxi;
    #[test]
    fn knock00() {
        assert_eq!(reverse("stressed"), "desserts".to_string());
    }
    #[test]
    fn knock01() {
        assert_eq!(patcar_taxi("パタトクカシーー"), "パトカー".to_string());
    }
}