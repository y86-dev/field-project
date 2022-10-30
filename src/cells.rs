use crate::*;
use core::cell::{Cell, UnsafeCell};

unsafe impl<'a, T: HasFields> Project<'a> for &'a UnsafeCell<T> {
    type Inner = T;
    type Output<U: 'a> = &'a UnsafeCell<U>;
    type Unwrap<U: 'a> = &'a UnsafeCell<U>;

    unsafe fn project_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Output<U> {
        unsafe {
            &*self
                .get()
                .cast::<u8>()
                .add(field.offset())
                .cast::<UnsafeCell<U>>()
        }
    }

    unsafe fn unwrap_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrap<U> {
        unsafe {
            &*self
                .get()
                .cast::<u8>()
                .add(field.offset())
                .cast::<UnsafeCell<U>>()
        }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, &'a UnsafeCell<T>>
    for Field<T, U, N>
{
    type ProjKind = Projected;
}

unsafe impl<'a, T: HasFields> Project<'a> for &'a Cell<T> {
    type Inner = T;
    type Output<U: 'a> = &'a Cell<U>;
    type Unwrap<U: 'a> = &'a Cell<U>;

    unsafe fn project_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Output<U> {
        unsafe {
            &*self
                .as_ptr()
                .cast::<u8>()
                .add(field.offset())
                .cast::<Cell<U>>()
        }
    }

    unsafe fn unwrap_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Output<U> {
        unsafe {
            &*self
                .as_ptr()
                .cast::<u8>()
                .add(field.offset())
                .cast::<Cell<U>>()
        }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, &'a Cell<T>> for Field<T, U, N> {
    type ProjKind = Projected;
}
