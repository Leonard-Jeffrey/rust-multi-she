
// use std::borrow::{Borrow, Cow};
// use rayon::join;
use serde::{Serialize, Deserialize};

use crate::traits::*;
use curv::arithmetic::traits::*;

use crate::keygen::*;


use crate::{Multi_SHE, KeyGenParam, PriKey, PubParam, BigInt}; 


impl KeyGenParam {
    pub fn key_generation(&self) -> (PriKey, PubParam) {
        (PriKey::from(self), PubParam::from(self))
    }
}

impl<'KGP> From<&'KGP KeyGenParam> for PriKey {
    fn from(kgp: &'KGP KeyGenParam) -> Self {
        PriKey {
            p: kgp.p.clone(),
            q: kgp.q.clone(),
            L: kgp.L.clone(),
        }
    }
}

impl<'KGP> From<&'KGP KeyGenParam> for PubParam {
    fn from(kgp: &'KGP KeyGenParam) -> Self {
        PubParam {
            k0: kgp.k0,
            k1: kgp.k1,
            k2: kgp.k2,
            N :kgp.N.clone(),
        }
    }
}


impl Encryption<PriKey, PubParam, u128, BigInt> for Multi_SHE {
    fn encrypt(pk: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
        let pt = BigInt::from(pt as u64);
        let r = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
        let rr = BigInt::sample_below(&(BigInt::from(2).pow(pp.k0.clone() as u32)));
        BigInt::mod_mul(
            &(r * pk.L.clone() + pt),
            &(BigInt::from(1) + rr * pk.p.clone()),
            &pp.N
        )
        // (r*L + m)(1 + rr*p) % N
     }

    // The entire ABA type encryption
    // fn encrypt_with_chosen_user_aba(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
    //     let pt = BigInt::from(pt as u64);
    //     let r_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
    //     let rr_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k0.clone() as u32)));
        
    //     let r_2 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
        
    //     // BigInt::mod_mul(
    //     //     &((BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt)),
    //     //     &(BigInt::from(1) + rr_1 * pk1.p.clone()),
    //     //     &pp.N,
    //     // )
    //     let first_c = (r_1 * pk1.L.clone() + pt) * (BigInt::from(1) + rr_1 * pk1.p.clone());
    //     BigInt::mod_mul(
    //         &(BigInt::from(1) + r_2 * pk2.L.clone()), 
    //         &first_c, 
    //         &(pp.N.clone()*pk2.L.clone()))
    //     // (1 + r2 * L2)(r1 * L1 + m)(1 + rr1 * p)
    // }



     // The entire AB type encryption
    fn encrypt_with_chosen_user_ab(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
        let pt = BigInt::from(pt as u64);
        let r_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
        let rr_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k0.clone() as u32)));
        
        let r_2 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
        
        // BigInt::mod_mul(
        //     &((BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt)),
        //     &(BigInt::from(1) + rr_1 * pk1.p.clone()),
        //     &pp.N,
        // )
        
        let first_c = (r_1 * pk1.L.clone() + pt) * (BigInt::from(1) + rr_1 * pk1.p.clone());
        BigInt::mod_mul(
            &(BigInt::from(1) + r_2 * pk2.L.clone()), 
            &first_c, 
            &(pp.N.clone()*pk2.L.clone()))
        // (1 + r2 * L2)(r1 * L1 + m)(1 + rr1 * p) % N*L2
    }


    fn encrypt_with_chosen_user_ab_prerandom(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, pt: &BigInt, r_1: &BigInt, rr_1: &BigInt, r_2: &BigInt) -> BigInt{
        let first_c = (r_1 * pk1.L.clone() + pt) * (BigInt::from(1) + rr_1 * pk1.p.clone());
        BigInt::mod_mul(
            &(BigInt::from(1) + r_2 * pk2.L.clone()), 
            &first_c, 
            &(pp.N.clone()*pk2.L.clone()))
        // (1 + r2 * L2)(r1 * L1 + m)(1 + rr1 * p) % N*L2
    }

    // The first part of ABA encryption
    // fn encrypt_with_chosen_user_aba_I(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
    //     let pt = BigInt::from(pt as u64);
    //     let r_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
    //     let rr_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k0.clone() as u32)));
        
    //     let r_2 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
        
    //     // BigInt::mod_mul(
    //     //     &((BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt)),
    //     //     &(BigInt::from(1) + rr_1 * pk1.p.clone()),
    //     //     &pp.N,
    //     // )
    //     (BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt) * (BigInt::from(1) + rr_1 * pk1.p.clone())

    // }

    // // The second part of ABA encryption
    // fn encrypt_with_chosen_user_aba_II(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
    //     let pt = BigInt::from(pt as u64);
    //     let r_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
    //     let rr_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k0.clone() as u32)));
        
    //     let r_2 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
        
    //     // BigInt::mod_mul(
    //     //     &((BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt)),
    //     //     &(BigInt::from(1) + rr_1 * pk1.p.clone()),
    //     //     &pp.N,
    //     // )
    //     (BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt) * (BigInt::from(1) + rr_1 * pk1.p.clone())
    // }


    // The first part of AB encryption
    fn encrypt_with_chosen_user_ab_I(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
        let pt = BigInt::from(pt as u64);

        let r_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));

        let rr_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k0.clone() as u32)));
        
        //let r_2 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
        
        // BigInt::mod_mul(
        //     &((BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt)),
        //     &(BigInt::from(1) + rr_1 * pk1.p.clone()),
        //     &pp.N, 
        // )
        BigInt::mod_mul(&(r_1 * pk1.L.clone() + pt), &(BigInt::from(1) + rr_1 * pk1.p.clone()), &pp.N)
        // (m + r1 * L1)(1 + rr1 * p) % N 
    }

    // The second part of AB encryption
    fn encrypt_with_chosen_user_ab_II(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, ct: &BigInt) -> BigInt {
        //let pt = BigInt::from(pt as u64);

        // let r_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));

        // let rr_1 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k0.clone() as u32)));
        
        let r_2 = BigInt::sample_below(&(BigInt::from(2).pow(pp.k2.clone() as u32)));
        
        // BigInt::mod_mul(
        //     &((BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt)),
        //     &(BigInt::from(1) + rr_1 * pk1.p.clone()),
        //     &pp.N,
        // )
        // (BigInt::from(1) + r_2 * pk2.L.clone()) * (r_1 * pk1.L.clone() + pt) * (BigInt::from(1) + rr_1 * pk1.p.clone())
        
        BigInt::mod_mul(
            &(BigInt::from(1) + r_2 * &pp.N),
            ct,
            &(&pp.N * &pk2.L)
        )

        // (1 + r2 * L2) * ct % N*L2
        // (1 + r2 * L2)(m + r1 * L1)(1 + rr1 * p) % N*L2 
    }

    // The she encryption in public-key setting
    fn encrypt_in_public_key_setting(pp: &PubParam, ct1: &BigInt, ct2: &BigInt, pt: u128) -> BigInt {
        let r1 = BigInt::sample_below(&BigInt::from(2).pow(pp.k2.clone() as u32));
        let r2 = BigInt::sample_below(&BigInt::from(2).pow(pp.k2.clone() as u32));
        //let ct = BigInt::from(pt as u64) + r1*ct1 + r2*ct2;
        let ct = BigInt::from(pt as u64) + r1 * ct1 + r2 * ct2;
        ct
    }

    fn encrypt_in_public_key_setting_with_prerandom(PP: &PubParam, ct1: &BigInt, ct2: &BigInt, pt: &BigInt, r1: &BigInt, r2: &BigInt) -> BigInt {
        let ct = pt + r1 * ct1 + r2 * ct2;
        ct
    }


}

impl Decryption<PriKey, BigInt, BigInt> for Multi_SHE {
    // A-B-A

    // fn decrypt_with_chosen_user_aba(pk1: &PriKey, pk2: &PriKey, ct: &BigInt) -> BigInt {
    //     ((ct % pk1.p.clone()) % pk2.L.clone()) % pk1.L.clone()
    // }

    // //
    // fn decrypt_with_chosen_user_aba_I(pk1: &PriKey, pk2: &PriKey, ct: &BigInt) -> BigInt {
    //     ((ct % pk1.p.clone()) % pk2.L.clone()) % pk1.L.clone()
    // }

    // //
    // fn decrypt_with_chosen_user_aba_II(pk1: &PriKey, pk2: &PriKey, ct: &BigInt) -> BigInt {
    //     ((ct % pk1.p.clone()) % pk2.L.clone()) % pk1.L.clone()
    // }


    // A-B
    fn decrypt_with_chosen_user_ab(pk1: &PriKey, pk2: &PriKey, ct: &BigInt) -> BigInt {
        ((ct % pk2.L.clone()) % pk1.p.clone()) % pk1.L.clone()
    }

    //
    fn decrypt_with_chosen_user_ab_I(pk2: &PriKey, ct: &BigInt) -> BigInt {
        ct % pk2.L.clone()
    }

    //
    fn decrypt_with_chosen_user_ab_II(pk1: &PriKey, ct: &BigInt) -> BigInt {
        (ct % pk1.p.clone()) % pk1.L.clone()
    }
}


impl Homomorphism<PriKey, PubParam, BigInt, usize> for Multi_SHE {
    fn s_Add(pp: &PubParam, ct: &BigInt, scalar: usize) -> BigInt {
        let s = BigInt::from(scalar as u64);
        (&s + ct) % pp.N.clone()
        //BigInt::mod_add(&s, &ct,&pp.N)
    }

    fn c_Add(pp: &PubParam, ct1: &BigInt, ct2: &BigInt) -> BigInt {
        (ct1 + ct2) % pp.N.clone()
        //BigInt::mod_add(&ct1, &ct2, &pp.N)
    }

    fn s_Mul( pp: &PubParam, ct: &BigInt, scalar: usize) -> BigInt {
        let s = BigInt::from(scalar as u64);
        (&s * ct) % pp.N.clone()
        //BigInt::mod_mul(&s, ct, &pp.N)
    }

    fn c_Mul(pp: &PubParam, ct1: &BigInt, ct2: &BigInt) -> BigInt {
        (ct1 * ct2) % pp.N.clone()
        //BigInt::mod_mul(ct1, ct2, &pp.N)
    }

    fn Multi_sAdd(pk: &PriKey, pp: &PubParam, ct: &BigInt, scalar: usize) -> BigInt {
        let s = BigInt::from(scalar as u64);
        //s + ct
        BigInt::mod_add(&s, &ct, &(&pp.N * &pk.L))
    }

    fn Multi_cAdd(pk: &PriKey, pp: &PubParam, ct1: &BigInt, ct2: &BigInt) -> BigInt {
        //(ct1 + ct2) 
        BigInt::mod_add(&ct1, &ct2, &(&pp.N * &pk.L))
    }

    fn Multi_sMul(pk: &PriKey, pp: &PubParam, ct: &BigInt, scalar: usize) -> BigInt {
        let s = BigInt::from(scalar as u64);
        //(&s * ct) 
        BigInt::mod_mul(&s, ct, &(&pp.N * &pk.L))
    }

    fn Multi_cMul(pk: &PriKey, pp: &PubParam, ct1: &BigInt, ct2: &BigInt) -> BigInt {
        //(ct1 * ct2) 
        BigInt::mod_mul(ct1, ct2, &(&pp.N * &pk.L))
    }
}
