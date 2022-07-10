//! 疑似乱数生成法`線形合同法`で乱数を生成する

/// 乱数生成器のパラメータを格納する構造体
struct Lcg32 {
    state: u32,
}

impl Lcg32 {
    /// 初期化する
    fn new(seed: u32) -> Lcg32 {
        return Lcg32 { state: seed };
    }

    /// 乱数を出力し生成器の内部状態を進める
    ///
    /// LCG32はパラメータに代表的な組み合わせがあるので複数記載している
    /// テストは一番上に記述しているパラメータで作成している
    fn next(&mut self) -> u32 {
        self.state = (self.state.wrapping_mul(1664525)).wrapping_add(1013904223);
        // self.state = (self.state.wrapping_mul(22695477)).wrapping_add(1);
        // self.state = (self.state.wrapping_mul(134775813)).wrapping_add(1);
        // self.state = (self.state.wrapping_mul(214013)).wrapping_add(2531011);
        self.state
    }
}

/// エントリーポイント
fn main() {
    let mut rng = Lcg32::new(0);
    for _ in 0..10 {
        println!("value: {}", rng.next());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcg32() {
        let expected: [u32; 10] = [
            1013904223, 1196435762, 3519870697, 2868466484, 1649599747, 2670642822, 1476291629,
            2748932008, 2180890343, 2498801434,
        ];
        let mut rng = Lcg32::new(0);
        for e in &expected {
            assert_eq!(*e, rng.next());
        }
    }
}
