//! hashアルゴリズム`crc32`でhash値を計算する

/// hash値を計算する
fn crc32_compute(data: &[u8]) -> u32 {
    let mut r: u32 = 0xFFFFFFFF;

    for c in data {
        r ^= *c as u32;
        for _ in 0..8 {
            if r & 1 == 1 {
                r = (r >> 1) ^ 0xEDB88320;
            } else {
                r >>= 1;
            }
        }
    }
    return r ^ 0xFFFFFFFF;
}

/// エントリーポイント
fn main() {
    let array = b"Hello world!";
    let digest: u32 = crc32_compute(array);
    println!("hash is ... {}", digest);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adler32() {
        let expected = 1746469787;
        let array: [u8; 5] = [11, 84, 3, 143, 247];
        let digest = crc32_compute(&array);
        assert_eq!(expected, digest);
    }
}
