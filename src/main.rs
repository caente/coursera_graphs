#![feature(drain_filter)]

mod week3;

fn main() -> std::io::Result<()> {
    let numbers = week3::load_numbers("median.txt")?;
    let medians:Vec<usize> = week3::medians(numbers);
    let sum:usize = medians.iter().sum();
    println!("{:?}", sum % 10000);
    Ok(())
}
