extern crate knock100_2015;

mod ch2_tests {
    use knock100_2015::ch2::knock10::count_lines;
    use knock100_2015::ch2::knock10::head;
    use knock100_2015::ch2::knock10::tail;
    use std::path::Path;

    #[test]
    fn knock10() {
        assert_eq!(count_lines(Path::new("hightenp.txt")), 24);
    }
    #[test]
    fn knock14() {
        assert_eq!(head(Path::new("hightenp.txt"),2).unwrap(), "高知県\t江川崎\t41\t2013-08-12\n埼玉県\t熊谷\t40.9\t2007-08-16\n".to_string());
    }

    #[test]
    fn knock15() {
        assert_eq!(tail(Path::new("hightenp.txt"),2).unwrap(), "山形県\t鶴岡\t39.9\t1978-08-03\n愛知県\t名古屋\t39.9\t1942-08-02\n".to_string());
    }
}