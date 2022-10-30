#![feature(negative_impls, const_refs_to_cell)]
use core::marker::PhantomData;

mod cells;
mod maybe_uninit;
mod pin;
mod pin_uninit;
mod refs;

pub use field_project_internal::{HasFields, PinProjections};

/// Representation of a field.
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

/// Project a field structurally.
pub struct Projected;
/// Unwrap a field.
pub struct Unwrapped;

mod sealed {
    pub unsafe trait IsField {}
    unsafe impl<T: super::HasFields, U, const N: usize> IsField for super::Field<T, U, N> {}

    pub unsafe trait IsProjKind {}
    unsafe impl IsProjKind for super::Projected {}
    unsafe impl IsProjKind for super::Unwrapped {}
}

/// Helper trait used to decide which kind of projection takes place.
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

/// Info trait to set the structural projection kind of a field.
pub trait Projectable<'a, P: Project<'a>>: sealed::IsField {
    /// The projection kind of this field.
    type ProjKind: sealed::IsProjKind;
}

/// Facilitates projections.
///
/// Wrappers like [`Pin`] should implement this trait. It allows projecting from
/// `Wrapper<&mut Struct>` to `Wrapper<&mut Field>`.
pub unsafe trait Project<'a>: 'a + Sized {
    /// The inner type that will be projected.
    type Inner: 'a + HasFields;
    /// The output type of structurally projected fields.
    type Output<U: 'a>: 'a
    where
        Self: 'a;
    /// The output type of not structurally projected fields.
    type Unwrap<U: 'a>: 'a
    where
        Self: 'a;

    /// Project this wrapper to the given field according to its structural projection kind.
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

    /// Project the given field regardless of its structural projection kind.
    ///
    /// # Safety
    ///
    /// Only call this function if the structural projection kind is indeed structural projection
    /// for the given field.
    unsafe fn project_true<U: 'a, const N: usize>(
        self,
        field: Field<Self::Inner, U, N>,
    ) -> Self::Output<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>;

    /// Unwraps the given field regardless of its structural projection kind.
    ///
    /// # Safety
    ///
    /// Only call this function if the structural projection kind is indeed no structural projection
    /// for the given field.
    unsafe fn unwrap_true<U: 'a, const N: usize>(
        self,
        field: Field<Self::Inner, U, N>,
    ) -> Self::Unwrap<U>
    where
        Field<Self::Inner, U, N>: Projectable<'a, Self>;
}

/// # Safety
///
/// - only structs with named fields implement this trait,
/// - the type provides for every field a constant with the same name and visibilty of type
/// [`Field<Self, FieldType, N>`] where `N` is unique to that field.
pub unsafe trait HasFields {}

impl<T> !HasFields for core::mem::MaybeUninit<T> {}
impl<T> !HasFields for core::cell::Cell<T> {}
impl<T> !HasFields for core::cell::UnsafeCell<T> {}
