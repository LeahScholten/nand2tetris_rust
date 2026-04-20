use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not};

use crate::bit::Bit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Byte([Bit; 8]);

impl Byte {
    pub const fn nand(mut self, other: Self) -> Self {
        let mut i = 0;
        while i < self.0.len() {
            self.0[i] = self.0[i].nand(other.0[i]);
            i += 1;
        }
        self
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

    pub const fn mux(mut self, other: Self, sel: Bit) -> Self {
        let mut i = 0;
        while i < self.0.len() {
            self.0[i] = self.0[i].and(sel.not()).or(other.0[i].and(sel));
            i += 1;
        }
        self
    }

    pub const fn dmux(self, sel: Bit) -> [Self; 2] {
        let mut output = [Byte([Bit::Low; 8]); 2];
        let mut i = 0;
        while i < self.0.len() {
            [output[0].0[i], output[0].0[i]] = [self.0[i].and(sel.not()), self.0[i].and(sel)];
            i += 1;
        }
        output
    }

    pub const fn dmux_4_way(self, sel: [Bit; 2]) -> [Self; 4] {
        let [ab, cd] = self.dmux(sel[0]);
        let [[a, b], [c, d]] = [ab.dmux(sel[1]), cd.dmux(sel[1])];
        [a, b, c, d]
    }

    pub const fn dmux_8_way(self, sel: [Bit; 3]) -> [Self; 8] {
        let [abcd, efgh] = self.dmux(sel[0]);
        let sel = [sel[1], sel[2]];
        let [[a, b, c, d], [e, f, g, h]] = [abcd.dmux_4_way(sel), efgh.dmux_4_way(sel)];
        [a, b, c, d, e, f, g, h]
    }

    pub const fn get(self, bit_index: usize) -> Option<Bit> {
        if bit_index < self.0.len() {
            Some(self.0[bit_index])
        } else {
            None
        }
    }
}

impl BitAnd for Byte {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitAndAssign for Byte {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Byte {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOrAssign for Byte {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Byte {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl BitXorAssign for Byte {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for Byte {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

impl Index<usize> for Byte {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
