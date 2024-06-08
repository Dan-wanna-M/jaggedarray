use generic_array::{sequence::GenericSequence, ArrayLength, GenericArray};
use num::traits::AsPrimitive;
use num::traits::ConstOne;
use num::traits::ConstZero;
use num::traits::Num;
use num::traits::NumAssignOps;
use std::{
    iter::zip,
    ops::{Index, IndexMut},
};
use typenum::{Const, IsEqual, NonZero, Sub1, ToUInt, Unsigned, B1, U, U2};

use crate::vec_ext::Ext;
use crate::vec_like::VecLike;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JaggedArray<TVal, TBuffer: VecLike, const N: usize>
where
    <TBuffer as VecLike>::TI: AsPrimitive<usize> + Num,
    U<N>: std::ops::Sub<B1>,
    <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
    Const<N>: ToUInt,
{
    indices: GenericArray<TBuffer, Sub1<U<N>>>,
    buffer: Vec<TVal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JaggedArrayView<'a, TVal, TNum, const N: usize>
where
    TNum: AsPrimitive<usize> + Num,
    U<N>: std::ops::Sub<B1>,
    <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
    Const<N>: ToUInt,
{
    indices: GenericArray<&'a [TNum], Sub1<U<N>>>,
    buffer: &'a [TVal],
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct JaggedArrayMutView<'a, TVal, TNum, const N: usize>
where
    TNum: AsPrimitive<usize> + Num,
    U<N>: std::ops::Sub<B1>,
    <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
    Const<N>: ToUInt,
{
    indices: GenericArray<&'a mut [TNum], Sub1<U<N>>>,
    buffer: &'a mut [TVal],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JaggedArrayOwnedView<TVal, TNum, const N: usize>
where
    TNum: AsPrimitive<usize> + Num,
    U<N>: std::ops::Sub<B1>,
    <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
    Const<N>: ToUInt,
{
    indices: GenericArray<Box<[TNum]>, Sub1<U<N>>>,
    buffer: Box<[TVal]>,
}

impl<TVal, TBuffer: VecLike, const N: usize> Default for JaggedArray<TVal, TBuffer, N>
where
    <TBuffer as VecLike>::TI: AsPrimitive<usize> + Num + ConstOne + ConstZero,
    U<N>: std::ops::Sub<B1>,
    <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
    Const<N>: ToUInt,
{
    #[inline]
    fn default() -> Self {
        Self {
            indices: GenericArray::generate(|_| {
                let mut a = TBuffer::default();
                a.push(TBuffer::TI::ZERO);
                a
            }),
            buffer: Default::default(),
        }
    }
}
// Methods that are unique to JaggedArray
impl<TVal, TBuffer: VecLike, const N: usize> JaggedArray<TVal, TBuffer, N>
where
    <TBuffer as VecLike>::TI:
        AsPrimitive<usize> + Num + NumAssignOps + std::cmp::PartialOrd + ConstOne + ConstZero,
    U<N>: std::ops::Sub<B1>,
    <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
    Const<N>: ToUInt,
    usize: num::traits::AsPrimitive<<TBuffer as VecLike>::TI>,
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn with_capacity(capacity: [usize; N]) -> Self {
        Self {
            indices: GenericArray::generate(|i| {
                let mut temp = TBuffer::default();
                temp.push(TBuffer::TI::ZERO);
                temp.reserve(capacity[i]);
                temp
            }),
            buffer: Vec::with_capacity(*capacity.last().unwrap()),
        }
    }
    #[inline]
    pub fn reserve(&mut self, additional: [usize; N]) {
        for (index, additional) in zip(self.indices.iter_mut(), additional.iter()) {
            index.reserve(*additional);
        }
        self.buffer.reserve(additional[N - 1]);
    }
    #[inline]
    pub fn buffer_reserve(&mut self, additional: usize) {
        self.buffer.reserve(additional);
    }
    #[inline]
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }
    #[inline]
    pub fn buffer_capacity(&self) -> usize {
        self.buffer.capacity()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer.clear();
        for index in self.indices.iter_mut() {
            index.clear();
            index.push(TBuffer::TI::ZERO);
        }
    }
    #[inline]
    pub fn new_row<const DIM: usize>(&mut self)
    where
        U<N>: std::ops::Sub<U<DIM>>,
        Sub1<<U<N> as std::ops::Sub<U<DIM>>>::Output>: Unsigned + NonZero,
        <U<N> as std::ops::Sub<U<DIM>>>::Output: std::ops::Sub<typenum::B1>,
        U<DIM>: ArrayLength,
        Const<N>: ToUInt,
        Const<DIM>: ToUInt,
    {
        let m = DIM;
        let buffer = &mut self.indices[m];
        let new_val = *buffer.last().unwrap();
        buffer.push(new_val);
        if m > 0 {
            *self.indices[m - 1].last_mut().unwrap() += TBuffer::TI::ONE;
        }
    }
    #[inline]
    pub fn push_to_last_row(&mut self, val: TVal) {
        self.buffer.push(val);
        if let Some(value) = self.indices.last_mut() {
            *value.last_mut().unwrap() += TBuffer::TI::ONE;
        }
    }
    #[inline]
    /// # Safety
    ///
    /// The caller must ensure that `self.buffer_len()` < `self.buffer_capacity()`
    pub unsafe fn push_to_last_row_unchecked(&mut self, val: TVal) {
        unsafe { self.buffer.unchecked_push(val) };
        if let Some(value) = self.indices.last_mut() {
            unsafe { *value.last_mut().unwrap_unchecked() += TBuffer::TI::ONE };
        }
    }
    #[inline]
    pub fn pop_from_last_row(&mut self) -> Option<TVal> {
        let mut iter = self.indices.last_mut().unwrap().iter_mut().rev();
        let last = iter.next().unwrap();
        if *last != TBuffer::TI::ZERO && iter.next().unwrap() < last {
            *last -= TBuffer::TI::ONE;
            self.buffer.pop()
        } else {
            None
        }
    }
    #[inline]
    pub fn extend_last_row(&mut self, values: impl Iterator<Item = TVal>) {
        let initial = self.buffer.len();
        self.buffer.extend(values);
        *self.indices.last_mut().unwrap().last_mut().unwrap() +=
            (self.buffer.len() - initial).as_();
    }
    #[inline]
    pub fn extend_last_row_from_slice(&mut self, values: &[TVal])
    where
        TVal: Clone,
    {
        let initial = self.buffer.len();
        self.buffer.extend_from_slice(values);
        *self.indices.last_mut().unwrap().last_mut().unwrap() +=
            (self.buffer.len() - initial).as_();
    }

    pub fn append_from_view<const M: usize>(
        &mut self,
        other: &JaggedArrayView<TVal, TBuffer::TI, M>,
    ) where
        U<N>: std::ops::Sub<U<M>>,
        <U<N> as std::ops::Sub<U<M>>>::Output: Unsigned,
        U<M>: std::ops::Sub<B1>,
        <U<M> as std::ops::Sub<B1>>::Output: ArrayLength,
        U<M>: ArrayLength,
        Const<N>: ToUInt,
        Const<M>: ToUInt,
        TVal: Clone,
    {
        let skipped = N - M;
        for (dst, src) in zip(self.indices.iter_mut().skip(skipped), other.indices.iter()) {
            let last = *dst.last().unwrap();
            dst.extend(src.iter().skip(1).map(|&x| x + last));
        }
        self.buffer.extend_from_slice(other.buffer);
    }

    pub fn append<const M: usize>(&mut self, other: JaggedArray<TVal, TBuffer, M>)
    where
        U<N>: std::ops::Sub<U<M>>,
        <U<N> as std::ops::Sub<U<M>>>::Output: Unsigned,
        U<M>: std::ops::Sub<B1>,
        <U<M> as std::ops::Sub<B1>>::Output: ArrayLength,
        U<M>: ArrayLength,
        Const<N>: ToUInt,
        Const<M>: ToUInt,
    {
        let skipped = N - M;
        for (dst, src) in zip(self.indices.iter_mut().skip(skipped), other.indices.iter()) {
            let last = *dst.last().unwrap();
            dst.extend(src.iter().skip(1).map(|&x| x + last));
        }
        self.buffer.extend(other.buffer);
    }

    // It may be possible to implement a drain-like variant of this method
    pub fn remove_last_row<const DIM: usize>(&mut self) -> bool
    where
        U<N>: std::ops::Sub<U<DIM>>,
        Sub1<<U<N> as std::ops::Sub<U<DIM>>>::Output>: Unsigned + NonZero,
        <U<N> as std::ops::Sub<U<DIM>>>::Output: std::ops::Sub<typenum::B1>,
        U<DIM>: ArrayLength,
        Const<N>: ToUInt,
        Const<DIM>: ToUInt,
    {
        self.truncate::<DIM>(self.indices[DIM].len() - 2)
    }

    pub fn truncate<const DIM: usize>(&mut self, row_length: usize) -> bool {
        if row_length < self.indices[DIM].len() {
            self.indices[DIM].truncate(row_length + 1);
            let mut end = self.indices[DIM][row_length];
            for index in self.indices.iter_mut().skip(DIM + 1) {
                index.truncate(end.as_() + 1);
                end = *index.last().unwrap();
            }
            self.buffer.truncate(end.as_());
            true
        } else {
            false
        }
    }
}
pub trait JaggedArrayViewTrait<TVal, TNum, const N: usize>: Index<[usize; N]>
where
    TNum: AsPrimitive<usize> + Num,
    U<N>: std::ops::Sub<B1>,
    <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
    Const<N>: ToUInt,
{
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn view<const M: usize, const R: usize>(
        &self,
        index: [usize; M],
    ) -> JaggedArrayView<TVal, TNum, R>
    where
        U<N>: std::ops::Sub<U2> + std::ops::Sub<typenum::B1>,
        <U<N> as std::ops::Sub<U2>>::Output: ArrayLength,
        <U<N> as std::ops::Sub<typenum::B1>>::Output: std::ops::Sub<typenum::B1>,
        <<U<N> as std::ops::Sub<typenum::B1>>::Output as std::ops::Sub<typenum::B1>>::Output:
            ArrayLength,
        U<N>: std::ops::Sub<U<M>>,
        <U<N> as std::ops::Sub<U<M>>>::Output: IsEqual<U<R>>,
        U<R>: std::ops::Sub<B1>,
        <U<R> as std::ops::Sub<B1>>::Output: ArrayLength,
        <<U<N> as std::ops::Sub<U<M>>>::Output as IsEqual<U<R>>>::Output: NonZero,
        Const<N>: ToUInt,
        Const<M>: ToUInt,
        Const<R>: ToUInt;
    /// # Safety
    ///
    /// This method is unsafe because it allows for unchecked indexing
    unsafe fn view_unchecked<const M: usize, const R: usize>(
        &self,
        index: [usize; M],
    ) -> JaggedArrayView<TVal, TNum, R>
    where
        U<N>: std::ops::Sub<U2> + std::ops::Sub<typenum::B1>,
        <U<N> as std::ops::Sub<U2>>::Output: ArrayLength,
        <U<N> as std::ops::Sub<typenum::B1>>::Output: std::ops::Sub<typenum::B1>,
        <<U<N> as std::ops::Sub<typenum::B1>>::Output as std::ops::Sub<typenum::B1>>::Output:
            ArrayLength,
        U<N>: std::ops::Sub<U<M>>,
        <U<N> as std::ops::Sub<U<M>>>::Output: IsEqual<U<R>>,
        U<R>: std::ops::Sub<B1>,
        <U<R> as std::ops::Sub<B1>>::Output: ArrayLength,
        <<U<N> as std::ops::Sub<U<M>>>::Output as IsEqual<U<R>>>::Output: NonZero,
        Const<N>: ToUInt,
        Const<M>: ToUInt,
        Const<R>: ToUInt;
    /// # Safety
    ///
    /// This method is unsafe because it allows for unchecked indexing
    unsafe fn get_unchecked(&self, index: [usize; N]) -> &TVal;
    fn get(&self, index: [usize; N]) -> Option<&TVal>;
    // unsafe fn get_unchecked(&self, index: [usize; N]) -> &TVal;
    fn to_owned(self) -> JaggedArrayOwnedView<TVal, TNum, N>
    where
        TVal: Clone;
}

pub trait JaggedArrayMutViewTrait<TVal, TNum, const N: usize>:
    JaggedArrayViewTrait<TVal, TNum, N> + IndexMut<[usize; N]>
where
    TNum: AsPrimitive<usize> + Num,
    U<N>: std::ops::Sub<B1>,
    <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
    Const<N>: ToUInt,
{
    fn view_mut<const M: usize, const R: usize>(
        &mut self,
        index: [usize; M],
    ) -> JaggedArrayMutView<TVal, TNum, R>
    where
        U<N>: std::ops::Sub<U2> + std::ops::Sub<typenum::B1>,
        <U<N> as std::ops::Sub<U2>>::Output: ArrayLength,
        <U<N> as std::ops::Sub<typenum::B1>>::Output: std::ops::Sub<typenum::B1>,
        <<U<N> as std::ops::Sub<typenum::B1>>::Output as std::ops::Sub<typenum::B1>>::Output:
            ArrayLength,
        U<N>: std::ops::Sub<U<M>>,
        <U<N> as std::ops::Sub<U<M>>>::Output: IsEqual<U<R>>,
        U<R>: std::ops::Sub<B1>,
        <U<R> as std::ops::Sub<B1>>::Output: ArrayLength,
        <<U<N> as std::ops::Sub<U<M>>>::Output as IsEqual<U<R>>>::Output: NonZero,
        Const<N>: ToUInt,
        Const<M>: ToUInt,
        Const<R>: ToUInt;
}

pub trait JaggedArray1DViewTrait<TVal, TNum>
where
    TNum: AsPrimitive<usize> + Num,
{
    fn as_slice(&self) -> &[TVal];
}
pub trait JaggedArray1DMutViewTrait<TVal, TNum>
where
    TNum: AsPrimitive<usize> + Num,
{
    fn as_slice_mut(&mut self) -> &mut [TVal];
}
macro_rules! impl_view {
    ($num:ty, $typ:ident< $( $gen:tt ),+>,$type1:ty,$type2:path) => {
        impl<$( $gen ),+,const N:usize> JaggedArrayViewTrait<TVal, $num, N>
            for $typ<$($gen),+, N>
        where $type1:$type2,
        $num: AsPrimitive<usize>+Num+ConstOne+ConstZero,
        U<N>: std::ops::Sub<B1>,
        U<N>:ArrayLength,
        <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
        Const<N>: ToUInt,
        {
            #[inline]
            fn len(&self) -> usize {
                match self.indices.first()
                {
                    Some(index)=>index.len()-1,
                    None=>self.buffer.len()
                }
            }
            #[inline]
            fn is_empty(&self) -> bool {
                self.buffer.is_empty()
            }
            /// Rust const generics does not support arithmetic, so we have to specify the view's dimension(R) as well
            fn view<const M: usize, const R: usize>(
                &self,
                index: [usize; M],
            ) -> JaggedArrayView<TVal, $num, R>
            where
            U<N>: std::ops::Sub<U2> + std::ops::Sub<typenum::B1>,
            <U<N> as std::ops::Sub<U2>>::Output: ArrayLength,
            <U<N> as std::ops::Sub<typenum::B1>>::Output: std::ops::Sub<typenum::B1>,
            <<U<N> as std::ops::Sub<typenum::B1>>::Output as std::ops::Sub<typenum::B1>>::Output:
                ArrayLength,
            U<N>: std::ops::Sub<U<M>>,
            <U<N> as std::ops::Sub<U<M>>>::Output: IsEqual<U<R>>,
            U<R>: std::ops::Sub<B1>,
            <U<R> as std::ops::Sub<B1>>::Output: ArrayLength,
            <<U<N> as std::ops::Sub<U<M>>>::Output as IsEqual<U<R>>>::Output: NonZero,
            Const<N>: ToUInt,
            Const<M>: ToUInt,
            Const<R>: ToUInt
            {
                let m = (M+1).min(self.indices.len());
                let (first,remaining) = self.indices.split_at(m);
                let (index_buffer, self_indices) = first.split_first().unwrap();
                let mut index_buffer = &index_buffer[..];
                for (&i, idx) in zip(index.iter(), self_indices.iter()) {
                    index_buffer = &idx[index_buffer[i].as_()..index_buffer[i + 1].as_() + 1]
                }
                let mut result = GenericArray::<&[$num], Sub1<U<R>>>::uninit();
                let (indices, buffer) = if R > 1 {
                    result[0].write(index_buffer);
                    for (src,dst) in remaining.iter().zip(result.iter_mut().skip(1)) {
                        dst.write(src);
                    }
                    // SAFETY: Now safe as we initialized all elements from 0 to R-1
                    (
                        unsafe { GenericArray::assume_init(result) },
                        &self.buffer[..],
                    )
                } else {
                    let start_index = index_buffer[*index.last().unwrap()].as_();
                    let end_index = index_buffer[*index.last().unwrap() + 1].as_();
                    // SAFETY: zero-sized arrays don't need initialization
                    (
                        unsafe { GenericArray::assume_init(result) },
                        &self.buffer[start_index..end_index],
                    )
                };
                JaggedArrayView { indices, buffer }
            }
            /// Rust const generics does not support arithmetic, so we have to specify the view's dimension(R) as well
            unsafe fn view_unchecked<const M: usize, const R: usize>(
                &self,
                index: [usize; M],
            ) -> JaggedArrayView<TVal, $num, R>
            where
            U<N>: std::ops::Sub<U2> + std::ops::Sub<typenum::B1>,
            <U<N> as std::ops::Sub<U2>>::Output: ArrayLength,
            <U<N> as std::ops::Sub<typenum::B1>>::Output: std::ops::Sub<typenum::B1>,
            <<U<N> as std::ops::Sub<typenum::B1>>::Output as std::ops::Sub<typenum::B1>>::Output:
                ArrayLength,
            U<N>: std::ops::Sub<U<M>>,
            <U<N> as std::ops::Sub<U<M>>>::Output: IsEqual<U<R>>,
            U<R>: std::ops::Sub<B1>,
            <U<R> as std::ops::Sub<B1>>::Output: ArrayLength,
            <<U<N> as std::ops::Sub<U<M>>>::Output as IsEqual<U<R>>>::Output: NonZero,
            Const<N>: ToUInt,
            Const<M>: ToUInt,
            Const<R>: ToUInt
            {
                let mut index_buffer = self.indices.get_unchecked(0).get_unchecked(..);
                let m = (M+1).min(self.indices.len());
                for i in 1..m {
                    index_buffer = self.indices.get_unchecked(i).get_unchecked((*index_buffer.get_unchecked(*index.get_unchecked(i-1))).as_()
                    ..(*index_buffer.get_unchecked(*index.get_unchecked(i-1)+1)+<$num>::ONE).as_());
                }
                let mut result = GenericArray::<&[$num], Sub1<U<R>>>::uninit();
                let (indices, buffer) = if R > 1 {
                    result.get_unchecked_mut(0).write(index_buffer);
                    for i in m..self.indices.len() {
                        result.get_unchecked_mut(i-m+1).write(self.indices.get_unchecked(i));
                    }
                    // SAFETY: Now safe as we initialized all elements from 0 to R-1
                    (
                        unsafe { GenericArray::assume_init(result) },
                        self.buffer.get_unchecked(..),
                    )
                } else {
                    let last = *index.get_unchecked(M-1);
                    let start_index = (*index_buffer.get_unchecked(last)).as_();
                    let end_index = (*index_buffer.get_unchecked(last+1)).as_();
                    // SAFETY: zero-sized arrays don't need initialization
                    (
                        unsafe { GenericArray::assume_init(result) },
                        self.buffer.get_unchecked(start_index..end_index),
                    )
                };
                JaggedArrayView { indices, buffer }
            }
            unsafe fn get_unchecked(&self, index: [usize; N]) -> &TVal {
                if N > 1 {
                    let mut buffer_ptr = self.indices.get_unchecked(0).get_unchecked(..).as_ptr();
                    for i in 1..N-1 {
                        let idx = self.indices.get_unchecked(i);
                        let id = (*index.get_unchecked(i-1));
                        let s = *buffer_ptr.add(id);
                        buffer_ptr = idx.as_ptr().add(s.as_());
                    }
                    let last = *index.get_unchecked(N - 2);
                    let start_index = (*buffer_ptr.add(last)).as_();
                    self.buffer.get_unchecked(start_index+*index.get_unchecked(N - 1))
                } else {
                    self.buffer.get_unchecked(*index.get_unchecked(0))
                }
            }

            fn get(&self, index:[usize;N])->Option<&TVal>{
                if N > 1 {
                    let mut buffer = &self.indices[0][..];
                    for (&i, idx) in zip(index.iter(), self.indices[1..].iter()) {
                        buffer = &idx.get(buffer.get(i)?.as_()..buffer.get(i + 1)?.as_() + 1)?
                    }
                    let last = index[index.len() - 2];
                    let start_index = buffer.get(last)?.as_();
                    let end_index = buffer.get(last + 1)?.as_();
                    self.buffer.get(start_index..end_index)?.get(index[index.len() - 1])
                } else {
                    self.buffer.get(index[0])
                }
            }

            fn to_owned(self) -> JaggedArrayOwnedView<TVal, $num, N> where TVal:Clone {
                let indices = self.indices.iter().map(|idx| idx.to_vec().into_boxed_slice()).collect();
                let buffer = self.buffer.to_vec().into_boxed_slice();
                JaggedArrayOwnedView { indices, buffer }
            }
        }

        impl<$( $gen ),+,const N:usize> Index<[usize; N]> for $typ<$($gen),+, N>
        where
        $num: AsPrimitive<usize>+Num,
        $type1:$type2,
            U<N>: std::ops::Sub<B1>,
            <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
            Const<N>: ToUInt,
        {
            type Output = TVal;
            fn index(&self, index: [usize; N]) -> &Self::Output {
                if N > 1 {
                    let mut buffer = &self.indices[0][..];
                    for (&i, idx) in zip(index.iter(), self.indices[1..].iter()) {
                        buffer = &idx[buffer[i].as_()..buffer[i + 1].as_() + 1]
                    }
                    let last = index[index.len() - 2];
                    let start_index = buffer[last].as_();
                    let end_index = buffer[last + 1].as_();
                    &self.buffer[start_index..end_index][index[index.len() - 1]]
                } else {
                    &self.buffer[index[0]]
                }
            }
        }
    };
}

macro_rules! impl_view_mut {
    ($num:ty, $typ:ident< $( $gen:tt ),+>,$type1:ty,$type2:path) => {
        impl<$( $gen ),+,const N:usize> JaggedArrayMutViewTrait<TVal, $num, N>
            for $typ<$($gen),+, N>
        where $type1:$type2,
        $num: AsPrimitive<usize>+Num+ConstOne+ConstZero,
        U<N>: std::ops::Sub<B1>,
        U<N>:ArrayLength,
        <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
        Const<N>: ToUInt,
        {
            /// Rust const generics does not support arithmetic, so we have to specify the view's dimension(R) as well
            fn view_mut<const M: usize, const R: usize>(
                &mut self,
                index: [usize; M],
            ) -> JaggedArrayMutView<TVal, $num, R>
            where
            U<N>: std::ops::Sub<U2> + std::ops::Sub<typenum::B1>,
            <U<N> as std::ops::Sub<U2>>::Output: ArrayLength,
            <U<N> as std::ops::Sub<typenum::B1>>::Output: std::ops::Sub<typenum::B1>,
            <<U<N> as std::ops::Sub<typenum::B1>>::Output as std::ops::Sub<typenum::B1>>::Output:
                ArrayLength,
            U<N>: std::ops::Sub<U<M>>,
            <U<N> as std::ops::Sub<U<M>>>::Output: IsEqual<U<R>>,
            U<R>: std::ops::Sub<B1>,
            <U<R> as std::ops::Sub<B1>>::Output: ArrayLength,
            <<U<N> as std::ops::Sub<U<M>>>::Output as IsEqual<U<R>>>::Output: NonZero,
            Const<N>: ToUInt,
            Const<M>: ToUInt,
            Const<R>: ToUInt
            {
                let m = (M+1).min(self.indices.len());
                let (first,remaining) = self.indices.split_at_mut(m);
                let (index_buffer, self_indices) = first.split_first_mut().unwrap();
                let mut index_buffer = &mut index_buffer[..];
                for (&i, idx) in zip(index.iter(), self_indices.iter_mut()) {
                    index_buffer = &mut idx[index_buffer[i].as_()..index_buffer[i + 1].as_() + 1]
                }
                let mut result = GenericArray::<&mut [$num], Sub1<U<R>>>::uninit();
                let (indices, buffer) = if R > 1 {
                    result[0].write(index_buffer);
                    for (src,dst) in remaining.iter_mut().zip(result.iter_mut().skip(1)) {
                        dst.write(src);
                    }
                    // SAFETY: Now safe as we initialized all elements from 0 to R-1
                    (
                        unsafe { GenericArray::assume_init(result) },
                        &mut self.buffer[..],
                    )
                } else {
                    let start_index = index_buffer[*index.last().unwrap()].as_();
                    let end_index = index_buffer[*index.last().unwrap() + 1].as_();
                    // SAFETY: zero-sized arrays don't need initialization
                    (
                        unsafe { GenericArray::assume_init(result) },
                        &mut self.buffer[start_index..end_index],
                    )
                };
                JaggedArrayMutView { indices, buffer }
            }
        }

        impl<$( $gen ),+,const N:usize> IndexMut<[usize; N]> for $typ<$($gen),+, N>
        where $type1:$type2,
        $num: AsPrimitive<usize>+Num,
            U<N>: std::ops::Sub<B1> + ArrayLength,
            <U<N> as std::ops::Sub<B1>>::Output: ArrayLength,
            Const<N>: ToUInt,
        {
            fn index_mut(&mut self, index: [usize; N]) -> &mut TVal {
                if N > 1 {
                    let mut buffer = &self.indices[0][..];
                    for (&i, idx) in zip(index.iter(), self.indices[1..].iter()) {
                        buffer = &idx[buffer[i].as_()..buffer[i + 1].as_() + 1]
                    }
                    let start_index = buffer[index[index.len() - 2]].as_();
                    let end_index = buffer[index[index.len() - 2] + 1].as_();
                    &mut self.buffer[start_index..end_index][index[index.len() - 1]]
                } else {
                    &mut self.buffer[index[0]]
                }
            }
        }
    };
}
macro_rules! impl_view1d_owned {
    ($num:ty, $typ:ident< $( $gen:tt ),+>,$type1:ty,$type2:path) => {
        impl<$( $gen ),+> JaggedArray1DViewTrait<TVal, $num> for $typ<$($gen),+,1> where $num: AsPrimitive<usize> + Num,$type1:$type2
        {
            fn as_slice(&self) -> &[TVal] {
                &self.buffer
            }
        }
    };
}
macro_rules! impl_view_mut1d_owned {
    ($num:ty, $typ:ident< $( $gen:tt ),+>,$type1:ty,$type2:path) => {
        impl<$( $gen ),+> JaggedArray1DMutViewTrait<TVal, $num> for $typ<$($gen),+,1> where $num: AsPrimitive<usize> + Num,$type1:$type2
        {
            fn as_slice_mut(&mut self) -> &mut [TVal] {
                &mut self.buffer
            }
        }
    };
}

impl<'a, TVal, TNum> JaggedArrayView<'a, TVal, TNum, 1>
where
    TNum: AsPrimitive<usize> + Num + NumAssignOps + std::cmp::PartialOrd + ConstOne + ConstZero,
    usize: num::traits::AsPrimitive<TNum>,
{
    pub fn as_slice(&self) -> &'a [TVal] {
        self.buffer
    }
}

impl<'a, TVal, TNum> JaggedArrayMutView<'a, TVal, TNum, 1>
where
    TNum: AsPrimitive<usize> + Num + NumAssignOps + std::cmp::PartialOrd + ConstOne + ConstZero,
    usize: num::traits::AsPrimitive<TNum>,
{
    pub fn as_slice<'b: 'a>(&'b self) -> &'a [TVal] {
        // We need this mysterious lifetime to make the borrow checker happy
        self.buffer
    }

    pub fn as_slice_mut(&mut self) -> &mut [TVal] {
        self.buffer
    }
}

impl_view!(<TBuffer as VecLike>::TI,JaggedArray<TVal, TBuffer>,TBuffer,VecLike);
impl_view!(TNum, JaggedArrayView<'a, TVal, TNum>, TNum, Num);
impl_view!(TNum, JaggedArrayMutView<'a, TVal, TNum>, TNum, Num);
impl_view!(TNum,JaggedArrayOwnedView<TVal, TNum>,TNum,Num);
impl_view1d_owned!(<TBuffer as VecLike>::TI,JaggedArray<TVal, TBuffer>,TBuffer,VecLike);
impl_view1d_owned!(TNum,JaggedArrayOwnedView<TVal, TNum>,TNum,Num);
impl_view_mut!(<TBuffer as VecLike>::TI,JaggedArray<TVal, TBuffer>,TBuffer,VecLike);
impl_view_mut!(TNum, JaggedArrayMutView<'a, TVal, TNum>, TNum, Num);
impl_view_mut!(TNum,JaggedArrayOwnedView<TVal, TNum>,TNum,Num);
impl_view_mut1d_owned!(TNum,JaggedArrayOwnedView<TVal, TNum>,TNum,Num);
