use std::borrow::{Borrow, BorrowMut};
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;

// Based on https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/

pub struct TreeManager {
    nodes: Vec<Node>,
}

impl TreeManager {

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

    fn alloc_parent_node(&mut self, left: &NodeHandle, right: &NodeHandle) -> NodeHandle {
        let id: NodeHandle = self.nodes.len();
        self.nodes.push(Node {
            id: id.clone(),
            data: None,
            left: Some(left.clone()),
            right: Some(right.clone()),
            parent: None,
            nest_level: 0,
        });
        self.nodes[*left].parent = Some(id);
        self.nodes[*right].parent = Some(id);
        self.update_nest_level(&id);
        id
    }

    fn explode_left(&mut self, value_left: u32, origin: &NodeHandle) {
        loop {
            let parent = self.nodes[*origin].parent;
            if parent.is_none() { break }
            if self.nodes[*origin] {}
        }
    }

    fn explode_right(&mut self, value_left: u32, origin: &NodeHandle) {

    }

    fn explode(&mut self, node_id: &NodeHandle) {
        let nest_level = self.nodes[*node_id].nest_level;
        if nest_level < 4 { panic!("We can only explode at 4") }
        let value_left = self.nodes[self.nodes[*node_id].left.unwrap()].data.unwrap();
        let value_right = self.nodes[self.nodes[*node_id].right.unwrap()].data.unwrap();
        self.explode_left(value_left, node_id);
        self.explode_right(value_right, node_id);
        self.nodes[*node_id] = Node {
            id: *node_id,
            data: Some(0),
            left: None,
            right: None,
            parent: None,
            nest_level: self.nodes[*node_id].nest_level, // Fix this
        }
    }

    fn split(&mut self, node_id: &NodeHandle) {
        let value = self.nodes[*node_id].data.unwrap();
        let lvalue = value / 2;
        let rvalue =  value - lvalue;
        let new_left = self.alloc_node(lvalue);
        let new_right = self.alloc_node(rvalue);

        self.nodes[*node_id] = Node {
            id: *node_id,
            data: None,
            left: Some(new_left),
            right: Some(new_right),
            parent: self.nodes[*node_id].parent,
            nest_level: self.nodes[*node_id].nest_level, // Fix this
        }

    }

    fn add(&mut self, left: &NodeHandle, right: &NodeHandle) -> NodeHandle {
        let node = self.alloc_parent_node(left, right);
        self.update_nest_level(&node);
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
    for snail_fish_number in snail_fish_numbers {
        println!("Read: {:?}", snail_fish_number);
    }
    println!("{:?} is the magnitude of the final sum ..", 0);
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
                let mut node = tree_manager.alloc_parent_node(&back[0], &back[1]);
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
