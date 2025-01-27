use std::mem;
#[derive(Debug)]
pub struct ListNode<T>{
    value: T,
    next: Option<Box<ListNode<T>>>
}
#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    size: usize,
    head: Option<Box<ListNode<T>>>
}

impl<T:std::fmt::Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            size:0,
            head:None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        self.size += 1;
        match self.head {
            Some(ref mut head) => {
                let mut curr = &mut head.next;
                while let Some(n) = curr {
                    curr = &mut n.next;
                }
                *curr = Some(Box::new(
                    ListNode {
                        value:element,
                        next:None
                    }
                ));
            },
            None => {
                self.head = Some(Box::new(ListNode {
                    value:element,
                    next:None
                }));
            }
        }
        /*
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
        */
        // println!("{:?}",self)
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            Some(ref mut head) => {
                self.size -= 1;
                let head_ptr: *mut Box<ListNode<T>> = &mut *head;
                unsafe {
                    let h = **head_ptr;
                }
                /*
                let mut prev = &mut *head;
                let mut curr = &mut head.next;
                while curr.is_some() && curr.unwrap().next.is_some() {
                    let prev = *curr;
                    curr = &mut curr.unwrap().next;
                }
                */
                None
            },
            None => None
        }   
        /*
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
        */
        /*
        if self.value.is_none() {
            None
        }else {
            let prev = &mut *self;
            let mut next = &self.next;
            while let Some(n) = next {
                println!("{:?}---n---{:?}",prev,n);
                *prev = **n;
                next = &mut n.next;
            }
            *next = None;
            None
        }
        */
    }

    pub fn peek(&self) -> Option<&T> {
        None
        /*
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
        */
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
impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        todo!()
    }
}


fn main() {
    let mut nodes = SimpleLinkedList::new();
    nodes.push(0);
    nodes.push(1);
    nodes.push(2);
    println!("{:?}",nodes);
    
}
