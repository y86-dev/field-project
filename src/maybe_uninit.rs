use crate::*;
use core::mem::MaybeUninit;

unsafe impl<'a, T: HasFields> Project<'a> for &'a mut MaybeUninit<T> {
    type Inner = T;
    type Output<U: 'a> = &'a mut MaybeUninit<U>;
    type Unwrap<U: 'a> = &'a mut MaybeUninit<U>;

    unsafe fn project_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Output<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>,
    {
        unsafe {
            &mut *self
                .as_mut_ptr()
                .cast::<u8>()
                .add(field.offset())
                .cast::<MaybeUninit<U>>()
        }
    }

    unsafe fn unwrap_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrap<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>,
    {
        unsafe {
            &mut *self
                .as_mut_ptr()
                .cast::<u8>()
                .add(field.offset())
                .cast::<MaybeUninit<U>>()
        }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, &'a mut MaybeUninit<T>>
    for Field<T, U, N>
{
    type ProjKind = Projected;
}