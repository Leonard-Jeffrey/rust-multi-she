use rust_she::*;
use time::Instant;

fn main() {

    let n = 10;

    let k0 = 2048;
    let k1 = 40;
    let k2 = 160;

    // 测量密钥生成时间
    let key_gen_time = Instant::now();
    // 生成 DSHE 参数
    
    for i in 0..1 {
        let (param1, param2) = SHE::key_gen_param_with_chosen_user_ab(k0, k1, k2); 
        let (pk1, pp1) = param1.key_generation();
        let (pk2, pp2) = param2.key_generation();
    }
    let key_gen_time = key_gen_time.elapsed();

    let (param1, param2) = SHE::key_gen_param_with_chosen_user_ab(k0, k1, k2);
    
    let (pk1, pp1) = param1.key_generation();
    let (pk2, pp2) = param2.key_generation();

    // 生成 public key setting 参数
    // 测量加密时间
    let mut vv:Vec<BigInt> = vec![];
    let enc_time = Instant::now();
    for i in 0..n {
        let e01 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, i as u128);
        //let e02 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, i as u128);
        vv.push(e01);
    }
    let enc_time = enc_time.elapsed();

    let e01 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, 10000);
    let e02 = SHE::encrypt_with_chosen_user_ab(&pk1, &pk2, &pp1, 10000);

    // 测量解密时间
    let dec_time = Instant::now();
    for i in 0..n {
        let m1 = SHE::decrypt_with_chosen_user_ab(&pk1, &pk2, &vv[i]);
        //let m2 = SHE::decrypt_with_chosen_user_ab(&pk1, &pk2, &e02);
    }
    let dec_time = dec_time.elapsed();

    //测量密文同态乘法时间
    let mult_time = Instant::now();
    for i in 0..n {
        //let product = &e01 * &e02;
        let product = SHE::Multi_cMul(&pk2, &pp2, &e01, &e02);
    }
    let mult_time = mult_time.elapsed();

    //测量标量乘法时间
    let scalar_mult_time = Instant::now();
    for i in 0..n {
        //let product_scalar = &e01 * BigInt::from(10000);
        let product = SHE::Multi_sMul(&pk2, &pp2, &e01, 10000);
    }
    let scalar_mult_time = scalar_mult_time.elapsed();

    //测量密文同态加法时间
    let sum_time = Instant::now();
    for i in 0..n {
        //let sum = &e01 + &e02;
        let product = SHE::Multi_cAdd(&pk2, &pp2, &e01, &e02);
    }
    let sum_time = sum_time.elapsed();

    //测量标量同态加法时间
    let scalar_sum_time = Instant::now();
    for i in 0..n {
        //let sum_scalar = &e01 + BigInt::from(1000000);
        let product = SHE::Multi_sAdd(&pk2, &pp2, &e01,10000);
    }
    let scalar_sum_time = scalar_sum_time.elapsed();


    println!("密钥生成时间：{}",key_gen_time/n as u32);
    println!("加密时间：{}", enc_time/(n as u32));
    println!("解密时间：{}", dec_time/(n as u32));
    println!("标量同态加法时间：{}", scalar_sum_time/n as u32);
    println!("标量同态乘法时间：{}", scalar_mult_time/n as u32);
    println!("密文同态加法时间：{}", sum_time/n as u32);
    println!("密文同态乘法时间：{}", mult_time/n as u32);


}