#![doc = include_str!("../README.md")]

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    ConstParam, Data, DeriveInput, Error, Fields, GenericArgument, Generics, Index, Lifetime,
    LifetimeParam, Token, TypeParam, WhereClause,
};

fn crate_root() -> TokenStream {
    quote!(::portable_hash)
}

#[proc_macro_derive(PortableHash)]
#[allow(non_snake_case)]
pub fn derive_portable_hash(input: TokenStream1) -> TokenStream1 {
    let root = crate_root();
    let hash = quote!(#root::PortableHash);
    let hasher_write = quote!(#root::PortableHasher);

    let mut input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let mut tokens = TokenStream::new();
    let mut types = Vec::new();

    match input.data {
        // Stability: structs are hashed in the order of their fields.
        Data::Struct(x) => match x.fields {
            Fields::Named(x) => {
                let fields = x.named.iter().map(|x| {
                    types.push(x.ty.clone());
                    x.ident.as_ref().unwrap()
                });
                quote! {
                    #( #hash::portable_hash(&self.#fields, state); )*
                }
                    .to_tokens(&mut tokens)
            }

            Fields::Unnamed(x) => {
                let fields = x.unnamed.iter().enumerate().map(|(i, x)| {
                    types.push(x.ty.clone());
                    Index::from(i)
                });
                quote! {
                    #( #hash::portable_hash(&self.#fields, state); )*
                }
                    .to_tokens(&mut tokens)
            }

            Fields::Unit => (),
        },

        // Stability: (TODO) enums must be keyed with a discriminant for DoS resistance.
        Data::Enum(x) => {
            let mut variant_tokens = TokenStream::new();

            for x in x.variants.iter() {
                let var = &x.ident;

                match &x.fields {
                    Fields::Named(x) => {
                        let fields: Vec<_> = x
                            .named
                            .iter()
                            .map(|x| {
                                types.push(x.ty.clone());
                                x.ident.as_ref().unwrap()
                            })
                            .collect();
                        quote! {
                            Self::#var { #(#fields),* } => { #( #hash::portable_hash(#fields, state); )* }
                        }
                            .to_tokens(&mut variant_tokens);
                    }

                    Fields::Unnamed(x) => {
                        let fields: Vec<_> = x
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, x)| {
                                types.push(x.ty.clone());
                                format_ident!("_{i}")
                            })
                            .collect();
                        quote! {
                            Self::#var(#(#fields),*) => { #( #hash::portable_hash(#fields, state); )* }
                        }
                            .to_tokens(&mut variant_tokens);
                    }

                    Fields::Unit => quote! {
                        Self::#var => (),
                    }
                        .to_tokens(&mut variant_tokens),
                }
            }

            quote! {
                // TODO(stability): use a portable discriminant for hashing
                //   named -> use hash(str(variant_name)) + hash(data)
                //   unnamed -> use hash(index) + hash(data)
                //   unit -> use hash(index)
                #hash::hash(&core::mem::discriminant(self), state);
                match self {
                    #variant_tokens
                }
            }
                .to_tokens(&mut tokens);
        }

        Data::Union(_) => {
            return Error::new(ident.span(), "can't derive `Hash` for union")
                .to_compile_error()
                .into()
        }
    }

    input.generics.make_where_clause();
    let wc = input.generics.where_clause.as_mut().unwrap();
    let where_ = fix_where(Some(wc));
    let SplitGenerics {
        lti,
        ltt,
        tpi,
        tpt,
        cpi,
        cpt,
        wc,
    } = split_generics(&input.generics);
    quote! {
        impl<#(#lti,)* #(#tpi,)* #(#cpi,)*> #hash for #ident<#(#ltt,)* #(#tpt,)* #(#cpt),*> #where_ #wc
            #( #types: #hash ),*
        {
            #[inline]
            fn portable_hash<H: #hasher_write>(&self, state: &mut H) {
                #tokens
            }
        }
    }
        .into()
}

// #[proc_macro]
// pub fn impl_core_hash(input: TokenStream1) -> TokenStream1 {
//     let root = crate_root();
//     let hash = quote!(#root::Hash);
//
//     let input = parse_macro_input!(input as IdentsWithGenerics);
//     let mut output = TokenStream::new();
//
//     for IdentWithGenerics {
//         impl_generics,
//         ident,
//         use_generics,
//         mut where_clause,
//     } in input.punctuated
//     {
//         let where_ = fix_where(where_clause.as_mut());
//         quote! {
//             impl #impl_generics ::core::hash::Hash for #ident #use_generics #where_ #where_clause
//                 Self: #hash,
//             {
//                 #[inline]
//                 fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
//                     <Self as #hash>::hash(
//                         self, &mut #root::internal::WrapCoreForHasherU64::new(state)
//                     )
//                 }
//             }
//         }
//             .to_tokens(&mut output);
//     }
//     output.into()
// }
//
// #[proc_macro]
// pub fn impl_core_hasher(input: TokenStream1) -> TokenStream1 {
//     let root = crate_root();
//     let hasher_t = quote!(#root::Hasher);
//     let hasher_write = quote!(#root::HasherWrite);
//
//     let input = parse_macro_input!(input as IdentsWithGenerics);
//     let mut output = TokenStream::new();
//
//     for IdentWithGenerics {
//         impl_generics,
//         ident,
//         use_generics,
//         mut where_clause,
//     } in input.punctuated
//     {
//         let mut body = quote! {
//             #[inline(always)]
//             fn finish(&self) -> u64 {
//                 <Self as #hasher_t::<u64>>::finish(self)
//             }
//
//             #[inline(always)]
//             fn write(&mut self, bytes: &[u8]) {
//                 <Self as #hasher_write>::write(self, bytes)
//             }
//         };
//
//         for t in [
//             quote!(u8),
//             quote!(u16),
//             quote!(u32),
//             quote!(u64),
//             quote!(u128),
//             quote!(usize),
//             quote!(i8),
//             quote!(i16),
//             quote!(i32),
//             quote!(i64),
//             quote!(i128),
//             quote!(isize),
//         ] {
//             let wid = format_ident!("write_{t}");
//             quote! {
//                 #[inline(always)]
//                 fn #wid(&mut self, i: #t) {
//                     <Self as #hasher_write>::#wid(self, i);
//                 }
//             }
//                 .to_tokens(&mut body);
//         }
//
//         let where_ = fix_where(where_clause.as_mut());
//         quote! {
//             impl #impl_generics ::core::hash::Hasher for #ident #use_generics #where_ #where_clause
//                 Self: #hasher_t<u64>,
//             {
//                 #body
//             }
//         }
//             .to_tokens(&mut output);
//     }
//     output.into()
// }
//
// #[proc_macro]
// pub fn impl_core_build_hasher(input: TokenStream1) -> TokenStream1 {
//     let root = crate_root();
//     let build_hasher_t = quote!(#root::BuildHasher);
//
//     let input = parse_macro_input!(input as IdentsWithGenerics);
//     let mut output = TokenStream::new();
//
//     for IdentWithGenerics {
//         impl_generics,
//         ident,
//         use_generics,
//         mut where_clause,
//     } in input.punctuated
//     {
//         let where_ = fix_where(where_clause.as_mut());
//         quote! {
//             impl #impl_generics ::core::hash::BuildHasher for #ident #use_generics #where_ #where_clause
//                 Self: #build_hasher_t<u64>,
//             {
//                 type Hasher = #root::internal::WrapHasherU64ForCore<<Self as #build_hasher_t::<u64>>::Hasher>;
//
//                 #[inline]
//                 fn build_hasher(&self) -> Self::Hasher {
//                     Self::Hasher::new(<Self as #build_hasher_t::<u64>>::build_hasher(self))
//                 }
//             }
//         }
//             .to_tokens(&mut output);
//     }
//     output.into()
// }
//
// #[proc_macro]
// #[allow(non_snake_case)]
// pub fn impl_hash(input: TokenStream1) -> TokenStream1 {
//     let root = crate_root();
//     let hash = quote!(#root::Hash);
//     let hasher_write = quote!(#root::HasherWrite);
//
//     let input = parse_macro_input!(input as IdentsWithGenerics);
//     let mut output = TokenStream::new();
//
//     for IdentWithGenerics {
//         impl_generics,
//         ident,
//         use_generics,
//         mut where_clause,
//     } in input.punctuated
//     {
//         let SplitGenerics {
//             lti,
//             ltt: _,
//             tpi,
//             tpt: _,
//             cpi,
//             cpt: _,
//             wc: _,
//         } = split_generics(&impl_generics);
//         let where_ = fix_where(where_clause.as_mut());
//
//         quote! {
//             impl<#(#lti,)* #(#tpi,)* #(#cpi,)*> #hash for #ident #use_generics #where_ #where_clause {
//                 #[inline]
//                 fn hash<H: #hasher_write>(&self, state: &mut H) {
//                     <Self as ::core::hash::Hash>::hash(
//                         self, &mut #root::internal::WrapHasherWriteForCore::new(state)
//                     )
//                 }
//             }
//         }
//             .to_tokens(&mut output);
//     }
//     output.into()
// }

fn fix_where(wc: Option<&mut WhereClause>) -> Option<Token![where]> {
    if let Some(wc) = wc {
        if wc.predicates.is_empty() {
            Some(wc.where_token)
        } else {
            if !wc.predicates.trailing_punct() {
                wc.predicates.push_punct(<Token![,]>::default());
            }
            None
        }
    } else {
        Some(<Token![where]>::default())
    }
}

struct SplitGenerics<
    'a,
    LTI: Iterator<Item = &'a LifetimeParam>,
    LTT: Iterator<Item = &'a Lifetime>,
    TPI: Iterator<Item = &'a TypeParam>,
    TPT: Iterator<Item = &'a Ident>,
    CPI: Iterator<Item = &'a ConstParam>,
    CPT: Iterator<Item = &'a Ident>,
> {
    lti: LTI,
    ltt: LTT,
    tpi: TPI,
    tpt: TPT,
    cpi: CPI,
    cpt: CPT,
    wc: &'a Option<WhereClause>,
}

fn split_generics(
    generics: &Generics,
) -> SplitGenerics<
    impl Iterator<Item = &LifetimeParam>,
    impl Iterator<Item = &Lifetime>,
    impl Iterator<Item = &TypeParam>,
    impl Iterator<Item = &Ident>,
    impl Iterator<Item = &ConstParam>,
    impl Iterator<Item = &Ident>,
> {
    SplitGenerics {
        lti: generics.lifetimes(),
        ltt: generics.lifetimes().map(|l| &l.lifetime),
        tpi: generics.type_params(),
        tpt: generics.type_params().map(|t| &t.ident),
        cpi: generics.const_params(),
        cpt: generics.const_params().map(|c| &c.ident),
        wc: &generics.where_clause,
    }
}

// struct IdentsWithGenerics {
//     punctuated: Punctuated<IdentWithGenerics, Token![;]>,
// }
//
// impl Parse for IdentsWithGenerics {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let punctuated = Punctuated::parse_terminated(input)?;
//         Ok(Self { punctuated })
//     }
// }
//
// struct IdentWithGenerics {
//     impl_generics: Generics,
//     ident: Ident,
//     use_generics: Option<GenericArguments>,
//     where_clause: Option<WhereClause>,
// }
//
// impl Parse for IdentWithGenerics {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let impl_generics = if Option::<Token![impl]>::parse(input)?.is_some() {
//             Generics::parse(input)?
//         } else {
//             Generics::default()
//         };
//         let ident = Ident::parse(input)?;
//         let use_generics = if input.peek(Token![<]) {
//             Some(GenericArguments::parse(input)?)
//         } else {
//             None
//         };
//         let where_clause = Option::<WhereClause>::parse(input)?;
//
//         Ok(Self {
//             impl_generics,
//             ident,
//             use_generics,
//             where_clause,
//         })
//     }
// }

struct GenericArguments {
    lt_token: Token![<],
    args: Punctuated<GenericArgument, Token![,]>,
    rt_token: Token![>],
}

impl Parse for GenericArguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lt_token = <Token![<]>::parse(input)?;

        let mut args = Punctuated::new();
        while let Ok(arg) = GenericArgument::parse(input) {
            args.push(arg);
            if let Ok(comma) = <Token![,]>::parse(input) {
                args.push_punct(comma);
            } else {
                break;
            }
        }

        let rt_token = <Token![>]>::parse(input)?;

        Ok(Self {
            lt_token,
            args,
            rt_token,
        })
    }
}

impl ToTokens for GenericArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lt_token.to_tokens(tokens);
        self.args.to_tokens(tokens);
        self.rt_token.to_tokens(tokens);
    }
}
