use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head)
                .ok()
                .expect("All other references should've been dropped at this point.")
                .into_inner()
                .elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail)
                .ok()
                .expect("All other references should've been dropped at this point.")
                .into_inner()
                .elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
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
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn push_pop() {
        let mut ll = List::new();
        assert_eq!(ll.pop_front(), None);

        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(4);

        assert_eq!(ll.pop_front(), Some(4));

        ll.push_front(3);

        assert_eq!(ll.pop_front(), Some(3));
        assert_eq!(ll.pop_front(), Some(2));
        assert_eq!(ll.pop_front(), Some(1));
        assert_eq!(ll.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut ll = List::new();

        assert!(ll.peek_front().is_none());
        assert!(ll.peek_front_mut().is_none());
        assert!(ll.peek_back().is_none());
        assert!(ll.peek_back_mut().is_none());
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);

        assert_eq!(&*ll.peek_front().unwrap(), &3);
        assert_eq!(&mut *ll.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*ll.peek_back().unwrap(), &1);
        assert_eq!(&mut *ll.peek_back_mut().unwrap(), &mut 1);

        if let Some(mut v) = ll.peek_front_mut() {
            *v += 2
        }
        if let Some(mut v) = ll.peek_back_mut() {
            *v += 1
        }

        assert_eq!(&*ll.peek_front().unwrap(), &5);
        assert_eq!(&*ll.peek_back().unwrap(), &2);
        assert_eq!(ll.pop_front(), Some(5));
        assert_eq!(ll.pop_back(), Some(2));
    }

    #[test]
    fn into_iter() {
        let mut ll = List::new();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);

        let mut iter = ll.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
