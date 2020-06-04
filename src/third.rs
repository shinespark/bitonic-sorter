use super::SortOrder;
use std::cmp::Ordering;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // * はポインタ？
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

// F: クロージャ, where F: Fn(&T, &T) -> Orderingで示すトレイト境界を指定
// 引数にクロージャを取る時には、クロージャは異なる匿名の型として扱われる為、必ずジェネリクスにしなければならない。
// クロージャは個々に異なる型になるため、もしクロージャを2つ取る関数を定義する場合には、それぞれ別の型パラメータが必要になる
pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {}",
            x.len()
        ))
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true, comparator);
        do_sort(&mut x[mid_point..], false, comparator);
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    // forward をOrdering値に変換しておく
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // 比較結果のOrderingのバリアントがswap_conditionと等しいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    // 親モジュール(first)のsort関数を使用する
    use super::sort;
    use crate::third::sort_by;
    use crate::utils::{is_sorted_ascending, is_sorted_descending, new_u32_vec};
    use crate::SortOrder::*;

    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with",
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC",
            ]
        );
    }

    // #[test]
    // fn sort_f64() {
    //     let mut x = vec![20.0, -30.0, 11.0, 10.0];
    //     sort(&mut x, &Ascending);
    // }
    // #[test]
    // fn sort_mixed() {
    //     // 実行されないがコンパイル時にエラーが検知される
    //     if false {
    //         let mut x = vec![10, 30, "a", "b"];
    //         sort(&mut x, &Ascending);
    //     }
    // }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];
        assert!(sort(&mut x, &Ascending).is_err());
    }

    // DebugトレイトとPartialEqトレイトの実装を自動導出する
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    // implementation: 実装
    impl Student {
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            // Self: impl対象の型(ここではStudent)の別名
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age,
            }
        }
    }

    #[test]
    fn sort_student_by_age_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        // 第2引数はソート順を決めるクロージャ
        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)), Ok(()));
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_student_by_name_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        // 姓asc, 名asc
        // then_withとは... https://doc.rust-lang.org/std/cmp/enum.Ordering.html
        assert_eq!(
            sort_by(&mut x, &|a, b| a
                .last_name
                .cmp(&b.last_name)
                .then_with(|| a.first_name.cmp(&b.first_name))),
            Ok(())
        );

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_large() {
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            assert!(is_sorted_descending(&x));
        }
    }
}
