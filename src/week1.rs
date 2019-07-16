use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub type Graph = HashMap<u32, Vec<u32>>;

pub fn create_graph_from_file(file_name:&str) -> Result<Graph,std::io::Error> {
    let file = File::open(file_name)?;
    let buf_reader = BufReader::new(file);
    let mut g:Graph = HashMap::new();
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
    Ok(g)
}

pub fn dfs_loop(g: &Graph) -> Vec<u32> {
    let mut keys: Vec<u32> = g.keys().fold(Vec::new(), |mut acc, &k| {
        acc.push(k);
        acc
    });
    keys.sort();
    let mut visited: HashSet<u32> = HashSet::new();
    while let Some(n) = keys.pop() {
        if !visited.contains(&n) {
            dfs(g, &n, &mut visited);
        }
    }
    let sizes = Vec::new();
    sizes
}
fn dfs(g: &Graph, n: &u32, visited: &mut HashSet<u32>) {
    visited.insert(*n);
    let mut to_visit = g.get(n).map(|adj| adj.clone()).unwrap_or(Vec::new());
    loop_visits(&g, &mut to_visit, visited);
    println!("Finished visiting {}", n);
}

fn loop_visits(g: &Graph, to_visit: &mut Vec<u32>, visited: &mut HashSet<u32>) {
    while let Some(next) = to_visit.pop() {
        if !visited.contains(&next) {
            visited.insert(next);
            let mut next_to_visit: Vec<u32> = g
                .get(&next)
                .map(|adj| adj.clone())
                .unwrap_or(Vec::new())
                .drain_filter(|a| !visited.contains(a))
                .collect::<Vec<_>>();
            to_visit.append(&mut next_to_visit);
        }
        println!("Finished visiting {}", next);
    }
}
