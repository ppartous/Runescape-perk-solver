use std::{default::Default, fmt::Debug, ops::Index, mem::MaybeUninit};
use uninit::extension_traits::MaybeUninitExt;

#[derive(Clone, Copy)]
pub struct StackVec<T: Copy, const N: usize> {
    ranks: MaybeUninit<[T; N]>,
    len: usize
}

impl<T: Copy, const N: usize> StackVec<T, N> {
    pub fn new(slice: &[T]) -> StackVec<T, N> {
        assert!(slice.len() <= N);
        let mut x = StackVec::<T, N> {
            len: slice.len(),
            ranks: MaybeUninit::uninit()
        };
        for (i, val) in slice.into_iter().enumerate() {
            unsafe {
                (x.ranks.as_mut_ptr() as *mut T).add(i).write(*val);
            }
        }
        x
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        unsafe {
            self.ranks.assume_init_by_ref()[0..self.len].iter()
        }
    }

    pub fn push(&mut self, val: T) {
        assert!(self.len < N);
        unsafe {
            (self.ranks.as_mut_ptr() as *mut T).add(self.len).write(val);
        }
        self.len += 1;
    }
}

impl<T: Copy, const N: usize> Index<usize> for StackVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.len);
        unsafe {
            self.ranks.assume_init_by_ref().get_unchecked(index)
        }
    }
}

impl<'a, T: Copy, const N: usize> IntoIterator for &'a StackVec<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Copy, const N: usize> std::ops::Deref for StackVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            &self.ranks.assume_init_by_ref()[0..self.len]
        }
    }
}

impl<T: Copy, const N: usize> std::ops::DerefMut for StackVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut self.ranks.assume_init_by_mut()[0..self.len]
        }
    }
}

impl<T: Copy + Debug, const N: usize> Debug for StackVec<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            Debug::fmt(&self.ranks.assume_init_by_ref()[0..self.len], f)
        }
    }
}

impl<T: Copy + PartialEq, const N: usize> PartialEq for StackVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        for (x, y) in self.iter().zip(other.iter()) {
            if x != y {
                return false;
            }
        }
        true
    }
}

impl<T: Copy + PartialEq, const N: usize> PartialEq<Vec<T>> for StackVec<T, N> {
    fn eq(&self, other: &Vec<T>) -> bool {
        if self.len != other.len() {
            return false;
        }
        for (x, y) in self.iter().zip(other.iter()) {
            if x != y {
                return false;
            }
        }
        true
    }
}

impl<T: Copy + PartialEq, const N: usize> PartialEq<StackVec<T, N>> for Vec<T> {
    fn eq(&self, other: &StackVec<T, N>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (x, y) in self.iter().zip(other.iter()) {
            if x != y {
                return false;
            }
        }
        true
    }
}

impl<T: Copy, const N: usize> Default for StackVec<T, N> {
    fn default() -> Self {
        StackVec::<T, N>::new(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_test1() {
        let expected: Vec<i64> = vec![];
        let actual: StackVec<i64, 4> = StackVec::new(&[]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn vec_test2() {
        let expected: Vec<i64> = vec![1, 2];
        let actual: StackVec<i64, 4> = StackVec::new(&[1, 2]);
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn vec_test3() {
        let _: StackVec<i64, 4> = StackVec::new(&[1, 2, 3, 4, 5]);
    }
}