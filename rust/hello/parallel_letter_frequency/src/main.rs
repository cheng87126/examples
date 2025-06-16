use std::{thread, sync::{Arc, Mutex}, collections::HashMap};

fn main() {
    frequency(&["aA"],4);
}

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let len = input.len();
    let arr = Arc::new(input.to_owned());
    let chunk_size = len.div_ceil(worker_count);
    let ret = Arc::new(Mutex::new(HashMap::new()));
    (0..worker_count)
        .map(|i| {
            let arr = Arc::clone(&arr);
            let ret = Arc::clone(&ret);
            let handle = thread::spawn(move || {
                let start = i * chunk_size;
                if start>=len  {
                    return;
                }
                let end = (start + chunk_size).min(len);
                let mut tmp = HashMap::new();
                arr[start..end].iter().for_each(|x| {
                    x.to_lowercase().chars().filter(|ch| ch.is_alphabetic()).for_each(|ch| {
                        *tmp.entry(ch).or_insert(0) += 1;
                    });
                });
                let mut hashmap = ret.lock().unwrap();
                for (k,v) in tmp {
                    *hashmap.entry(k).or_insert(0) += v;
                }
            });
            handle
        })
        .for_each(|h| h.join().unwrap());

    Arc::try_unwrap(ret).unwrap().into_inner().unwrap()
}
