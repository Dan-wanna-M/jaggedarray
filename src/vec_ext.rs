//! Modified from: https://github.com/rust-lang/rust/issues/100486#issuecomment-2102599123
pub trait Ext<T> {
    /// Appends an element to the back of a collection without checking
    /// if the collection has enough capacity.
    ///
    /// # Safety
    /// The user must ensure that `self.len() < self.capacity()`.
    ///
    unsafe fn unchecked_push(&mut self, value: T);
}

// ===== impl Vec =====

impl<T> Ext<T> for Vec<T> {
    unsafe fn unchecked_push(&mut self, value: T) {
        let len = self.len();
        self.as_mut_ptr().add(len).write(value);
        self.set_len(len + 1);
    }
}