use aoc2021::get_input;
use glam::{ivec3, IVec3};
use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Vec<Vec<IVec3>> {
    input
        .split("\n\n")
        .map(|scanner| {
            let mut seen_becons = Vec::new();

            for line in scanner.lines().skip(1) {
                let mut digits = line.split(',');
                seen_becons.push(ivec3(
                    digits.next().unwrap().parse::<i32>().unwrap(),
                    digits.next().unwrap().parse::<i32>().unwrap(),
                    digits.next().unwrap().parse::<i32>().unwrap(),
                ));
            }

            seen_becons
        })
        .collect()
}

#[derive(Debug, Default)]
struct Scanner {
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn new(beacon_pos: &[IVec3]) -> Self {
        let mut beacons = Vec::new();

        for i in 0..beacon_pos.len() {
            let mut beacon = Beacon::new(beacon_pos[i]);

            for j in 0..beacon_pos.len() {
                if i != j {
                    let a = beacon_pos[i];
                    let b = beacon_pos[j];

                    let distance = (a - b).abs();

                    beacon
                        .distances
                        .insert(distance.x + distance.y + distance.z, j);
                }
            }

            beacons.push(beacon);
        }

        Self { beacons }
    }
}

#[derive(Debug)]
struct Beacon {
    distances: HashMap<i32, usize>,
    pos: IVec3,
}

impl Beacon {
    fn new(pos: IVec3) -> Self {
        Self {
            distances: Default::default(),
            pos,
        }
    }

    fn is_same(&self, other: &Beacon) -> bool {
        let me = self.distances.iter().map(|d| d.0).collect::<HashSet<_>>();

        let mut hits = 0;

        for distance in other.distances.iter().map(|d| d.0) {
            if me.contains(&distance) {
                hits += 1;
                if hits > 5 {
                    return true;
                }
            }
        }

        false
    }
}

struct IMat3(IVec3, IVec3, IVec3);

impl IMat3 {
    fn apply(&self, rhs: IVec3) -> IVec3 {
        ivec3(
            self.0.x * rhs.x + self.1.x * rhs.y + self.2.x * rhs.z,
            self.0.y * rhs.x + self.1.y * rhs.y + self.2.y * rhs.z,
            self.0.z * rhs.x + self.1.z * rhs.y + self.2.z * rhs.z,
        )
    }
}

fn find_projection(overlap: &[(&Beacon, &Beacon)]) -> IVec3 {
    //let possible_transforms = Vec::new();

    //for i in

    ivec3(0, 0, 0)
}

fn solve(input: &str) -> i32 {
    let scanner_beacon_positions = parse(input);

    let scanners = scanner_beacon_positions
        .iter()
        .map(|bp| Scanner::new(bp))
        .collect::<Vec<_>>();

    let one = &scanners[0];
    let two = &scanners[1];

    let mut overlap = Vec::new();

    'outer: for b1 in &one.beacons {
        for b2 in &two.beacons {
            if b1.is_same(b2) {
                overlap.push((b1, b2));

                if overlap.len() == 12 {
                    break 'outer;
                }

                continue 'outer;
            }
        }
    }

    let x = overlap[0].0.pos.x + overlap[0].1.pos.x;
    let y = overlap[0].0.pos.y + overlap[0].1.pos.y;
    let z = overlap[0].0.pos.z + overlap[0].1.pos.z;

    println!("{:#?}", ivec3(x, y, z));

    0
}

fn main() {
    let input = get_input();
    let start = std::time::Instant::now();
    let res = solve(&input);
    let end = start.elapsed();
    println!(
        "Day {} ({:?}): {}",
        aoc2021::get_program_name(),
        end.as_micros(),
        res
    );
}

#[cfg(test)]
mod test {
    #[test]
    fn test_20a() {
        let input = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        assert_eq!(crate::solve(input), 79);
    }
}
