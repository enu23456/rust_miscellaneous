//! バブルソート

/// `bubble_sort_simple`を改良＆拡張したもの
///
/// 改良
/// + `if array[j + 1] < array[j]`内の処理(Rustはswapを使って簡潔に書ける)
///
/// 拡張
/// + `<T: Ord>`で境界を設定し、大小比較できる型なら関数を使えるように
/// + ※`bubble_sort_simple`はu8の型にしか使えない
fn bubble_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                array.swap(j, j + 1);
            }
        }
    }
    return;
}

/// シンプルなバブルソートの実装
fn bubble_sort_simple(array: &mut [u8]) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                let tmp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = tmp;
            }
        }
    }
    return;
}

/// エントリーポイント
fn main() {
    let mut temp = [3, 1, 8, 4];
    println!("arr: {:?}", temp);
    bubble_sort(&mut temp); // bubble_sort_simple(&mut temp);
    println!("arr: {:?}", temp);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_simple() {
        let mut temp = [3, 1, 8, 4];
        bubble_sort_simple(&mut temp);
        assert_eq!([1, 3, 4, 8], temp);
    }
    #[test]
    fn test_bubble_sort() {
        let mut arr_1 = [3, 1, 8, 4];
        bubble_sort(&mut arr_1);
        assert_eq!([1, 3, 4, 8], arr_1);
        let mut arr_2 = ['x', 'g', 'd', 'o'];
        bubble_sort(&mut arr_2);
        assert_eq!(['d', 'g', 'o', 'x'], arr_2);
    }
}
