use crate::*;
use core::{mem::MaybeUninit, pin::Pin};

unsafe impl<'a, T: HasFields> Project<'a> for Pin<&'a mut MaybeUninit<T>> {
    type Inner = T;
    type Projected<U: 'a> = Pin<&'a mut MaybeUninit<U>>;
    type Unwrapped<U: 'a> = &'a mut MaybeUninit<U>;

    unsafe fn project_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Projected<U> {
        unsafe {
            Pin::new_unchecked(
                &mut *Pin::into_inner_unchecked(self)
                    .as_mut_ptr()
                    .cast::<u8>()
                    .add(field.offset())
                    .cast::<MaybeUninit<U>>(),
            )
        }
    }

    unsafe fn unwrap_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrapped<U> {
        unsafe {
            &mut *Pin::into_inner_unchecked(self)
                .as_mut_ptr()
                .cast::<u8>()
                .add(field.offset())
                .cast::<MaybeUninit<U>>()
        }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, Pin<&'a mut MaybeUninit<T>>>
    for Field<T, U, N>
where
    Field<T, U, N>: Projectable<'a, Pin<&'a mut T>>,
{
    type ProjKind = <Field<T, U, N> as Projectable<'a, Pin<&'a mut T>>>::ProjKind;
}
