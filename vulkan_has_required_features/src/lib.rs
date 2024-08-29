use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data};

#[proc_macro_derive(CheckRequiredFeatures)]
pub fn check_required_features(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident;

    let Data::Struct(struct_data) = input.data else {
        unimplemented!("enums");
    };

    let feature_checks = struct_data.fields.iter().map(|field| {
        let Some(ident) = &field.ident else {
            unimplemented!("tuple structs");
        };
        quote!( if self.#ident == 0 && required_features.#ident == 1 {
            return false;
        })
    });

    let expanded = quote! {
        #[automatically_derived]
        impl CheckRequiredFeatures for #name {
            fn check_for_required_features(&self, required_features: &Self) -> bool {
                #(#feature_checks)*
                return true;
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromPhysicalDeviceFeatures)]
pub fn from_duplicated_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let Data::Struct(struct_data) = input.data else {
        unimplemented!("enums");
    };

    let field_copies = struct_data.fields.iter().map(|field| {
        let Some(ident) = &field.ident else {
            unimplemented!("tuple structs");
        };
        quote!(#ident: other.#ident,)
    });

    let expanded = quote! {
        #[automatically_derived]
        impl FromPhysicalDeviceFeatures for #name {
            fn from_duplicated_type(other: vk::PhysicalDeviceFeatures) -> Self{
                Self {
                    #(#field_copies)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromPhysicalDeviceFeatures12)]
pub fn from_duplicated_type12(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let Data::Struct(struct_data) = input.data else {
        unimplemented!("enums");
    };

    let field_copies = struct_data.fields.iter().map(|field| {
        let Some(ident) = &field.ident else {
            unimplemented!("tuple structs");
        };
        quote!(#ident: other.#ident,)
    });

    let expanded = quote! {
        #[automatically_derived]
        impl FromPhysicalDeviceFeatures12 for #name {
            fn from_duplicated_type(other: vk::PhysicalDeviceVulkan12Features) -> Self{
                Self {
                    #(#field_copies)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromPhysicalDeviceFeatures13)]
pub fn from_duplicated_type13(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let Data::Struct(struct_data) = input.data else {
        unimplemented!("enums");
    };

    let field_copies = struct_data.fields.iter().map(|field| {
        let Some(ident) = &field.ident else {
            unimplemented!("tuple structs");
        };
        quote!(#ident: other.#ident,)
    });

    let expanded = quote! {
        #[automatically_derived]
        impl FromPhysicalDeviceFeatures13 for #name {
            fn from_duplicated_type(other: vk::PhysicalDeviceVulkan13Features) -> Self{
                Self {
                    #(#field_copies)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
