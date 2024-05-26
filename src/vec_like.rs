use std::ops::{Deref, DerefMut, Index, IndexMut, Range, RangeFull};

use tinyvec::{Array, ArrayVec};
pub trait VecLike:
    Deref<Target = [Self::TI]>
    + DerefMut
    + Index<usize,Output=Self::TI>
    + Index<RangeFull,Output=[Self::TI]>
    + Index<Range<usize>,Output=[Self::TI]>
    + IndexMut<usize>
    + IndexMut<RangeFull,Output=[Self::TI]>
    + IndexMut<Range<usize>,Output=[Self::TI]>
    + IntoIterator<Item = Self::TI>
    + AsRef<[Self::TI]>
    + AsMut<[Self::TI]>
    + Default
    + FromIterator<Self::TI>
    + Extend<Self::TI>
{
    type TI;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn reserve(&mut self, additional: usize);
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn remove(&mut self, index: usize) -> Self::Item;
    fn insert(&mut self, index: usize, item: Self::Item);
    fn clear(&mut self);
    fn truncate(&mut self, len: usize);
}

impl<T> VecLike for Vec<T>
{
    type TI = T;
    #[inline]
    fn len(&self) -> usize {
        Vec::len(self)
    }
    #[inline]
    fn push(&mut self, item: T) {
        Vec::push(self, item)
    }
    #[inline]
    fn pop(&mut self) -> Option<T> {
        Vec::pop(self)
    }
    #[inline]
    fn remove(&mut self, index: usize) -> T {
        Vec::remove(self, index)
    }
    #[inline]
    fn insert(&mut self, index: usize, item: T) {
        Vec::insert(self, index, item)
    }
    #[inline]
    fn clear(&mut self) {
        Vec::clear(self)
    }
    #[inline]
    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional)
    }
    #[inline]
    fn truncate(&mut self, len: usize) {
        Vec::truncate(self, len)
    }
}

impl<A: Array+Clone> VecLike for ArrayVec<A>
{
    type TI = A::Item;
    #[inline]
    fn len(&self) -> usize {
        ArrayVec::len(self)
    }
    #[inline]
    fn push(&mut self, item: A::Item) {
        ArrayVec::push(self, item)
    }
    #[inline]
    fn pop(&mut self) -> Option<A::Item> {
        ArrayVec::pop(self)
    }
    #[inline]
    fn remove(&mut self, index: usize) -> A::Item {
        ArrayVec::remove(self, index)
    }
    #[inline]
    fn insert(&mut self, index: usize, item: A::Item) {
        ArrayVec::insert(self, index, item)
    }
    #[inline]
    fn clear(&mut self) {
        ArrayVec::clear(self)
    }
    #[inline]
    fn reserve(&mut self, _: usize) {
        
    }
    #[inline]
    fn truncate(&mut self, len: usize) {
        ArrayVec::truncate(self, len)
    }
}
