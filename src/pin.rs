use crate::*;
use core::pin::Pin;

unsafe impl<'a, T: HasFields> Project<'a> for Pin<&'a mut T> {
    type Inner = T;
    type Output<U: 'a> = Pin<&'a mut U>;
    type Unwrap<U: 'a> = &'a mut U;

    unsafe fn project_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Output<U> {
        unsafe {
            Pin::new_unchecked(
                &mut *(Pin::into_inner_unchecked(self) as *mut T)
                    .cast::<u8>()
                    .add(field.offset())
                    .cast::<U>(),
            )
        }
    }
    unsafe fn unwrap_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrap<U> {
        unsafe {
            &mut *(Pin::into_inner_unchecked(self) as *mut T)
                .cast::<u8>()
                .add(field.offset())
                .cast::<U>()
        }
    }
}
