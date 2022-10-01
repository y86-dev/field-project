impl FieldProject for UnsafeCellProj {
    type Wrapper<'a, T> = &'a UnsafeCell<T> where T:'a;
    type WrapperMut<'a, T> = &'a UnsafeCell<T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        self,
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        &*f(this.get()).cast::<UnsafeCell<U>>()
    }

    unsafe fn field_project_mut<'a, T, U>(
        self,
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        &*f(this.get()).cast::<UnsafeCell<U>>()
    }
}

#[derive(Default)]
pub struct UnsafeCellProj;

impl<T> Projectable for &UnsafeCell<T> {
    type FP = UnsafeCellProj;
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
    let b: &UnsafeCell<Bar> = project!(&f.b);
    let b: &UnsafeCell<u32> = project!(&b.b);
    unsafe { b.get().write(84) };
    let a: &UnsafeCell<usize> = project!(&f.a);
    unsafe { a.get().write(42) };
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
