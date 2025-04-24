use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(FieldTypeInfo)]
pub fn derive_field_type_info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Extract the fields from the struct
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // Implementation of the trait
    let output = quote! {
        impl ::field_info::FieldTypeInfo for #struct_name {
            fn field_types() -> &'static [&'static str] {
                &[#(stringify!(#field_types)),*]
            }
        }
    };

    output.into()
}
