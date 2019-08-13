#![feature(drain_filter)]

mod week4;

fn main() -> std::io::Result<()> {
    let numbers = week4::load_numbers("2sum1.txt")?;
    let result = week4::sum2(numbers, std::ops::Range { start: 3, end: 10+ 1});
    println!("{:?}", result);
    Ok(())
}
