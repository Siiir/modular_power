mod app;

use {
    modular_power::pow_mod,
    app::read_vars,

    std::{
        io::{stdin,Read},
    },
    num_bigint::{BigUint},
};

/*
1. Greets user.
2. Prompts for input.
3. Prints algorithm output, possibly error.
4. Waits for terminating input.
*/
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