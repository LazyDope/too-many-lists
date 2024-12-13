use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail: *mut _ = Box::into_raw(Box::new(Node {
            elem,
            next: ptr::null_mut(),
        }));

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = new_tail;
            }
        } else {
            self.head = new_tail;
        }

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let head = unsafe { Box::from_raw(self.head) };
            self.head = head.next;

            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }

            Some(head.elem)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref() }.map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut() }.map(|node| &mut node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: unsafe { self.head.as_ref() },
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: unsafe { self.head.as_mut() },
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_ref() };
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_mut() };
            &mut node.elem
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

    #[test]
    fn into_iter() {
        let mut ll = List::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        let mut iter = ll.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut ll = List::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut ll = List::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        let mut iter = ll.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        if let Some(v) = iter.next() {
            *v += 4
        }
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);

        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn peek() {
        let mut ll = List::new();

        assert_eq!(ll.peek(), None);
        assert_eq!(ll.peek_mut(), None);
        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.peek(), Some(&1));
        assert_eq!(ll.peek_mut(), Some(&mut 1));

        if let Some(v) = ll.peek_mut() {
            *v += 3
        }

        assert_eq!(ll.peek(), Some(&4));
        assert_eq!(ll.pop(), Some(4));
    }
}
