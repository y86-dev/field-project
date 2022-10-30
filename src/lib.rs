#![feature(negative_impls, const_refs_to_cell)]
use core::marker::PhantomData;

mod cells;
mod maybe_uninit;
mod pin;
mod pin_uninit;
mod refs;

pub use field_project_internal::{HasFields, PinProjections};

pub struct Field<T: HasFields, U, const N: usize> {
    offset: usize,
    phantom: PhantomData<fn(T, U) -> (T, U)>,
}

impl<T: HasFields, U, const N: usize> Field<T, U, N> {
    pub const unsafe fn new(offset: usize) -> Self {
        Self {
            offset,
            phantom: PhantomData,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

pub struct Projected;
pub struct Unwrapped;

mod sealed {
    pub unsafe trait IsField {}
    unsafe impl<T: super::HasFields, U, const N: usize> IsField for super::Field<T, U, N> {}

    pub unsafe trait IsProjKind {}
    unsafe impl IsProjKind for super::Projected {}
    unsafe impl IsProjKind for super::Unwrapped {}
}

pub unsafe trait ProjSelector<'a, P: Project<'a>>: sealed::IsProjKind {
    type Output<U: 'a>: 'a
    where
        Self: 'a;
    unsafe fn select_proj<U, const N: usize>(
        proj: P,
        field: Field<P::Inner, U, N>,
    ) -> Self::Output<U>
    where
        Field<P::Inner, U, N>: Projectable<'a, P>;
}

unsafe impl<'a, P: Project<'a>> ProjSelector<'a, P> for Projected {
    type Output<U: 'a> = P::Output<U>;
    unsafe fn select_proj<U, const N: usize>(
        proj: P,
        field: Field<P::Inner, U, N>,
    ) -> Self::Output<U>
    where
        Field<P::Inner, U, N>: Projectable<'a, P>,
    {
        P::project_true(proj, field)
    }
}

unsafe impl<'a, P: Project<'a>> ProjSelector<'a, P> for Unwrapped {
    type Output<U: 'a> = P::Unwrap<U>;
    unsafe fn select_proj<U, const N: usize>(
        proj: P,
        field: Field<P::Inner, U, N>,
    ) -> Self::Output<U>
    where
        Field<P::Inner, U, N>: Projectable<'a, P>,
    {
        P::unwrap_true(proj, field)
    }
}

pub trait Projectable<'a, P: Project<'a>>: sealed::IsField {
    type ProjKind: sealed::IsProjKind;
}

pub unsafe trait Project<'a>: 'a + Sized {
    type Inner: 'a + HasFields;
    type Output<U: 'a>: 'a
    where
        Self: 'a;
    type Unwrap<U: 'a>: 'a
    where
        Self: 'a;

    fn project<U: 'a, const N: usize>(
        self,
        field: Field<Self::Inner, U, N>,
    ) -> <<Field<Self::Inner, U, N> as Projectable<'a, Self>>::ProjKind as ProjSelector<'a, Self>>::Output<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>,
        <Field<Self::Inner, U, N> as Projectable<'a, Self>>::ProjKind: ProjSelector<'a, Self>,
    {
        unsafe {
            <Field<Self::Inner, U, N> as Projectable<'a, Self>>::ProjKind::select_proj(self, field)
        }
    }

    unsafe fn project_true<U: 'a, const N: usize>(
        self,
        field: Field<Self::Inner, U, N>,
    ) -> Self::Output<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>;

    unsafe fn unwrap_true<U: 'a, const N: usize>(
        self,
        field: Field<Self::Inner, U, N>,
    ) -> Self::Unwrap<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>;
}

pub unsafe trait HasFields {}

impl<T> !HasFields for core::mem::MaybeUninit<T> {}
impl<T> !HasFields for core::cell::Cell<T> {}
impl<T> !HasFields for core::cell::UnsafeCell<T> {}
