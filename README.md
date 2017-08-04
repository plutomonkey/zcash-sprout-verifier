# Zcash Sprout zk-SNARK Proof Verifier

This is a proof verifier following the zk-SNARK protocol of Parno et al. [[PGHR13]](https://eprint.iacr.org/2013/279), for use with [Zcash](https://z.cash) Sprout.

It is written in pure Rust and uses the [bn](https://github.com/zcash/bn) pairing cryptography library, which uses the Barreto-Naehrig (BN) curve construction from [[BCTV2015]](https://eprint.iacr.org/2013/879.pdf).

The proof verifier is written to closely resemble the verification procedure in figure 10 (page 25) of [[BCTV2015]](https://eprint.iacr.org/2013/879.pdf).

Currently, its primary use is to independently verify the zero-knowledge proofs in the Zcash blockchain, which might be helpful if there turns out to be a bug in [libsnark](https://github.com/scipr-lab/libsnark) or in the way that the Zcash client uses libsnark.

All blocks in the main chain up to and including height 161007 (hash 000000001ec1da40b9c5d8e7d26110bc8f239dfc0ca58685e92ee05903871a07) on Zcash currently pass this verifier.
