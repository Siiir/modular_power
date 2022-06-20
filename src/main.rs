// Algorithm optimized for 64-bit proccessor.

use std::{io::{Read, Stdin, stdin}};

fn pow_mod(base:u32, exp:u64, modulo:u32)->u32{
    let y= 1u64;
    let base= u64::from(base);
    let modulo= u64::from(modulo);
    loop{
        if exp&1==1{
            y= y*x%modulo;
        }
        exp>>=1;
        if exp==0{break;}
        x= x*x%modulo;
    }
    return y as u32;
}

fn read_var<T>(stdin:&mut Stdin, buf: &mut String, var: &mut T)->Some(()){
    buf.clear();
    if let Err(e) = stdin.read_line(&mut buf){
        println!("An OS error has occured: {e}");
        return None;
    }
    *var= match buf.parse(){
        Ok(v)=>v,
        Err(e)=>{
            println!("Value parsing error has occured: {e}");
            return None;
        },
    };
    return Some(());
}

fn read_vars(stdin:&mut Stdin)->Option<(u64,u64,u32)>{
    let (mut x, mut y, mut z):(i128,u64,u32);
    let mut buf= String::new();

    read_var(stdin, &mut buf, &mut x)?;
    read_var(stdin, &mut buf, &mut y)?;
    read_var(stdin, &mut buf, &mut z)?;


    return res
}

fn main() {
    let stdin= stdin();
 
    println!("Welcome to super fast `modular_power` algorithm by Tomasz Nehring.");
    println!("I'll calculate [k^n mod m] for you, where k∈Z, n∈N∪\{0}, m∈N.");
    println!("Now input value for these variables.");

    if let Some((k,n,m)) = read_vars(&mut stdin){
        println("\n{k}^{n} mod {m} = {}", pow_mod(k, n, m));
    }
    
    println!("\nPress \{ENTER} to terminate the program."); // add backslash ?
    stdin.bytes().next()
}
