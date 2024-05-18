mod encrypt;

// use std::marker::PhantomData;

// use cpazk::poseidon_circuit::{HashCircuit, MyConfig, PoseidonSpec};
// use group::ff::PrimeField;
// use halo2_gadgets::poseidon::primitives::{self as poseidon, ConstantLength};
// use halo2_proofs::{
//     circuit::Value,
//     dev::MockProver,
//     pasta::{pallas, vesta, Fp},
//     plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, SingleVerifier},
//     poly::commitment::Params,
//     transcript::{Blake2bRead, Blake2bWrite, Challenge255},
// };

// const L: usize = 2;
// const MSGSIZE: usize = 8;
// const WIDTH: usize = 3;
// const RATE: usize = 2;
// type S = PoseidonSpec<WIDTH, RATE>;
// const K: u32 = 8;

fn main() {
    // let mut rng = rand::rngs::OsRng;

    // Initialize the information for the encryption
    // println!("params new");
    // let params: Params<vesta::Affine> = Params::new(K);

    // println!("msg");
    // let message = (0..MSGSIZE)
    //     .map(|_| pallas::Base::one())
    //     .collect::<Vec<_>>()
    //     .try_into()
    //     .expect("array of wrong size");
    // let key = Fp::one();
    // let nonce = Fp::one();

    // let circuit = HashCircuit::<S, WIDTH, RATE, MSGSIZE> {
    //     message: Value::known(message),
    //     key: Value::known(key),
    //     nonce: Value::known(nonce),
    //     _spec: PhantomData,
    // };

    // let vk = keygen_vk(&params, &circuit).expect("keygen_vk should not fail");
    // let pk = keygen_pk(&params, vk, &circuit).expect("keygen_pk should not fail");

    // // Compute the encryption
    // let hasher = || poseidon::Hash::<_, S, ConstantLength<L>, WIDTH, RATE>::init();
    // let a = hasher().hash([key, nonce]);
    // let output: Vec<_> = message
    //     .into_iter()
    //     .enumerate()
    //     .map(|(i, msg_i)| {
    //         let i_ff = Fp::from_u128(i.try_into().unwrap());
    //         let r_i = hasher().hash([a, i_ff]);
    //         vec![msg_i + &r_i]
    //     })
    //     .collect();

    // // Create a proof
    // println!("creating proof");

    // let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    // create_proof(
    //     &params,
    //     &pk,
    //     &[circuit],
    //     &[&output.as_ref()],
    //     &mut rng,
    //     &mut transcript,
    // )
    // .unwrap();

    // let proof = transcript.finalize();
    // println!("finished creating proof");

    // let strategy = SingleVerifier::new(&params);
    // let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    // println!("verifying proof");
    // let verify_proof_result: Result<(), Error> = verify_proof(
    //     &params,
    //     pk.get_vk(),
    //     strategy,
    //     &[&[&output]],
    //     &mut transcript,
    // );
    // verify_proof_result.expect("verification should pass")
}
