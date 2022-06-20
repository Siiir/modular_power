// Algorithm optimized for 64-bit proccessor.

use std::{
    fmt::Debug,
    io::{Read, stdin},
    io::{Write, stdout},
    str::FromStr,

    stringify,
};

fn pow_mod(base:u32, mut exp:u64, modulo:u32)->u32{
    // Shadowing variables with incompatible constraint types.
    let mut base= u64::from(base);
    let modulo= u64::from(modulo);

    // The main algorithm.
    let mut y= 1u64;
    loop{
        if exp&1==1{
            y= y*base%modulo;
        }
        exp>>=1;
        if exp==0{break;}
        base= base*base%modulo;
    }
    return y as u32;
}

fn read_var<T>(buf: &mut String)->Option<T> where T:FromStr, <T as FromStr>::Err: Debug{
    loop{
        // Reading from `stdin` to `buf`
        buf.clear();
        if let Err(e) = stdin().read_line(buf){
            println!("\nAn error cought while reading line from standard input: {e:?}");
            return None;
        }

        // Correcting input.
        buf.pop();
        if buf.as_bytes()[buf.len()-1]==b'\r'{buf.pop();}

        // Returning parsed input
        return match buf.parse(){
            Ok(v)=>Some(v),
            Err(e)=>{
                println!(
                    concat!(
                        "An error has occured while parsing your input: {:?}",
                        "\nNote: app input parser (Rust str::parse::<{}>)",
                        " does not allow some number formats.",
                        "\nTry again to type value correctly. Remember that variable domain maters.",
                    ),
                    e, std::any::type_name::<T>(),
                );
                continue
            },
        };
    }
}

macro_rules! read_var {
    ($var:ident, $buf:expr) => {
        {
            print!(concat!(stringify!($var), "= "));
            if let Err(e) = stdout().flush(){
                println!("An error cought while flushing stdout buffer: {e:?}");
            }
            $var= read_var(&mut $buf)?;
        }
    };
}

fn read_vars()->Option<(u32,u64,u32)>{
    /*k Can be possibly extended to i128 with some implied edits.*/
    let (k, n, mut m):(u128,u64,u32);
    let mut buf= String::new();

    read_var!(k, buf);
    read_var!(n, buf);
    loop {
        read_var!(m, buf);
        if m!=0{break;}
        print!(concat!(
            "Error:  m=0 is out of domain.\n",
            "Try again to enter a correct value.\n"
        ));
    }
    
    // Simplify inputed values.
    let k= k.rem_euclid(m.into()) as u32;

    return Some((k, n, m));
}

fn main() {
    println!("Welcome to super fast `modular_power` algorithm by Tomasz Nehring.");
    println!("I'll calculate [k^n mod m] for you, where k, n, m are integers, k>=0, n>=0, m>0.");
    println!("Now input values for these three variables.");

    if let Some((k,n,m)) = read_vars(){
        println!("\nk^n mod m = {}", pow_mod(k, n, m));
    }
    
    println!("\nPress {{ENTER}} to terminate the program.");
    stdin().bytes().next();
}