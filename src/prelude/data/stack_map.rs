use std::{default::Default, fmt::Debug, marker::PhantomData, iter::Zip, slice::Iter, ops::Index};

#[derive(Debug)]
pub struct StackMap<K, V, const N: usize>
where
    K: Default + Copy,
    V: Default + Copy,
    usize: From<K>
{
    data: [V; N],
    keys: [K; N],
    phantom: PhantomData<K>
}

impl<K, V, const N: usize> StackMap<K, V, N>
where
    V: Default + Copy,
    K: Default + Copy,
    usize: From<K>
{
    pub fn new() -> Self {
        StackMap {
            data: [V::default(); N],
            keys: [K::default(); N],
            phantom: PhantomData
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = usize::from(key);
        self.keys[index] = key;
        self.data[index] = value;
    }

    pub fn get(&self, key: K) -> & V {
        let index = usize::from(key);
        debug_assert!(index < N);
        unsafe {
            self.data.get_unchecked(index)
        }
    }

    pub fn iter(&self) -> Zip<Iter<'_, K>, Iter<'_, V>> {
        self.keys.iter().zip(self.data.iter())
    }
}

impl<K, V, const N: usize> Index<K> for StackMap<K, V, N>
where
    V: Default + Copy,
    K: Default + Copy,
    usize: From<K>
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get(index)
    }
}

impl<K, V, const N: usize> Default for StackMap<K, V, N>
where
    V: Default + Copy,
    K: Default + Copy,
    usize: From<K>
{
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + stack_map::count!($($xs)*));
}
pub(crate) use count;

macro_rules! stack_map {
    ($($key:expr => $val:expr,)*) => {
        {
            const SIZE: usize = stack_map::count!($($key)*);
            let mut map = StackMap::<_, _, SIZE>::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
    ($($key:expr => $val:expr),*) => {
        stack_map!($($key => $val,)*)
    };
}
pub(crate) use stack_map;