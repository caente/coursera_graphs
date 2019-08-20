#![feature(drain_filter)]

mod week4;

fn main() -> std::io::Result<()> {
    let mut numbers = week4::load_numbers("2sum.txt")?;
    let result = week4::sum2(
        &mut numbers,
        std::ops::Range {
            start: -10000,
            end: 10000,
        },
    );
    println!("{:?}", result);
    Ok(())
}
