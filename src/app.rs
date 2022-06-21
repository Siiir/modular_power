use{
    std::{
        str::FromStr,
        fmt::Debug,

        io::{stdin},
        io::{Write, stdout},
    },

    num_traits::Zero,
};

/*Reads subsequent lines from stdin.
Attempts to parse each as `T`. On success returns `T` instance.
If fails due to incorrect format. Repeats attempt with next line.
Upon failure caused by stdin, returns None.
*/
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

/*
Prompts for inputing variable value.
Next, runs read_var function.
*/
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

/*Reads 3 generic values that are needed for pow_mod algorithm.*/
pub fn read_vars<K,N,M>()->Option<(K,N,M)>where
    K:FromStr, <K as FromStr>::Err: Debug,
    N:FromStr, <N as FromStr>::Err: Debug,
    M:FromStr, <M as FromStr>::Err: Debug, M:Zero
    {
    /*k Can be possibly extended to BigInt with some implied edits.*/
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