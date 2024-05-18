use std::ops::Add;
use std::sync::Arc;

use rand::thread_rng;
use zkhash::ff::{Field, PrimeField};
use zkhash::fields::bls12::FpBLS12;
use zkhash::reinforced_concrete::reinforced_concrete::ReinforcedConcrete;
use zkhash::reinforced_concrete::reinforced_concrete_instances::RC_BLS_PARAMS;
use zkhash::reinforced_concrete::reinforced_concrete_params::ReinforcedConcreteParams;
use zkhash::utils::random_scalar_rng;

struct RCEnc<F: PrimeField> {
    rc: ReinforcedConcrete<F>,
}

enum Mode {
    Encrypt,
    Decrypt,
}

impl<F: PrimeField> RCEnc<F> {
    fn new(params: &Arc<ReinforcedConcreteParams<F>>) -> Self {
        Self {
            rc: ReinforcedConcrete::new(params),
        }
    }

    fn encrypt(&self, message: Vec<F>, key: F) -> (F, Vec<F>) {
        let nonce = random_scalar_rng::<F, _>(false, &mut thread_rng());
        self.encdec(message, nonce, key, Mode::Encrypt)
    }

    fn decrypt(&self, message: Vec<F>, nonce: F, key: F) -> Vec<F> {
        self.encdec(message, nonce, key, Mode::Decrypt).1
    }

    fn encdec(&self, message: Vec<F>, nonce: F, key: F, mode: Mode) -> (F, Vec<F>) {
        let a = self.rc.hash(&nonce, &key);

        let res = message
            .into_iter()
            .enumerate()
            .map(|(i, mut msg_i)| {
                let i_ff = F::from_str(&i.to_string()).expect("field conversion failed");
                let r_i = self.rc.hash(&a, &i_ff);
                match mode {
                    Mode::Encrypt => msg_i.add_assign(&r_i),
                    Mode::Decrypt => msg_i.sub_assign(&r_i),
                };
                msg_i
            })
            .collect();

        (nonce, res)
    }
}

// fn decrypt(nonce: FpBLS12, e: Vec<impl PrimeField>, key: impl Field) -> Vec<impl Field> {
//     let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
//     let a = rc.hash(&nonce, key);

//     let mut r = vec![];
//     for i in 1..e.len() {
//         r.push(rc.hash(&a, &FpBLS12::from_str(i.to_string().as_str()).unwrap()))
//     }

//     let mut res = vec![];
//     for (i, m) in r.zip(e) {
//         res.push(m - i);
//     }

//     res
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let message = vec![random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng()); 10];
        let key = random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng());

        let rcenc = RCEnc::new(&RC_BLS_PARAMS);

        let (nonce, enc) = rcenc.encrypt(message.clone(), key);
        let dec = rcenc.decrypt(enc, nonce, key);
        assert_eq!(message, dec)
    }
}
