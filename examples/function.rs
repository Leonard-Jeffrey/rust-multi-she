use rust_she::*;

fn main() {
    let param = SHE::key_gen_param(2048, 40, 160);
    let (pk, pp) = param.key_generation();

    // Encryption test
    let c1: BigInt = SHE::encrypt(&pk, &pp, 1000);

    // Scalar mul test
    let r1 = BigInt::sample_below(&BigInt::from(2^1000));
    let r_mul_c = &r1 * &c1;
    //let r_mul_c = SHE::s_Mul(&pp, &c1, 100);

    // Cipher mul test
    let c2: BigInt = SHE::encrypt(&pk, &pp, 1000);
    let c_mul_c = &c1 * &c2;

    // scalar add test
    let r2 = BigInt::sample_below(&BigInt::from(2^1000));
    let r_add_c = &r2 + &c1;

    // Cipher add test
    let c_add_c = &c1 + &c2;

    let ct1 = SHE::encrypt(&pk, &pp, 0);
    let ct2 = SHE::encrypt(&pk, &pp, 0);
    let ct3 = SHE::encrypt_in_public_key_setting(&pp, &ct1, &ct2, 1000);

    // Decryption test
    let m1 = SHE::decrypt(&pk, c1);
    let m2 = SHE::decrypt(&pk, c2);
    let m3 = SHE::decrypt(&pk, r_mul_c);
    let m4 = SHE::decrypt(&pk, c_mul_c);
    let m5 = SHE::decrypt(&pk, r_add_c);
    let m6 = SHE::decrypt(&pk, c_add_c);
    let m7 = SHE::decrypt(&pk, ct3);
    
    println!("m1 = {}, m2 = {}", m1, m2);
    println!("r1 * m1 = {} * {} = {}", r1, m1, m3);
    println!("m1 * m2 = {} * {} = {}", m1, m2, m4);
    println!("r2 + m1 = {} + {} = {}", r2, m1, m5);
    println!("m1 + m2 = {} + {} = {}", m1, m2, m6);
    println!("1 + 0 + 0 = {}", m7);
    
}