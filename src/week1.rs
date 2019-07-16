use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub type Graph = HashMap<u32, Vec<u32>>;

pub fn create_graph_from_file(file_name: &str) -> Result<Graph, std::io::Error> {
    let file = File::open(file_name)?;
    let buf_reader = BufReader::new(file);
    let mut g: Graph = HashMap::new();
    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                let mut two_values = l.split(' ');
                let node = two_values.next().unwrap().parse::<u32>().unwrap();
                let adj = two_values.next().unwrap().parse::<u32>().unwrap();
                g.entry(node)
                    .and_modify(|adjs| adjs.push(adj))
                    .or_insert(vec![adj]);
            }
            _ => (),
        }
    }
    Ok(g)
}

pub fn dfs_loop(g: &Graph) -> Vec<u32> {
    let mut keys: Vec<u32> = g.keys().fold(Vec::new(), |mut acc, &k| {
        acc.push(k);
        acc
    });
    keys.sort();
    //println!("KEYS:{:?}", keys);
    let mut visited: HashSet<u32> = HashSet::new();
    let mut finishing_order = Vec::new();
    while let Some(n) = keys.pop() {
        if !visited.contains(&n) {
            dfs(g, &n, &mut finishing_order, &mut visited);
        }
    }
    println!("Finished visiting {:?}", finishing_order);
    let sizes = Vec::new();
    sizes
}
fn dfs(g: &Graph, n: &u32, finishing_order: &mut Vec<u32>, visited: &mut HashSet<u32>) {
    let mut unfinished = Vec::new();
    let mut to_visit = vec![*n];
    while let Some(next) = to_visit.pop() {
        visited.insert(next);
        //println!("visited: {:?}", visited);
        //println!("next: {}", next);
        let mut current_adj: Vec<u32> = adjacents(g, next, visited);
        //println!("adj: {:?}", current_adj);
        match current_adj.pop() {
            Some(a) => {
                to_visit.push(a);
                unfinished.push(next);
            }
            None => {
                finishing_order.push(next);
                to_visit.append(&mut unfinished);
                //println!("finishing_order: {:?}", finishing_order);
            }
        }
        //println!("to_visit: {:?}", to_visit);
        //println!("unfinished: {:?}", unfinished);
    }
    //println!("Finished visiting {:?}", finishing_order);
}

fn adjacents(g: &Graph, n: u32, visited: &mut HashSet<u32>) -> Vec<u32> {
    g.get(&n)
        .map(|adj| adj.clone())
        .unwrap_or(Vec::new())
        .drain_filter(|a| !visited.contains(a))
        .collect::<Vec<_>>()
}
