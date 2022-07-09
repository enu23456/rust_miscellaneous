//! # 疑似乱数生成法`splitmix64`で乱数を生成する
//!
//! ## Reference
//! https://prng.di.unimi.it/splitmix64.c

/// 乱数生成器のパラメータを格納する構造体
struct Splitmix64 {
    state: u64,
}

impl Splitmix64 {
    /// 初期化する
    fn new(seed: u64) -> Splitmix64 {
        return Splitmix64 { state: seed };
    }

    /// 乱数を出力し生成器の内部状態を進める
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97f4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        return z ^ (z >> 31);
    }
}

/// エントリーポイント
fn main() {
    let mut rng = Splitmix64::new(0);
    for _ in 0..10 {
        println!("value: {}", rng.next());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_splitmix64() {
        let expected: [u64; 10] = [
            16294208416658607535,
            7960286522194355700,
            487617019471545679,
            17909611376780542444,
            1961750202426094747,
            6038094601263162090,
            3207296026000306913,
            14232521865600346940,
            4532161160992623299,
            17561866513979060390,
        ];
        let mut rng = Splitmix64::new(0);
        for e in &expected {
            assert_eq!(*e, rng.next());
        }
    }
}
