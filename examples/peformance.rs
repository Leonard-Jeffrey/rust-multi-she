<<<<<<< HEAD
use rust_multi_she::*;
=======
use rust_she::*;
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
use time::Instant;

fn main() {

    let n = 100000;
    println!("test for {} times", n);

    // keys generation
    let t0 = Instant::now();
<<<<<<< HEAD
    let param = Multi_SHE::KeyGenParam_with_length(2048, 20, 160);
=======
    let param = SHE::KeyGenParam_with_length(2048, 20, 160);
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
    let (pk, pp) = param.key_generation();
    let t0 = t0.elapsed();

    // Encryption time test
    let t1 = Instant::now();
    for i in 1..n {
<<<<<<< HEAD
        let c1: BigInt = Multi_SHE::encrypt(&pk, &pp, i as u128);
=======
        let c1: BigInt = SHE::encrypt(&pk, &pp, i as u128);
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
    }
    let t1 = t1.elapsed();
 

    // Scalar multiplication time test
<<<<<<< HEAD
    let c1: BigInt = Multi_SHE::encrypt(&pk, &pp, 10000);
=======
    let c1: BigInt = SHE::encrypt(&pk, &pp, 10000);
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
    let r = BigInt::sample_below(&BigInt::from(2^1000));
    let t2 = Instant::now();
    for i in 1..n {
        //let rc =  &r * &c1;
<<<<<<< HEAD
        let rc = Multi_SHE::s_Mul(&pp, &c1, 1234);
=======
        let rc = SHE::s_Mul(&pp, &c1, 1234);
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
    }
    let t2 = t2.elapsed();


   
<<<<<<< HEAD
    let c2: BigInt = Multi_SHE::encrypt(&pk, &pp, 1000);
    let t3 = Instant::now();
    for i in 1..n {
        //let cc = &c1 * &c2;
        let cc = Multi_SHE::c_Mul(&pp, &c1, &c2);
=======
    let c2: BigInt = SHE::encrypt(&pk, &pp, 1000);
    let t3 = Instant::now();
    for i in 1..n {
        //let cc = &c1 * &c2;
        let cc = SHE::c_Mul(&pp, &c1, &c2);
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
    }
    let t3 = t3.elapsed();


    let t4 = Instant::now();
    for i in 1..n {
        //let r_c = BigInt::from(10000) + &c1;
<<<<<<< HEAD
        let r_c = Multi_SHE::s_Add(&pp, &c1, 1234);
=======
        let r_c = SHE::s_Add(&pp, &c1, 1234);
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
    }
    let t4 = t4.elapsed();


    let t5 = Instant::now();
    for i in 1..n {
        //let c_c = &c1 + &c2;
<<<<<<< HEAD
        let c_c = Multi_SHE::c_Add(&pp, &c1, &c2);
=======
        let c_c = SHE::c_Add(&pp, &c1, &c2);
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
    }
    let t5 = t5.elapsed();

 
    
    let t6 = Instant::now();
    for i in 1..n {
<<<<<<< HEAD
        let m1 = Multi_SHE::decrypt(&pk, c1.clone());
=======
        let m1 = SHE::decrypt(&pk, c1.clone());
>>>>>>> 69757af4c2ede6a20930e0858307ee7808fde90f
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