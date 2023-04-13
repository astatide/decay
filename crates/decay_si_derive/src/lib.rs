extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use crate::quote::ToTokens;
use syn::{parse_macro_input, parse_quote, DeriveInput};

use proc_macro_error::proc_macro_error;

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

const SI: [(&str, f64); 25] = [
    ("Quetta", 30.0),
    ("Ronna", 27.0),
    ("Yotta", 24.0),
    ("Zetta", 21.0),
    ("Exa", 18.0),
    ("Peta", 15.0),
    ("Tera", 12.0),
    ("Giga", 9.0),
    ("Mega", 6.0),
    ("Kilo", 3.0),
    ("Hecto", 2.0),
    ("Deka", 1.0),
    ("One", 0.0),
    ("Deci", -1.0),
    ("Centi", -2.0),
    ("Milli", -3.0),
    ("Micro", -6.0),
    ("Nano", -9.0),
    ("Pico", -12.0),
    ("Femto", -15.0),
    ("Atto", -18.0),
    ("Zepto", -21.0),
    ("Yocto", -24.0),
    ("Ronto", -27.0),
    ("Quecto", -30.0),
];

#[proc_macro_error]
#[proc_macro_derive(SITypes)]
pub fn derive_SI(input: TokenStream) -> TokenStream {
    let mut output = String::new(); // we'll be adding everything into this.
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident; // basic name, such as "Meter"
    // let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let tt = ast.generics.type_params().next().unwrap(); // think f32 or f64
    let target = quote! { #tt };
    for (i, si_1) in SI.iter().enumerate() {
        // create the basic type.
        // don't forget the deref macro!
        output += "#[derive(SIDeref, PartialEq, Debug)]";
        output += format!("struct {}{}<{}>({}) where {}: FloatCore + Add + Mul + Sub + Div + std::ops::Mul<f64, Output = {}>;", si_1.0, name, target, target, target, target).as_str(); // ex: struct KiloMeter<f32>(f32);
        let si1_form = format!("{}{}<{}>", si_1.0, name, target);
        for (j, si_2) in SI.iter().enumerate() {
            if si_1.0 != si_2.0 {
                // good news!  create the add/mul/sub/divide types.
                let base: f64 = 10.0;
                let diff: f64 = (si_2.1 - si_1.1) as f64;
                let power_diff = base.powf(diff) as f64;
                if (diff <= 6.0) {
                    let si2_form = format!("{}{}<{}>", si_2.0, name, target);
                    output += format!("impl<{}> Add<{}> for {} where {}: FloatCore + Add + Mul + Sub + Div + std::ops::Mul<f64, Output = {}> {{", target, si1_form.as_str(), si2_form.as_str(), target, target).as_str();
                    output += format!("type Output = {};", si2_form).as_str();
                    output += "";
                    output += format!("fn add(self, other: {}) -> {} {{", si1_form.as_str(), si2_form.as_str()).as_str();
                    // here's where we'd do some handling for types; honestly, the only ones we can handle are within one or two different prefixes.9
                    output += format!("{}{}::<{}>(self.0 + (other.0 * {:.1}))", si_2.0, name, target, power_diff as f64).as_str();
                    output += "}";
                    output += "}";
                }
                // impl<f32> Add for Meters<f32> {
                //     type Output = Meters<f32>;

                //     fn add(self, other: Meters<f32>) -> Self::Output {
                //         if std::f32::DIGITS >= (self.1 - other.1) {
                //             // there's enough significance to make it work.
                //             self
                //         }
                //         else {
                //             // not enough significance in the underlying float type to make a difference.
                //             self
                //         }
                //     } 
                // }
            }
        }
    }
    // 
    // let prefix = &tIter.next();
    // // let SIprefixes = syn::parse_str::4<TokenStream>("kilo");
    // let SIprefixes: [proc_macro2::TokenStream; 2] = [(String::from("kilo")+&name.to_string()).parse().unwrap(), (String::from("nano")+&name.to_string()).parse().unwrap()];
    // // let typeName = prefix + name;
    // let output = quote! {
    //     // struct prefix <#target> (#target, SI);
    //     #( struct #SIprefixes <#target>(#target, SI);
    //     impl #impl_generics SITypes for #SIprefixes #impl_generics #where_clause {
    //         type Target = SI;

    //         fn base(&self) -> &Self::Target {
    //             &self.1
    //         }
    //     }
    //     )*
    //     // struct #(#SIprefixes)* #name <#target> (#target, SI);
    // };

    // TokenStream::from(output)
    output.parse().unwrap()
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
