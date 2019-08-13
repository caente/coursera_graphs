use std::collections::BTreeMap;
use std::collections::HashMap;
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
    let mut numbers_v = Vec::new();
    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                let number = l.parse::<i64>().unwrap();
                numbers.insert(number);
                numbers_v.push(number);
            }
            _ => (),
        }
    }
    numbers_v.sort();
    let max = numbers_v.pop().unwrap();
    numbers_v.reverse();
    let min = numbers_v.pop().unwrap();
    println!("min:{}", min);
    println!("max:{}", max);
    println!("len:{}", max - min);
    Ok(numbers)
}

pub fn count(numbers: HashSet<i64>, ts: Range<i64>) -> usize {
    let mut regions: Vec<Range<i64>> = numbers.iter().fold(vec![], |mut regions, &x| {
        let mut t = ts.start;
        while t - x == 0 {
            t += 1;
        }
        let y = t - x;
        let end = ts.end - t;
        let new_region = y..(y + end);
        regions.push(new_region);
        regions
    });
    let intersected_regions: Vec<Range<i64>> = intersect_regions(&mut regions);
    println!("regions: {:?}", regions.len());
    println!("intersected_regions: {:?}", intersected_regions.len());
    println!("numbers: {:?}", numbers.len());
    let mut counter = 0;
    for n in numbers {
        for r in &intersected_regions {
            if r.contains(&n) {
                counter += 1;
            }
        }
    }
    counter
}

fn intersect(r1: &Range<i64>, r2: &Range<i64>) -> Vec<Range<i64>> {
    if r1.start <= r2.start && r1.end >= r2.start {
        vec![r1.start..r2.end]
    } else if r2.start <= r1.start && r2.end >= r1.start {
        vec![r2.start..r1.end]
    } else {
        vec![r1.start..r1.end, r2.start..r2.end]
    }
}

fn intersect_regions(regions: &mut Vec<Range<i64>>) -> Vec<Range<i64>> {
    regions.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    regions.iter().fold(vec![], |mut new_regions, new_r| {
        let mut rs: Vec<Range<i64>> = match new_regions.pop() {
            Some(r) => intersect(&r, new_r),
            None => vec![new_r.start..new_r.end],
        };
        new_regions.append(&mut rs);
        new_regions
    })
}

fn explore(r1: &Range<i64>, r2: &Range<i64>) -> (Range<i64>, Option<Range<i64>>) {
    if r1.start < r2.start && r1.end > r2.start {
        (r1.start..r2.end, Some(r1.end + 1..r2.end))
    } else if r2.start < r1.start && r2.end > r1.start {
        (r2.start..r1.end, Some(r2.start..r1.end - 1))
    } else if r1.start == r2.start {
        (r1.start..r1.end, None)
    } else {
        (r1.start..r1.start, Some(r2.start..r2.end))
    }
}

fn new_region(x: &i64, ts: &Range<i64>) -> Range<i64> {
    let mut t = ts.start;
    while t - x == 0 {
        t += 1;
    }
    let start = t - x;
    let end = ts.end - t;
    start..(start + end)
}

pub fn sum2(numbers: HashSet<i64>, ts: Range<i64>) -> usize {
    let mut numbers_v = numbers.iter().fold(vec![], |mut acc, n| {
        acc.push(n);
        acc
    });
    numbers_v.sort();
    let mut L = 1;
    let mut numbers_count = BTreeMap::new();
    while let Some(n) = numbers_v.pop() {
        numbers_count.insert(n, L);
        println!("n:{} | L:{}", n, L);
        L += 1;
    }
    println!("numbers_count:{:?}", numbers_count.len());
    let mut regions: Vec<Range<i64>> = numbers.iter().fold(vec![], |mut regions, x| {
        let region = new_region(x, &ts);
        regions.push(region);
        regions
    });

    regions.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    println!("regions {:?}", regions.len());
    let mut counter = 0;
    let exploration =
        regions.iter().fold(
            None,
            |previously_explored, region| match previously_explored {
                Some(previously_explored) => {
                    let (explored, unexplored) = explore(&previously_explored, &region);
                    // println!("previously_explored {:?}", previously_explored);
                    // println!("unexplored {:?}", unexplored);
                    // println!("explored {:?}", explored);
                    for u in unexplored {
                        let mut left = numbers_count.clone();
                        let mut center = left.split_off(&u.start);
                        let right = center.split_off(&(u.end + 1));
                        //println!("left {:?}", left);
                        //println!("center {:?}", center);
                        //println!("right {:?}", right);
                        //let start = **center.keys().min().unwrap_or(&&0);
                        let end = **right.keys().min().unwrap_or(&&0);
                        //println!("start:{}", start);
                        println!("end:{}", end);
                        let count = center.len();
                        counter += count;
                        //println!("count {:?}", count);
                        println!("counter {:?}", counter);
                    }
                    Some(explored)
                }
                None => {
                    // println!("region:{:?}", region);
                    let mut left = numbers_count.clone();
                    let mut center = left.split_off(&region.start);
                    let right = center.split_off(&(region.end + 1));
                    //println!("left {:?}", left);
                    //println!("center {:?}", center);
                    //println!("right {:?}", right);
                    //let start = **center.keys().min().unwrap_or(&&0);
                    //let end = **center.keys().max().unwrap_or(&&0);
                    //println!("start:{}", start);
                    //println!("end:{}", end);
                    let count = center.len();
                    counter += count;
                    // println!("unexplored {:?}", region);
                    // println!("count {:?}", count);
                    Some(region.start..region.end)
                }
            },
        );
    println!("exploration {:?}", exploration);
    counter
}
