use std::path::PathBuf;

use clap::Parser;

use flipull_util::*;

/// FC『フリップル』の配牌をシミュレートする。
#[derive(Debug, Parser)]
struct Cli {
    /// 原作の ROM ファイルのパス。
    path_ines: PathBuf,

    /// 面 (0-based)。
    #[arg(value_parser = parse_int::parse::<u8>)]
    stage: u8,

    /// 乱数シード (u16, ビッグエンディアン)。
    #[arg(value_parser = parse_int::parse::<u16>)]
    rng_state: u16,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rom = Rom::from_ines_file(&cli.path_ines)?;

    let rng_state = cli.rng_state.to_be_bytes();

    let blocks = deal_blocks(&rom, cli.stage, rng_state);

    println!("{}", blocks_display(&blocks));

    Ok(())
}
