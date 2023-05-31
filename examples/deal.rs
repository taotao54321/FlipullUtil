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

    let rng_state = [
        u8::try_from(cli.rng_state >> 8).unwrap(),
        u8::try_from(cli.rng_state & 0xFF).unwrap(),
    ];

    let blocks = deal(&rom, cli.stage, rng_state);

    print_blocks(&blocks);

    Ok(())
}

type Blocks = [Option<Block>; 8 * 6];

fn print_blocks(blocks: &Blocks) {
    for y in 0..6 {
        for x in 0..8 {
            let idx = 8 * y + x;
            let ch = match blocks[idx] {
                None => '.',
                Some(Block::Normal1) => '1',
                Some(Block::Normal2) => '2',
                Some(Block::Normal3) => '3',
                Some(Block::Normal4) => '4',
                Some(Block::Wild) => '5',
            };
            print!("{ch}");
        }
        println!();
    }
}

fn deal(rom: &Rom, stage: u8, rng_state: [u8; 2]) -> Blocks {
    let params_first = DealParams::new(stage, rng_state, false);
    let params_second = DealParams::new(stage, rng_state, true);

    let mut rng = {
        let carry = (stage & (1 << 1)) != 0;
        Rng::new(rng_state, carry)
    };

    let mut blocks = [None; 8 * 6];

    deal_part(rom, &params_first, &mut rng, &mut blocks);
    deal_part(rom, &params_second, &mut rng, &mut blocks);

    blocks
}

fn deal_part(rom: &Rom, params: &DealParams, rng: &mut Rng, blocks: &mut Blocks) {
    let mut remains = params.block_counts;

    for &idx in params.idxs {
        let block = loop {
            let block = rng.gen_block(rom);
            if remains[block.to_index()] > 0 {
                remains[block.to_index()] -= 1;
                break block;
            }
        };

        assert_eq!(blocks[idx], None);
        blocks[idx] = Some(block);
    }
}

#[derive(Debug)]
struct DealParams {
    block_counts: [u8; 4],
    idxs: &'static [usize],
}

impl DealParams {
    fn new(stage: u8, rng_state: [u8; 2], second: bool) -> Self {
        #[rustfmt::skip]
        const STAGE_KIND_TABLE: [usize; 0x20] = [
            0, 0, 0, 1, 1, 1, 1, 2,
            2, 2, 2, 1, 2, 2, 2, 1,
            1, 1, 1, 1, 1, 1, 1, 2,
            2, 2, 2, 2, 2, 0, 1, 1,
        ];

        const BLOCK_COUNTS_TABLE: [[[u8; 4]; 2]; 12] = [
            [[3, 2, 2, 2], [4, 4, 4, 4]],
            [[2, 3, 2, 2], [4, 4, 4, 4]],
            [[2, 2, 3, 2], [4, 4, 4, 4]],
            [[2, 2, 2, 3], [4, 4, 4, 4]],
            [[2, 2, 3, 3], [5, 5, 5, 5]],
            [[3, 3, 2, 2], [5, 5, 5, 5]],
            [[2, 3, 2, 3], [5, 5, 5, 5]],
            [[3, 2, 3, 2], [5, 5, 5, 5]],
            [[2, 3, 3, 3], [7, 6, 6, 6]],
            [[3, 2, 3, 3], [6, 7, 6, 6]],
            [[3, 3, 2, 3], [6, 6, 7, 6]],
            [[3, 3, 3, 2], [6, 6, 6, 7]],
        ];

        #[rustfmt::skip]
        const IDXS_TABLE: [[&[usize]; 2]; 3] = [
            [
                &[8, 9, 10, 11, 12, 20, 28, 36, 44],
                &[16, 17, 18, 19, 24, 25, 26, 27, 32, 33, 34, 35, 40, 41, 42, 43],
            ],
            [
                &[0, 1, 2, 3, 4, 12, 20, 28, 36, 44],
                &[8, 9, 10, 11, 16, 17, 18, 19, 24, 25, 26, 27, 32, 33, 34, 35, 40, 41, 42, 43],
            ],
            [
                &[0, 1, 2, 3, 4, 5, 13, 21, 29, 37, 45],
                &[8, 9, 10, 11, 12, 16, 17, 18, 19, 20, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 40, 41, 42, 43, 44],
            ],
        ];

        let stage_kind = STAGE_KIND_TABLE[usize::from(stage & 0x1F)];

        let block_counts = {
            let idx = (stage_kind << 2) | usize::from((rng_state[0] >> 2) & 3);
            BLOCK_COUNTS_TABLE[idx][usize::from(second)]
        };

        let idxs = IDXS_TABLE[stage_kind][usize::from(second)];

        Self { block_counts, idxs }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rng {
    state: [u8; 2],
    ptr: u16,
}

impl Rng {
    fn new(state: [u8; 2], carry: bool) -> Self {
        let ptr = 0x8000 | (u16::from(state[0] & 0x1F) << 8) | u16::from(ror4(state[1], carry));

        Self { state, ptr }
    }

    /*
    fn state(self) -> [u8; 2] {
        self.state
    }

    fn ptr(self) -> u16 {
        self.ptr
    }
    */

    fn gen_block(&mut self, rom: &Rom) -> Block {
        self.state[0] = self.state[0].wrapping_add(1);
        self.state[1] = self.state[1].wrapping_sub(1);

        let x = rom.prg()[usize::from(self.ptr - 0x8000)];

        self.ptr = {
            let rhs = u16::from(self.state[0]) | (u16::from(self.state[1]) << 8);
            0x8000 | (self.ptr.wrapping_add(rhs) & 0x1FFF)
        };

        let ptr = {
            let lhs = u16::from(self.state[1]) | (u16::from(self.state[0]) << 8);
            0x8000 | (lhs.wrapping_sub(self.ptr) & 0x1FFF)
        };
        let y = rom.prg()[usize::from(ptr - 0x8000)];

        let block = 1 + match (y >> 1) & 3 {
            0 => x & 3,
            1 => (x >> 2) & 3,
            2 => (x >> 4) & 3,
            3 => (x >> 6) & 3,
            _ => unreachable!(),
        };
        Block::from_inner(block).unwrap()
    }
}

fn ror4(x: u8, carry: bool) -> u8 {
    (x >> 4) | ((x & 7) << 5) | (u8::from(carry) << 4)
}
