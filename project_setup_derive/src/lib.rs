use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{DeriveInput, LitStr, Meta, Token, parse_macro_input, parse_str};
#[proc_macro_derive(LoopableNumberedEnum, attributes(numbered_enum))]
pub fn numbered_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut loop_within: Option<usize> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("numbered_enum") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("loop_within") {
                    let value = meta.value()?;
                    let lit: syn::LitInt = value.parse()?;
                    loop_within = lit.base10_parse().ok();
                }
                Ok(())
            });
        }
    }
    let expanded = quote! {
        impl #name {
            pub fn num(&self) -> usize {
                self.to_usize().unwrap()
            }
            pub fn next(&self) -> #name {
                return #name::from_usize(self.next_index()).unwrap();
            }
            pub fn prev(&self) -> #name {
                return #name::from_usize(self.prev_index()).unwrap();
            }
            pub fn next_index(&self) -> usize {
                return (self.num() + 1) % #loop_within;
            }
            pub fn prev_index(&self) -> usize {
                return (self.num() + #loop_within - 1) % #loop_within;
            }
        }
    };
    TokenStream::from(expanded)
}
#[proc_macro_derive(RadioOption)]
pub fn radio_option_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl super::RadioOptionValue for #name {
            fn selectable(&self) -> bool {true}
        }
    };
    TokenStream::from(expanded)
}
#[proc_macro_derive(InnerState)]
pub fn inner_state_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl super::InnerState for #name {
            fn state(self) -> InnerCommonState {
                self.common_state
            }

            fn with_state(&mut self, state: InnerCommonState) -> &mut Self {
                self.common_state = state;
                self
            }
        }
    };
    TokenStream::from(expanded)
}
#[proc_macro_derive(EnumFunc, attributes(enum_func))]
pub fn enum_func_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let mut func_branches: HashMap<String, Vec<proc_macro2::TokenStream>> = HashMap::new();
    if let syn::Data::Enum(data_enum) = &input.data {
        for v in &data_enum.variants {
            let variant_name = &v.ident;
            for attr in &v.attrs {
                if !attr.path().is_ident("enum_func") {
                    continue;
                }
                if let Ok(nested) = attr.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated,
                ) {
                    for meta in nested {
                        if let Meta::List(ml) = meta {
                            let mut func_name: String = String::new();
                            if let Some(func_name_ident) = ml.path.get_ident() {
                                func_name = func_name_ident.to_string();
                            }
                            if !func_name.is_empty()
                                && let Ok(s) = ml.parse_args::<LitStr>()
                            {
                                let func_value = s.value();
                                func_branches
                                    .entry(func_name)
                                    .or_insert(Vec::new())
                                    .push(quote! {
                                        #enum_name::#variant_name => #func_value.to_string()
                                    });
                            }
                        }
                    }
                }
            }
        }
    }
    let generated_functions = func_branches.iter().map(|(func_name, branches)| {
        let func_name = parse_str::<proc_macro2::TokenStream>(func_name).unwrap();
        quote! {
            fn #func_name(self) -> String {
                match self {
                    #(#branches),*
                }
            }
        }
    });
    let expanded = quote! {
        impl #enum_name {
            #(#generated_functions)*
        }
    };
    TokenStream::from(expanded)
}
#[proc_macro_derive(ExecutableEnum, attributes(exe))]
pub fn executable_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("ExecutableEnum can only be derived for enums"),
    };
    let match_arms: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            // 查找 #[exe("...")] 属性
            let mut exe_value = None;
            for attr in &variant.attrs {
                if attr.path().is_ident("exe") {
                    let value = attr
                        .parse_args::<syn::LitStr>()
                        .expect("Expected string literal in #[exe] attribute");
                    exe_value = Some(value.value());
                    break;
                }
            }
            let exe_str = match exe_value {
                Some(s) => s,
                None => panic!("Variant {} is missing #[exe] attribute", variant_name),
            };
            quote! {
                #name::#variant_name => #exe_str,
            }
        })
        .collect();
    let expanded = quote! {
        impl crate::common::ExecutableEnumTrait for #name {
            fn exe(&self) -> String {
                match self {
                    #(#match_arms)*
                }.to_string()
            }
        }
    };
    TokenStream::from(expanded)
}
