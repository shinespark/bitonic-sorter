use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGの初期化。再現性を持たせるためにシード値を固定
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    // イテレータチェイン
    rng.sample_iter(&Standard).take(n).collect()
}
