use std::iter::{Chain, Rev};
use std::slice::{Iter as SliceIter, IterMut as SliceIterMut};

#[derive(Debug)]
struct Ring<T>
where
    T: std::fmt::Debug,
{
    capacity: usize,
    index: usize,
    contents: Vec<T>,
    _iter_start: usize,
}

/// An iterator over `Ring<T>`.
pub type Iter<'a, T> = Chain<Rev<SliceIter<'a, T>>, Rev<SliceIter<'a, T>>>;

/// A mutable iterator over `CircularQueue<T>`.
pub type IterMut<'a, T> = Chain<Rev<SliceIterMut<'a, T>>, Rev<SliceIterMut<'a, T>>>;

impl<T> Ring<T>
where
    T: std::fmt::Debug,
{
    fn new(capacity: usize) -> Ring<T> {
        Ring {
            capacity,
            index: 0,
            contents: Vec::with_capacity(capacity),
            _iter_start: 0,
        }
    }

    fn push(&mut self, value: T) {
        println!("{:?}", self);
        if self.contents.len() <= self.index {
            // Size only changes before we get up to capacity
            self.contents.push(value);
        } else {
            self.contents[self.index] = value;
        }
        self.index += 1;
        self.index %= self.capacity;
        self._iter_start = self.index;
        println!("{:?}", self);
    }

    fn pop(&mut self) -> Option<T> {
        println!("{:?}", self);
        if self.contents.len() <= 0 {
            None
        } else {
            if self.index == 0 {
                self.index = self.capacity - 1;
            } else {
                self.index -= 1;
            }
            let popped = self.contents.remove(self.index);
            Some(popped)
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        let (a, b) = self.contents.split_at(self.index);
        a.iter().rev().chain(b.iter().rev())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (a, b) = self.contents.split_at_mut(self.index);
        a.iter_mut().rev().chain(b.iter_mut().rev())
    }
}

#[test]
fn ring_test_push_pop() {
    let mut ring = Ring::new(3);
    ring.push("a");
    ring.push("b");
    assert_eq!(ring.contents.len(), 2);
    let popped = ring.pop();
    assert_eq!(popped, Some("b"));
    assert_eq!(ring.contents.len(), 1);
    let popped = ring.pop();
    assert_eq!(popped, Some("a"));
    assert_eq!(ring.contents.len(), 0);
    let popped = ring.pop();
    assert_eq!(popped, None);
    assert_eq!(ring.contents.len(), 0);
}

#[test]
fn ring_test_overwrite() {
    let mut ring = Ring::new(3);
    ring.push("a");
    ring.push("b");
    ring.push("c");
    ring.push("d");
    ring.push("e");
    ring.push("f");
    assert_eq!(ring.contents.len(), 3);
    assert_eq!(ring.pop(), Some("f"));
    assert_eq!(ring.pop(), Some("e"));
    assert_eq!(ring.pop(), Some("d"));
    assert_eq!(ring.pop(), None);
}

#[test]
fn empty_queue() {
    let q = Ring::<i32>::new(5);

    assert_eq!(q.iter().next(), None);
}

#[test]
fn partially_full_queue() {
    let mut q = Ring::new(5);
    q.push(1);
    q.push(2);
    q.push(3);

    assert_eq!(q.contents.len(), 3);

    let res: Vec<_> = q.iter().map(|&x| x).collect();
    assert_eq!(res, [3, 2, 1]);
}
