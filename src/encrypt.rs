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

impl<F: PrimeField> RCEnc<F> {
    fn new(params: &Arc<ReinforcedConcreteParams<F>>) -> Self {
        Self {
            rc: ReinforcedConcrete::new(params),
        }
    }

    fn encrypt(&self, message: Vec<F>, key: F) -> (F, Vec<F>) {
        let nonce = random_scalar_rng::<F, _>(false, &mut thread_rng());
        let a = self.rc.hash(&nonce, &key);

        message.into_iter().enumerate().map(|(i, msg_i)| {
            let i_ff = i.into();
            let r_i = self.rc.hash(&a, i_ff);
            msg_i + r_i
        });

        let mut res = vec![];
        for (i, m) in r.zip(message) {
            res.push(m + i);
        }

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

#[test]
fn run() {
    let message = vec![random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng()); 10];
    let key = random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng());
}
