use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::new(Node { elem, next: None });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
    }

    pub fn pop(&'a mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = None;
            }

            head.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn push_pop() {
        let mut ll = List::new();
        assert_eq!(ll.pop(), None);

        ll.push(1);
        ll.push(2);
        ll.push(4);

        assert_eq!(ll.pop(), Some(4));

        ll.push(3);

        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), None);
    }
}
