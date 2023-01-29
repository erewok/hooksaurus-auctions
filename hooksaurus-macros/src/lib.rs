mod derive_to_form;

#[proc_macro_derive(ToForm)]
pub fn derive_to_form(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_to_form::derive_proc_macro_impl(input)
}
