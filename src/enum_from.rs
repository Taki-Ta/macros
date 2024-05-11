use proc_macro2::TokenStream;
use syn::DeriveInput;
use quote::quote;

pub fn process_enum_from(input:DeriveInput)->TokenStream{
    print!("{:#?}",input);
    let ident = input.ident;
    let gen=input.generics;
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {} 
                } else {
                    let field = fields.unnamed.first().expect("unreachable").clone();
                    let ty = &field.ty;
                    quote! {
                        impl #gen From<#ty> for #ident #gen {
                            fn from(v:#ty) -> Self{
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    quote! {
        #(#from_impls)*
    }
}