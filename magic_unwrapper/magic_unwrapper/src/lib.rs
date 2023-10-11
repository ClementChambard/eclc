extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

trait ToSnakeCase {
    fn to_snake_case(&self) -> String;
}

impl ToSnakeCase for String {
    fn to_snake_case(&self) -> String {
        let mut result = String::new();
        let mut last_char_was_upper = false;

        for c in self.chars() {
            if c.is_uppercase() {
                if !last_char_was_upper {
                    if !result.is_empty() {
                        result.push('_');
                    }
                }
                result.push(c.to_lowercase().next().unwrap());
                last_char_was_upper = true;
            } else {
                result.push(c);
                last_char_was_upper = false;
            }
        }

        result
    }
}

#[proc_macro_derive(EnumUnwrap)]
pub fn enum_unwrap(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    if let Data::Enum(data_enum) = &ast.data {
        let enum_name = &ast.ident;

        let mut generated_methods = quote! {};

        for variant in &data_enum.variants {
            let variant_name = &variant.ident;
            let method_name = variant_name.to_string().to_snake_case();
            let method_name = Ident::new(&method_name, variant_name.span());
            if let Fields::Unnamed(fields) = &variant.fields {
                let method_name2 = Ident::new(
                    &format!("{}_or", method_name.to_string()),
                    variant_name.span(),
                );
                if fields.unnamed.is_empty() {
                    continue;
                }
                if fields.unnamed.len() == 1 {
                    let unnamed = &fields.unnamed;
                    generated_methods.extend(quote! {
                        pub fn #method_name(self) -> #unnamed {
                            if let Self::#variant_name(inner) = self {
                                inner
                            } else {
                                panic!();
                            }
                        }

                        pub fn #method_name2<T>(self, err: T) -> Result<#unnamed, T> {
                            if let Self::#variant_name(inner) = self {
                                Ok(inner)
                            } else {
                                Err(err)
                            }
                        }
                    });
                } else {
                    let mut inner = quote! { f0 };
                    for i in 1..fields.unnamed.len() {
                        let id = Ident::new(&format!("f{}", i), variant_name.span());
                        inner.extend(quote! { , #id });
                    }
                    generated_methods.extend(quote! {
                        pub fn #method_name(self) -> #fields {
                            if let Self::#variant_name(#inner) = self {
                                (#inner)
                            } else {
                                panic!();
                            }
                        }

                        pub fn #method_name2<T>(self, err: T) -> Result<#fields, T> {
                            if let Self::#variant_name(#inner) = self {
                                Ok((#inner))
                            } else {
                                Err(err)
                            }
                        }
                    });
                }
            }
        }

        let expanded = quote! {
            impl #enum_name {
                #generated_methods
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("EnumUnwrap macro can only be applied to enums.");
    }
}
