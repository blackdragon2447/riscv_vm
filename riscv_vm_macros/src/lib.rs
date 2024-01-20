use inst::inst_internal;
use proc_macro::TokenStream;

mod inst;

#[proc_macro]
pub fn inst(input: TokenStream) -> TokenStream {
    inst_internal(input)
}
