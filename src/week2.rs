use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Eq, Copy, Hash)]
pub struct Edge {
    start: usize,
    end: usize,
    cost: usize,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.start == self.start && self.end == self.end
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

type Graph = HashMap<usize, Vec<Edge>>;

pub fn create_graph_from_file(file_name: &str) -> Result<Graph, std::io::Error> {
    let file = File::open(file_name)?;
    let buf_reader = BufReader::new(file);
    buf_reader.lines().fold(Ok(HashMap::new()), |acc, line| {
        let l: String = line?;
        let mut row = l.split(' ');
        let node_s = row.next().unwrap();
        let start = node_s.parse::<usize>().unwrap();
        let mut adj = Vec::new();
        for tuple_s in row {
            let mut tuple_it = tuple_s.split(',');
            let end = tuple_it.next().unwrap().parse::<usize>().unwrap();
            let cost = tuple_it.next().unwrap().parse::<usize>().unwrap();
            adj.push(Edge { start, end, cost });
        }
        let mut g = acc?;
        g.insert(start, adj);
        Ok(g)
    })
}

pub fn dijkstra(g: &Graph) -> HashMap<usize, usize> {
    let mut A = HashMap::new();
    A.insert(1, 0);
    let mut nodes = all_nodes(&g);
    let mut edges = all_edges(&g);
    nodes.remove(&1);
    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(1);
    let mut heap = BinaryHeap::from(edges);
    let mut queue = VecDeque::new();
    queue.push_back(Edge{start:1,end:1,cost:0});
    while let Some(edge) = queue.pop_front() {
        println!("edge: {:?}", edge);
        if visited.contains(&edge.start) && !visited.contains(&edge.end) {
            visited.insert(edge.end);
            let cost = edge.cost + A[&edge.start];
            heap.push(Edge {
                start: edge.start,
                end: edge.end,
                cost,
            });
            println!("heap: {:?}", heap);
            A.entry(edge.end).(edge.end, cost);
            println!("A: {:?}", A);
        } else {
            queue.push_back(edge)
        }
    }
    while let Some(edge) = edges.pop() {
        println!("visited: {:?}", visited);
        println!("edge: {:?}", edge);
        if visited.contains(&edge.start) && !visited.contains(&edge.end) {
            visited.insert(edge.end);
            let cost = edge.cost + A[&edge.start];
            heap.push(Edge {
                start: edge.start,
                end: edge.end,
                cost,
            });
        println!("heap: {:?}", heap);
            A.insert(edge.end, cost);
            println!("A: {:?}", A);
        } else {
            heap.push(edge);
        }
    }

    println!("heap: {:?}", heap);
    A
}

fn all_edges(g: &Graph) -> Vec<Edge> {
    let mut edges = vec![Edge {
        start: 1,
        end: 1,
        cost: 0,
    }];
    for (_, adj) in g {
        let mut adj_c = adj.clone();
        while let Some(a) = adj_c.pop() {
            edges.push(a);
        }
    }
    edges
}

fn all_nodes(g: &Graph) -> HashSet<usize> {
    let mut node_set = HashSet::new();
    for (&node, adj) in g {
        node_set.insert(node);
        let mut adj_c = adj.clone();
        while let Some(a) = adj_c.pop() {
            node_set.insert(a.end);
        }
    }
    node_set
}
