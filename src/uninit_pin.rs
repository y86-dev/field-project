impl FieldProject for PinMaybeUninitProj {
    type Wrapper<'a, T> = Pin<&'a MaybeUninit<T>> where T:'a;
    type WrapperMut<'a, T> = Pin<&'a mut MaybeUninit<T>> where T:'a;

    unsafe fn field_project<'a, T, U>(
        self,
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        Pin::new_unchecked(&*f(this.as_ptr()).cast::<MaybeUninit<U>>())
    }

    unsafe fn field_project_mut<'a, T, U>(
        self,
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        Pin::new_unchecked(
            &mut *f(Pin::get_unchecked_mut(this).as_mut_ptr()).cast::<MaybeUninit<U>>(),
        )
    }
}

#[derive(Default)]
pub struct PinMaybeUninitProj;

impl<T> Projectable for Pin<&MaybeUninit<T>> {
    type FP = PinMaybeUninitProj;
}

impl<T> Projectable for Pin<&mut MaybeUninit<T>> {
    type FP = PinMaybeUninitProj;
}

#[test]
fn uninit_pin() {
    let mut foo: Pin<Box<MaybeUninit<Foo>>> = Box::pin(MaybeUninit::uninit());
    let mut f = foo.as_mut();
    let r = f.as_mut();
    let mut bar: Pin<&mut MaybeUninit<Bar>> = project!(&mut r.b);
    let b = bar.as_mut();
    let mut b: Pin<&mut MaybeUninit<u32>> = project!(&mut b.b);
    b.write(1337);
    let mut a: Pin<&mut MaybeUninit<usize>> = project!(&mut bar.a);
    a.write(42);
    let r = f.as_mut();
    let mut a: Pin<&mut MaybeUninit<usize>> = project!(&mut r.a);
    a.write(0);
    let foo = unsafe { Pin::new_unchecked(Pin::into_inner_unchecked(foo).assume_init()) };
    println!("{foo:?}");
    assert_eq!(
        foo,
        Box::pin(Foo {
            a: 0,
            b: Bar {
                a: 42,
                b: 1337,
                _pin: PhantomPinned,
            },
        })
    );
}
