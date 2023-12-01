use std::{mem, ptr};

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let raw_tail: *mut _ = Box::into_raw(Box::new(Node {
            elem,
            next: ptr::null_mut(),
        }));

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = raw_tail;
            }
        } else {
            self.head = raw_tail;
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.head.is_null() {
            let head = mem::replace(&mut self.head, ptr::null_mut());
            let head = unsafe { *head };
            self.head = head.next;

            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }

            Some(head.elem)
        } else {
            None
        }
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
        ll.push(3);

        assert_eq!(ll.pop(), Some(1));

        ll.push(4);

        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(4));
        assert_eq!(ll.pop(), None);

        ll.push(5);
        ll.push(6);

        assert_eq!(ll.pop(), Some(5));
        assert_eq!(ll.pop(), Some(6));
        assert_eq!(ll.pop(), None);
    }
}
