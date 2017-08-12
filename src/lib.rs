extern crate bn;
extern crate libc;

use bn::{pairing, Fr, G1, G2, Group};
use std::panic::catch_unwind;
use std::slice;

struct VerifyingKey {
    a: G2,
    b: G1,
    c: G2,
    z: G2,
    gamma: G2,
    gamma_beta_1: G1,
    gamma_beta_2: G2,
    ic: Vec<G1>,
}

struct Proof {
    a: G1,
    a_prime: G1,
    b: G2,
    b_prime: G1,
    c: G1,
    c_prime: G1,
    k: G1,
    h: G1,
}

fn verify(vk: &VerifyingKey, primary_input: &[Fr], proof: &Proof) -> bool {
    let p2 = G2::one();

    // 1. compute accumulated input circuit
    let mut acc = vk.ic[0];
    for (&x, &ic) in primary_input.iter().zip(vk.ic[1..].iter()) {
        acc = acc + (ic * x);
    }

    // 2. check validity of knowledge commitments for A, B, C:
    pairing(proof.a, vk.a) == pairing(proof.a_prime, p2) &&
    pairing(vk.b, proof.b) == pairing(proof.b_prime, p2) &&
    pairing(proof.c, vk.c) == pairing(proof.c_prime, p2) &&

    // 3. check same coefficients were used:
    pairing(proof.k, vk.gamma) ==
    pairing(acc + proof.a + proof.c, vk.gamma_beta_2) * pairing(vk.gamma_beta_1, proof.b) &&

    // 4. check QAP divisibility
    pairing(acc + proof.a, proof.b) == pairing(proof.h, vk.z) * pairing(proof.c, p2)
}

#[no_mangle]
pub extern "system" fn rust_sprout_verifier(
    // verifying key
    a: *const G2,
    b: *const G1,
    c: *const G2,
    z: *const G2,
    gamma: *const G2,
    gamma_beta_1: *const G1,
    gamma_beta_2: *const G2,
    ic_ptr: *const G1,
    ic_len: libc::size_t,

    // primary input
    primary_input_ptr: *const Fr,
    primary_input_len: libc::size_t,

    // proof
    proof_a: *const G1,
    proof_a_prime: *const G1,
    proof_b: *const G2,
    proof_b_prime: *const G1,
    proof_c: *const G1,
    proof_c_prime: *const G1,
    proof_k: *const G1,
    proof_h: *const G1,
) -> libc::uint8_t {
    let vk = unsafe {
        assert!(!ic_ptr.is_null());
        VerifyingKey {
            a: *a,
            b: *b,
            c: *c,
            z: *z,
            gamma: *gamma,
            gamma_beta_1: *gamma_beta_1,
            gamma_beta_2: *gamma_beta_2,
            ic: slice::from_raw_parts(ic_ptr, ic_len).to_vec(),
        }
    };
    let primary_input = unsafe {
        assert!(!primary_input_ptr.is_null());
        slice::from_raw_parts(primary_input_ptr, primary_input_len)
    };
    let proof = unsafe {
        Proof {
            a: *proof_a,
            a_prime: *proof_a_prime,
            b: *proof_b,
            b_prime: *proof_b_prime,
            c: *proof_c,
            c_prime: *proof_c_prime,
            k: *proof_k,
            h: *proof_h,
        }
    };

    match catch_unwind(|| { verify(&vk, primary_input, &proof) }) {
        Ok(result) => result as libc::uint8_t,
        Err(_) => 0,
    }
}
