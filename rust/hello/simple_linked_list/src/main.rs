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

impl<T> SimpleLinkedList<T> {
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
        self.head = Some(Box::new(
            ListNode {
                value:element,
                next:self.head.take()
            }
        ));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        }else {
            self.size -= 1;
            let head = self.head.take().unwrap();
            self.head = head.next;
            Some(head.value)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref head) => {
                let node = head.as_ref();
                Some(&node.value)
            }
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut link = SimpleLinkedList::new();
        let mut curr = self.head;
        while let Some(n) = curr {
            link.push(n.value);
            curr = n.next;
        }
        link
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter
            .into_iter()
            .fold(SimpleLinkedList::new(), |mut acc, x| {
                acc.push(x);
                acc
            })
    }
}
impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut ret = vec![];
        let mut curr = linked_list.head;
        while let Some(n) = curr {
            ret.push(n.value);
            curr = n.next;
        }
        ret
    }
}

fn main() {
    let mut nodes = SimpleLinkedList::new();
    nodes.push(0);
    nodes.push(1);
    nodes.push(2);
    println!("{:?}",nodes);
    let rev = nodes.rev();
    println!("{:?}",rev);
}
