// Algorithm optimized for 64-bit proccessor.

use {
    std::{
        fmt::Debug,
        io::{Read, stdin},
        io::{Write, stdout},
        str::FromStr,
        
        stringify,
    },
    num_traits::{Zero,One},
    num_bigint::{BigUint},
};

fn pow_mod(mut base:BigUint, exp:&BigUint, modulus:&BigUint)->BigUint{
    // Simplifying base
    base%= modulus;

    // The main algorithm.
    let mut y= BigUint::one();
    for mut u32_dig in exp.iter_u32_digits(){
        for _ in 0..32{
            if u32_dig&1==1{
                y*=&base; y%=modulus;
            }
            u32_dig>>=1;
            base=&base*&base; base%=modulus;
        }
    }
    return y;
}

fn read_var<T>(buf: &mut String)->Option<T> where T:FromStr, <T as FromStr>::Err: Debug{
    loop{
        // Reading from `stdin` to `buf`
        buf.clear();
        if let Err(e) = stdin().read_line(buf){
            println!("\nAn error cought while reading line from standard input: {:?}",e);
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
                println!("An error cought while flushing stdout buffer: {:?}",e);
            }
            $var= read_var(&mut $buf)?;
        }
    };
}

fn read_vars<K,N,M>()->Option<(K,N,M)>where
    K:FromStr, <K as FromStr>::Err: Debug,
    N:FromStr, <N as FromStr>::Err: Debug,
    M:FromStr, <M as FromStr>::Err: Debug, M:Zero
    {
    /*k Can be possibly extended to i128 with some implied edits.*/
    let (k, n, mut m):(K,N,M);
    let mut buf= String::new();
    
    read_var!(k, buf);
    read_var!(n, buf);
    loop {
        read_var!(m, buf);
        if !m.is_zero(){break;}
        print!(concat!(
            "Error:  m=0 is out of domain.\n",
            "Try again to enter a correct value.\n"
        ));
    }

    return Some((k, n, m));
}

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