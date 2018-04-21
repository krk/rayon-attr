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

/*
Unit test panics, possibly because we are a proc-macro crate:

---- tests::for_loop stdout ----
        thread 'tests::for_loop' panicked at 'proc_macro::__internal::with_sess() called before set_parse_sess()!', libproc_macro\lib.rs:898:9
stack backtrace:
   0: std::rt::lang_start_internal
   1: std::sys::windows::c::TryAcquireSRWLockShared
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: std::panicking::rust_panic_with_hook
   5: proc_macro::__internal::CURRENT_SESS::__getit
   6: proc_macro::__internal::in_sess
   7: <proc_macro::TokenStream as core::str::FromStr>::from_str
   8: core::str::{{impl}}::parse<proc_macro::TokenStream>
             at C:\projects\rust\src\libcore\str\mod.rs:2534
   9: alloc::str::{{impl}}::parse<proc_macro::TokenStream>
             at C:\projects\rust\src\liballoc\str.rs:1798
  10: rayon_attr::tests::for_loop
             at .\src\lib.rs:44
  11: rayon_attr::__test::TESTS::{{closure}}
             at .\src\lib.rs:43
  12: core::ops::function::FnOnce::call_once<closure,()>
             at C:\projects\rust\src\libcore\ops\function.rs:223
  13: <unknown>
  14: _rust_maybe_catch_panic
  15: test::stats::winsorize
  16: <test::TestOpts as core::fmt::Debug>::fmt
  17: _rust_maybe_catch_panic
  18: test::stats::winsorize
  19: std::sync::mpsc::blocking::WaitToken::wait_max_until
  20: std::sys::windows::thread::Thread::new
  21: BaseThreadInitThunk
  22: RtlUserThreadStart

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro::TokenStream;

    #[test]
    fn for_loop() {
        let input = quote!{
            for x in 0..100 {
                println!("{}", x);
            }
        }.to_string().parse().unwrap();

        let transformed = parallel(TokenStream::empty(), input);
        
        let output = format!("{}", transformed);

        assert_eq!(output, "");
    }
}
*/
