use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGの初期化。再現性を持たせるためにシード値を固定
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    // イテレータチェイン
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    // windows(2)は元のイテレータから1要素刻みで2要素ずつ値を取り出す
    // e.g. [1,2], [2,3], [3,4]
    // allは一度もクロージャがfalseを返さなかったらtrueを返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}
