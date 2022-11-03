use crate::*;

unsafe impl<'a, T: HasFields> Project<'a> for &'a mut T {
    type Inner = T;
    type Projected<U: 'a> = &'a mut U;
    type Unwrapped<U: 'a> = &'a mut U;

    unsafe fn project_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Projected<U> {
        unsafe {
            &mut *(self as *mut T)
                .cast::<u8>()
                .add(field.offset())
                .cast::<U>()
        }
    }

    unsafe fn unwrap_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrapped<U> {
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
    type Projected<U: 'a> = &'a U;
    type Unwrapped<U: 'a> = &'a U;

    unsafe fn project_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Projected<U> {
        unsafe {
            &*(self as *const T)
                .cast::<u8>()
                .add(field.offset())
                .cast::<U>()
        }
    }

    unsafe fn unwrap_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrapped<U> {
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
