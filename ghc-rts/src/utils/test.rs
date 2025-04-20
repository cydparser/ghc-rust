pub use quickcheck::{Arbitrary, Gen};
use std::cell::UnsafeCell;
pub use std::mem::ManuallyDrop;

pub trait HasReferences {
    type Owned;
    type Pointees;

    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self;

    fn owned(&self) -> Self::Owned;
}

pub struct WithReferences<T: HasReferences> {
    pointees: *mut T::Pointees,
    inner: ManuallyDrop<UnsafeCell<T>>,
}

impl<T: HasReferences> Drop for WithReferences<T> {
    fn drop(&mut self) {
        // SAFETY: `inner` must be droped before `pointees`.
        unsafe { ManuallyDrop::drop(&mut self.inner) };
        // SAFETY: `pointees` is safe to drop after `inner`.
        let _ = unsafe { Box::from_raw(self.pointees) };
    }
}

impl<T: HasReferences> AsRef<UnsafeCell<T>> for WithReferences<T> {
    fn as_ref(&self) -> &UnsafeCell<T> {
        &self.inner
    }
}

impl<T: HasReferences> Clone for WithReferences<T>
where
    T::Owned: Clone,
    T::Pointees: Clone,
{
    fn clone(&self) -> Self {
        // SAFETY: Self is immutably borrowed.
        let owned: T::Owned = unsafe { &*self.inner.get() }.owned();
        // SAFETY: Self is immutably borrowed.
        let pointees: *mut T::Pointees =
            Box::into_raw(Box::new(unsafe { (*self.pointees).clone() }));

        WithReferences {
            pointees,
            inner: ManuallyDrop::new(T::from_parts(owned, pointees).into()),
        }
    }
}

impl<T: HasReferences + 'static> Arbitrary for WithReferences<T>
where
    T::Owned: Arbitrary,
    T::Pointees: Arbitrary,
{
    fn arbitrary(g: &mut Gen) -> Self {
        let owned: T::Owned = Arbitrary::arbitrary(g);
        let pointees: *mut T::Pointees = Box::into_raw(Box::new(Arbitrary::arbitrary(g)));

        WithReferences {
            pointees,
            inner: ManuallyDrop::new(T::from_parts(owned, pointees).into()),
        }
    }
}
