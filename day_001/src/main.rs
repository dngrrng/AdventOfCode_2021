///////////////////////////////
// 🎅‍ Advent of Code 2021 🎅‍ //
//    by dngrrng in rust    //
//////////////////////////////

// https://adventofcode.com/2021/day/1

//                 🌟
//               🌲🌲🌲
//              🌲🌲🌲🌲
//             🌲🌲🌲🌲🌲
//            🌲🌲🌲🌲🌲🌲
//           🌲🌲🌲🌲🌲🌲🌲
//          🌲🌲🌲🌲🌲🌲🌲🌲
//         🌲🌲🌲🌲🌲🌲🌲🌲🌲
//        🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲
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

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

//////////
// Main //
//////////

fn main() -> io::Result<()> {
    // Load input
    let f = File::open("./input/input.txt")?;
    let f = BufReader::new(f);

    // At each new line keep track of last (three) numbers
    let mut x0 = f64::NAN;
    let mut x1 = f64::NAN;
    let mut x2;

    let mut s0 = f64::NAN;
    let mut s1;

    // Running totals for each part
    let mut n = 0;
    let mut m = 0; // for part II

    // Stream over each number in file
    for line in f.lines() {

        x2 = x1;
        x1 = x0;
        // Update new line number
        x0 = line.unwrap().parse::<f64>().unwrap();

        // Keep a sum
        s1 = s0;
        s0 = x0 + x1 + x2; // New sum

        // Part I
        if x0 > x1 {
            n +=  1;
            // println!("{} is larger than {}", x0, x1);
        }

        // Part II
        if s1 < s0 {
            m += 1;
        }

    }
    //////////////////
    // Pretty Print //
    //////////////////

    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!("🎄🎄 Part I Answer:  {} 🎄🎄",n);
    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!();
    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!("🎄🎄 Part II Answer: {} 🎄🎄",m);
    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");

    Ok(())
}

// completed.
// answer I : 1301
// answer II: 1346
