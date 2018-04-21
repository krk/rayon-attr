#![feature(proc_macro)]

extern crate proc_macro;

#[feature(full)]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::ExprForLoop;

/// Converts a for-loop to rayon parallel for_each.
///
/// Only valid on when applied to a for statement, with #![feature(stmt_expr_attributes)] present.
#[proc_macro_attribute]
pub fn parallel(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Return the input unchanged if it failed to parse. The compiler will show
    // the right diagnostics.
    let input: ExprForLoop =
        syn::parse(input.clone()).expect("parallel attribute may only be applied to for loops.");

    // TODO support input.label
    // TODO support input.attrs
    let expr = input.expr.clone().into_tokens();
    let body = input.body.clone().into_tokens();
    let pat = input.pat.clone().into_tokens();

    // TODO check for early returns in the for loop, they would be errors in the parallel version.
    let parallelized = quote!((#expr).into_par_iter().for_each(|#pat| #body));

    parallelized.to_string().parse().unwrap()
}
