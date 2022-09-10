impl FieldProject for MaybeUninit<()> {
    type Wrapper<'a, T> = &'a MaybeUninit<T>where T:'a;
    type WrapperMut<'a, T> = &'a mut MaybeUninit<T>where T:'a;

    unsafe fn field_project<'a, T, U>(
        this: Self::Wrapper<'a, T>,
        f: impl FnOnce(*const T) -> *const U,
    ) -> Self::Wrapper<'a, U> {
        &*f(this.as_ptr()).cast::<MaybeUninit<U>>()
    }

    unsafe fn field_project_mut<'a, T, U>(
        this: Self::WrapperMut<'a, T>,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> Self::WrapperMut<'a, U> {
        &mut *f(this.as_mut_ptr()).cast::<MaybeUninit<U>>()
    }
}

#[test]
fn uninit() {
    let mut foo: MaybeUninit<Foo> = MaybeUninit::uninit();
    let r = &mut foo;
    let bar: &mut MaybeUninit<Bar> = project!(&mut r.b as MaybeUninit<()>);
    let b: &mut MaybeUninit<u32> = project!(&mut bar.b as MaybeUninit<()>);
    b.write(1337);
    let a: &mut MaybeUninit<usize> = project!(&mut bar.a as MaybeUninit<()>);
    a.write(42);
    let a: &mut MaybeUninit<usize> = project!(&mut r.a as MaybeUninit<()>);
    a.write(0);
    let foo = unsafe { foo.assume_init() };
    println!("{foo:?}");
    assert_eq!(
        foo,
        Foo {
            a: 0,
            b: Bar {
                a: 42,
                b: 1337,
                _pin: PhantomPinned,
            },
        }
    );
}
