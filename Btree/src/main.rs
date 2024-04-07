use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem;

struct Node<T>{
    keys : Vec<T>,
    children : Vec<Node<T>>
}

pub struct Btree<T>{
    root : Node<T>,
    props : BtreeProps,
}

struct BtreeProps {
    degree : usize,
    max_keys : usize,
    mid_key_idx : usize
}

impl<T> Node<T>
where
    T: Ord
{
    fn new(degree : usize, _keys : Option<Vec<T>>, _children : Option<Vec<Node<T>>>) -> Self {
        Node {
            keys : match _keys {
                Some(_keys) => _keys,
                None => Vec::with_capacity(degree - 1),
            },
            children : match _children {
                Some(_child) => _child,
                None => Vec::with_capacity(degree)
            } 
        }
    }  
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}


fn main() {
    println!("Hello, world!");
}
