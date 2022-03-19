extern crate proc_macro;

mod jwt;

use jwt::impl_jwt_helper;
use proc_macro::TokenStream;
use syn;


#[proc_macro_derive(JwtHelper)]
pub fn jwt_helper_deriver(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_jwt_helper(&ast)
}
