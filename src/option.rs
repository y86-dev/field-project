impl FieldProject for OptionProj {
    type Wrapper<'a, T> = Option<&'a T> where T:'a;
    type WrapperMut<'a, T> = Option<&'a mut T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        self,
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        Option::map(this, |i| &*f(i))
    }

    unsafe fn field_project_mut<'a, T, U>(
        self,
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        Option::map(this, |i| &mut *f(i))
    }
}

#[derive(Default)]
pub struct OptionProj;

impl<T> Projectable for Option<&T> {
    type FP = OptionProj;
}
impl<T> Projectable for Option<&mut T> {
    type FP = OptionProj;
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
    let b: Option<&mut Bar> = project!(&mut r.b);
    let b: Option<&mut u32> = project!(&mut b.b);
    b.map(|i| *i = 84);
    let r = f.as_mut();
    let a: Option<&mut usize> = project!(&mut r.a);
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
