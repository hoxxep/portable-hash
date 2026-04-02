#![doc = include_str!("../README.md")]

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input,
    spanned::Spanned,
    ConstParam, Data, DeriveInput, Error, Expr, Fields, Generics, Index, Lifetime,
    LifetimeParam, Lit, Token, TypeParam, WhereClause,
};

fn crate_root() -> TokenStream {
    quote!(::portable_hash)
}

/// FNV-1a 64-bit hash for compile-time variant name hashing.
///
/// This function is used by the proc-macro to hash enum variant names into stable `u64`
/// discriminants at compile time. The output is baked into generated code as literal values.
///
/// **This function MUST NEVER be changed**, as doing so would silently break all derived
/// enum hashes that use name-based discriminants.
///
/// Reference: <http://www.isthe.com/chongo/tech/comp/fnv/>
fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x00000100000001B3);
        i += 1;
    }
    hash
}

// ---------------------------------------------------------------------------
// Enum configuration types
// ---------------------------------------------------------------------------

/// Controls how enum variant discriminants are computed.
#[derive(Clone, Copy, PartialEq)]
enum DiscriminantMode {
    /// Hash the variant name via FNV-1a 64-bit (default).
    Name,
    /// Use the variant's Rust discriminant value (0, 1, 2, ... or explicit `= N`).
    Index,
}

/// Controls the write method used for all discriminants in an enum.
#[derive(Clone, Copy, PartialEq, Debug)]
enum DiscriminantWidth {
    /// `write_u8` for all variants.
    U8,
    /// `write_u16` for all variants.
    U16,
    /// `write_u32` for all variants.
    U32,
    /// `write_u64` for all variants (default for name/index modes).
    U64,
    /// `write_isize` for all variants (writes as i64 portably).
    Isize,
    /// Match the enum's `#[repr(...)]` type. Defaults to `isize` if no repr.
    /// Only valid in compat mode.
    Repr,
}

/// Parsed enum-level configuration.
struct EnumConfig {
    mode: DiscriminantMode,
    width: Option<DiscriminantWidth>,
}

/// Parsed per-variant attributes.
struct VariantConfig {
    /// Manual discriminant override: `#[portable_hash(discriminant = 42)]`
    discriminant_override: Option<(u64, Span)>,
    /// Rename for hashing: `#[portable_hash(rename = "OldName")]`
    rename: Option<(String, Span)>,
}

// ---------------------------------------------------------------------------
// Attribute parsing
// ---------------------------------------------------------------------------

/// Parse enum-level `#[portable_hash(...)]` attributes.
fn parse_enum_attrs(attrs: &[syn::Attribute]) -> Result<EnumConfig, Error> {
    let mut config = EnumConfig {
        mode: DiscriminantMode::Name,
        width: None,
    };

    for attr in attrs {
        if !attr.path().is_ident("portable_hash") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("discriminant") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                match &lit {
                    Lit::Str(s) => match s.value().as_str() {
                        "name" => config.mode = DiscriminantMode::Name,
                        "index" => config.mode = DiscriminantMode::Index,
                        other => {
                            return Err(Error::new(
                                s.span(),
                                format!(
                                    "unknown discriminant mode `{other}`, expected `\"name\"` or `\"index\"`"
                                ),
                            ))
                        }
                    },
                    _ => {
                        return Err(Error::new(
                            lit.span(),
                            "expected a string literal (`\"name\"` or `\"index\"`)",
                        ))
                    }
                }
            } else if meta.path.is_ident("discriminant_width") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                match &lit {
                    Lit::Str(s) => {
                        config.width = Some(match s.value().as_str() {
                            "u8" => DiscriminantWidth::U8,
                            "u16" => DiscriminantWidth::U16,
                            "u32" => DiscriminantWidth::U32,
                            "u64" => DiscriminantWidth::U64,
                            "isize" => DiscriminantWidth::Isize,
                            "repr" => DiscriminantWidth::Repr,
                            other => {
                                return Err(Error::new(
                                    s.span(),
                                    format!(
                                        "unknown discriminant_width `{other}`, expected \
                                         `\"u8\"`, `\"u16\"`, `\"u32\"`, `\"u64\"`, `\"isize\"`, or `\"repr\"`"
                                    ),
                                ))
                            }
                        });
                    }
                    _ => {
                        return Err(Error::new(
                            lit.span(),
                            "discriminant_width must be a string literal",
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    meta.path.span(),
                    "unknown portable_hash enum attribute, expected \
                     `discriminant` or `discriminant_width`",
                ));
            }
            Ok(())
        })?;
    }

    Ok(config)
}

/// Parse variant-level `#[portable_hash(...)]` attributes.
fn parse_variant_attrs(attrs: &[syn::Attribute], config: &EnumConfig) -> Result<VariantConfig, Error> {
    let mut var_config = VariantConfig {
        discriminant_override: None,
        rename: None,
    };

    for attr in attrs {
        if !attr.path().is_ident("portable_hash") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("discriminant") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                match &lit {
                    Lit::Int(i) => {
                        let val: u64 = i.base10_parse().map_err(|_| {
                            Error::new(i.span(), "discriminant must be a valid u64 integer")
                        })?;
                        var_config.discriminant_override = Some((val, i.span()));
                    }
                    _ => {
                        return Err(Error::new(
                            lit.span(),
                            "variant discriminant must be an integer literal",
                        ))
                    }
                }
            } else if meta.path.is_ident("rename") {
                let value = meta.value()?;
                let lit: Lit = value.parse()?;
                match &lit {
                    Lit::Str(s) => {
                        var_config.rename = Some((s.value(), s.span()));
                    }
                    _ => {
                        return Err(Error::new(
                            lit.span(),
                            "rename must be a string literal",
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    meta.path.span(),
                    "unknown portable_hash variant attribute, expected `discriminant` or `rename`",
                ));
            }
            Ok(())
        })?;
    }

    // Validate attribute combinations.
    if let (Some((_, disc_span)), Some(_)) = (&var_config.discriminant_override, &var_config.rename) {
        return Err(Error::new(
            *disc_span,
            "cannot combine `discriminant` and `rename` on the same variant",
        ));
    }

    if var_config.rename.is_some() && config.mode == DiscriminantMode::Index {
        let (_, span) = var_config.rename.as_ref().unwrap();
        return Err(Error::new(
            *span,
            "`rename` is only valid with name-based discriminants (the default), \
             not with index or compat mode",
        ));
    }

    Ok(var_config)
}

// ---------------------------------------------------------------------------
// Discriminant value helpers
// ---------------------------------------------------------------------------

/// Parse a Rust discriminant expression (`= 42`, `= -1`) into an i128.
fn parse_discriminant_expr(expr: &Expr) -> Result<i128, Error> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Int(i) => i.base10_parse::<i128>().map_err(|_| {
                Error::new(i.span(), "discriminant value out of range")
            }),
            _ => Err(Error::new(
                lit.span(),
                "expected an integer literal for discriminant value",
            )),
        },
        Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            let inner = parse_discriminant_expr(&unary.expr)?;
            Ok(-inner)
        }
        _ => Err(Error::new(
            expr.span(),
            "unsupported discriminant expression; use a literal integer or \
             `#[portable_hash(discriminant = N)]` for complex expressions",
        )),
    }
}

/// Resolve `#[repr(...)]` from enum attributes to a `DiscriminantWidth`.
fn resolve_repr_width(attrs: &[syn::Attribute]) -> DiscriminantWidth {
    for attr in attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }
        // Parse the repr(...) content.
        if let Ok(list) = attr.meta.require_list() {
            let repr_str = list.tokens.to_string();
            return match repr_str.as_str() {
                "u8" | "i8" => DiscriminantWidth::U8,
                "u16" | "i16" => DiscriminantWidth::U16,
                "u32" | "i32" => DiscriminantWidth::U32,
                "u64" | "i64" => DiscriminantWidth::U64,
                "isize" | "usize" => DiscriminantWidth::Isize,
                _ => DiscriminantWidth::Isize, // C, other reprs → default to isize
            };
        }
    }
    // No repr attribute → default to isize (matches std's default enum repr).
    DiscriminantWidth::Isize
}

/// Determine the effective width for the enum.
fn effective_width(config: &EnumConfig, all_attrs: &[syn::Attribute]) -> DiscriminantWidth {
    if let Some(w) = config.width {
        if w == DiscriminantWidth::Repr {
            resolve_repr_width(all_attrs)
        } else {
            w
        }
    } else {
        DiscriminantWidth::U64
    }
}

/// Generate the write method identifier and literal for a discriminant value.
fn disc_write_tokens(width: DiscriminantWidth, value: i128, span: Span) -> Result<(Ident, TokenStream), Error> {
    match width {
        DiscriminantWidth::U8 => {
            if value < 0 || value > u8::MAX as i128 {
                return Err(Error::new(span, format!(
                    "discriminant value {value} does not fit in u8 (0..=255)"
                )));
            }
            let lit = Literal::u8_suffixed(value as u8);
            Ok((Ident::new("write_u8", span), quote!(#lit)))
        }
        DiscriminantWidth::U16 => {
            if value < 0 || value > u16::MAX as i128 {
                return Err(Error::new(span, format!(
                    "discriminant value {value} does not fit in u16 (0..=65535)"
                )));
            }
            let lit = Literal::u16_suffixed(value as u16);
            Ok((Ident::new("write_u16", span), quote!(#lit)))
        }
        DiscriminantWidth::U32 => {
            if value < 0 || value > u32::MAX as i128 {
                return Err(Error::new(span, format!(
                    "discriminant value {value} does not fit in u32"
                )));
            }
            let lit = Literal::u32_suffixed(value as u32);
            Ok((Ident::new("write_u32", span), quote!(#lit)))
        }
        DiscriminantWidth::U64 => {
            if value < 0 || value > u64::MAX as i128 {
                return Err(Error::new(span, format!(
                    "discriminant value {value} does not fit in u64"
                )));
            }
            let lit = Literal::u64_suffixed(value as u64);
            Ok((Ident::new("write_u64", span), quote!(#lit)))
        }
        DiscriminantWidth::Isize | DiscriminantWidth::Repr => {
            // write_isize; our PortableHasher converts to i64 portably.
            if value < i64::MIN as i128 || value > i64::MAX as i128 {
                return Err(Error::new(span, format!(
                    "discriminant value {value} does not fit in i64/isize"
                )));
            }
            let v = value as i64;
            Ok((Ident::new("write_isize", span), quote!(#v as isize)))
        }
    }
}

// ---------------------------------------------------------------------------
// Derive macro
// ---------------------------------------------------------------------------

/// Derives [`PortableHash`] for structs and enums.
///
/// # Structs
///
/// Fields are hashed in declaration order. Reordering or removing fields changes the hash
/// output. Renaming fields is safe.
///
/// # Enums
///
/// By default, each variant is identified by a **name-based discriminant**: the variant's
/// name is hashed at compile time via FNV-1a 64-bit, and the resulting `u64` is written to
/// the hasher. This means:
///
/// - **Reordering** variants is safe (hash output is unchanged).
/// - **Renaming** a variant is a breaking change (hash output changes).
///
/// ## Enum Attributes
///
/// ### `#[portable_hash(discriminant = "...")]`
///
/// Controls how variant discriminants are computed:
///
/// - `"name"` (default) — hash the variant name at compile time.
/// - `"index"` — use the variant's Rust discriminant value (0, 1, 2, ... or explicit `= N`).
///   Reordering breaks hashes but renaming is safe.
///
/// ### `#[portable_hash(discriminant_width = "...")]`
///
/// Controls the write method used for all discriminants in the enum:
///
/// - `"u64"` (default) — `write_u64` for all variants.
/// - `"u8"`, `"u16"`, `"u32"` — fixed width; compile error if any discriminant doesn't fit.
/// - `"isize"` — `write_isize` (converts to i64 portably).
/// - `"repr"` — match the enum's `#[repr(...)]` type (defaults to `isize` if no repr).
///
/// ## Variant Attributes
///
/// ### `#[portable_hash(discriminant = <integer>)]`
///
/// Overrides the discriminant for a specific variant with an explicit `u64` value.
///
/// ### `#[portable_hash(rename = "OldName")]`
///
/// Hashes "OldName" instead of the variant's actual name. Only valid in name mode.
///
/// ## Explicit Rust Discriminant Values
///
/// In index and compat modes, explicit Rust discriminant values are respected:
///
/// ```ignore
/// #[derive(PortableHash)]
/// #[portable_hash(discriminant = "index")]
/// enum MyEnum {
///     A = 5,  // discriminant 5
///     B,      // discriminant 6 (auto-incremented)
///     C = 20, // discriminant 20
/// }
/// ```
#[proc_macro_derive(PortableHash, attributes(portable_hash))]
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
        Data::Struct(x) => {
            // Reject any #[portable_hash] attributes on structs (they're only for enums).
            for attr in &input.attrs {
                if attr.path().is_ident("portable_hash") {
                    return Error::new(
                        attr.path().span(),
                        "`#[portable_hash(...)]` attributes are only supported on enums",
                    )
                    .to_compile_error()
                    .into();
                }
            }

            match x.fields {
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
            }
        }

        Data::Enum(x) => {
            // Phase 1: Parse enum-level configuration.
            let config = match parse_enum_attrs(&input.attrs) {
                Ok(c) => c,
                Err(e) => return e.to_compile_error().into(),
            };

            let width = effective_width(&config, &input.attrs);

            // Phase 2: Compute discriminant values for all variants.
            struct VariantInfo<'a> {
                variant: &'a syn::Variant,
                discriminant: i128,
            }

            let mut variant_infos = Vec::new();
            let mut next_rust_discriminant: i128 = 0;

            for variant in x.variants.iter() {
                let var_config = match parse_variant_attrs(&variant.attrs, &config) {
                    Ok(c) => c,
                    Err(e) => return e.to_compile_error().into(),
                };

                let discriminant: i128 = match config.mode {
                    DiscriminantMode::Name => {
                        if let Some((val, _)) = var_config.discriminant_override {
                            val as i128
                        } else {
                            let name = var_config.rename
                                .map(|(s, _)| s)
                                .unwrap_or_else(|| variant.ident.to_string());
                            fnv1a_64(name.as_bytes()) as i128
                        }
                    }
                    DiscriminantMode::Index => {
                        if let Some((val, _)) = var_config.discriminant_override {
                            // Manual #[portable_hash(discriminant = N)] takes priority.
                            let v = val as i128;
                            next_rust_discriminant = v + 1;
                            v
                        } else if let Some((_, ref expr)) = variant.discriminant {
                            // Explicit Rust discriminant: `Variant = 42`
                            let v = match parse_discriminant_expr(expr) {
                                Ok(v) => v,
                                Err(e) => return e.to_compile_error().into(),
                            };
                            next_rust_discriminant = v + 1;
                            v
                        } else {
                            // Auto-increment.
                            let v = next_rust_discriminant;
                            next_rust_discriminant = v + 1;
                            v
                        }
                    }
                };

                variant_infos.push(VariantInfo {
                    variant,
                    discriminant,
                });
            }

            // Phase 3: Check uniqueness of discriminant values.
            {
                let mut sorted: Vec<(i128, &Ident)> = variant_infos
                    .iter()
                    .map(|vi| (vi.discriminant, &vi.variant.ident))
                    .collect();
                sorted.sort_by_key(|(val, _)| *val);
                for window in sorted.windows(2) {
                    if window[0].0 == window[1].0 {
                        return Error::new(
                            Span::call_site(),
                            format!(
                                "portable_hash discriminant collision: variants `{}` and `{}` \
                                 both have discriminant value {}",
                                window[0].1, window[1].1, window[0].0
                            ),
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }

            // Phase 4: Generate match arms.
            let mut variant_tokens = TokenStream::new();

            for vi in &variant_infos {
                let var = &vi.variant.ident;

                let (disc_method, disc_lit) = match disc_write_tokens(width, vi.discriminant, var.span()) {
                    Ok(t) => t,
                    Err(e) => return e.to_compile_error().into(),
                };

                match &vi.variant.fields {
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
                            Self::#var { #(#fields),* } => {
                                state.#disc_method(#disc_lit);
                                #( #hash::portable_hash(#fields, state); )*
                            }
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
                                format_ident!("_{}", i)
                            })
                            .collect();
                        quote! {
                            Self::#var(#(#fields),*) => {
                                state.#disc_method(#disc_lit);
                                #( #hash::portable_hash(#fields, state); )*
                            }
                        }
                            .to_tokens(&mut variant_tokens);
                    }

                    Fields::Unit => quote! {
                        Self::#var => {
                            state.#disc_method(#disc_lit);
                        },
                    }
                        .to_tokens(&mut variant_tokens),
                }
            }

            quote! {
                match self {
                    #variant_tokens
                }
            }
                .to_tokens(&mut tokens);
        }

        Data::Union(_) => {
            return Error::new(ident.span(), "can't derive `PortableHash` for union")
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

// ---------------------------------------------------------------------------
// Generics helpers
// ---------------------------------------------------------------------------

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
    '_,
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
