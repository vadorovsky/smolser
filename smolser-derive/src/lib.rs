use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, parse_quote};

#[proc_macro_derive(Pod)]
pub fn derive_pod(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive(input)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

fn expand_derive(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let DeriveInput {
        ident,
        mut generics,
        data,
        ..
    } = input;

    let has_type_generics = generics.type_params().next().is_some();

    let mut size_terms = Vec::new();
    match data {
        Data::Struct(data) => {
            let where_clause = generics.make_where_clause();
            match data.fields {
                Fields::Named(named) => {
                    for field in named.named {
                        let ty = field.ty;
                        where_clause
                            .predicates
                            .push(parse_quote!(#ty: ::smolser::Pod));
                        size_terms.push(quote!(::core::mem::size_of::<#ty>()));
                    }
                }
                Fields::Unnamed(unnamed) => {
                    for field in unnamed.unnamed {
                        let ty = field.ty;
                        where_clause
                            .predicates
                            .push(parse_quote!(#ty: ::smolser::Pod));
                        size_terms.push(quote!(::core::mem::size_of::<#ty>()));
                    }
                }
                Fields::Unit => {}
            }
        }
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "#[derive(Pod)] can only be used with structs",
            ));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let size_sum = if let Some((first, rest)) = size_terms.split_first() {
        let rest_iter = rest.iter();
        quote!(#first #( + #rest_iter)*)
    } else {
        quote!(0)
    };

    let padding_assert = (!has_type_generics)
        .then(|| {
            quote! {
                const _: fn() = || {
                    #[doc(hidden)]
                    struct TypeWithoutPadding([u8; #size_sum]);
                    let _ = ::core::mem::transmute::<#ident #ty_generics, TypeWithoutPadding>;
                };
            }
        })
        .into_iter();

    Ok(quote! {
        #(#padding_assert)*
        unsafe impl #impl_generics ::smolser::Pod for #ident #ty_generics #where_clause {}
    })
}
