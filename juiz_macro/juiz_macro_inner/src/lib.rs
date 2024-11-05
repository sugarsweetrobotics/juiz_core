extern crate proc_macro;



#[proc_macro_attribute]
pub fn pte(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    pte_impl(attr.into(), item.into()).into()
}
