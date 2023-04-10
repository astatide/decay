extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};
use crate::quote::ToTokens;

use proc_macro::TokenStream;

#[proc_macro_derive(SIDeref)]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let target = ast.generics.type_params().next();
    let output = quote! {
        impl #impl_generics std::ops::Deref for #name #impl_generics #where_clause {
            type Target = #target;
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    TokenStream::from(output)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
