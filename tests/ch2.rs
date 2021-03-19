extern crate knock100_2015;

mod ch2_tests {
    use knock100_2015::ch2::knock10::count_lines;
    use std::path::Path;

    #[test]
    fn knock10() {
        assert_eq!(count_lines(Path::new("hightenp.txt")), 24);
    }
}