use std::{default::Default, fmt::Debug, ops::Index};

#[derive(Debug, Clone, Copy)]
pub struct StackVec<T, const N: usize>
where
    T: Copy + Default + Debug
{
    ranks: [T; N],
    len: usize
}

impl<T, const N: usize> StackVec<T, N>
where
    T: Copy + Default + Debug
{
    pub fn new(slice: &[T]) -> StackVec<T, N> {
        assert!(slice.len() <= N);
        let mut x = StackVec::<T, N> {
            len: slice.len(),
            ranks: [T::default(); N]
        };
        x.ranks[0..slice.len()].clone_from_slice(&slice);
        x
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.ranks[0..self.len].iter()
    }
}

impl<T, const N: usize> Index<usize> for StackVec<T, N>
where
    T: Copy + Default + Debug
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
    T: Copy + Default + Debug
{
    type Item = T;
    type IntoIter = std::iter::Take<std::array::IntoIter<T, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranks.into_iter().take(self.len)
    }
}

impl<T, const N: usize> IntoIterator for &StackVec<T, N>
where
    T: Copy + Default + Debug
{
    type Item = T;
    type IntoIter = std::iter::Take<std::array::IntoIter<T, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranks.into_iter().take(self.len)
    }
}