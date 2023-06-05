use std::fmt::Write as _;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

pub type Blocks = [Option<Block>; 8 * 6];

pub fn blocks_display(blocks: &Blocks) -> BlocksDisplay {
    BlocksDisplay(blocks)
}

#[derive(Debug)]
pub struct BlocksDisplay<'a>(&'a Blocks);

impl std::fmt::Display for BlocksDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..8 {
                let idx = 8 * y + x;
                let ch = match self.0[idx] {
                    None => '.',
                    Some(Block::Normal1) => '1',
                    Some(Block::Normal2) => '2',
                    Some(Block::Normal3) => '3',
                    Some(Block::Normal4) => '4',
                    Some(Block::Wild) => '5',
                };
                f.write_char(ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
