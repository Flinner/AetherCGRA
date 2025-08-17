use std::collections::HashMap;

use crate::hycube::{Config, Configs, OpCode, XbarConfig};

use super::{HyIns, XBarInput};

/// Binary File Format:
/// NPB,CONSTVALID,CONST,OPCODE,REGWEN,TREGWEN,REGBYPASS,PRED,OP1,OP2,NORTH,WEST,SOUTH,EAST
/// ```md
/// 0
/// Y=0 X=0,0100000000000000001100110000111110000000000111111111111111111111
/// Y=0 X=1,0100000000000000001100110000111110000000000111111111111111100111
/// Y=0 X=2,0100000000000000001100110000111110000000000111111111111111111111
/// Y=0 X=3,0100000000000000001100110000111110000000000111111111111111111111
/// Y=1 X=0,0100000000000000001100110000111110000000000111111111111111111111
/// Y=1 X=1,0100000000000000001100110000111110000000000111111001111111111100
/// Y=1 X=2,0100000000000000001100110000111110000000000111001010011111111111
/// Y=1 X=3,0100000000000000001100110000111110000000000111111111111111111111
/// Y=2 X=0,0100000000000000001100110000111110000000000111111111000111111100
/// Y=2 X=1,0100000000000000001100110000111110000000000111111010010001111100
/// Y=2 X=2,0100000000000000001100110000111110000000000111111111010111101111
/// Y=2 X=3,0100000000000000001100110000111110000000000111111111111111111111
/// Y=3 X=0,0100000000000000001100110000111110000000000111111111111111111100
/// Y=3 X=1,0100000000000000001100110000111110000000000111111010100111111010
/// Y=3 X=2,0100000000000000001100110000111110000000000111111010111011111111
/// Y=3 X=3,0100000000000000001100110000111110000000000111111111111111111111
///
/// 1
/// .....
/// ```
///
/// |`NPB`        |  1 bit   |
/// |`CONSTVALID` |  1 bit   |
/// |`CONST`      |  27 bits |
/// |`OPCODE`     |  5 bits  |
/// |`REGWEN`     |  4 bits  | South, North, West, East
/// |`TREGWEN`    |  1 bit   |
/// |`REGBYPASS`  |  4 bits  | South, North, West, East
/// |`PRED`       |  3 bit   | South, North, West, East
/// |`OP1`        |  3 bits  |
/// |`OP2`        |  3 bits  |
/// |`NORTH`      |  3 bits  |
/// |`WEST`       |  3 bits  |
/// |`SOUTH`      |  3 bits  |
/// |`EAST`       |  3 bits  |
fn decode(s: &str) -> Configs {
    let mut x: usize;
    let mut y: usize;
    let mut configs: Configs = vec![];
    let mut config: Config = vec![];

    for line in s.lines() {
        if let Ok(_cfg_number) = line.parse::<i32>() {
            configs.push(config.clone());
            config = vec![];
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let (prefix, bits) = line.split_once(",").expect("Faulty Bin File!");
        let (y_str, x_str) = prefix
            .strip_prefix("Y=")
            .unwrap()
            .split_once(" X=")
            .expect("Fault Bin File!");
        x = x_str.parse().unwrap();
        y = y_str.parse().unwrap();

        // Grow outer vector
        if y >= config.len() {
            config.resize_with(y + 1, Vec::new);
        }

        // Grow inner vector
        if x >= config[y].len() {
            config[y].resize_with(x + 1, HyIns::default);
        }
        config[y][x] = decode_instr(bits);
    }
    configs.push(config);
    // this is because the config number isn't a temrinator
    // The first config is always empty...
    configs.remove(0);
    eprintln!("{configs:#?}");
    configs
}

#[rustfmt::skip]
pub fn decode_instr(instr: &str) -> HyIns {
    // "NPB,CONSTVALID,CONST,OPCODE,REGWEN,TREGWEN,REGBYPASS,PRED,OP1,OP2,NORTH,WEST,SOUTH,EAST\n"
    // negated_predicate
    let mut instr =  instr;

    let npb         = next_chunk(&mut instr, 1  );
    let const_valid = next_chunk(&mut instr, 1  );
    let const_bits  = next_chunk(&mut instr, 27 );
    let opcode      = next_chunk(&mut instr, 5  );
    let regwen      = next_chunk(&mut instr, 4  );
    let tregwen     = next_chunk(&mut instr, 1  );
    let regbypass   = next_chunk(&mut instr, 4  );
    let pred        = next_chunk(&mut instr, 3  );
    let op1         = next_chunk(&mut instr, 3  );
    let op2         = next_chunk(&mut instr, 3  );
    let north       = next_chunk(&mut instr, 3  );
    let west        = next_chunk(&mut instr, 3  );
    let south       = next_chunk(&mut instr, 3  );
    let east        = next_chunk(&mut instr, 3  );

    let xb = XbarConfig {
        p:       to_xbar(pred  ),
        i1:      to_xbar(op1   ),
        i2:      to_xbar(op2   ),
        north_o: to_xbar(north ),
        east_o:  to_xbar(east  ),
        west_o:  to_xbar(west  ),
        south_o: to_xbar(south ),
    };

    
    HyIns{
        xB: xb,
        reg_we:           regwen.chars().map(|c| -> bool { c=='1' }).collect::<Vec<bool>>().try_into().unwrap(), 
        reg_no_bypass: regbypass.chars().map(|c| -> bool { c=='1' }).collect::<Vec<bool>>().try_into().unwrap(),
        opcode: unsafe {::std::mem::transmute(bin2i32(opcode))},
        // bit shifting to handle signed numbers :)
        constant: (bin2i32(const_bits) << 5) >> 5,
        constValid: const_valid == "1",
        npb: npb == "1",
        treg_we: tregwen == "1",
    }
}

pub fn next_chunk<'a>(s: &mut &'a str, n: usize) -> &'a str {
    let (chunk, rest) = s.split_at(n);
    *s = rest;
    chunk
}

pub fn bin2i32(s: &str) -> i32 {
    i32::from_str_radix(s, 2).expect("NOT A FUCKING BINARY NUMBER")
}

pub fn to_xbar(s: &str) -> XBarInput {
    unsafe { ::std::mem::transmute(bin2i32(s) as u8) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_instr() {
        let input: &str = "0100000000000000001100110000111110000000000111111111111111111111";
        decode_instr(input);
    }

    #[test]
    fn test_decode() {
        let input: &str = r"
0
Y=0 X=0,1100000000000000001100110000111110000000000111111111111111111111
Y=0 X=1,0100000000000000001100110000111110000000000111111111111111100111
Y=0 X=2,0100000000000000001100110000111110000000000111111111111111111111
Y=0 X=3,0100000000000000001100110000111110000000000111111111111111111111
Y=1 X=0,0100000000000000001100110000111110000000000111111111111111111111
Y=1 X=1,0100000000000000001100110000111110000000000111111001111111111100
Y=1 X=2,0100000000000000001100110000111110000000000111001010011111111111
Y=1 X=3,0100000000000000001100110000111110000000000111111111111111111111
Y=2 X=0,0100000000000000001100110000111110000000000111111111000111111100
Y=2 X=1,0100000000000000001100110000111110000000000111111010010001111100
Y=2 X=2,0100000000000000001100110000111110000000000111111111010111101111
Y=2 X=3,0100000000000000001100110000111110000000000111111111111111111111
Y=3 X=0,0100000000000000001100110000111110000000000111111111111111111100
Y=3 X=1,0100000000000000001100110000111110000000000111111010100111111010
Y=3 X=2,0100000000000000001100110000111110000000000111111010111011111111
Y=3 X=3,0100000000000000001100110000111110000000000111111111111111111111

1
Y=0 X=0,0100000000000000001100110000111110000000000111111111111111111111
Y=0 X=1,0100000000000000001100110000111110000000000111111111111111100111
Y=0 X=2,0100000000000000001100110000111110000000000111111111111111111111
Y=0 X=3,0100000000000000001100110000111110000000000111111111111111111111
Y=1 X=0,0100000000000000001100110000111110000000000111111111111111111111
Y=1 X=1,0100000000000000001100110000111110000000000111111001111111111100
Y=1 X=2,0100000000000000001100110000111110000000000111001010011111111111
Y=1 X=3,0100000000000000001100110000111110000000000111111111111111111111
Y=2 X=0,0100000000000000001100110000111110000000000111111111000111111100
Y=2 X=1,0100000000000000001100110000111110000000000111111010010001111100
Y=2 X=2,0100000000000000001100110000111110000000000111111111010111101111
Y=2 X=3,0100000000000000001100110000111110000000000111111111111111111111
Y=3 X=0,0100000000000000001100110000111110000000000111111111111111111100
Y=3 X=1,0100000000000000001100110000111110000000000111111010100111111010
Y=3 X=2,0100000000000000001100110000111110000000000111111010111011111111
Y=3 X=3,0100000000000000001100110000111110000000000111111111111111111111

";
        decode(input);
        assert!(false);
    }
}
