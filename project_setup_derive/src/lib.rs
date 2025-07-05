use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};
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
