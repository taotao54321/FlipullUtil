use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;

use flipull_util::*;

/// FC『フリップル』の配牌パターンを列挙し、重複を除いた個数を調べる。
#[derive(Debug, Parser)]
struct Cli {
    /// 原作の ROM ファイルのパス。
    path_ines: PathBuf,

    /// 面 (0-based)。
    #[arg(value_parser = parse_int::parse::<u8>)]
    stage: u8,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rom = Rom::from_ines_file(&cli.path_ines)?;

    let xs = enumerate(&rom, cli.stage);

    println!("len: {}", xs.len());

    //println!();
    //for (_blocks, seeds) in xs {
    //    println!("{seeds:?}");
    //}

    Ok(())
}

fn enumerate(rom: &Rom, stage: u8) -> Vec<(Blocks, Vec<u16>)> {
    let mut map = HashMap::<Blocks, Vec<u16>>::new();

    for seed in 0..=u16::MAX {
        let rng_state = seed.to_be_bytes();
        let blocks = deal_blocks(rom, stage, rng_state);
        map.entry(blocks).or_default().push(seed);
    }

    let mut res: Vec<_> = map.into_iter().collect();
    res.sort_unstable_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs));

    res
}
