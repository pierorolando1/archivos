
use std::fs::File;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

use std::fs::read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    key: i32,
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: i32, value: String) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, key: i32, value: String) {
        if key < self.key {
            if let Some(ref mut left) = self.left {
                left.insert(key, value);
            } else {
                self.left = Some(Box::new(Node::new(key, value)));
            }
        } else if key > self.key {
            if let Some(ref mut right) = self.right {
                right.insert(key, value);
            } else {
                self.right = Some(Box::new(Node::new(key, value)));
            }
        }
    }
}


pub fn save_tree_to_binary(tree: &Node, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let encoded: Vec<u8> = bincode::serialize(tree).expect("Error al serializar el árbol");
    file.write_all(&encoded)?;
    Ok(())
}


pub fn load_tree_from_binary(filename: &str) -> Result<Node, Box<dyn std::error::Error>> {
    let data = read(filename)?;
    let tree: Node = bincode::deserialize(&data)?;
    Ok(tree)
}
