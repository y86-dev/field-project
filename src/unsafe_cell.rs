impl FieldProject for UnsafeCell<()> {
    type Wrapper<'a, T> = &'a UnsafeCell<T> where T:'a;
    type WrapperMut<'a, T> = &'a UnsafeCell<T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        &*f(this.get()).cast::<UnsafeCell<U>>()
    }

    unsafe fn field_project_mut<'a, T, U>(
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        &*f(this.get()).cast::<UnsafeCell<U>>()
    }
}

#[test]
fn unsafe_cell() {
    let foo = UnsafeCell::new(Foo {
        a: 0,
        b: Bar {
            a: 1,
            b: 2,
            _pin: PhantomPinned,
        },
    });
    let f = &foo;
    let b: &UnsafeCell<Bar> = project!(&f.b as UnsafeCell<()>);
    let b: &UnsafeCell<u32> = project!(&b.b as UnsafeCell<()>);
    unsafe {
        b.get().write(84);
    }
    let a: &UnsafeCell<usize> = project!(&f.a as UnsafeCell<()>);
    unsafe {
        a.get().write(42);
    }
    let foo = foo.into_inner();
    println!("{foo:?}");
    assert_eq!(
        foo,
        Foo {
            a: 42,
            b: Bar {
                a: 1,
                b: 84,
                _pin: PhantomPinned,
            },
        }
    );
}
