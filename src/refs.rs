use crate::*;

unsafe impl<'a, T: HasFields> Project<'a> for &'a mut T {
    type Inner = T;
    type Output<U: 'a> = &'a mut U;
    type Unwrap<U: 'a> = &'a mut U;

    unsafe fn project_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Output<U> {
        unsafe {
            &mut *(self as *mut T)
                .cast::<u8>()
                .add(field.offset())
                .cast::<U>()
        }
    }

    unsafe fn unwrap_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrap<U> {
        unsafe {
            &mut *(self as *mut T)
                .cast::<u8>()
                .add(field.offset())
                .cast::<U>()
        }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, &'a mut T> for Field<T, U, N> {
    type ProjKind = Projected;
}

unsafe impl<'a, T: HasFields> Project<'a> for &'a T {
    type Inner = T;
    type Output<U: 'a> = &'a U;
    type Unwrap<U: 'a> = &'a U;

    unsafe fn project_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Output<U> {
        unsafe {
            &*(self as *const T)
                .cast::<u8>()
                .add(field.offset())
                .cast::<U>()
        }
    }

    unsafe fn unwrap_true<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrap<U> {
        unsafe {
            &*(self as *const T)
                .cast::<u8>()
                .add(field.offset())
                .cast::<U>()
        }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, &'a T> for Field<T, U, N> {
    type ProjKind = Projected;
}
