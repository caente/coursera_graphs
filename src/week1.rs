use std::collections::HashMap;
use std::collections::HashSet;

pub type Graph = HashMap<u32, Vec<u32>>;
/*
fn createGraph<'a>(g: &Graph, node: &u32, adj: &u32) -> &'a Graph {
    g.entry(*node).or_insert(vec![Box::new(*adj)]);
    &g
}
*/

pub fn dfs_loop(g: &Graph) -> Vec<u32> {
    let mut keys: Vec<u32> = g.keys().fold(Vec::new(), |mut acc, &k| {
        acc.push(k);
        acc
    });
    keys.sort();
    //println!("KEYS: {:?}", keys);
    let mut visited: HashSet<u32> = HashSet::new();
    while let Some(n) = keys.pop() {
        if !visited.contains(&n) {
            dfs(g, &n, &mut visited);
        }
    }
    let sizes = Vec::new();
    sizes
}
pub fn dfs(g: &Graph, n: &u32, visited: &mut HashSet<u32>) {
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
