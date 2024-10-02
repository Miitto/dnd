use proc_macro::TokenStream;
use quote::quote;

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
        _ => panic!("Only structs are supported"),
    };

    let fields = match &data.fields {
        syn::Fields::Named(f) => &f.named,
        _ => panic!("Only named fields are supported"),
    };

    if fields.len() != 1 {
        panic!("Only structs with a single field are supported");
    }

    let field = match fields.first() {
        Some(f) => f,
        None => panic!("Struct must have a single field"),
    };

    let field_name = match &field.ident {
        Some(i) => i,
        None => panic!("Struct field must have a name"),
    };

    let gen = quote! {
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.#field_name)
            }
        }

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

    dbg!(gen.to_string());

    gen.into()
}
