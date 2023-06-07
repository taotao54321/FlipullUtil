use std::path::Path;

use anyhow::{ensure, Context as _};

use crate::util::SliceExt as _;

const PRG_LEN: usize = 0x8000;
const CHR_LEN: usize = 0x8000;

#[derive(Debug)]
pub struct Rom {
    prg: Box<[u8; PRG_LEN]>,
    chr: Box<[u8; CHR_LEN]>,
}

impl Rom {
    pub fn from_ines_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        Self::_from_ines_file(path.as_ref())
    }

    fn _from_ines_file(path: &Path) -> anyhow::Result<Self> {
        let ines =
            std::fs::read(path).with_context(|| format!("can't read '{}'", path.display()))?;

        Self::_from_ines_bytes(&ines)
    }

    pub fn from_ines_bytes(ines: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        Self::_from_ines_bytes(ines.as_ref())
    }

    fn _from_ines_bytes(ines: &[u8]) -> anyhow::Result<Self> {
        let (header, body) = ines.try_split_at(16).context("incomplete iNES header")?;
        ensure!(header.starts_with(b"NES\x1A"), "iNES magic not found");

        let (prg, chr) = body.try_split_at(PRG_LEN).context("incomplete PRG")?;
        ensure!(
            chr.len() == CHR_LEN,
            "CHR size mismatch (expect={CHR_LEN:#06X}, actual={:#06X})",
            chr.len()
        );

        let prg: Box<[u8; PRG_LEN]> = prg.to_vec().try_into().unwrap();
        let chr: Box<[u8; CHR_LEN]> = chr.to_vec().try_into().unwrap();

        Ok(Self { prg, chr })
    }

    pub fn prg(&self) -> &[u8; PRG_LEN] {
        &self.prg
    }

    pub fn chr(&self) -> &[u8; CHR_LEN] {
        &self.chr
    }
}
