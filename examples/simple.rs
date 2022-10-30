#![feature(const_refs_to_cell, new_uninit)]
#![allow(dead_code)]

use core::{marker::PhantomPinned, mem::MaybeUninit, pin::Pin};

use field_project::*;

#[derive(Debug, HasFields, PinProjections)]
struct Foo<T> {
    bar: u8,
    #[pin]
    baz: Bar<T>,
}

#[derive(Debug, HasFields, PinProjections)]
struct Bar<T> {
    phantom: PhantomPinned,
    t: T,
}

fn main() {
    let mut foo = Box::pin(Foo {
        bar: 0,
        baz: Bar {
            phantom: PhantomPinned,
            t: 42,
        },
    });

    let _: Pin<&mut Bar<i32>> = foo.as_mut().project(Foo::baz);
    let mut foo: Pin<Box<MaybeUninit<Foo<i32>>>> = Box::pin(MaybeUninit::uninit());
    foo.as_mut().project(Foo::bar).write(42);
    foo.as_mut().project(Foo::baz).set(MaybeUninit::new(Bar {
        phantom: PhantomPinned,
        t: 86,
    }));
    let foo = unsafe {
        Pin::new_unchecked(Box::<MaybeUninit<Foo<_>>>::assume_init(
            Pin::into_inner_unchecked(foo),
        ))
    };
    println!("{foo:?}");
}
