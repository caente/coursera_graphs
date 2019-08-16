extern crate vpsearch;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;
//427

pub fn load_numbers(file_name: &str) -> Result<HashSet<i64>, std::io::Error> {
    let file = File::open(file_name)?;
    let buf_reader = BufReader::new(file);
    let mut numbers = HashSet::new();
    let mut c = 0;
    for line in buf_reader.lines() {
        if c >= 15 {
            break;
        }
        c += 1;
        match line {
            Ok(l) => {
                let number = l.parse::<i64>().unwrap();
                numbers.insert(number);
            }
            _ => (),
        }
    }
    Ok(numbers)
}

pub fn sum2(numbers: HashSet<i64>, ts: Range<i64>) -> usize {
    let mut numbers_v: Vec<i64> = numbers.iter().fold(vec![], |mut acc, n| {
        acc.push(*n);
        acc
    });
    numbers_v.sort();
    println!("numbers_v:{:?}", numbers_v);
    let min = numbers_v[0];
    println!("min:{}", min);

    let mut regions: Vec<Range<i64>> = numbers.iter().fold(vec![], |mut regions, x| {
        let region = candidates_region(x, &ts);
        regions.push(region);
        regions
    });
    regions.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    println!("regions {:?}", regions);

    let mut counter = 0;
    let mut found = vec![];
    let exploration = regions.iter().fold(
        regions[0].start..regions[0].start,
        |previously_explored, region| {
            let (explored, unexplored) = find_unexplored(&previously_explored, &region);
            println!("region {:?}", region);
            println!("previously_explored {:?}", previously_explored);
            println!("unexplored {:?}", unexplored);
            let upper_bound: Option<usize> = unexplored
                .and_then(|u| {
                    let lower_bound = numbers_v.iter().position(|x| x >= &&u.start);
                    let upper_bound = numbers_v.iter().position(|x| x >= &&u.end);
                    lower_bound.and_then(|lower_bound| {
                        upper_bound.map(|upper_bound| (lower_bound, upper_bound))
                    })
                })
                .map(|(lower_bound, upper_bound)| {
                    println!("lower_bound:{}", lower_bound);
                    println!("upper_bound:{}", upper_bound);
                    println!("low:{}", numbers_v[lower_bound]);
                    println!("up:{}", numbers_v[upper_bound]);
                    let mut v = numbers_v[lower_bound..upper_bound + 1].to_vec();
                    found.append(&mut v);
                    let count = numbers_v[lower_bound..upper_bound].len() + 1;
                    counter += count;
                    println!("count {:?}", count);
                    println!("counter {:?}", counter);
                    upper_bound
                });
            let final_explored = match upper_bound {
                Some(upper_bound) => explored.start..numbers_v[upper_bound].max(explored.end),
                None => explored,
            };
            println!("explored final {:?}", final_explored);
            final_explored
        },
    );
    println!("exploration {:?}", exploration);
    counter
}

fn find_unexplored(
    explored: &Range<i64>,
    candidate: &Range<i64>,
) -> (Range<i64>, Option<Range<i64>>) {
    if explored.contains(&candidate.start) && candidate.contains(&explored.end) {
        (
            explored.start..candidate.end,
            Some(explored.end + 1..candidate.end),
        )
    } else if explored.contains(&candidate.start) && explored.contains(&candidate.end) {
        (explored.start..explored.end, None)
    } else if candidate.start > explored.start && candidate.end > explored.end {
        (
            explored.start..candidate.end,
            Some(candidate.start..candidate.end),
        )
    } else {
        (explored.start..explored.end, None)
    }
}

fn candidates_region(x: &i64, ts: &Range<i64>) -> Range<i64> {
    let mut t = ts.start;
    while t - x == 0 {
        t += 1;
    }
    let start = t - x;
    let end = ts.end - t;
    start..(start + end)
}
