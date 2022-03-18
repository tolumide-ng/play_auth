use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn impl_jwt_helper(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl JwtHelper for #name {
            fn get_user(&self) -> uuid::Uuid {
                return self.user_id;
            }
        }
    };

    gen.into()
}