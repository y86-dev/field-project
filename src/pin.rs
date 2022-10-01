impl FieldProject for PinProj {
    type Wrapper<'a, T> = Pin<&'a T> where T:'a;
    type WrapperMut<'a, T> = Pin<&'a mut T> where T:'a;

    unsafe fn field_project<'a, T, U>(
        self,
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        Pin::new_unchecked(&*f(&*this))
    }

    unsafe fn field_project_mut<'a, T, U>(
        self,
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        Pin::new_unchecked(&mut *f(Pin::get_unchecked_mut(this)))
    }
}

#[derive(Default)]
pub struct PinProj;

impl<T> Projectable for Pin<&T> {
    type FP = PinProj;
}

impl<T> Projectable for Pin<&mut T> {
    type FP = PinProj;
}

#[test]
fn pin() {
    let mut foo = Box::pin(Foo {
        a: 0,
        b: Bar {
            a: 1,
            b: 2,
            _pin: PhantomPinned,
        },
    });
    let mut f = foo.as_mut();
    let r = f.as_mut();
    let b: Pin<&mut Bar> = project!(&mut r.b);
    let mut b: Pin<&mut u32> = project!(&mut b.b);
    *b = 84;
    let r = f.as_mut();
    let mut a: Pin<&mut usize> = project!(&mut r.a);
    *a = 42;
    println!("{foo:?}");
    assert_eq!(
        foo,
        Box::pin(Foo {
            a: 42,
            b: Bar {
                a: 1,
                b: 84,
                _pin: PhantomPinned,
            },
        })
    );
}
