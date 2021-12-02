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
