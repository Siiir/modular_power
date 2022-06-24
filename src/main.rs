//! A console app to quickly calculate k^n mod m on arbitrary non-negative integers.

//! It is a demonstrational part of
//!  [modular_power project](https://github.com/Siiir/modular_power),
//! which also encompases library containing public `pow_mod` function,
//! that is responsible for calculations in this console application.

//! # Examples
//! | **With proper input.** |
//! ----------------------------
//! | Welcome to super fast `modular_power` algorithm by Tomasz Nehring. |
//! | I'll calculate [k^n mod m] for you, where: |
//! | k, n, m are integers and 0<=k, 0<=n, 0<m . |
//! | Now input values for these three variables. |
//! | k= 6 |
//! | n= 3 |
//! | m= 20 |
//! |  |
//! | 6 ^ 3 mod 20 = 16 |
//! | |
//! | Press {ENTER} to terminate the program. |



mod app;
mod lib;

use {
    lib::pow_mod,
    app::read_vars,

    std::{
        io::{stdin,Read},
    },
    num_bigint::{BigUint},
};

#[warn(missing_docs)]


/// 1. Greets user.
/// 2. Prompts for input.
/// 3. Prints algorithm output, possibly error.
/// 4. Waits for terminating input.
fn main() {
    type K= BigUint; type N= BigUint; type M= BigUint;
    println!("Welcome to super fast `modular_power` algorithm by Tomasz Nehring.");
    println!("I'll calculate [k^n mod m] for you, where:");
    println!("  k, n, m are integers and 0<=k, 0<=n, 0<m .");
    println!("Now input values for these three variables.");

    if let Some((k,n,m)) = read_vars::<K,N,M>(){
        print!("\n{} ^ {} mod {} = ", &k, &n, &m);
        print!("{}\n", pow_mod(k, &n, &m));
    }
    
    println!("\nPress {{ENTER}} to terminate the program.");
    stdin().bytes().next();
}