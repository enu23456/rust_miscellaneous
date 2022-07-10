//! hashアルゴリズム`joaat(jenkins_one_at_a_time)`でhash値を計算する
//!
//! # References
//! https://en.wikipedia.org/wiki/Jenkins_hash_function

/// 配列をいじれないようにして呼び出す
fn joaat_compute(data: &[u8]) -> u32 {
    let mut hash: u32 = 0;
    for i in data {
        hash = hash.wrapping_add(*i as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }
    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);
    return hash;
}

fn main() {
    let array = b"Hello world!";
    let digest: u32 = joaat_compute(array);
    println!("hash is ... {}", digest);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_joaat() {
        let expected = 2689957701;
        let array: [u8; 5] = [11, 84, 3, 143, 247];
        let digest = joaat_compute(&array);
        assert_eq!(expected, digest);
    }
}
