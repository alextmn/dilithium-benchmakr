
use rand::{ RngCore, FromEntropy, ChaChaRng };
use dilithium::params::*;
use dilithium::sign::{ keypair, sign, verify };
use std::time::Instant;
use std::env;

fn sign_test(n : i32) {

    println!("testing {} signatures", n);

    let mut all_sign: Vec<([u8;PUBLICKEYBYTES], [u8;SECRETKEYBYTES]) > = Vec::new();
    let mut all_sign_v: Vec<([u8;PUBLICKEYBYTES], [u8;BYTES]) > = Vec::new();

    let mut now = Instant::now();
    let now_total = Instant::now();

    // generating sigatures
    for _ in 0..n {
        let mut rng = ChaChaRng::from_entropy();
        let mut message = [0; 59];
        let (mut pk, mut sk) = ([0; PUBLICKEYBYTES], [0; SECRETKEYBYTES]);
        rng.fill_bytes(&mut message);
        keypair(&mut rng, &mut pk, &mut sk);

        all_sign.push((pk, sk));
    }
    let mut elapsed = now.elapsed();
    println!("Keygen in: {:.2?}", elapsed);
    now = Instant::now();

    // signing 
    let mut rng = ChaChaRng::from_entropy();
    let mut message = [0; 59];
    rng.fill_bytes(&mut message);

    for i in &all_sign {
        let (pk, sk) = i;
        let mut sig = [0; BYTES];
        sign(&mut sig, &message, &sk);
        all_sign_v.push((*pk, sig));
    }

    elapsed = now.elapsed();
    println!("signed in: {:.2?}", elapsed);
    now = Instant::now();

    //verification
    for i in &all_sign_v {
        let (pk, sig) = i;
        let ok = verify(&message, &sig, &pk);
        assert!(ok)
    }
    
    elapsed = now.elapsed();
    println!("verified in: {:.2?}", elapsed);

    let elapsed = now_total.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut n = 500;
    if args.len() > 1 {
        n = args[1].parse().unwrap();
    }
    sign_test(n);    
}
