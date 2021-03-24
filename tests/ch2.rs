extern crate knock100_2015;

mod ch2_tests {
    use knock100_2015::ch2::knock10::count_lines;
    use knock100_2015::ch2::knock10::head;
    use knock100_2015::ch2::knock10::tail;
    use knock100_2015::ch2::knock10::split;
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
    #[test]
    fn knock16() {
        assert_eq!(split(Path::new("hightenp.txt"),2).unwrap(), vec!["高知県\t江川崎\t41\t2013-08-12\n埼玉県\t熊谷\t40.9\t2007-08-16".to_string(), 
                                                                    "岐阜県\t多治見\t40.9\t2007-08-16\n山形県\t山形\t40.8\t1933-07-25".to_string(), 
                                                                    "山梨県\t甲府\t40.7\t2013-08-10\n和歌山県\tかつらぎ\t40.6\t1994-08-08".to_string(), 
                                                                    "静岡県\t天竜\t40.6\t1994-08-04\n山梨県\t勝沼\t40.5\t2013-08-10".to_string(), 
                                                                    "埼玉県\t越谷\t40.4\t2007-08-16\n群馬県\t館林\t40.3\t2007-08-16".to_string(), 
                                                                    "群馬県\t上里見\t40.3\t1998-07-04\n愛知県\t愛西\t40.3\t1994-08-05".to_string(), 
                                                                    "千葉県\t牛久\t40.2\t2004-07-20\n静岡県\t佐久間\t40.2\t2001-07-24".to_string(), 
                                                                    "愛媛県\t宇和島\t40.2\t1927-07-22\n山形県\t酒田\t40.1\t1978-08-03".to_string(), 
                                                                    "岐阜県\t美濃\t40\t2007-08-16\n群馬県\t前橋\t40\t2001-07-24".to_string(), 
                                                                    "千葉県\t茂原\t39.9\t2013-08-11\n埼玉県\t鳩山\t39.9\t1997-07-05".to_string(), 
                                                                    "大阪府\t豊中\t39.9\t1994-08-08\n山梨県\t大月\t39.9\t1990-07-19".to_string(), 
                                                                    "山形県\t鶴岡\t39.9\t1978-08-03\n愛知県\t名古屋\t39.9\t1942-08-02".to_string()]);
    }
}