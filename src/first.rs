pub fn sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[..mid_point], false);
        sub_sort(x, up)
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    // 未実装を示すマクロ
    unimplemented!();
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}
