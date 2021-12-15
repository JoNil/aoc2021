use assert_cmd::Command;

macro_rules! aoc_test {
    ($name:ident => $value:literal) => {
        #[test]
        fn $name() {
            let cmd = Command::cargo_bin(stringify!($name).trim_start_matches("test_"))
                .unwrap()
                .assert()
                .success();

            let output = cmd.get_output();

            assert_eq!(
                std::str::from_utf8(&output.stdout)
                    .unwrap()
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .trim(),
                $value
            );
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
aoc_test!(test_6b => "1639643057051");
aoc_test!(test_7a => "356922");
aoc_test!(test_7b => "100347031");
aoc_test!(test_8a => "479");
aoc_test!(test_9a => "518");
aoc_test!(test_9b => "949905");
aoc_test!(test_10a => "294195");
aoc_test!(test_10b => "3490802734");
aoc_test!(test_11a => "1599");
aoc_test!(test_11b => "418");
aoc_test!(test_12a => "3563");
aoc_test!(test_12b => "105453");
aoc_test!(test_13a => "661");
aoc_test!(test_14a => "3048");
aoc_test!(test_14b => "3288891573057");
aoc_test!(test_15a => "423");
