use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Low,
    High,
}

impl Bit {
    pub const fn nand(self, other: Self) -> Self {
        if matches!((self, other), (Self::High, Self::High)) {
            Self::Low
        } else {
            Self::High
        }
    }

    pub const fn not(self) -> Self {
        self.nand(self)
    }

    pub const fn and(self, other: Self) -> Self {
        self.nand(other).not()
    }

    pub const fn or(self, other: Self) -> Self {
        self.not().nand(other.not())
    }

    pub const fn xor(self, other: Self) -> Self {
        self.nand(other).and(self.or(other))
    }

    pub const fn mux(self, other: Self, sel: Self) -> Self {
        self.and(sel.not()).or(other.and(sel))
    }

    pub const fn dmux(self, sel: Self) -> [Self; 2] {
        [self.and(sel.not()), self.and(sel)]
    }

    pub const fn dmux_4_way(self, sel: [Self; 2]) -> [Self; 4] {
        let [ab, cd] = self.dmux(sel[0]);
        let [[a, b], [c, d]] = [ab.dmux(sel[1]), cd.dmux(sel[1])];
        [a, b, c, d]
    }

    pub const fn dmux_8_way(self, sel: [Self; 3]) -> [Self; 8] {
        let [abcd, efgh] = self.dmux(sel[0]);
        let sel = [sel[1], sel[2]];
        let [[a, b, c, d], [e, f, g, h]] = [abcd.dmux_4_way(sel), efgh.dmux_4_way(sel)];
        [a, b, c, d, e, f, g, h]
    }
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

/// Performs an or operation between all the values in the array and returns the result
pub const fn or_n_way<const N: usize>(values: [Bit; N]) -> Bit {
    if values.is_empty() {
        return Bit::Low;
    }

    let mut result = values[0];
    let mut i = 1;
    while i < values.len() {
        result = result.or(values[i]);
        i += 1;
    }
    result
}

/// Performs an or operation between all the values in the slice and returns the result
pub const fn or_slice(values: &[Bit]) -> Bit {
    if values.is_empty() {
        return Bit::Low;
    }

    let mut result = values[0];
    let mut i = 1;
    while i < values.len() {
        result = result.or(values[i]);
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::bit::{Bit, or_n_way, or_slice};

    #[test]
    const fn nand_test() {
        assert!(matches!(Bit::Low.nand(Bit::Low), Bit::High));
        assert!(matches!(Bit::Low.nand(Bit::High), Bit::High));
        assert!(matches!(Bit::High.nand(Bit::Low), Bit::High));
        assert!(matches!(Bit::High.nand(Bit::High), Bit::Low));
    }

    #[test]
    const fn not_test() {
        assert!(matches!(Bit::Low.not(), Bit::High));
        assert!(matches!(Bit::High.not(), Bit::Low));

        assert!(!matches!(Bit::Low, Bit::High));
        assert!(!matches!(Bit::High, Bit::Low));
    }

    #[test]
    fn and_test() {
        const {
            assert!(matches!(Bit::Low.and(Bit::Low), Bit::Low));
            assert!(matches!(Bit::Low.and(Bit::High), Bit::Low));
            assert!(matches!(Bit::High.and(Bit::Low), Bit::Low));
            assert!(matches!(Bit::High.and(Bit::High), Bit::High));
        }

        assert_eq!(Bit::Low & Bit::Low, Bit::Low);
        assert_eq!(Bit::Low & Bit::High, Bit::Low);
        assert_eq!(Bit::High & Bit::Low, Bit::Low);
        assert_eq!(Bit::High & Bit::High, Bit::High);
    }

    #[test]
    fn or_test() {
        const {
            assert!(matches!(Bit::Low.or(Bit::Low), Bit::Low));
            assert!(matches!(Bit::Low.or(Bit::High), Bit::High));
            assert!(matches!(Bit::High.or(Bit::Low), Bit::High));
            assert!(matches!(Bit::High.or(Bit::High), Bit::High));
        }

        assert_eq!(Bit::Low | Bit::Low, Bit::Low);
        assert_eq!(Bit::Low | Bit::High, Bit::High);
        assert_eq!(Bit::High | Bit::Low, Bit::High);
        assert_eq!(Bit::High | Bit::High, Bit::High);
    }

    #[test]
    fn xor_test() {
        const {
            assert!(matches!(Bit::Low.xor(Bit::Low), Bit::Low));
            assert!(matches!(Bit::Low.xor(Bit::High), Bit::High));
            assert!(matches!(Bit::High.xor(Bit::Low), Bit::High));
            assert!(matches!(Bit::High.xor(Bit::High), Bit::Low));
        }

        assert_eq!(Bit::Low ^ Bit::Low, Bit::Low);
        assert_eq!(Bit::Low ^ Bit::High, Bit::High);
        assert_eq!(Bit::High ^ Bit::Low, Bit::High);
        assert_eq!(Bit::High ^ Bit::High, Bit::Low);
    }

    #[test]
    const fn mux_test() {
        assert!(matches!(Bit::Low.mux(Bit::Low, Bit::Low), Bit::Low));
        assert!(matches!(Bit::Low.mux(Bit::Low, Bit::High), Bit::Low));
        assert!(matches!(Bit::Low.mux(Bit::High, Bit::Low), Bit::Low));
        assert!(matches!(Bit::Low.mux(Bit::High, Bit::High), Bit::High));
        assert!(matches!(Bit::High.mux(Bit::Low, Bit::Low), Bit::High));
        assert!(matches!(Bit::High.mux(Bit::Low, Bit::High), Bit::Low));
        assert!(matches!(Bit::High.mux(Bit::High, Bit::Low), Bit::High));
        assert!(matches!(Bit::High.mux(Bit::High, Bit::High), Bit::High));
    }

    #[test]
    const fn dmux_test() {
        assert!(matches!(Bit::Low.dmux(Bit::Low), [Bit::Low, Bit::Low]));
        assert!(matches!(Bit::Low.dmux(Bit::High), [Bit::Low, Bit::Low]));
        assert!(matches!(Bit::High.dmux(Bit::Low), [Bit::High, Bit::Low]));
        assert!(matches!(Bit::High.dmux(Bit::High), [Bit::Low, Bit::High]));
    }

    #[test]
    const fn multi_or_test() {
        assert!(matches!(or_n_way([Bit::Low, Bit::Low]), Bit::Low));
        assert!(matches!(or_n_way([Bit::Low, Bit::High]), Bit::High));
        assert!(matches!(or_n_way([Bit::High, Bit::Low]), Bit::High));
        assert!(matches!(or_n_way([Bit::High, Bit::High]), Bit::High));

        assert!(matches!(or_slice(&[Bit::Low, Bit::Low]), Bit::Low));
        assert!(matches!(or_slice(&[Bit::Low, Bit::High]), Bit::High));
        assert!(matches!(or_slice(&[Bit::High, Bit::Low]), Bit::High));
        assert!(matches!(or_slice(&[Bit::High, Bit::High]), Bit::High));
    }

    #[test]
    const fn dmux_4_way_test() {
        assert!(matches!(
            Bit::Low.dmux_4_way([Bit::Low, Bit::Low]),
            [Bit::Low, Bit::Low, Bit::Low, Bit::Low]
        ));
        assert!(matches!(
            Bit::Low.dmux_4_way([Bit::Low, Bit::High]),
            [Bit::Low, Bit::Low, Bit::Low, Bit::Low]
        ));
        assert!(matches!(
            Bit::Low.dmux_4_way([Bit::High, Bit::Low]),
            [Bit::Low, Bit::Low, Bit::Low, Bit::Low]
        ));
        assert!(matches!(
            Bit::Low.dmux_4_way([Bit::High, Bit::High]),
            [Bit::Low, Bit::Low, Bit::Low, Bit::Low]
        ));
        assert!(matches!(
            Bit::High.dmux_4_way([Bit::Low, Bit::Low]),
            [Bit::High, Bit::Low, Bit::Low, Bit::Low]
        ));
        assert!(matches!(
            Bit::High.dmux_4_way([Bit::Low, Bit::High]),
            [Bit::Low, Bit::High, Bit::Low, Bit::Low]
        ));
        assert!(matches!(
            Bit::High.dmux_4_way([Bit::High, Bit::Low]),
            [Bit::Low, Bit::Low, Bit::High, Bit::Low]
        ));
        assert!(matches!(
            Bit::High.dmux_4_way([Bit::High, Bit::High]),
            [Bit::Low, Bit::Low, Bit::Low, Bit::High]
        ));
    }

    fn dmux_8_way_test() {
        assert!(matches!(
            Bit::Low.dmux_8_way([Bit::Low, Bit::Low, Bit::Low]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::Low.dmux_8_way([Bit::Low, Bit::Low, Bit::High]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::Low.dmux_8_way([Bit::Low, Bit::High, Bit::Low]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::Low.dmux_8_way([Bit::Low, Bit::High, Bit::High]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::Low.dmux_8_way([Bit::High, Bit::Low, Bit::Low]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::Low.dmux_8_way([Bit::High, Bit::Low, Bit::High]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::Low.dmux_8_way([Bit::High, Bit::High, Bit::Low]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::Low.dmux_8_way([Bit::High, Bit::High, Bit::High]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));

        assert!(matches!(
            Bit::High.dmux_8_way([Bit::Low, Bit::Low, Bit::Low]),
            [
                Bit::High,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::High.dmux_8_way([Bit::Low, Bit::Low, Bit::High]),
            [
                Bit::Low,
                Bit::High,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::High.dmux_8_way([Bit::Low, Bit::High, Bit::Low]),
            [
                Bit::Low,
                Bit::Low,
                Bit::High,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::High.dmux_8_way([Bit::Low, Bit::High, Bit::High]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::High,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::High.dmux_8_way([Bit::High, Bit::Low, Bit::Low]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::High,
                Bit::Low,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::High.dmux_8_way([Bit::High, Bit::Low, Bit::High]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::High,
                Bit::Low,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::High.dmux_8_way([Bit::High, Bit::High, Bit::Low]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::High,
                Bit::Low
            ]
        ));
        assert!(matches!(
            Bit::High.dmux_8_way([Bit::High, Bit::High, Bit::High]),
            [
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::Low,
                Bit::High
            ]
        ));
    }
}
