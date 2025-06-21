fn main() {

}

/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    let mut a_vec = a.collect::<Vec<_>>();
    for i in b {
        a_vec.push(i);
    }
    a_vec.into_iter()
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut v = vec![];
    for i in nested_iter {
        for j in i {
            v.push(j);
        }
    }
    v.into_iter()
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    let mut v = vec![];
    for i in iter {
        if predicate(&i) {
            v.push(i);
        }
    }
    v.into_iter()
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
}

pub fn length<I: Iterator>(iter: I) -> usize {
    // todo!("return the total number of items within iter")
    let mut n = 0;
    for _i in iter {
        n += 1;
    }
    n
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    let mut v = vec![];
    for i in iter {
        v.push(function(i));
    }
    v.into_iter()
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
}

pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut ret = initial;
    while let Some(v) = iter.next() {
        ret = function(ret,v);
    }
    ret
    // todo!("starting with initial, fold (reduce) each iter item into the accumulator from the left")
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut ret = initial;
    while let Some(v) = iter.next_back() {
        ret = function(ret,v);
    }
    ret
    // todo!("starting with initial, fold (reduce) each iter item into the accumulator from the right")
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    let mut v = vec![];
    for i in iter {
        v.insert(0, i);
    }
    v.into_iter()
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
}