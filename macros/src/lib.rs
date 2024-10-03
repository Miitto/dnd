use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Error};

#[proc_macro_derive(SingleSerialize)]
pub fn single_serialize_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_single_serialize_macro(&ast)
}

fn impl_single_serialize_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let data = match &ast.data {
        syn::Data::Struct(d) => d,
        _ => {
            let span = match &ast.data {
                syn::Data::Enum(e) => e.enum_token.span,
                syn::Data::Union(u) => u.union_token.span,
                syn::Data::Struct(_) => unreachable!("Reached Nested Struct in SingleSerialize"),
            };
            return syn::Error::into_compile_error(Error::new(span, "Only Structs are supported"))
                .into();
        }
    };

    let fields = match &data.fields {
        syn::Fields::Named(f) => &f.named,
        _ => {
            return syn::Error::into_compile_error(Error::new(
                name.span(),
                "Only Support Named Fields",
            ))
            .into()
        }
    };

    if let Some(second) = fields.iter().nth(1) {
        return syn::Error::into_compile_error(Error::new(
            second.span(),
            "Only Structs with a single field are supported",
        ))
        .into();
    }

    let field = fields.first().unwrap();

    let field_name = match &field.ident {
        Some(i) => i,
        None => {
            return syn::Error::into_compile_error(Error::new(
                name.span(),
                "Only Support Named Fields",
            ))
            .into()
        }
    };

    let gen = quote! {
        #[automatically_derived]
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.#field_name)
            }
        }

        #[automatically_derived]
        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(Self { #field_name: s })
            }
        }
    };

    gen.into()
}
