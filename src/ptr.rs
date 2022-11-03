use crate::*;

unsafe impl<'a, T: HasFields + 'a> Project<'a> for *mut T {
    type Inner = T;
    type Projected<U: 'a> = *mut U;
    type Unwrapped<U: 'a> = *mut U;

    unsafe fn project_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Projected<U> {
        self.cast::<u8>().add(field.offset()).cast::<U>()
    }

    unsafe fn unwrap_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Projected<U> {
        self.cast::<u8>().add(field.offset()).cast::<U>()
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, *mut T> for Field<T, U, N> {
    type ProjKind = Projected;
}

unsafe impl<'a, T: HasFields + 'a> Project<'a> for *const T {
    type Inner = T;
    type Projected<U: 'a> = *const U;
    type Unwrapped<U: 'a> = *const U;

    unsafe fn project_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Projected<U> {
        self.cast::<u8>().add(field.offset()).cast::<U>()
    }

    unsafe fn unwrap_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Projected<U> {
        self.cast::<u8>().add(field.offset()).cast::<U>()
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, *const T> for Field<T, U, N> {
    type ProjKind = Projected;
}
