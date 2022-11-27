extern crate proc_macro;
use proc_macro::TokenStream;

static mut ID_COUNTER: usize = 0;

#[proc_macro]
pub fn id(_item: TokenStream) -> TokenStream {
    unsafe {
        ID_COUNTER += 1;
        format!("{}", ID_COUNTER << 16).parse().unwrap()
    }
}
