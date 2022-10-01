impl FieldProject for RefCellProj {
    type Wrapper<'a, T> = Ref<'a, T> where T:'a;
    type WrapperMut<'a, T> = RefMut<'a, T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        self,
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        Ref::map(this, |i| &*f(i))
    }

    unsafe fn field_project_mut<'a, T, U>(
        self,
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        RefMut::map(this, |i| &mut *f(i))
    }
}
#[derive(Default)]
pub struct RefCellProj;

impl<T> Projectable for Ref<'_, T> {
    type FP = RefCellProj;
}

impl<T> Projectable for RefMut<'_, T> {
    type FP = RefCellProj;
}

#[test]
fn ref_cell() {
    use core::cell::RefCell;
    let foo = RefCell::new(Foo {
        a: 0,
        b: Bar {
            a: 1,
            b: 2,
            _pin: PhantomPinned,
        },
    });
    let f = foo.borrow_mut();
    let b: RefMut<'_, Bar> = project!(&mut f.b);
    let mut b: RefMut<'_, u32> = project!(&mut b.b);
    *b = 84;
    drop(b);
    let f = foo.borrow_mut();
    let mut a: RefMut<'_, usize> = project!(&mut f.a);
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
