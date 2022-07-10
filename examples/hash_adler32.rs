//! hashアルゴリズム`adler32`でhash値を計算する
//!
//! # Reference
//! + https://ja.wikipedia.org/wiki/Adler-32

/// hash値を計算する
fn adler32_compute(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    for i in data {
        a = (a.wrapping_add(*i as u32)) % 65521;
        b = (b.wrapping_add(a)) % 65521;
    }

    return (b << 16).wrapping_add(a);
}

/// エントリーポイント
fn main() {
    let array = b"Hello world!";
    let digest: u32 = adler32_compute(array);
    println!("hash is ... {}", digest);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adler32() {
        let expected = 61473257;
        let array: [u8; 5] = [11, 84, 3, 143, 247];
        let digest = adler32_compute(&array);
        assert_eq!(expected, digest);
    }
}
