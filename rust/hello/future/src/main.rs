use std::{fs, thread, time::{Duration,Instant}};
use tokio::{fs as tokio_fs, try_join};
#[tokio::main]
async fn main() {
    let start = Instant::now();
    let lock = fs::read_to_string("../Cargo.lock").unwrap();
    let toml = fs::read_to_string("../Cargo.toml").unwrap();
    
    fs::write("./lock.txt", lock).unwrap();
    fs::write("./toml.txt", toml).unwrap();
    println!("{:?}",start.elapsed());

    let start = Instant::now();
    let lock_handle = thread::spawn(|| {
        let lock = fs::read_to_string("../Cargo.lock").unwrap();
        fs::write("./lock.txt", lock).unwrap();
    });
    let toml_handle = thread::spawn(|| {
        let toml = fs::read_to_string("../Cargo.toml").unwrap();
        fs::write("./toml.txt", toml).unwrap();
    });
    lock_handle.join().unwrap();
    toml_handle.join().unwrap();
    println!("{:?}",start.elapsed());
    
    let start = Instant::now();
    let tokio_lock = tokio_fs::read_to_string("../Cargo.lock");
    let tokio_toml = tokio_fs::read_to_string("../Cargo.toml");
    let (lock,toml) = try_join!(tokio_lock,tokio_toml).unwrap();
    let tokio_lock = tokio_fs::write("./lock.txt", lock);
    let tokio_toml = tokio_fs::write("./toml.txt", toml);
    try_join!(tokio_lock,tokio_toml).unwrap();
    println!("{:?}",start.elapsed());
    /*
    let start = Instant::now();
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 10..20 {
        println!("{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    let duration = start.elapsed();
    println!("{:?}",duration);
    */
}

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (from_node, to_node, weight) = edge;
        let item = self.adjacency_table.entry(from_node.to_string()).or_insert(vec![]);
        (*item).push((to_node.to_string(), weight));
        let item = self.adjacency_table.entry(to_node.to_string()).or_insert(vec![]);
        (*item).push((from_node.to_string(), weight));
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
		true
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}