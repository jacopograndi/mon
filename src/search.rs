use std::cmp::{min, max};

pub trait State {
    fn next(&self) -> Vec<Self> where Self: Sized;    
    fn eval(&self) -> i32;
}

pub fn minimax<T: Clone + State> (
    node: &T, 
    depth : i32, 
    maxer : bool) -> i32 
{
    let children = node.next();
    if depth == 0 || children.len() == 0 {
        node.eval()
    } else {
        if maxer {
            let mut value = i32::MIN;
            for child in children {
                value = max(value, minimax(&child, depth-1, false));
            }
            value
        } else {
            let mut value = i32::MAX;
            for child in children {
                value = min(value, minimax(&child, depth-1, true));
            }
            value
        }
    }
}
