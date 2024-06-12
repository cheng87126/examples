use std::fmt::Display;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug)]
enum Cache<T> {
    Node(T,Box<Cache<T>>),
    Nil
}
fn _cache(){
    let _node = Cache::Node(0,Box::new(Cache::Nil));
    let _nil = Cache::<i32>::Nil;
}

impl<T:Display> Cache<T>{
    fn new(node:T) -> Cache<T>{
        Cache::Node(node, Box::new(Cache::Nil))
    }
    fn println(&self)->String{
        match self {
            Cache::Node(n,next) => format!("{}{}",n,next.println()),
            Cache::Nil => format!("nil")
        }
    }
    fn last(self) -> Cache<T>{
        let mut cur = self;
        loop {
            match cur {
                Cache::Node(_, next) => cur = *next,
                Cache::Nil => break
            }
        };
        cur
    }
    fn prepend(self,node:T) -> Self {
        Cache::Node(node, Box::new(self))
    }
    // fn append(&self,node:T) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::Cache;

    #[test]
    fn test_cache(){
        let mut cache = Cache::new(0);
        assert_eq!(cache.println(),"0nil");
        cache = cache.prepend(1);
        assert_eq!(cache.println(),"10nil");
        println!("{:?}",cache.last());
    }
}