#![feature(generic_associated_types, new_uninit)]
use core::{
    cell::{Cell, Ref, RefCell, RefMut, UnsafeCell},
    marker::PhantomPinned,
    mem::MaybeUninit,
    pin::Pin,
};

pub trait FieldProject {
    type Wrapper<'a, T>
    where
        T: 'a;
    type WrapperMut<'a, T>
    where
        T: 'a;

    /// Safety: closure must only do a field projection and not access the inner data
    unsafe fn field_project<'a, T, U>(
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U>;

    /// Safety: closure must only do a field projection and not access the inner data
    unsafe fn field_project_mut<'a, T, U>(
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U>;
}

#[macro_export]
macro_rules! project {
    (&$e:ident.$f:ident as $t:ty) => {
        unsafe {
            <$t as $crate::FieldProject>::field_project($e, |i| ::core::ptr::addr_of!((*i).$f))
        }
    };
    (&mut $e:ident.$f:ident as $t:ty) => {
        unsafe {
            <$t as $crate::FieldProject>::field_project_mut($e, |i| {
                ::core::ptr::addr_of_mut!((*i).$f)
            })
        }
    };
}
#[derive(Debug, Eq, PartialEq)]
struct Foo {
    a: usize,
    b: Bar,
}

#[derive(Debug, Eq, PartialEq)]
struct Bar {
    a: usize,
    b: u32,
    _pin: PhantomPinned,
}

include!("uninit.rs");
include!("uninit_pin.rs");
include!("pin.rs");
include!("cell.rs");
include!("unsafe_cell.rs");
include!("ref_cell.rs");
include!("option.rs");

fn main() {}
