use std::{default::Default, fmt::Debug, ops::Index};

#[derive(Clone, Copy)]
pub struct StackVec<T, const N: usize>
where
    T: Copy + Default
{
    ranks: [T; N],
    len: usize
}

impl<T, const N: usize> StackVec<T, N>
where
    T: Copy + Default
{
    pub fn new(slice: &[T]) -> StackVec<T, N> {
        assert!(slice.len() <= N);
        let mut x = StackVec::<T, N> {
            len: slice.len(),
            ranks: [T::default(); N]
        };
        x.ranks[0..slice.len()].clone_from_slice(slice);
        x
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.ranks[0..self.len].iter()
    }

    pub fn push(&mut self, val: T) {
        self.ranks[self.len] = val;
        self.len += 1;
    }
}

impl<T, const N: usize> Index<usize> for StackVec<T, N>
where
    T: Copy + Default
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.len);
        unsafe {
            self.ranks.get_unchecked(index)
        }
    }
}

impl<T, const N: usize> IntoIterator for StackVec<T, N>
where
    T: Copy + Default
{
    type Item = T;
    type IntoIter = std::iter::Take<std::array::IntoIter<T, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranks.into_iter().take(self.len)
    }
}

impl<T, const N: usize> IntoIterator for &StackVec<T, N>
where
    T: Copy + Default
{
    type Item = T;
    type IntoIter = std::iter::Take<std::array::IntoIter<T, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranks.into_iter().take(self.len)
    }
}

impl<T, const N: usize> std::ops::Deref for StackVec<T, N>
where
    T: Copy + Default
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.ranks[0..self.len]
    }
}

impl<T, const N: usize> std::ops::DerefMut for StackVec<T, N>
where
    T: Copy + Default
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ranks[0..self.len]
    }
}

impl<T, const N: usize> Debug for StackVec<T, N>
where
    T: Copy + Default + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.ranks[0..self.len], f)
    }
}

impl<T, const N: usize> PartialEq for StackVec<T, N>
where
    T: Copy + Default + PartialEq
{
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

impl<T, const N: usize> Default for StackVec<T, N>
where
    T: Copy + Default
{
    fn default() -> Self {
        StackVec::<T, N>::new(&[])
    }
}