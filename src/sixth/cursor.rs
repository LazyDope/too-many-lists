use std::mem;

use super::{Link, LinkedList};

pub struct CursorMut<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
    index: usize,
}

impl<'a, T> CursorMut<'a, T> {
    pub fn on_list(list: &'a mut LinkedList<T>) -> Self {
        CursorMut {
            list,
            cur: None,
            index: 0,
        }
    }

    pub fn index(&self) -> Option<usize> {
        self.cur?;
        Some(self.index)
    }

    pub fn move_next(&mut self) {
        if let Some(cur) = self.cur.take() {
            self.cur = unsafe { (*cur.as_ptr()).back };
            // if self.cur is None then index doesn't matter
            self.index += 1;
        } else {
            // will just be None if an empty list
            self.cur = self.list.front;
            self.index = 0;
        }
    }

    pub fn move_prev(&mut self) {
        if let Some(cur) = self.cur.take() {
            self.cur = unsafe { (*cur.as_ptr()).front };
            self.index = self.index.wrapping_sub(1);
        } else {
            self.cur = self.list.back;
            self.index = self.list.len.wrapping_sub(1);
        }
    }

    pub fn current(&mut self) -> Option<&mut T> {
        unsafe { self.cur.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            match self.cur {
                Some(cur) => (*cur.as_ptr()).back,
                None => self.list.front,
            }
            .map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            match self.cur {
                Some(cur) => (*cur.as_ptr()).front,
                None => self.list.back,
            }
            .map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn split_before(&mut self) -> LinkedList<T> {
        if let Some(cur) = self.cur {
            let new_list = LinkedList {
                front: self.list.front.replace(cur),
                back: unsafe { (*cur.as_ptr()).front.take() },
                len: self.index,
            };
            self.list.len -= self.index;
            self.index = 0;
            new_list
        } else {
            mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn split_after(&mut self) -> LinkedList<T> {
        if let Some(cur) = self.cur {
            let new_list = LinkedList {
                front: unsafe { (*cur.as_ptr()).back.take() },
                back: self.list.back.replace(cur),
                len: self.list.len - self.index,
            };
            self.list.len = self.index + 1;
            new_list
        } else {
            mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn splice_before(&mut self, mut other: LinkedList<T>) {
        if other.is_empty() {
            return;
        }

        if self.list.is_empty() {
            *(self.list) = other;
            return;
        }

        let (other_front, other_back) = unsafe {
            (
                other.front.take().unwrap_unchecked(),
                other.back.take().unwrap_unchecked(),
            )
        };
        match self.cur {
            Some(cur) => {
                if self.index == 0 {
                    self.list.front = Some(other_front);
                } else {
                    unsafe {
                        // SAFETY: both self.list and other are known to not be empty
                        let link_before = (*cur.as_ptr()).front.unwrap_unchecked();
                        (*link_before.as_ptr()).back = Some(other_front);
                        (*other_front.as_ptr()).front = Some(link_before);
                    }
                };

                unsafe {
                    (*cur.as_ptr()).front = Some(other_back);
                    (*other_back.as_ptr()).back = Some(cur);
                }
                self.index += other.len;
            }
            None => {
                unsafe {
                    // SAFETY: both self.list and other are known to not be empty
                    let list_back = self.list.back.unwrap_unchecked();
                    (*list_back.as_ptr()).back = Some(other_front);
                    (*other_front.as_ptr()).front = Some(list_back);
                }
                self.list.back = Some(other_back);
            }
        }
        self.list.len += other.len;
        other.len = 0;
    }

    pub fn splice_after(&mut self, mut other: LinkedList<T>) {
        if other.is_empty() {
            return;
        }

        if self.list.is_empty() {
            *(self.list) = other;
            return;
        }

        let (other_front, other_back) = unsafe {
            (
                other.front.take().unwrap_unchecked(),
                other.back.take().unwrap_unchecked(),
            )
        };
        match self.cur {
            Some(cur) => unsafe {
                // SAFETY: both self.list and other are known to not be empty
                if self.index == self.list.len - 1 {
                    self.list.back = Some(other_back);
                } else {
                    let link_after = (*cur.as_ptr()).back.unwrap_unchecked();
                    (*link_after.as_ptr()).front = Some(other_back);
                    (*other_back.as_ptr()).back = Some(link_after);
                };

                (*cur.as_ptr()).back = Some(other_front);
                (*other_front.as_ptr()).front = Some(cur);
            },
            None => {
                unsafe {
                    // SAFETY: both self.list and other are known to not be empty
                    let list_front = self.list.front.unwrap_unchecked();
                    (*list_front.as_ptr()).front = Some(other_back);
                    (*other_back.as_ptr()).back = Some(list_front);
                }
                self.list.front = Some(other_front);
            }
        }
        self.list.len += other.len;
        other.len = 0;
    }

    pub fn insert_before(&mut self, elem: T) {
        let mut other = LinkedList::new();
        other.push_front(elem);
        self.splice_before(other);
    }

    pub fn insert_after(&mut self, elem: T) {
        let mut other = LinkedList::new();
        other.push_front(elem);
        self.splice_after(other);
    }

    pub fn remove_current(&mut self) -> Option<T> {
        self.cur.map(|cur| unsafe {
            let node = Box::from_raw(cur.as_ptr());
            match node.front {
                Some(front) => (*front.as_ptr()).back = node.back,
                None => self.list.front = node.back,
            }
            match node.back {
                Some(back) => (*back.as_ptr()).front = node.front,
                None => self.list.back = node.front,
            }
            self.cur = node.back;
            self.list.len -= 1;
            node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::LinkedList;
    #[test]
    fn test_cursor_move_peek() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.peek_next(), Some(&mut 2));
        assert_eq!(cursor.peek_prev(), None);
        assert_eq!(cursor.index(), Some(0));
        cursor.move_prev();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.peek_next(), Some(&mut 3));
        assert_eq!(cursor.peek_prev(), Some(&mut 1));
        assert_eq!(cursor.index(), Some(1));

        let mut cursor = m.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 6));
        assert_eq!(cursor.peek_next(), None);
        assert_eq!(cursor.peek_prev(), Some(&mut 5));
        assert_eq!(cursor.index(), Some(5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 5));
        assert_eq!(cursor.peek_next(), Some(&mut 6));
        assert_eq!(cursor.peek_prev(), Some(&mut 4));
        assert_eq!(cursor.index(), Some(4));
    }

    #[test]
    fn test_cursor_mut_insert() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.index, 0);
        cursor.splice_before(Some(7).into_iter().collect());
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.index, 1);
        cursor.splice_after(Some(8).into_iter().collect());
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[7, 1, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        cursor.splice_before(Some(9).into_iter().collect());
        cursor.splice_after(Some(10).into_iter().collect());
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[10, 7, 1, 8, 2, 3, 4, 5, 6, 9]
        );

        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        assert_eq!(cursor.remove_current(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.remove_current(), Some(7));
        cursor.move_prev();
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.remove_current(), Some(9));
        cursor.move_next();
        assert_eq!(cursor.remove_current(), Some(10));
        assert_eq!(cursor.list.len, 7);
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[1, 8, 2, 3, 4, 5, 6]
        );

        assert_eq!(m.len, 7);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        let mut p: LinkedList<u32> = LinkedList::new();
        p.extend([100, 101, 102, 103]);
        let mut q: LinkedList<u32> = LinkedList::new();
        q.extend([200, 201, 202, 203]);
        cursor.splice_after(p);
        cursor.splice_before(q);
        assert_eq!(cursor.list.len, 15);
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101, 102, 103, 8, 2, 3, 4, 5, 6]
        );
        assert_eq!(m.len, 15);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        let tmp = cursor.split_before();
        assert_eq!(cursor.list.len, 0);
        assert_eq!(m.into_iter().collect::<Vec<_>>(), &[]);
        m = tmp;
        assert_eq!(m.len, 15);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.index, 6);
        let tmp = cursor.split_after();
        assert_eq!(cursor.list.len, 7);
        assert_eq!(tmp.len, 9);
        assert_eq!(
            tmp.into_iter().collect::<Vec<_>>(),
            &[102, 103, 8, 2, 3, 4, 5, 6]
        );
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101]
        );
    }

    fn check_links<T: Eq + std::fmt::Debug>(list: &LinkedList<T>) {
        let mut last = None;
        let mut maybe_node = list.front;
        while let Some(node) = maybe_node {
            unsafe {
                assert_eq!(last, (*node.as_ptr()).front);
                maybe_node = (*node.as_ptr()).back;
                last = Some(node);
            }
        }

        let from_front: Vec<_> = list.iter().collect();
        let from_back: Vec<_> = list.iter().rev().collect();
        let re_reved: Vec<_> = from_back.into_iter().rev().collect();

        assert_eq!(from_front, re_reved);
    }
}
