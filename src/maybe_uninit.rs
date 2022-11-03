use crate::*;
use core::mem::MaybeUninit;

unsafe impl<'a, T: HasFields> Project<'a> for &'a mut MaybeUninit<T> {
    type Inner = T;
    type Projected<U: 'a> = &'a mut MaybeUninit<U>;
    type Unwrapped<U: 'a> = &'a mut MaybeUninit<U>;

    unsafe fn project_field<U: 'a, const N: usize>(
        self,
        field: Field<T, U, N>,
    ) -> Self::Projected<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>,
    {
        unsafe { &mut *self.as_mut_ptr().project(field).cast::<MaybeUninit<U>>() }
    }

    unsafe fn unwrap_field<U: 'a, const N: usize>(self, field: Field<T, U, N>) -> Self::Unwrapped<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>,
    {
        unsafe { &mut *self.as_mut_ptr().project(field).cast::<MaybeUninit<U>>() }
    }
}

impl<'a, T: 'a + HasFields, U: 'a, const N: usize> Projectable<'a, &'a mut MaybeUninit<T>>
    for Field<T, U, N>
{
    type ProjKind = Projected;
}
