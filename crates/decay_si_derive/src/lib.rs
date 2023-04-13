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

// https://www.nist.gov/pml/owm/metric-si-prefixes
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

const OPS: [(&str, &str, &str); 4] = [("Add", "add", "+"), ("Sub", "sub", "-"), ("Mul", "mul", "*"), ("Div", "div", "/")];
const OPS_ASSIGN: [(&str, &str, &str); 4] = [("AddAssign", "add_assign", "+"), ("SubAssign", "sub_assign", "-"), ("MulAssign", "mul_assign", "*"), ("DivAssign", "div_assign", "/")];

#[proc_macro_error]
#[proc_macro_derive(SITypes)]
pub fn derive_SI(input: TokenStream) -> TokenStream {
    let mut output = String::new(); // we'll be adding everything into this.
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident; // basic name, such as "Meter"
    let tt = ast.generics.type_params().next().unwrap(); // think f32 or f64
    let target = quote! { #tt };
    let where_clause = format!("where {}: FloatCore + Add + Mul + Sub + Div + std::ops::Mul<f64, Output = {}>", target, target);
    for (i, si_1) in SI.iter().enumerate() {
        // create the basic type.
        // don't forget the deref and other macros!
        output += "#[derive(SIDeref, PartialEq, Debug, Copy, Clone)]";
        output += format!("struct {}{}<{}>({}) {};", si_1.0, name, target, target, where_clause).as_str(); // ex: struct KiloMeter<f32>(f32);
        let si1_form = format!("{}{}<{}>", si_1.0, name, target);
        for (j, si_2) in SI.iter().enumerate() {
            {
                // good news!  create the add/mul/sub/divide types.
                let base: f64 = 10.0;
                let diff: f64 = (si_2.1 - si_1.1);
                let power_diff = base.powf(diff) as f64;
                let t_diff = if format!("{target}") == "f64" {
                    f64::DIGITS
                } else if format!("{target}") == "f32" {
                    f32::DIGITS
                } else if format!("{target}") == "f16" {
                    3
                } else {
                    7
                };
                if (diff <= t_diff.into()) {
                    let si2_form = format!("{}{name}<{target}>", si_2.0);
                    for (k, op) in OPS.iter().enumerate() {
                        let op_name = op.0;
                        let op_nlow = op.1;
                        let op_symb = op.2;
                        output += format!("impl<{target}> {op_name}<{si1_form}> for {si2_form} {where_clause} {{").as_str();
                        output += format!("type Output = {si2_form};").as_str();
                        output += format!(
                            "fn {op_nlow}(self, other: {si1_form}) -> {si2_form} {{"
                        )
                        .as_str();
                        // here's where we'd do some handling for types; honestly, the only ones we can handle are within one or two different prefixes.
                        output += format!(
                            "{}{name}::<{target}>(self.0 {op_symb} (other.0 * {power_diff:.6}))",
                            si_2.0
                        )
                        .as_str();
                        output += "} }";
                    }
                    for (k, op) in OPS_ASSIGN.iter().enumerate() {
                        let op_name = op.0;
                        let op_nlow = op.1;
                        let op_symb = op.2;
                        output += format!("impl<{target}> {op_name}<{si1_form}> for {si2_form} {where_clause} {{").as_str();
                        output += format!(
                            "fn {op_nlow}(&mut self, other: {si1_form}) {{"
                        )
                        .as_str();
                        // here's where we'd do some handling for types; honestly, the only ones we can handle are within one or two different prefixes.
                        output += format!("*self = Self {{").as_str();
                        output += format!("0: self.0 {op_symb} (other.0 * {power_diff:.6})").as_str();
                        output += "}; } }";
                    }
                }
            }
        }
    }
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
