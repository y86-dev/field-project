impl FieldProject for RefCell<()> {
    type Wrapper<'a, T> = Ref<'a, T> where T:'a;
    type WrapperMut<'a, T> = RefMut<'a, T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        Ref::map(this, |i| &*f(i))
    }

    unsafe fn field_project_mut<'a, T, U>(
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        RefMut::map(this, |i| &mut *f(i))
    }
}

#[test]
fn ref_cell() {
    let foo = RefCell::new(Foo {
        a: 0,
        b: Bar {
            a: 1,
            b: 2,
            _pin: PhantomPinned,
        },
    });
    let f = foo.borrow_mut();
    let b: RefMut<'_, Bar> = project!(&mut f.b as RefCell<()>);
    let mut b: RefMut<'_, u32> = project!(&mut b.b as RefCell<()>);
    *b = 84;
    drop(b);
    let f = foo.borrow_mut();
    let mut a: RefMut<'_, usize> = project!(&mut f.a as RefCell<()>);
    *a = 42;
    drop(a);
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
