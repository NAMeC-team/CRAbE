// blackboard_macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Slots)]
pub fn derive_slots(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let slots_name = syn::Ident::new(&format!("{}Slots", name), name.span());

    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        panic!("#[derive(Slots)] only works on structs");
    };

    let slot_fields = fields.iter().map(|f| {
        let fname = &f.ident;
        let fty = &f.ty;
        quote! { pub #fname: crate::behaviors::blackboard::Slot<#name, #fty> }
    });

    let slot_inits = fields.iter().map(|f| {
        let fname = &f.ident;
        quote! {
            #fname: crate::behaviors::blackboard::Slot::new(|s: &#name| &s.#fname, |s: &mut #name| &mut s.#fname)
        }
    });

    let expanded = quote! {
        pub struct #slots_name {
            #(#slot_fields),*
        }

        impl #slots_name {
            pub fn new() -> Self {
                Self {
                    #(#slot_inits),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Writers)]
pub fn writers_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let writers_name = syn::Ident::new(&format!("{}Writers", struct_name), struct_name.span());

    let fields = if let Data::Struct(data_struct) = &input.data {
        match &data_struct.fields {
            Fields::Named(named) => named.named.iter().collect::<Vec<_>>(),
            _ => panic!("Writers only works on structs with named fields"),
        }
    } else {
        panic!("Writers only works on structs");
    };

    // Collect into a Vec to avoid iterator move issues
    let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let expanded = quote! {
        #[derive(Copy, Clone)]
        pub struct #writers_name {
            #(pub #field_idents: Writer<#struct_name, #field_types>),*
        }

        impl #writers_name {
            pub const fn new() -> Self {
                Self {
                    #(
                        #field_idents: Writer(|parent: &mut #struct_name, value: #field_types| {
                            parent.#field_idents = value
                        }),
                    )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
