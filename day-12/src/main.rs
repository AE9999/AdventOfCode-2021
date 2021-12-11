use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Debug)]
struct Node {
    name: String
}

impl Node {
    fn is_big(&self) -> bool { self.name.to_uppercase() == self.name }

    fn is_end(&self) -> bool { self.name == "end" }

    fn is_start(&self) -> bool { self.name == "start" }
}

struct Graph {
    node2nodes: HashMap<Node, Vec<Node>>
}

impl Graph {

    fn count_distinct_paths(&self, current:Node, visited: HashSet<Node>) -> u64 {
        if current.is_end() {
            1
        } else {
            self.node2nodes.get(&current)
                           .unwrap()
                           .iter().filter(|node| node.is_big() || ! visited.contains(node))
                           .map(|node_to_visit| {
                               let mut visited = visited.clone();
                               visited.insert(current.clone());
                               self.count_distinct_paths(node_to_visit.clone(), visited)
                           }).fold(0, |acc,x| acc + x)
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let graph = read_lines(input).unwrap();
    let answer = graph.count_distinct_paths(graph.node2nodes.keys().find(|x|x.is_start()).unwrap().clone(),
                                           HashSet::new());
    println!("{:?} many paths through this cave system are there that visit small caves at most once ..", answer);
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Graph> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let mut node2nodes : HashMap<Node, Vec<Node>> = HashMap::new();
    file_reader.lines()
                                               .map(|x|x.unwrap())
                                               .for_each(|line| {
                                                    let mut split = line.split("-").into_iter();
                                                    let a = Node { name: String::from(split.next().unwrap()) };
                                                    let b = Node { name: String::from(split.next().unwrap()) };
                                                    if !(node2nodes.contains_key(&a)) { node2nodes.insert(a.clone(), Vec::new()); }
                                                    if !(node2nodes.contains_key(&b)) { node2nodes.insert(b.clone(), Vec::new()); }
                                                    node2nodes.get_mut(&a).unwrap().push(b.clone());
                                                    node2nodes.get_mut(&b).unwrap().push(a.clone());
                                               });
    Ok(Graph { node2nodes })
}
