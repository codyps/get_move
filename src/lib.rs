use std::usize;

pub trait Get {
    type Output;

    fn get_move(&self, index: usize) -> Option<Self::Output>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index_move(&self, index: usize) -> Self::Output {
        self.get_move(index).unwrap()
    }

    fn iter<'a>(&'a self) -> Iter<'a, Self>
    where
        Self: Sized,
    {
        Iter {
            inner: self,
            start: 0,
            // NOTE: we only need this field because we want to make this double-ended. Without the
            // double-ended constraint, we could shrink the iterator.
            end: self.len(),
        }
    }

    fn chain<B>(self, other: B) -> Chain<Self, B>
    where
        Self: Sized,
        B: Get<Output = Self::Output>,
    {
        Chain::new(self, other)
    }
}

/// Combine 2 `Get`s in sequence into a Chain that also impliments `Get`
#[cfg_attr(features = "defmt", derive(defmt::Debug))]
#[derive(Debug)]
pub struct Chain<A, B> {
    a: A,
    b: B,
}

impl<A, B> Chain<A, B> {
    fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<O, A: Get<Output = O>, B: Get<Output = O>> Get for Chain<A, B> {
    type Output = O;

    fn get_move(&self, index: usize) -> Option<Self::Output> {
        let al = self.a.len();
        if index >= al {
            self.b.get_move(index - al)
        } else {
            self.a.get_move(index)
        }
    }

    fn len(&self) -> usize {
        self.a.len() + self.b.len()
    }
}

#[cfg_attr(features = "defmt", derive(defmt::Debug))]
#[derive(Debug, Clone, Copy)]
pub struct Iter<'a, G> {
    inner: &'a G,
    start: usize,
    end: usize,
}

impl<'a, G> Iter<'a, G> {
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl<'a, O, G: Get<Output = O>> Iterator for Iter<'a, G> {
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        if Self::is_empty(self) {
            return None;
        }

        let v = self.inner.get_move(self.start);
        self.start += 1;

        v
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = ExactSizeIterator::len(self);
        (l, Some(l))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.len() < n {
            None
        } else {
            self.start += n - 1;
            self.next()
        }
    }
}

impl<'a, O, G: Get<Output = O>> ExactSizeIterator for Iter<'a, G> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'a, O, G: Get<Output = O>> DoubleEndedIterator for Iter<'a, G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == self.start {
            None
        } else {
            self.end -= 1;
            self.inner.get_move(self.end)
        }
    }
}

impl<'a, T: Copy> Get for &'a [T] {
    type Output = T;

    fn len(&self) -> usize {
        (*self).len()
    }

    fn get_move(&self, i: usize) -> Option<Self::Output> {
        (*self).get(i).copied()
    }
}

impl<O: Copy, const N: usize> Get for [O; N] {
    type Output = O;

    fn get_move(&self, index: usize) -> Option<Self::Output> {
        self.get(index).copied()
    }

    fn len(&self) -> usize {
        N
    }
}
