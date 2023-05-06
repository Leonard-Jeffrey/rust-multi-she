use rust_she::*;
use time::Instant;

fn main() {

    let n = 1000;

    println!("============== ab encryption test ==============");
    // Key param generation
    let (param1, param2) = SHE::key_gen_param_with_chosen_user_ab(2048, 40, 160);
    let (pk1, pp1) = param1.key_generation();
    let (pk2, pp2) = param2.key_generation();

    // Encryption time
    let t1 = Instant::now();
    for i in 1..n {
        SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, 1);
    }
    let t1 = t1.elapsed();

    let c1 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, 1);
    
    // Decryption time
    let t2 = Instant::now();
    for i in 1..n {
        let m1 = SHE::decrypt_with_chosen_user_ab(&pk1, &pk2, &c1);
    }
    let t2 = t2.elapsed();

    let c1 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, 123);
    let c2 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, 124);
    
    // Homomorphic add time
    let t3 = Instant::now();
    let c = &c1 + &c2;
    let t3 = t3.elapsed();
    let m = SHE::decrypt_with_chosen_user_ab(&pk1, &pk2, &c);

    println!("Enc_time = {}", t1/n);
    println!("Dec_time = {}", t2/n);
    println!("c_Add time = {}", t3);

    println!("m = {}", m);


    println!("============== aba encryption test ==============");
    // Key param generation
    let (param1, param2) = SHE::key_gen_param_with_chosen_user_aba(2048, 40, 160);
    let (pk1, pp1) = param1.key_generation();
    let (pk2, pp2) = param2.key_generation();

    // Encryption time
    let t1 = Instant::now();
    for i in 1..n {
        SHE::encrypt_with_chosen_user_aba(&pk1, &pk2, &pp1, 1);
    }
    let t1 = t1.elapsed();

    let c1 = SHE::encrypt_with_chosen_user_aba(&pk1, &pk2, &pp1, 1);
    
    // Decryption time
    let t2 = Instant::now();
    for i in 1..n {
        let m1 = SHE::decrypt_with_chosen_user_aba(&pk1, &pk2, &c1);
    }
    let t2 = t2.elapsed();

    let c1 = SHE::encrypt_with_chosen_user_aba(&pk1, &pk2, &pp1, 123);
    let c2 = SHE::encrypt_with_chosen_user_aba(&pk1, &pk2, &pp1, 124);
    
    // Homomorphic add time
    let t3 = Instant::now();
    let c = &c1 + &c2;
    let t3 = t3.elapsed();
    let m = SHE::decrypt_with_chosen_user_aba(&pk1, &pk2, &c);

    println!("Enc_time = {}", t1/n);
    println!("Dec_time = {}", t2/n);
    println!("c_Add time = {}", t3);

    println!("m = {}", m);


    println!("============== pk encryption test ==============");
    // keygeneration
    let (param1, param2) = SHE::key_gen_param_with_chosen_user_ab(2048, 40, 160);
    let (pk1, pp1) = param1.key_generation();
    let (pk2, pp2) = param2.key_generation();

    // public key generation
    let c0_1 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, 0);
    let c0_2 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, 0);
    let c1 = SHE::encrypt_in_public_key_setting(&pp1, &c0_1, &c0_2, 1000);
    let c2 = SHE::encrypt_in_public_key_setting(&pp1, &c0_1, &c0_2, 10);

    // decryption
    let m1 = SHE::decrypt_with_chosen_user_ab(&pk1, &pk2, &c1);
    let m2 = SHE::decrypt_with_chosen_user_ab(&pk1, &pk2, &c2);
    let m3 = SHE::decrypt_with_chosen_user_ab(&pk1, &pk2, &(&c1 + &c2));

    println!("m1 = {}", m1);
    println!("m2 = {}", m2);
    println!("m3 = {}", m3);


    println!("============== pk encryption test ==============");
    // keygeneration
    let param = SHE::key_gen_param(2048, 40, 160);
    let (pk1, pp1) = param.key_generation();

    // encryption
    let c1 = SHE::encrypt(&pk1, &pp1, 0);
    let c2 = SHE::encrypt(&pk1, &pp1, 0);
    let c3 = SHE::encrypt_in_public_key_setting(&pp1, &c1, &c2, 100);

    let m = SHE::decrypt(&pk1, c3);
    println!("m = {}", m);

}