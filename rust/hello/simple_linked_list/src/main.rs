#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    // dummy: ::std::marker::PhantomData<T>,
    value: Option<T>,
    next: Option<Box<SimpleLinkedList<T>>>
}

impl<T:std::fmt::Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            value:None,
            next:None
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {        
        if self.value.is_none() {
            return 0;
        }else {
            let mut len = 0;
            let mut next = &self.next;
            while let Some(n) = next {
                len += 1;
                next = &n.next;
            }
            return len;
        }
    }

    pub fn push(&mut self, element: T) {
        if self.value.is_none() {
            self.value = Some(element);
        }else {
            let mut next = &mut self.next;
            while let Some(n) = next {
                next = &mut n.next;
            }
            let node = SimpleLinkedList {
                value:Some(element),
                next:None
            };
            *next = Some(Box::new(node));
        }
        // println!("{:?}",self)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.value.is_none() {
            None
        }else {
            let prev = self;
            let mut next = &mut self.next;
            while let Some(n) = next {
                println!("{:?}---n---{:?}",prev,n);
                *prev = **n;
                next = &mut n.next;
            }
            *next = None;
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.value.is_none() {
            None
        }else {
            let mut next = &self.next;
            while let Some(n) = next {
                next = &n.next;
                // if next.is_none() {
                //     return Some(&n.value);
                // }
            }
            return None;
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        todo!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        todo!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        todo!()
    }
}
