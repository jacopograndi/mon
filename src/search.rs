pub trait Next {
    fn next(&self) -> Vec<Self> where Self: Sized;    
}

pub fn search<T: Clone + Next> (init: &T) -> T {
    let nexts = init.next();
    nexts[0].clone()
}
