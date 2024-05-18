use group::ff::{Field, PrimeField};
use rand::thread_rng;
use zkhash::fields::bls12::FpBLS12;
use zkhash::reinforced_concrete::reinforced_concrete::ReinforcedConcrete;
use zkhash::reinforced_concrete::reinforced_concrete_instances::RC_BLS_PARAMS;
use zkhash::utils::random_scalar_rng;

fn encrypt(message: Vec<impl Field>, key: impl Field) -> Vec<impl PrimeField> {
    let nonce = random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng());

    let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
    let a = rc.hash(&nonce, key);
    
    let r = vec![];
    
    for i in 1..message.len() {
        r.push(rc.hash(&a, FpBLS12::from))
    }
    
    let res = vec![];
    for (i, m) in r.zip(message) {
        res.push(m + i);
    }
    res
}