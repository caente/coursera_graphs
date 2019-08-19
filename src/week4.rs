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
    for line in buf_reader.lines() {
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
    let max = numbers_v.last().unwrap();
    println!("min:{}", min);
    println!("max:{}", max);

    let mut regions: Vec<(&i64, Range<i64>)> = numbers.iter().fold(vec![], |mut regions, x| {
        let region = candidates_region(x, &ts);
        if *x > region.start {
            regions.push((x, region));
        }
        regions
    });
    regions.sort_by(|r1, r2| r1.1.start.cmp(&r2.1.start));
    //println!("regions {:?}", regions);

    let mut found = HashSet::new();
    let mut seen = HashSet::new();
    let exploration = regions.iter().fold(
        regions[0].1.start..regions[0].1.start,
        |previously_explored, (x, region)| {
            let (explored, unexplored) = find_unexplored(&previously_explored, &region);
            println!("x {:?}", x);
            println!("region {:?}", region);
            //println!("previously_explored {:?}", previously_explored);
            //println!("unexplored {:?}", unexplored);
            unexplored.and_then(|region| {
                let lower_bound =
                    search_positition_in_region(&numbers_v, &region.start, &region, |x| {
                        *x >= region.start
                    })?;
                //println!("lower_bound:{}", lower_bound);
                //println!("low:{}", numbers_v[lower_bound]);
                let _upper_bound =
                    search_positition_in_region(&numbers_v, &region.end, &region, |x| {
                        *x >= region.end || x == numbers_v.last().unwrap()
                    })?;
                //println!("upper_bound:{}", _upper_bound);
                //println!("up:{}", numbers_v[_upper_bound]);
                let upper_bound = if numbers.contains(&numbers_v[_upper_bound]) {
                    _upper_bound + 1
                } else {
                    _upper_bound
                };
                let candidates: Vec<i64> = numbers_v[lower_bound..upper_bound].to_vec();
                //println!("candidates {:?}", candidates);
                let ys = candidates.iter().fold(vec![], |mut ys, y| {
                    let t = y + *x;
                    //println!("{} + {} = {}", x, y, t);
                    if !found.contains(&t) {
                        ys.push(y);
                    };
                    ys
                });
                //println!("ys {:?}", ys);
                let count = ys.len();
                println!("count:{}", count);
                for y in ys {
                    if !(ts.start..ts.end + 1).contains(&(*x + y)) {
                        panic!("oh oh");
                    }
                    seen.insert(*y);
                    found.insert(*x + *y);
                }
                //println!("seen {:?}", seen);
                if found.len() > 427 {
                    panic!("too bad");
                }
                Some(upper_bound)
            });
            println!("found {:?}", found.len());
            println!("explored final {:?}", explored.end);
            println!("-----------------------");
            explored
        },
    );
    println!("exploration {:?}", exploration);
    found.len()
}

fn search_positition_in_region<F>(
    numbers: &[i64],
    n: &i64,
    _region: &Range<i64>,
    f: F,
) -> Option<usize>
where
    F: FnMut(&i64) -> bool,
{
    //println!("slice len:{}", numbers.len());
    //println!("latest:{}", _region.end);
    //println!("region:{:?}", _region);
    //println!("region len:{}", _region.end - _region.start);
    let inclusive_region = _region.start.._region.end + 1;
    numbers.iter().position(f).and_then(|pos| {
        //println!("pos:{}", pos);
        //println!("numbers[pos]:{}", numbers[pos]);
        //println!("inclusive_region:{:?}", inclusive_region);
        //println!(
        //    "inclusive_region.contains(numbers[pos]):{:?}",
        //    inclusive_region.contains(&numbers[pos])
        //);
        //if pos > 0 {
        //    println!("numbers[pos -1]:{}", numbers[pos - 1]);
        //    println!(
        //        "inclusive_region.contains(numbers[pos-1]):{:?}",
        //        inclusive_region.contains(&numbers[(pos - 1)])
        //    );
        //}
        if numbers[pos] == *n {
            Some(pos)
        } else if pos > 0 && inclusive_region.contains(&numbers[pos - 1]) {
            Some(pos - 1)
        } else if !inclusive_region.contains(&numbers[pos]) {
            None
        } else {
            Some(pos)
        }
    })
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
    } else if candidate.start >= explored.start && candidate.end > explored.end {
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
