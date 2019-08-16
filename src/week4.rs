extern crate vpsearch;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;
use std::rc::Rc;
//427

struct MyImpl;
impl vpsearch::MetricSpace<MyImpl> for i64 {
    type UserData = ();
    type Distance = u64;
    fn distance(&self, other: &Self, _user_data: &()) -> u64 {
        (self - other).abs() as u64
    }
}
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
    if r1.start < r2.start && r1.end > r2.start && r1.end > r2.start && r1.end < r2.end {
        (r1.start..r2.end, Some(r1.end + 1..r2.end))
    } else if r1.contains(&r2.start) && r1.contains(&r2.end) {
        (r1.start..r1.end, None)
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

#[derive(Debug)]
enum NodeType {
    Full {
        value: i64,
        left: Rc<STree>,
        right: Rc<STree>,
    },
    Left {
        value: i64,
        left: Rc<STree>,
    },
    Right {
        value: i64,
        right: Rc<STree>,
    },
}

#[derive(Debug)]
enum STree {
    Node(NodeType),
    Leaf(i64),
    Empty,
}

trait SearchTree {
    fn search_successor(&self, x: i64) -> Option<i64>;
    fn insert(&self, x: i64) -> STree;
    fn new(nums: &Vec<i64>) -> STree;
}

impl SearchTree for STree {
    fn insert(&self, x: i64) -> STree {
        match self {
            STree::Node(NodeType::Full { value, left, right }) => {
                if x > *value {
                    right.insert(x)
                } else {
                    left.insert(x)
                }
            }
            STree::Node(NodeType::Left { value, left }) => {
                if x > *value {
                    STree::Node(NodeType::Full {
                        value: *value,
                        left: left.clone(),
                        right: Rc::new(STree::Leaf(x)),
                    })
                } else {
                    STree::Node(NodeType::Left {
                        value: *value,
                        left: Rc::new(left.insert(x)),
                    })
                }
            }
            STree::Node(NodeType::Right { value, right }) => {
                if x > *value {
                    STree::Node(NodeType::Right {
                        value: *value,
                        right: Rc::new(right.insert(x)),
                    })
                } else {
                    STree::Node(NodeType::Full {
                        value: *value,
                        left: Rc::new(STree::Leaf(x)),
                        right: right.clone(),
                    })
                }
            }
            STree::Leaf(value) => {
                if x > *value {
                    STree::Node(NodeType::Right {
                        value: *value,
                        right: Rc::new(STree::Leaf(x)),
                    })
                } else {
                    STree::Node(NodeType::Left {
                        value: *value,
                        left: Rc::new(STree::Leaf(x)),
                    })
                }
            }
            STree::Empty => STree::Leaf(x),
        }
    }
    fn new(nums: &Vec<i64>) -> STree {
        let mut tree = STree::Empty;
        for x in nums {
            tree = tree.insert(*x)
        }
        tree
    }
    fn search_successor(&self, x: i64) -> Option<i64> {
        match self {
            STree::Node(NodeType::Full { value, left, right }) => None,
            STree::Node(NodeType::Left { value, left }) => None,
            STree::Node(NodeType::Right { value, right }) => None,
            STree::Leaf(value) => None,
            STree::Empty => None,
        }
    }
}

pub fn sum2(numbers: HashSet<i64>, ts: Range<i64>) -> usize {
    let mut numbers_v: Vec<i64> = numbers.iter().fold(vec![], |mut acc, n| {
        acc.push(*n);
        acc
    });
    numbers_v.sort();
    //let mut L = 1;
    // let numbers_count = HashMap::new();
    // for n in &numbers_v {
    //     numbers_count.insert(n, L);
    //     println!("n:{} | L:{}", n, L);
    //     L += 1;
    // }
    println!("numbers_v:{:?}", numbers_v);
    let mut regions: Vec<Range<i64>> = numbers.iter().fold(vec![], |mut regions, x| {
        let region = new_region(x, &ts);
        regions.push(region);
        regions
    });
    let min = numbers_v[0];
    println!("min:{}", min);
    regions.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    //let mut numbers_iter = numbers_v.iter();
    println!("regions {:?}", regions);
    let mut counter = 0;
    let exploration =
        regions.iter().fold(
            None,
            |previously_explored, region| match previously_explored {
                Some(previously_explored) => {
                    let (explored, unexplored) = explore(&previously_explored, &region);
                    println!("region {:?}", region);
                    println!("previously_explored {:?}", previously_explored);
                    println!("unexplored {:?}", unexplored);
                    let fallback = explored.clone();
                    unexplored
                        .and_then(|u| {
                            let lower_bound = numbers_v.iter().position(|x| x >= &&u.start);
                            let upper_bound = numbers_v.iter().position(|x| x >= &&u.end);
                            lower_bound.and_then(|lower_bound| {
                                upper_bound.map(|upper_bound| (lower_bound, upper_bound))
                            })
                        })
                        .map(|(lower_bound, _upper_bound)| {
                            let upper_bound = if _upper_bound <= lower_bound {
                                lower_bound
                            } else {
                                _upper_bound
                            };
                            println!("lower_bound:{}", lower_bound);
                            println!("upper_bound:{}", upper_bound);
                            println!("low:{}", numbers_v[lower_bound]);
                            println!("up:{}", numbers_v[upper_bound]);
                            let count = numbers_v[lower_bound..upper_bound].len() + 1;
                            //numbers_v.iter() = numbers_v[lower_bound ..].iter();
                            counter += count;
                            println!("count {:?}", count);
                            println!("counter {:?}", counter);
                            let result = explored.start..numbers_v[upper_bound].max(explored.end);
                            println!("explored final {:?}", result);
                            result
                        })
                        .or(Some(fallback))
                }
                None => {
                    println!("region:{:?}", region);
                    let lower_bound = numbers_v.iter().position(|x| x >= &&region.start).unwrap();
                    let _upper_bound = numbers_v.iter().position(|x| x >= &&region.end).unwrap();
                    let upper_bound = if _upper_bound <= lower_bound {
                        lower_bound + 1
                    } else {
                        _upper_bound
                    };
                    println!("lower_bound:{}", lower_bound);
                    println!("upper_bound:{}", upper_bound);
                    println!("low:{}", numbers_v[lower_bound]);
                    println!("up:{}", numbers_v[upper_bound]);
                    let count = numbers_v[lower_bound..upper_bound].len() + 1;
                    counter += count;
                    println!("count {:?}", count);
                    println!("counter {:?}", counter);
                    Some(region.start..numbers_v[upper_bound].max(region.end))
                }
            },
        );
    println!("exploration {:?}", exploration);
    counter
}
