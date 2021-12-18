use std::borrow::BorrowMut;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::{Rc, Weak};


// Based on https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/

#[derive(Hash, Clone, Debug)]
struct Node {
     // I'm to stupid to learn to deal with mem pointers for now
    id: Uuid,
    parent: Option<Weak<RefCell<Node>>>,
    value: Option<u32>,
    children: Vec<Node>,
}

impl Node {
    fn add_child(&mut self, mut child: Node) {
        child.parent = self;
        self.children.push(child);
    }

    fn is_leaf(&self) -> bool { self.value.is_some() }

    fn value(&self) -> u32 {  self.value.unwrap()  }

    unsafe fn add_left(&mut self, id: Uuid, value: u32) {
        if self.children[0].id == id {
            if self.parent != ptr::null_mut() {
                (*self.parent).add_left(self.id, value)
            }
            return
        }
        self.children[0].add_value(value)
    }

    unsafe fn add_right(&mut self, id: Uuid, value: u32) {
        if self.children[1].id == id {
            if self.parent != ptr::null_mut() {
                (*self.parent).add_left(self.id, value)
            }
            return
        }
        self.children[1].add_value(value)
    }

    fn add_value(&mut self, value: u32) {
        self.value = Option::from(self.value() + value)
    }

    unsafe fn explode(&mut self) {
        if !self.children[0].is_leaf() || !self.children[0].is_leaf() { panic!("Cannot handle this yet ..") }
        self.parent.read_unaligned().add_left(self.id,self.children[0].value());
        self.parent.read_unaligned().add_right(self.id, self.children[1].value());
    }

    fn split(&mut self) {

        let value_left = Option::from(self.value() / 2);
        let value_right = Option::from(self.value() - value_left.unwrap());
        self.value = None;
        self.children.push(Node { id: Uuid::new_v4(),
                                        parent: &mut *self,
                                        value: value_left,
                                        children: Vec::new()});
        self.children.push(Node { id: Uuid::new_v4(),
                                        parent: &mut *self,
                                        value: value_right,
                                        children: Vec::new() });

    }



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
    let file_in = File::open(filename)?;
    let nodes = BufReader::new(file_in).lines()
                                             .map(|x|parse_line(x.unwrap().as_str()))
                                             .collect();
    Ok(nodes)
}

fn parse_line(line: &str) -> Node {
    let mut nodes: Vec<Vec<Node>> = Vec::new();
    for c in line.chars() {
        match c {
            '[' => {  nodes.push(Vec::new()) },
            ']' => {
                let back = nodes.pop().unwrap();
                let mut node = Node {
                                   id: Uuid::new_v4(),
                                   parent: ptr::null_mut(),
                                   value: None,
                                   children: Vec::new(),
                                };
                for child in back { node.add_child(child); }

                if nodes.is_empty() {
                    return node;
                } else {
                    nodes.last_mut().unwrap().push( node)
                }
            },
            ',' => {  /* skip */ },
            _  => { nodes.last_mut().unwrap().push( Node {
                    id: Uuid::new_v4(),
                    parent: ptr::null_mut(),
                    value: Option::from(c.to_digit(10).unwrap()),
                    children: Vec::new(),
            })}
        }
    }
    panic!("Should not be reached ..");
}
