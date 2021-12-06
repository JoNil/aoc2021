use assert_cmd::Command;

macro_rules! aoc_test {
    ($name:ident => $value:literal) => {
        #[test]
        fn $name() {
            Command::cargo_bin(stringify!($name).trim_start_matches("test_"))
                .unwrap()
                .assert()
                .success()
                .stdout(concat!($value, "\n"));
        }
    };
}

aoc_test!(test_1a => "1709");
aoc_test!(test_1b => "1761");
aoc_test!(test_2a => "1868935");
aoc_test!(test_2b => "1965970888");
aoc_test!(test_3a => "1540244");
aoc_test!(test_3b => "4203981");
aoc_test!(test_4a => "44736");
aoc_test!(test_4b => "1827");
aoc_test!(test_5a => "5197");
aoc_test!(test_5b => "18605");
aoc_test!(test_6a => "362346");
