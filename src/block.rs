#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Normal1 = 1,
    Normal2,
    Normal3,
    Normal4,
    Wild,
}

impl Block {
    pub const NUM: usize = 5;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 5;

    pub fn from_inner(inner: u8) -> Option<Self> {
        Self::is_valid(inner).then(|| unsafe { Self::from_inner_unchecked(inner) })
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub fn to_inner(self) -> u8 {
        self as u8
    }

    pub fn to_index(self) -> usize {
        usize::from(self.to_inner() - 1)
    }

    pub fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    pub fn all() -> [Self; Self::NUM] {
        [
            Self::Normal1,
            Self::Normal2,
            Self::Normal3,
            Self::Normal4,
            Self::Wild,
        ]
    }
}
