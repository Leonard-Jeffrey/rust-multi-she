use rust_multi_she::*;
use time::Instant;

fn main() {

    let n = 100000;
    println!("test for {} times", n);

    // keys generation
    let t0 = Instant::now();
    let param = Multi_SHE::KeyGenParam_with_length(2048, 20, 160);
    let (pk, pp) = param.key_generation();
    let t0 = t0.elapsed();

    // Encryption time test
    let t1 = Instant::now();
    for i in 1..n {
        let c1: BigInt = Multi_SHE::encrypt(&pk, &pp, i as u128);
    }
    let t1 = t1.elapsed();
 

    // Scalar multiplication time test
    let c1: BigInt = Multi_SHE::encrypt(&pk, &pp, 10000);
    let r = BigInt::sample_below(&BigInt::from(2^1000));
    let t2 = Instant::now();
    for i in 1..n {
        //let rc =  &r * &c1;
        let rc = Multi_SHE::s_Mul(&pp, &c1, 1234);
    }
    let t2 = t2.elapsed();


   
    let c2: BigInt = Multi_SHE::encrypt(&pk, &pp, 1000);
    let t3 = Instant::now();
    for i in 1..n {
        //let cc = &c1 * &c2;
        let cc = Multi_SHE::c_Mul(&pp, &c1, &c2);
    }
    let t3 = t3.elapsed();


    let t4 = Instant::now();
    for i in 1..n {
        //let r_c = BigInt::from(10000) + &c1;
        let r_c = Multi_SHE::s_Add(&pp, &c1, 1234);
    }
    let t4 = t4.elapsed();


    let t5 = Instant::now();
    for i in 1..n {
        //let c_c = &c1 + &c2;
        let c_c = Multi_SHE::c_Add(&pp, &c1, &c2);
    }
    let t5 = t5.elapsed();

 
    
    let t6 = Instant::now();
    for i in 1..n {
        let m1 = Multi_SHE::decrypt(&pk, c1.clone());
    }
    let t6 = t6.elapsed();

    println!("Keygen time：{}", t0/n);
    println!("Enc time: {}", t1/n);
    println!("Dec time: {}", t6/n);

    println!("Cipher mul time: {}", t3/n);
    println!("Scalar mul time: {}", t2/n);
    
    println!("Cipher add time: {}", t5/n);
    println!("Scalar add time: {}", t4/n);

}