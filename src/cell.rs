impl FieldProject for CellProj {
    type Wrapper<'a, T> = &'a Cell<T> where T:'a;
    type WrapperMut<'a, T> = &'a Cell<T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        self,
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        &*f(this.as_ptr()).cast::<Cell<U>>()
    }

    unsafe fn field_project_mut<'a, T, U>(
        self,
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        &*f(this.as_ptr()).cast::<Cell<U>>()
    }
}

#[derive(Default)]
pub struct CellProj;

impl<T> Projectable for &Cell<T> {
    type FP = CellProj;
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
    let b: &Cell<Bar> = project!(&f.b);
    let b: &Cell<u32> = project!(&b.b);
    b.set(84);
    let a: &Cell<usize> = project!(&f.a);
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
