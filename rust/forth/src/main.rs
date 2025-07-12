use std::vec;

fn main() {
    println!("Hello, world!");
    let mut f = Forth::new();
    assert!(f.eval("1 2 3 +").is_ok());
    assert_eq!(f.stack(), [1, 5]);
}

use std::collections::HashMap;
pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack:Vec<Value>,
    custom:HashMap<String,String>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack:vec![],
            custom:HashMap::new()
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut ret = vec![];
        for x in input.to_lowercase().split_whitespace() {
            match x {
                "+" => {
                    let mut sum = pop(&mut ret)?;
                    sum += pop(&mut ret)?;
                    ret.push(sum);
                },
                "-" => {
                    let mut sum = pop(&mut ret)?;
                    sum -= pop(&mut ret)?;
                    ret.push(sum);
                },
                "*" => {
                    let mut sum = pop(&mut ret)?;
                    sum *= pop(&mut ret)?;
                    ret.push(sum);
                },
                "/" => {
                    let mut sum = pop(&mut ret)?;
                    let n = pop(&mut ret)?;
                    if n == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    sum /= n;
                    ret.push(sum);
                },
                "dup" => {
                    if let Some(n) = ret.last() {
                        ret.push(*n);
                    }
                },
                "swap" => {
                    let last = pop(&mut ret)?;
                    let n = pop(&mut ret)?;
                    ret.push(last);
                    ret.push(n);
                },
                "over" => {
                    if let Some(n) = ret.get(ret.len()-2) {
                        ret.push(*n);
                    }
                },
                ":" =>{
                    let input_vec = input.to_lowercase().split_whitespace().map(|x| x.to_owned()).collect::<Vec<_>>();
                    // let k = input_vec[0];
                    let k = input_vec.get(1).unwrap();
                    let v = input_vec.get(2..input_vec.len()-1).unwrap().join(" ");
                    let kv = self.custom.entry(k.to_owned()).or_insert("".to_string());
                    *kv = v;
                    return Ok(());
                },
                _ => {
                    if let Ok(n) = x.parse::<Value>() {
                        ret.push(n);
                    }else{
                        return Err(Error::InvalidWord);
                    }
                }
            };
        }
        self.stack = ret;
        Ok(())
    }
}

fn pop(v:&mut Vec<Value>) -> std::result::Result<Value, Error> {
    if let Some(n) = v.pop() {
        Ok(n)
    }else{
        return Err(Error::StackUnderflow);
    }
}