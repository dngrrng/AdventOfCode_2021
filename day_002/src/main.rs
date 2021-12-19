///////////////////////////////
// 🎅‍ Advent of Code 2021 🎅‍ //
//    by dngrrng in rust    //
//////////////////////////////

// https://adventofcode.com/2021/day/2

//                 🌟
//               🌲🌲🌲
//              🌲🌲🌲🌲
//             🌲🌲🌲🌲🌲
//            🌲🌲🌲🌲🌲🌲
//           🌲🌲🌲🌲🌲🌲🌲
//          🌲🌲🌲🌲🌲🌲🌲🌲
//         🌲🌲🌲🌲🌲🌲🌲🌲🌲
//        🌲🌲🦀🌲🌲🌲🌲🌲🌲🌲
//       🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲
//      🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲
//     🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲
//    🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲
//   🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲
//  🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🦀🌲🌲
// 🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲
//               🪵

///////////////
// Libraries //
///////////////

use std::fs::{read_to_string};

///////////////
// Functions //
///////////////

fn string_sum_part1(s : &str, data: &String) -> i32 {
    let tmp: Vec<_> = data.lines()
        .filter(|x| x.contains(s))
        .map(|x| x.split_once(" ").unwrap())
        .map(|x| x.1.parse::<i32>().unwrap())
        .collect();
    tmp.iter().sum::<i32>()
}
//d this feels like the idiomatic rust way
//d But also can't see how to do part II like this

fn aim_part2(data: &String) -> [f64; 2] {

    let mut aim = 0.0;
    let mut depth = 0.0;
    let mut forward = 0.0;

    for line in data.lines() {
        let x = line.split_once(" ").unwrap();

        if x.0 == "down" {
            aim += x.1.parse::<f64>().unwrap();
        }

        if x.0 == "up" {
            aim -= x.1.parse::<f64>().unwrap();
        }

        if x.0 == "forward" {
            forward += x.1.parse::<f64>().unwrap();
            depth += aim * x.1.parse::<f64>().unwrap();
        }

    }

    return [depth, forward]
}

//////////
// Main //
//////////
fn main() {
    // Load input
    let data = read_to_string("./input/input.txt").unwrap();

    let forward = string_sum_part1("forward",&data);
    let down = string_sum_part1("down",&data);
    let up = string_sum_part1("up",&data);

    let depth = down-up;

    let [depth_part2, forward_part2] = aim_part2(&data);

    //////////////////
    // Pretty Print //
    //////////////////

    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!("🎄🎄 Part I Answer:  {}  🎄🎄",depth*forward);
    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!();
    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!("🎄🎄 Part II Answer: {} 🎄🎄",(depth_part2*forward_part2) as i64);
    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");

}

// completed.
// answer I : 1840243
// answer II: 1727785422
