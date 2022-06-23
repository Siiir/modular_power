#[cfg(test)]
mod tests;

use{
    num_traits::{One},
    num_bigint::{BigUint},
};

/*
Returns (self ^ exponent) % modulus.
Panics if the modulus is zero.
*/
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