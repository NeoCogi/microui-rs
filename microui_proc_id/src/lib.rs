extern crate proc_macro;
use proc_macro::TokenStream;

static mut id_counter : usize = 0;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    unsafe {
        id_counter += 1;
        format!("{}", id_counter).parse().unwrap()
    }
}
