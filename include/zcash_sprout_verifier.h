#ifndef ZCASH_SPROUT_VERIFIER_INCLUDE_H_
#define ZCASH_SPROUT_VERIFIER_INCLUDE_H_

#include <stdint.h>
#include "libsnark/zk_proof_systems/ppzksnark/r1cs_ppzksnark/r1cs_ppzksnark.hpp"

using namespace libsnark;

typedef alt_bn128_pp::G1_type curve_G1;
typedef alt_bn128_pp::G2_type curve_G2;
typedef alt_bn128_pp::Fp_type curve_Fr;

extern "C" {
    uint8_t rust_sprout_verifier(
        const curve_G2* a,
        const curve_G1* b,
        const curve_G2* c,
        const curve_G2* z,
        const curve_G2* gamma,
        const curve_G1* gamma_beta_1,
        const curve_G2* gamma_beta_2,
        const curve_G1* ic_ptr,
        size_t ic_len,

        const curve_Fr* primary_input_ptr,
        size_t primary_input_len,

        const curve_G1* proof_a,
        const curve_G1* proof_a_prime,
        const curve_G2* proof_b,
        const curve_G1* proof_b_prime,
        const curve_G1* proof_c,
        const curve_G1* proof_c_prime,
        const curve_G1* proof_k,
        const curve_G1* proof_h
    );
}

#endif // ZCASH_SPROUT_VERIFIER_INCLUDE_H_
