use group::ff::Field;
use rand::thread_rng;
use zkhash::ff::PrimeField;
use zkhash::fields::bls12::FpBLS12;
use zkhash::reinforced_concrete::reinforced_concrete::ReinforcedConcrete;
use zkhash::reinforced_concrete::reinforced_concrete_instances::RC_BLS_PARAMS;
use zkhash::utils::random_scalar_rng;

fn encrypt(message: Vec<impl Field>, key: impl Field) -> (FpBLS12, Vec<impl PrimeField>) {
    let nonce = random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng());

    let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
    let a = rc.hash(&nonce, key);
    
    let mut r = vec![];
    for i in 1..message.len() {
        r.push(rc.hash(&a, &FpBLS12::from_str(i.to_string().as_str()).unwrap()))
    }
    
    let mut res = vec![];
    for (i, m) in r.zip(message) {
        res.push(m + i);
    }

    (nonce, res)
}

fn decrypt(nonce: FpBLS12, e: Vec<impl PrimeField>, key: impl Field) -> Vec<impl Field> {
    let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
    let a = rc.hash(&nonce, key);

    let mut r = vec![];
    for i in 1..e.len() {
        r.push(rc.hash(&a, &FpBLS12::from_str(i.to_string().as_str()).unwrap()))
    }

    let mut res = vec![];
    for (i, m) in r.zip(e) {
        res.push(m - i);
    }
    
    res
}

#[test]
fn run() {
    let message = vec![random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng()); 10];
    let key = random_scalar_rng::<FpBLS12, _>(false, &mut thread_rng());
    
}