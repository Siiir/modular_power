//! A library containing `pow_mod` function  that is synonym to
//! <a href="https://docs.rs/num-bigint/0.3.1/num_bigint/struct.BigUint.html#method.modpow">
//!     <code>num_bigint::BigUint::modpow</code>
//! </a> .
//!
//! It was build as a part of [modular_power project][repo],
//! which also contains a binary app that lanuches function on user input.
//!
//! [repo]:https://github.com/Siiir/modular_power

#[cfg(test)]
mod tests;

use{
    num_traits::{One},
    num_bigint::{BigUint},
};

#[warn(missing_docs)]


/// Returns _(self ^ exponent) mod modulus_ .
///
/// Assuming that <i>self ^ exponent</i> is much bigger then
///   <i>2^`usize::BITS`</i> and <var>modulus</var>,
/// this function will be faster than performing respectively exponentation and then modulo. 
///
/// # Examples
/// ```
/// fn example()->Option<()>{
///     use {
///         num_bigint::ToBigUint,
///         modular_power::pow_mod,
///     };
///
///     let base= 4u32.to_biguint()?;
///     let exp= 2u32.to_biguint()?;
///     let modulus= 10u32.to_biguint()?;
///     assert_eq!( pow_mod(base,&exp,&modulus), 6u32.to_biguint()? );
///
///     Some(())
/// }
/// ```
///
/// # Panics
/// Function panics with `"attempt to divide by zero"` if _modulus_ is zero.
///
/// # Details
/// Synonymous function:
/// <a href="https://docs.rs/num-bigint/0.3.1/num_bigint/struct.BigUint.html#method.modpow">
///     <code>num_bigint::BigUint::modpow</code>
/// </a>

pub fn pow_mod(mut base:BigUint, exp:&BigUint, modulus:&BigUint)->BigUint{
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