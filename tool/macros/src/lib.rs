use proc_macro::TokenStream;

mod modules;

#[proc_macro]
pub fn modules(input: TokenStream) -> TokenStream {
    modules::modules(input)
}
