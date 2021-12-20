use std::collections::HashSet;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;

// Based on https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/

pub struct TreeManager {
    nodes: Vec<Node>,
}

impl TreeManager {

    fn traverse(&mut self, root: NodeHandle) -> bool {
        let mut seen: HashSet<NodeHandle> = HashSet::new();
        let mut stack: Vec<NodeHandle> = Vec::new();
        stack.push(root);
        while !(stack.is_empty()) {
            let root = stack.pop().unwrap();
            if !seen.contains(&root) {
                seen.insert(root);

                if self.is_leaf(root) {
                    if self.get_leaf_value(root) >= 10 {
                        self.split(root);
                        return false;
                    }
                } else {
                    if self.nest_level(root) >= 4
                       && self.is_leaf(self.get_left(root))
                       && self.is_leaf(self.get_right(root)) {
                        self.explode(root);
                        return false;
                    }
                    stack.push(self.get_right(root));
                    stack.push(self.get_left(root));
                }
            }
        }
        return true;
    }

    fn print(&mut self, root: NodeHandle) {
        let mut seen: HashSet<NodeHandle> = HashSet::new();
        let mut stack: Vec<(NodeHandle, i32)> = Vec::new();
        stack.push((root, 0));
        while !(stack.is_empty()) {
            let (root, t) = stack.pop().unwrap();
            if !seen.contains(&root) {
                seen.insert(root);

                if self.is_leaf(root) {
                    print!("{:?}", self.get_leaf_value(root));
                    if t == 1 { print!(",") }
                } else {
                    print!("[");
                    stack.push((root, 0));
                    stack.push((self.get_right(root), 2));
                    stack.push((self.get_left(root), 1));
                }
            } else if !(self.is_leaf(root)) {
                print!("],");
            }
        }
        print!("\n");
    }

    fn magnitude(&self, node: NodeHandle) -> u32 {
        if self.is_leaf(node) {
            self.get_leaf_value(node)
        } else {
            (3 * self.magnitude(self.get_left(node)))
            + (2 * self.magnitude(self.get_right(node)))
        }
    }

    fn parent(&self, node: NodeHandle) -> Option<NodeHandle> {
        self.nodes[node].parent
    }

    fn get_left(&self, node: NodeHandle) -> NodeHandle {
        self.nodes[node].left.unwrap()
    }

    fn get_right(&self, node: NodeHandle) -> NodeHandle {
        self.nodes[node].right.unwrap()
    }

    fn is_leaf(&self, node: NodeHandle) -> bool {
        self.nodes[node].data.is_some()
    }

    fn get_leaf_value(&self, node: NodeHandle) -> u32 {
        self.nodes[node].data.unwrap()
    }

    fn nest_level(&self, node: NodeHandle) -> usize {
        self.nodes[node].nest_level
    }

    fn update_nest_level(&mut self, node: &NodeHandle) {
        self.nodes[*node].nest_level += 1;
        if self.nodes[*node].left.is_some() {
            self.update_nest_level(&(self.nodes[*node].left.unwrap()))
        }
        if self.nodes[*node].right.is_some() {
            self.update_nest_level(&(self.nodes[*node].right.unwrap()))
        }
    }

    fn alloc_node(&mut self, data: u32) -> NodeHandle {
        let id = self.nodes.len();
        self.nodes.push(Node {
            id: id.clone(),
            data: Some(data),
            left: None,
            right: None,
            parent: None,
            nest_level: 1,
        });
        id
    }

    fn alloc_parent_node(&mut self, left: NodeHandle, right: NodeHandle) -> NodeHandle {
        let id: NodeHandle = self.nodes.len();
        self.nodes.push(Node {
            id: id.clone(),
            data: None,
            left: Some(left.clone()),
            right: Some(right.clone()),
            parent: None,
            nest_level: 0,
        });
        self.nodes[left].parent = Some(id);
        self.nodes[right].parent = Some(id);
        self.update_nest_level(&id);
        id
    }

    fn explode_left(&mut self, value_left: u32, mut origin: NodeHandle) {
        loop {
            let parent = self.parent(origin);
            if parent.is_none() { return } // no value to add
            let parent = parent.unwrap();
            let mut potential = self.get_left(parent);
            if  potential != origin {
                loop {
                    if self.is_leaf(potential) {
                        self.nodes[potential].data = Some(self.nodes[potential].data.unwrap() + value_left);
                        break;
                    }
                    potential = self.get_right(potential); // Search right in the tree
                }
                break;
            } else {
                origin = parent;
            }
        }
    }

    fn explode_right(&mut self, value_left: u32, mut origin: NodeHandle) {
        loop {
            let parent = self.parent(origin);
            if parent.is_none() { return } // no value to add
            let parent = parent.unwrap();
            let mut potential = self.get_right(parent);
            if  potential != origin {
                loop {
                    if self.is_leaf(potential) {
                        self.nodes[potential].data = Some(self.nodes[potential].data.unwrap() + value_left);
                        break;
                    }
                    potential = self.get_left(potential); // Search left in the tree
                }
                break;
            } else {
                origin = parent;
            }
        }
    }

    fn explode(&mut self, node_id: NodeHandle) {
        let nest_level = self.nodes[node_id].nest_level;
        if nest_level < 4 { panic!("We can only explode at 4") }
        let value_left = self.nodes[self.nodes[node_id].left.unwrap()].data.unwrap();
        let value_right = self.nodes[self.nodes[node_id].right.unwrap()].data.unwrap();
        self.explode_left(value_left, node_id);
        self.explode_right(value_right, node_id);
        self.nodes[node_id] = Node {
            id: node_id,
            data: Some(0),
            left: None,
            right: None,
            parent: None,
            nest_level: self.nodes[node_id].nest_level, // Fix this
        }
    }

    fn split(&mut self, node_id: NodeHandle) {
        let value = self.nodes[node_id].data.unwrap();
        let lvalue = value / 2;
        let rvalue =  value - lvalue;
        let new_left = self.alloc_node(lvalue);
        let new_right = self.alloc_node(rvalue);

        self.nodes[node_id] = Node {
            id: node_id,
            data: None,
            left: Some(new_left),
            right: Some(new_right),
            parent: self.nodes[node_id].parent,
            nest_level: self.nodes[node_id].nest_level, // Fix this
        }
    }

    fn add(&mut self, left: NodeHandle, right: NodeHandle) -> NodeHandle {
        self.print(left);
        self.print(right);

        let node = self.alloc_parent_node(left, right);

        self.update_nest_level(&node);
        self.print(node);
        loop {
            if self.traverse(node) { break; }
        }
        node
    }
}

type NodeHandle = usize;

#[derive(Debug)]
struct Node {
    id: usize,
    data: Option<u32>,
    left: Option<NodeHandle>,
    right: Option<NodeHandle>,
    parent: Option<NodeHandle>,
    nest_level: usize,
}

fn main() -> io::Result<()> {
    let mut tree_manager = TreeManager { nodes: Vec::new() } ;
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let snail_fish_numbers = read_lines(input, &mut tree_manager).unwrap();
    let mut root = snail_fish_numbers[0];
    tree_manager.print(root);
    for i in 1..snail_fish_numbers.len() {
        root = tree_manager.add(root, snail_fish_numbers[i]);
        tree_manager.print(root);
    }

    println!("{:?} is the magnitude of the final sum ..", tree_manager.magnitude(root));
    Ok(())
}

fn read_lines(filename: &String, tree_manager: &mut TreeManager) -> io::Result<Vec<NodeHandle>> {
    let file_in = File::open(filename)?;
    let nodes : Vec<NodeHandle> = BufReader::new(file_in).lines()
                                             .map(|x|parse_line(x.unwrap().as_str(), tree_manager))
                                             .collect();
    Ok(nodes)
}

fn parse_line(line: &str, tree_manager: &mut TreeManager) -> NodeHandle {
    let mut nodes : Vec<Vec<NodeHandle>> = Vec::new();
    for c in line.chars() {
        match c {
            '[' => {  nodes.push(Vec::new()) },
            ']' => {
                let back = nodes.pop().unwrap();
                if back.len() != 2 { panic!("We are building binary trees here") }
                let node = tree_manager.alloc_parent_node(back[0], back[1]);
                if nodes.is_empty() {
                    return node;
                } else {
                    nodes.last_mut().unwrap().push( node)
                }
            },
            ',' => {  /* skip */ },
            _  => {
                let node = tree_manager.alloc_node (c.to_digit(10).unwrap());
                nodes.last_mut().unwrap().push(node)
            }
        }
    }
    panic!("Should not be reached ..");
}
