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
    let min = numbers_v[0];
    let max = numbers_v.last().unwrap();
    println!("min:{}", min);
    println!("max:{}", max);

    let mut regions: Vec<(&i64, Range<i64>)> = numbers.iter().fold(vec![], |mut regions, x| {
        let region = candidates_region(x, &ts);
        if *x > region.end {
            regions.push((x, region));
        }
        regions
    });
    regions.sort_by(|r1, r2| r1.1.start.cmp(&r2.1.start));
    let (found, _, _) = regions.iter().fold(
        (HashSet::new(), regions[0].1.start..regions[0].1.start, 0),
        |(mut found, previously_explored, offset), (x, region)| {
            println!("x {:?}", x);
            println!("region {:?}", region);
            let sliced_numbers = &numbers_v[offset..];
            let new_offset =
                find_unexplored(&previously_explored, &region).and_then(|unexplored| {
                    println!("sliced_numbers:{:?}", sliced_numbers.len());
                    let lower_bound = search_positition_in_region(
                        &sliced_numbers,
                        &unexplored.start,
                        &unexplored,
                        |x| *x >= unexplored.start,
                    )? + offset;
                    let _upper_bound = search_positition_in_region(
                        &sliced_numbers,
                        &unexplored.end,
                        &unexplored,
                        |x| *x >= unexplored.end || x == numbers_v.last().unwrap(),
                    )? + offset;
                    let upper_bound = if numbers.contains(&numbers_v[_upper_bound]) {
                        _upper_bound + 1
                    } else {
                        _upper_bound
                    };
                    let candidates: Vec<i64> = numbers_v[lower_bound..upper_bound].to_vec();
                    let ys = candidates.iter().fold(vec![], |mut ys, y| {
                        let t = y + *x;
                        if !found.contains(&t) {
                            ys.push(y);
                        };
                        ys
                    });
                    for y in ys {
                        if !(ts.start..ts.end + 1).contains(&(*x + y)) {
                            panic!("{} + {} must be within {:?}", x, y, ts);
                        }
                        found.insert(*x + *y);
                    }
                    Some(upper_bound)
                });
            println!("found {:?}", found.len());
            println!("-----------------------");
            let explored = previously_explored.start..previously_explored.end.max(region.end);
            (found, explored, new_offset.unwrap_or(offset))
        },
    );
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
    let inclusive_region = _region.start.._region.end + 1;
    numbers.iter().position(f).and_then(|pos| {
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

fn find_unexplored(explored: &Range<i64>, candidate: &Range<i64>) -> Option<Range<i64>> {
    if explored.contains(&candidate.start) && candidate.contains(&explored.end) {
        Some(explored.end + 1..candidate.end)
    } else if explored.contains(&candidate.start) && explored.contains(&candidate.end) {
        None
    } else if candidate.start >= explored.start && candidate.end > explored.end {
        Some(candidate.start..candidate.end)
    } else {
        None
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
