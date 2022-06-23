mod pow_mod{
    use {
        crate::pow_mod,

        num_bigint::{
            BigUint,
            RandBigInt,
            ToBigUint,
        },

        rand::{thread_rng,Rng},
    };

    #[cfg(not(u64_digit))]
    type BigDigit = u32;
    #[cfg(u64_digit)]
    type BigDigit = u64;

    #[inline]
    fn gen_small_rand_bu<R:Rng+?Sized>(rng: &mut R)->BigUint{
        let bit_q= rng.gen_range(0u64..=640);
        rng.gen_biguint(bit_q)
    }
    #[inline]
    fn gen_big_rand_bu<R:Rng+?Sized>(rng: &mut R)->BigUint{
        let bit_q= rng.gen_range(0u64..=6400);
        rng.gen_biguint(bit_q)
    }

    #[inline]
    fn u64_wrapped_pow_mod(base:u64, exp:u64, modulus:u64)->BigUint{
        pow_mod(base.to_biguint().unwrap(), &exp.to_biguint().unwrap(), &modulus.to_biguint().unwrap())
    }

    mod modulo_by_positive{
        use super::*;

        #[test]
        fn fixed() {
            assert_eq!(u64_wrapped_pow_mod(6,3,20), 16.to_biguint().unwrap());
            assert_eq!(u64_wrapped_pow_mod(0,0,46336401), 1.to_biguint().unwrap());
            assert_eq!(u64_wrapped_pow_mod(2345,0,7546336401), 1.to_biguint().unwrap());
            assert_eq!(u64_wrapped_pow_mod(0,435454,78665785), 0.to_biguint().unwrap());
        }

        #[test]
        fn rand(){
            for _ in 1..500{
                let mut rng = rand::thread_rng();
                let base= gen_small_rand_bu(&mut rng);
                let exp= gen_small_rand_bu(&mut rng);
                let modulus= BigDigit::from(1u32)+ gen_small_rand_bu(&mut rng);

                let expected= (&base).modpow(&exp, &modulus);
                let got= pow_mod(base, &exp, &modulus);

                assert_eq!(expected, got);
            }
        }
    }
    
    mod modulo_by_zero{
        use super::*;
        
        mod fixed{
            use super::*;

            #[test]
            #[should_panic(expected="attempt to divide by zero")]
            fn tc0() {
                u64_wrapped_pow_mod(6,3,0);
            }

            #[test]
            #[should_panic(expected="attempt to divide by zero")]
            fn tc1() {
                u64_wrapped_pow_mod(0,0,0);
            }

            #[test]
            #[should_panic(expected="attempt to divide by zero")]
            fn tc2() {
                u64_wrapped_pow_mod(0,24534,0);
            }
        }

        mod rand{
            use super::*;

            #[inline]
            fn gen_tcn_func_description(){
                let mut rng = thread_rng();
                let base= gen_big_rand_bu(&mut rng);
                let exp= gen_big_rand_bu(&mut rng);

                pow_mod(base, &exp, &BigDigit::from(0u32).to_biguint().unwrap());
            }

            #[test]
            #[should_panic(expected="attempt to divide by zero")]
            fn tc0() {
                gen_tcn_func_description();
            }

            #[test]
            #[should_panic(expected="attempt to divide by zero")]
            fn tc1() {
                gen_tcn_func_description();
            }

            #[test]
            #[should_panic(expected="attempt to divide by zero")]
            fn tc2() {
                gen_tcn_func_description();
            }
        }
    }
}