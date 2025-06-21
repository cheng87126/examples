fn main() {
    println!("Hello, world!");
}

#[macro_export]
macro_rules! hashmap {
    (,) => {
        use ::std::collections::HashMap;
        let ret:HashMap::<_,_> = HashMap::new();
        ret
    };
    ($($k:expr => $v:expr),*$(,)?) => {
        {
            use ::std::collections::HashMap;
            let mut ret = HashMap::new();
            $(
                ret.insert($k, $v);
            )*
            ret
        }
    };
}
