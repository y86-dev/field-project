use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::*;

#[proc_macro_derive(HasFields)]
pub fn derive_has_fields(item: TokenStream) -> TokenStream {
    let ItemStruct {
        generics,
        ident,
        fields,
        ..
    } = parse_macro_input!(item);
    let (impl_generics, ty_generics, whr) = generics.split_for_impl();
    assert!(matches!(fields, Fields::Named(_)));
    let fields = fields
        .into_iter()
        .enumerate()
        .map(
            |(
                i,
                Field {
                    ident: field, ty, ..
                },
            )| {
                quote! {
                    const #field: ::field_project::Field<#ident #ty_generics, #ty, #i> = unsafe {
                        ::field_project::Field::new(::memoffset::offset_of!(#ident #ty_generics, #field))
                    };
                }
            },
        )
        .collect::<TokenStream2>();
    quote! {
        #[allow(non_upper_case_globals)]
        impl #impl_generics #ident #ty_generics
            #whr
        {
            #fields
        }
        unsafe impl #impl_generics ::field_project::HasFields for #ident #ty_generics {}
    }
    .into()
}

#[proc_macro_derive(PinProjections, attributes(pin))]
pub fn derive_pin_projections(item: TokenStream) -> TokenStream {
    let ItemStruct {
        generics,
        ident,
        fields,
        ..
    } = parse_macro_input!(item);
    let (_, ty_generics, whr) = generics.split_for_impl();
    let mut generics = generics.clone();
    generics.params.insert(0, parse2(quote! {'___pin}).unwrap());
    let (impl_generics, _, _) = generics.split_for_impl();
    fields
        .into_iter()
        .enumerate()
        .map(|(i, Field { attrs, ty, .. })| {
            if attrs.iter().any(|a| a.path.is_ident("pin") && matches!(a.style, AttrStyle::Outer)) {
                quote! {
                    impl #impl_generics ::field_project::Projectable<'___pin, ::core::pin::Pin<&'___pin mut #ident #ty_generics>> for ::field_project::Field<#ident #ty_generics, #ty, #i>
                        #whr
                    {
                        type ProjKind = ::field_project::Projected;
                    }
                }
            } else {
                quote! {
                    impl #impl_generics ::field_project::Projectable<'___pin, ::core::pin::Pin<&'___pin mut #ident #ty_generics>> for ::field_project::Field<#ident #ty_generics, #ty, #i>
                        #whr
                    {
                        type ProjKind = ::field_project::Unwrapped;
                    }
                }
            }
        })
        .collect::<TokenStream2>().into()
}
