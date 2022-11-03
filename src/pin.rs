use crate::*;
use core::pin::Pin;

unsafe impl<'a, T: HasFields> Project<'a> for Pin<&'a mut T> {
    type Inner = T;
    type Projected<U: 'a> = Pin<&'a mut U>;
    type Unwrapped<U: 'a> = &'a mut U;

    unsafe fn project_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Projected<U> {
        unsafe {
            Pin::new_unchecked(
                &mut *(Pin::into_inner_unchecked(self) as *mut T)
                    .cast::<u8>()
                    .add(field.offset())
                    .cast::<U>(),
            )
        }
    }
    unsafe fn unwrap_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrapped<U> {
        unsafe {
            &mut *(Pin::into_inner_unchecked(self) as *mut T)
                .cast::<u8>()
                .add(field.offset())
                .cast::<U>()
        }
    }
}
