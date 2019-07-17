use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub type Graph = HashMap<usize, Vec<usize>>;

pub fn run_algo(file_name: &str) -> std::io::Result<()> {
    let g = create_graph_from_file(file_name)?;
    println!("graph created");
    //println!("g:{:?}",g);
    let g_rev = reverse_graph(&g);
    println!("reversed graph created");
    //println!("g_rev:{:?}",g_rev);
    let Finished {
        mut finishing_times,
        ..
    } = dfs_loop(&g_rev);
    println!("finishing_times done");
    let renamed_graph = rename_graph(&g, &mut finishing_times);
    println!("renamed graph");
    //println!("renamed_graph:{:?}",renamed_graph);
    let Finished { mut sizes, .. } = dfs_loop(&renamed_graph);
    sizes.sort();
    sizes.reverse();
    sizes.truncate(5);
    println!("Sizes: {:?}", sizes);
    Ok(())
}

pub fn create_graph_from_file(file_name: &str) -> Result<Graph, std::io::Error> {
    let file = File::open(file_name)?;
    let buf_reader = BufReader::new(file);
    let mut g: Graph = HashMap::new();
    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                let mut two_values = l.split(' ');
                let node = two_values.next().unwrap().parse::<usize>().unwrap();
                let adj = two_values.next().unwrap().parse::<usize>().unwrap();
                g.entry(node)
                    .and_modify(|adjs| adjs.push(adj))
                    .or_insert(vec![adj]);
            }
            _ => (),
        }
    }
    Ok(g)
}

fn reverse_graph(g: &Graph) -> Graph {
    let mut rev: Graph = HashMap::new();
    for (&node, _adj) in g {
        let mut adj = _adj.clone();
        while let Some(a) = adj.pop() {
            rev.entry(a)
                .and_modify(|adj| adj.push(node))
                .or_insert(vec![node]);
        }
    }
    rev
}

struct Finished {
    pub finishing_times: Vec<usize>,
    pub sizes: Vec<usize>,
}

fn rename_graph(g: &Graph, finishing_times: &mut Vec<usize>) -> Graph {
    let mut new_labels: HashMap<usize, usize> = HashMap::new();
    let mut renamed: Graph = HashMap::new();
    let mut counter = 0;
    finishing_times.reverse();
    while let Some(n) = finishing_times.pop() {
        counter = counter + 1;
        new_labels.insert(n, counter);
    }
    for (&node, adj) in g {
        let mut ad_cl = adj.clone();
        let mut renamed_adj = Vec::new();
        while let Some(a) = ad_cl.pop() {
            renamed_adj.push(new_labels[&a]);
        }
        renamed.insert(new_labels[&node], renamed_adj);
    }
    renamed
}

fn dfs_loop(g: &Graph) -> Finished {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut finishing_times = Vec::new();
    let mut sizes = Vec::new();
    let mut nodes = sorted_nodes(&g);
    while let Some(n) = nodes.pop() {
        if !visited.contains(&n) {
            let mut finishing_batch = dfs_for_finishing_times(g, &n, &mut visited);
            sizes.push(finishing_batch.len());
            finishing_times.append(&mut finishing_batch);
        }
    }
    Finished {
        finishing_times,
        sizes,
    }
}

fn sorted_nodes(g: &Graph) -> Vec<usize> {
    let mut node_set = HashSet::new();
    for (&node, adj) in g {
        node_set.insert(node);
        let mut adj_c = adj.clone();
        while let Some(a) = adj_c.pop() {
            node_set.insert(a);
        }
    }
    let mut keys: Vec<usize> = node_set.drain().fold(Vec::new(), |mut acc, k| {
        acc.push(k);
        acc
    });
    keys.sort();
    keys
}

fn dfs_for_finishing_times(g: &Graph, n: &usize, visited: &mut HashSet<usize>) -> Vec<usize> {
    let mut to_visit = vec![*n];
    let mut finishing_times = Vec::new();
    while let Some(node) = to_visit.pop() {
        visited.insert(node);
        let mut current_adj: Vec<usize> = adjacents(g, node, visited);
        match current_adj.pop() {
            Some(a) => {
                to_visit.push(node); //not done with `node` yet, putting back to the stack
                to_visit.push(a); //next node to visit is the first adjacent of `node`
            }
            None => {
                finishing_times.push(node);
            }
        }
    }
    finishing_times
}

fn adjacents(g: &Graph, n: usize, visited: &mut HashSet<usize>) -> Vec<usize> {
    g.get(&n)
        .map(|adj| adj.clone())
        .unwrap_or(Vec::new())
        .drain_filter(|a| !visited.contains(a))
        .collect::<Vec<_>>()
}
