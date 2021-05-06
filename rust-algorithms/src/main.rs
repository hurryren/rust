use std::fmt::{self, Display,Formatter};
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next:Option<NonNull<Node<T>>>,
    prev:Option<NonNull<Node<T>>>,
}

impl <T> Node<T>{
    fn new(t:T) ->Node<T>{
        Node{
            val:t,
            prev:None,
            next:None,
        }
    }
}

pub struct LinkedList<T> {
    length:u32,
    start : Option<NonNull<Node<T>>>,
    end : Option<NonNull<Node<T>>>,
}

impl <T> LinkedList<T>{
    pub fn new() -> Self {
        Self {
            length:0,
            start:None,
            end:None,
        }
    }

    pub fn add(&mut self, obj:T){
        let mut node = Box::new(Node::new(obj));
        // since we are adding node at the end, next will always be None
        node.next = None;
        node.prev = self.end;
        // get a pointer to node
        let node_ptr = Some(unsafe{NonNull::new_unchecked(Box::into_raw(node))});
        match self.end{
            // this is the case of empty list
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe{(*end_ptr.as_ptr()).next = node_ptr},
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index:i32) -> Option<&T>{
        self.get_ith_ndoe(self.start,index)
    }

    fn get_ith_ndoe(&mut self, node: Option<NonNull<Node<T>>>, index:i32) -> Option<&T>{
        match node {
            None => None,
            Some(next_ptr) => match index{
                0 => Some(unsafe{&(*next_ptr.as_ptr()).val}),
                _ => self.get_ith_ndoe(unsafe{(*next_ptr.as_ptr()).next}, index-1),
            },
        }
    }

}



fn main() {
    println!("Hello, world!");
}
