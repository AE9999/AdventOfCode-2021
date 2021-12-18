use std::borrow::BorrowMut;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};


// Based on https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/

pub struct Tree {
    // All the nodes are owned by the `nodes` vector. Throughout the code, a
    // NodeHandle value of 0 means "none".
    root: NodeHandle,
    nodes: Vec<Node>,
    count: usize,
}

impl Tree {

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

    fn explode(&mut self, node: &NodeHandle) {
        /*self.nodes[node]

        self.update_left();
        self.update_right();
        self.nodes[node] = Node {
            id: *node,
            data: Some(0),
            left: None,
            right: None,
            parent: None,
            nest_level: 1,
        }*/
    }

    fn split(&mut self, node: &NodeHandle) {
        
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
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let snail_fish_numbers = read_lines(input).unwrap();
    for snail_fish_number in snail_fish_numbers {
        println!("Read: {:?}", snail_fish_number);
    }
    println!("{:?} is the magnitude of the final sum ..", 0);
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Vec<Node>> {
    // let file_in = File::open(filename)?;
    // let nodes = BufReader::new(file_in).lines()
    //                                          .map(|x|parse_line(x.unwrap().as_str()))
    //                                          .collect();
    Ok(Vec::new())
}

fn parse_line(line: &str, tree: &mut Tree) -> NodeHandle {
    let mut nodes : Vec<Vec<NodeHandle>> = Vec::new();
    for c in line.chars() {
        match c {
            '[' => {  nodes.push(Vec::new()) },
            ']' => {
                let back = nodes.pop().unwrap();
                if back.len() != 2 { panic!("We are building binary trees here") }
                let mut node = tree.alloc_parent_node(&back[0], &back[1]);
                if nodes.is_empty() {
                    return node;
                } else {
                    nodes.last_mut().unwrap().push( node)
                }
            },
            ',' => {  /* skip */ },
            _  => {
                let node = tree.alloc_node (c.to_digit(10).unwrap());
                nodes.last_mut().unwrap().push(node)
            }
        }
    }
    panic!("Should not be reached ..");
}
