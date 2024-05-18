// use std::sync::Arc;

// use rand::thread_rng;
// use zkhash::ff::PrimeField;
// use zkhash::reinforced_concrete::reinforced_concrete::ReinforcedConcrete;
// use zkhash::reinforced_concrete::reinforced_concrete_params::ReinforcedConcreteParams;
// use zkhash::utils::random_scalar_rng;

// struct RCEnc<F: PrimeField> {
//     rc: ReinforcedConcrete<F>,
// }

// enum Mode {
//     Encrypt,
//     Decrypt,
// }

// type Plaintext<F> = Vec<F>;
// type Ciphertext<F> = (F, Vec<F>);

// impl<F: PrimeField> RCEnc<F> {
//     fn new(params: &Arc<ReinforcedConcreteParams<F>>) -> Self {
//         Self {
//             rc: ReinforcedConcrete::new(params),
//         }
//     }

//     fn encrypt(&self, key: F, plaintext: Plaintext<F>) -> Ciphertext<F> {
//         let nonce = random_scalar_rng::<F, _>(false, &mut thread_rng());
//         self.encdec(key, nonce, plaintext, Mode::Encrypt)
//     }

//     fn decrypt(&self, key: F, ciphertext: Ciphertext<F>) -> Plaintext<F> {
//         let (nonce, message) = ciphertext;
//         self.encdec(key, nonce, message, Mode::Decrypt).1
//     }

//     fn encdec(&self, key: F, nonce: F, message: Vec<F>, mode: Mode) -> (F, Vec<F>) {
//         let a = self.rc.hash(&nonce, &key);

//         let res = message
//             .into_iter()
//             .enumerate()
//             .map(|(i, mut msg_i)| {
//                 let i_ff = F::from_str(&i.to_string()).expect("field conversion failed");
//                 let r_i = self.rc.hash(&a, &i_ff);
//                 match mode {
//                     Mode::Encrypt => msg_i.add_assign(&r_i),
//                     Mode::Decrypt => msg_i.sub_assign(&r_i),
//                 };
//                 msg_i
//             })
//             .collect();

//         (nonce, res)
//     }
// }

// // fn decrypt(nonce: FpBLS12, e: Vec<impl PrimeField>, key: impl Field) -> Vec<impl Field> {
// //     let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
// //     let a = rc.hash(&nonce, key);

// //     let mut r = vec![];
// //     for i in 1..e.len() {
// //         r.push(rc.hash(&a, &FpBLS12::from_str(i.to_string().as_str()).unwrap()))
// //     }

// //     let mut res = vec![];
// //     for (i, m) in r.zip(e) {
// //         res.push(m - i);
// //     }

// //     res
// // }

// #[cfg(test)]
// mod tests {
//     use zkhash::{
//         fields::bls12::FpBLS12, reinforced_concrete::reinforced_concrete_instances::RC_BLS_PARAMS,
//     };

//     use super::*;
//     #[test]
//     fn test() {
//         let message = vec![random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng()); 10];
//         let key = random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng());

//         let rcenc = RCEnc::new(&RC_BLS_PARAMS);

//         let ct = rcenc.encrypt(key, message.clone());
//         let dec = rcenc.decrypt(key, ct);
//         assert_eq!(message, dec)
//     }
// }
