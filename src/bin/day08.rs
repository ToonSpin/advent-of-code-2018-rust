use std::io;
use std::io::prelude::*;

struct Node {
    metadata: Vec<u8>,
    children: Vec<Node>,
}

impl Node {
    fn new() -> Node {
        Node {
            metadata: Vec::new(),
            children: Vec::new(),
        }
    }

    fn parse(&mut self, mut input: &[u8]) -> usize {
        let num_children = input[0];
        let num_metadata = input[1] as usize;

        input = &input[2..];
        let mut num_read_total: usize = num_metadata + 2;

        for _i in 0..num_children {
            let mut child = Node::new();
            let num_read = child.parse(input);
            self.children.push(child);

            num_read_total += num_read;
            input = &input[num_read..];
        }

        for i in 0..num_metadata {
            self.metadata.push(input[i as usize]);
        }

        num_read_total
    }

    fn sum_of_metadata(&self) -> u32 {
        let mut sum = 0;

        for child in self.children.iter() {
            sum += child.sum_of_metadata();
        }

        for i in 0..self.metadata.len() {
            sum += self.metadata[i] as u32;
        }

        sum
    }

    fn value(&self) -> u32 {
        if self.children.len() == 0 {
            self.sum_of_metadata()
        } else {
            let mut value = 0;

            for i in 0..self.metadata.len() {
                let index = self.metadata[i] as usize;
                if let Some(child) = self.children.get(index - 1) {
                    value += child.value();
                }
            }

            value
        }
    }
}

fn main() -> io::Result<()> {
    let mut input: Vec<u8> = Vec::new();

    for line in io::stdin().lock().lines() {
        for entry in line.unwrap().split(' ') {
            input.push(entry.parse().unwrap());
        }
    }

    let mut root = Node::new();
    root.parse(&input[..]);

    println!("The sum of all metadata: {}", root.sum_of_metadata());
    println!("The value of the root node: {}", root.value());

    Ok(())
}
