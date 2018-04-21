#![feature(proc_macro)]
#![feature(stmt_expr_attributes)]

extern crate rayon;
extern crate rayon_attr;

use rayon::prelude::*;
use rayon_attr::parallel;

#[test]
fn for_loop() {
    #[parallel]
    for x in 0..10 {
        println!("{}", x);
    }
}
