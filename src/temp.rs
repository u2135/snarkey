use rand::thread_rng;
use zkhash::ff::{Field, PrimeField};
use zkhash::fields::bls12::FpBLS12;
use zkhash::reinforced_concrete::reinforced_concrete::ReinforcedConcrete;
use zkhash::reinforced_concrete::reinforced_concrete_instances::RC_BLS_PARAMS;
use zkhash::utils::random_scalar_rng;

fn encrypt(message: Vec<FpBLS12>, key: FpBLS12) -> (FpBLS12, Vec<FpBLS12>) {
    let nonce = random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng());

    let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
    let a = rc.hash(&nonce, &key);
    
    let mut r = vec![];
    for i in 0..message.len() {
        r.push(rc.hash(&a, &FpBLS12::from_str(i.to_string().as_str()).unwrap()))
    }

    let mut res = message.clone();
    for (idx, i) in r.iter().enumerate() {
        res[idx].add_assign(i);
    }

    (nonce, res)
}

fn decrypt(nonce: FpBLS12, e: Vec<FpBLS12>, key: FpBLS12) -> Vec<FpBLS12> {
    let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
    let a = rc.hash(&nonce, &key);

    let mut r = vec![];
    for i in 0..e.len() {
        r.push(rc.hash(&a, &FpBLS12::from_str(i.to_string().as_str()).unwrap()))
    }

    let mut res = e.clone();
    for (idx, i) in r.iter().enumerate() {
        res[idx].sub_assign(i);
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let message = vec![random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng()); 10];
        let key = random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng());

        let (nonce, enc) = encrypt(message.clone(), key);
        let dec = decrypt(nonce, enc, key);
        assert_eq!(message, dec)
    }
}
