#![feature(drain_filter)]

mod week1;

fn main() -> std::io::Result<()> {
    week1::run_algo("graph.txt")
}
