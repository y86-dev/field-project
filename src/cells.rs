use crate::*;
use core::cell::{Cell, UnsafeCell};

unsafe impl<'a, T: HasFields> Project<'a> for &'a UnsafeCell<T> {
    type Inner = T;
    type Projected<U: 'a> = &'a UnsafeCell<U>;
    type Unwrapped<U: 'a> = &'a UnsafeCell<U>;

    unsafe fn project_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Projected<U> {
        unsafe { &*self.get().project(field).cast::<UnsafeCell<U>>() }
    }

    unsafe fn unwrap_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Unwrapped<U> {
        unsafe { &*self.get().project(field).cast::<UnsafeCell<U>>() }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, &'a UnsafeCell<T>>
    for Field<T, U, N>
{
    type ProjKind = Projected;
}

unsafe impl<'a, T: HasFields> Project<'a> for &'a Cell<T> {
    type Inner = T;
    type Projected<U: 'a> = &'a Cell<U>;
    type Unwrapped<U: 'a> = &'a Cell<U>;

    unsafe fn project_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Projected<U> {
        unsafe { &*self.as_ptr().project(field).cast::<Cell<U>>() }
    }

    unsafe fn unwrap_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Projected<U> {
        unsafe { &*self.as_ptr().project(field).cast::<Cell<U>>() }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, &'a Cell<T>> for Field<T, U, N> {
    type ProjKind = Projected;
}
