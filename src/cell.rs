impl FieldProject for Cell<()> {
    type Wrapper<'a, T> = &'a Cell<T> where T:'a;
    type WrapperMut<'a, T> = &'a Cell<T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        &*f(this.as_ptr()).cast::<Cell<U>>()
    }

    unsafe fn field_project_mut<'a, T, U>(
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        &*f(this.as_ptr()).cast::<Cell<U>>()
    }
}

#[test]
fn cell() {
    let foo = Cell::new(Foo {
        a: 0,
        b: Bar {
            a: 1,
            b: 2,
            _pin: PhantomPinned,
        },
    });
    let f = &foo;
    let b: &Cell<Bar> = project!(&f.b as Cell<()>);
    let b: &Cell<u32> = project!(&b.b as Cell<()>);
    b.set(84);
    let a: &Cell<usize> = project!(&f.a as Cell<()>);
    a.set(42);
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
