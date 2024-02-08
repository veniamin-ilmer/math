#[derive(Debug, Copy, Clone)]
pub enum Trit {
    Neg,
    Zro,
    Pos,
}

impl std::ops::BitXor for Trit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Neg, Trit::Neg) => Trit::Pos,
            (Trit::Neg, Trit::Zro) => Trit::Neg,
            (Trit::Neg, Trit::Pos) => Trit::Zro,
            (Trit::Zro, Trit::Neg) => Trit::Neg,
            (Trit::Zro, Trit::Zro) => Trit::Zro,
            (Trit::Zro, Trit::Pos) => Trit::Pos,
            (Trit::Pos, Trit::Neg) => Trit::Zro,
            (Trit::Pos, Trit::Zro) => Trit::Pos,
            (Trit::Pos, Trit::Pos) => Trit::Neg,
        }
    }
}

impl std::ops::BitAnd for Trit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Neg, Trit::Neg) => Trit::Neg,
            (Trit::Neg, Trit::Zro) => Trit::Zro,
            (Trit::Neg, Trit::Pos) => Trit::Zro,
            (Trit::Zro, Trit::Neg) => Trit::Zro,
            (Trit::Zro, Trit::Zro) => Trit::Zro,
            (Trit::Zro, Trit::Pos) => Trit::Zro,
            (Trit::Pos, Trit::Neg) => Trit::Zro,
            (Trit::Pos, Trit::Zro) => Trit::Zro,
            (Trit::Pos, Trit::Pos) => Trit::Pos,
        }
    }
}

impl std::ops::BitOr for Trit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Neg, Trit::Neg) => Trit::Neg,
            (Trit::Neg, Trit::Zro) => Trit::Neg,
            (Trit::Zro, Trit::Neg) => Trit::Neg,
            (Trit::Neg, Trit::Pos) => Trit::Zro,
            (Trit::Zro, Trit::Zro) => Trit::Zro,
            (Trit::Pos, Trit::Neg) => Trit::Zro,
            (Trit::Zro, Trit::Pos) => Trit::Pos,
            (Trit::Pos, Trit::Zro) => Trit::Pos,
            (Trit::Pos, Trit::Pos) => Trit::Pos,
        }
    }
}

impl std::ops::Not for Trit {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Trit::Neg => Trit::Pos,
            Trit::Zro => Trit::Zro,
            Trit::Pos => Trit::Neg,
        }
    }
}

impl Trit {
    pub fn wrapping_add(self, rhs: Trit) -> Trit {
        self ^ rhs
    }

    pub fn overflowing_add(self, rhs: Trit) -> (Trit, Trit) {
        (self ^ rhs, self & rhs)
    }

    pub fn carrying_add(self, rhs: Trit, carry: Trit) -> (Trit, Trit) {
        (self ^ rhs ^ carry, (self & rhs) | (carry & (self ^ rhs)))
    }

    pub fn wrapping_sub(self, rhs: Trit) -> Trit {
        self ^ !rhs
    }
    
    pub fn overflowing_sub(self, rhs: Trit) -> (Trit, Trit) {
        (self ^ !rhs, self & !rhs)
    }

    pub fn carrying_sub(self, rhs: Trit, carry: Trit) -> (Trit, Trit) {
        (self ^ !rhs ^ !carry, (self & !rhs) | (!carry & (self ^ !rhs)))
    }
}
