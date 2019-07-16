#![feature(drain_filter)]

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

mod week1;



fn main() -> std::io::Result<()>{
    let file = File::open("SCC.txt")?;
    let buf_reader = BufReader::new(file);
    let mut g:week1::Graph = HashMap::new();
    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                let mut two_values= l.split(' ');
                let node=   two_values.next().unwrap().parse::<u32>().unwrap();
                let adj = two_values.next().unwrap().parse::<u32>().unwrap();
                g.entry(node)
                    .and_modify(|adjs| adjs.push(adj))
                    .or_insert(vec![adj]);
            },
            _ => ()
        }
    }
    //println!("{}",g.len());
    //println!("{:?}",g);
    week1::dfs_loop(&g);
    Ok(())
   //let mut g = HashMap::new();
   //week1::dfs(&g, &0)
}
