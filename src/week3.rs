use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn load_numbers(file_name: &str) -> Result<Vec<usize>, std::io::Error> {
    let file = File::open(file_name)?;
    let buf_reader = BufReader::new(file);
    let mut numbers = Vec::new();
    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                let number = l.parse::<usize>().unwrap();
                numbers.push(number);
            }
            _ => (),
        }
    }
    Ok(numbers)
}

pub fn medians(numbers: Vec<usize>) -> Vec<usize> {
    let mut lower: BinaryHeap<usize> = BinaryHeap::new();
    let mut upper: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut median = Vec::new();
    for &number in numbers.iter() {
        match lower.peek() {
            Some(&l) if number < l => lower.push(number),
            Some(_)  => upper.push(Reverse(number)),
            None => lower.push(number)
        }
        balance_heaps(&mut lower, &mut upper);
        match lower.peek() {
            Some(m) => median.push(*m),
            None => (),
        }
    }
    median
}

fn balance_heaps(lower: &mut BinaryHeap<usize>, upper: &mut BinaryHeap<Reverse<usize>>) {
    let len_lower = lower.len() as i32;
    let len_upper = upper.len() as i32;
    if len_lower - len_upper > 1 {
        match lower.pop() {
            Some(l) => upper.push(Reverse(l)),
            None => (),
        }
    } else if len_upper > len_lower {
        match upper.pop() {
            Some(Reverse(u)) => lower.push(u),
            None => (),
        }
    }
}
