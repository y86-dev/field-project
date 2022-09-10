impl FieldProject for Option<()> {
    type Wrapper<'a, T> = Option<&'a T> where T:'a;
    type WrapperMut<'a, T> = Option<&'a mut T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        Option::map(this, |i| &*f(i))
    }

    unsafe fn field_project_mut<'a, T, U>(
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        Option::map(this, |i| &mut *f(i))
    }
}

#[test]
fn option() {
    let mut foo = Some(Foo {
        a: 0,
        b: Bar {
            a: 1,
            b: 2,
            _pin: PhantomPinned,
        },
    });
    let mut f = foo.as_mut();
    let r = f.as_mut();
    let b: Option<&mut Bar> = project!(&mut r.b as Option<()>);
    let b: Option<&mut u32> = project!(&mut b.b as Option<()>);
    b.map(|i| *i = 84);
    let r = f.as_mut();
    let a: Option<&mut usize> = project!(&mut r.a as Option<()>);
    a.map(|i| *i = 42);
    println!("{foo:?}");
    assert_eq!(
        foo,
        Some(Foo {
            a: 42,
            b: Bar {
                a: 1,
                b: 84,
                _pin: PhantomPinned,
            },
        })
    );
}
