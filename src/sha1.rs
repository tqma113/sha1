///
/// sha1
///
/// block length: 512
/// byte count of peer block: 512 / 8 = 6
/// latest block length: 448
/// byte count in latest block: 448 / 8 = 56
/// <length> part length: 512 - 448 = 64
/// byte count of <length> part: 64 / 8 = 8
///

static BYTE_LENGTH_OF_PEER_BLOCK: usize = 64;
static BYTE_LENGTH_OF_LATEST_BLOCK: usize = 56;

static STATE: (u32, u32, u32, u32, u32) =
    (0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0);
static K: (u32, u32, u32, u32) = (0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6);

pub fn sha1(bytes: &[u8]) -> Result<String, String> {
    if (bytes.len() * 8) as u64 > 1 << 64 - 1 {
        Err("The length of input is more than 1<<64!".to_string())
    } else {
        let u8_blocks = format_bytes_to_u8_blocks(bytes);
        let u32_blocks = u8_blocks_to_u32_blocks(u8_blocks.clone());
        let blocks_after_expand = expand_u32_blocks(u32_blocks.clone());
        let state = transform_blocks(blocks_after_expand);
        let hash = format!(
            "{:X}{:X}{:X}{:X}{:X}",
            state.0, state.1, state.2, state.3, state.4
        );

        Ok(hash)
    }
}

fn format_bytes_to_u8_blocks(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let bytes_length = bytes.len();

    // 前面完整块
    let blocks_length = bytes_length / BYTE_LENGTH_OF_PEER_BLOCK;
    for i in 0..blocks_length {
        let start = i * 64;
        let end = (i + 1) * 64;
        blocks.push(bytes[start..end].to_vec());
    }

    // 最后一个不完整块
    let rest_bytes = &bytes
        [((bytes_length / BYTE_LENGTH_OF_PEER_BLOCK) * BYTE_LENGTH_OF_PEER_BLOCK)..bytes_length];
    let mut next_block: Vec<u8> = Vec::new();
    for byte in rest_bytes {
        next_block.push(*byte)
    }
    // 添加第一个补位 8 位数
    next_block.push(1 << 7);
    let rest_bytes_length = rest_bytes.len();
    if rest_bytes_length >= BYTE_LENGTH_OF_LATEST_BLOCK {
        // 当前块用 0 补满，长度在下一个块
        let need_fill_block_count = BYTE_LENGTH_OF_PEER_BLOCK - rest_bytes_length - 1;
        for _ in 0..need_fill_block_count {
            next_block.push(0);
        }
        blocks.push(next_block);
        // 最后一个块
        let mut latest_block: Vec<u8> = Vec::new();
        for _ in 0..BYTE_LENGTH_OF_LATEST_BLOCK {
            latest_block.push(0)
        }
        // 长度部分
        let mark: usize = (2 << 8) - 1;
        let bit_length = bytes_length * 8;
        for bit in (0..64).step_by(8) {
            let right_bit = BYTE_LENGTH_OF_PEER_BLOCK - bit - 8;
            let cur_u8 = (bit_length >> right_bit) & mark;
            latest_block.push(cur_u8 as u8);
        }
        blocks.push(latest_block);
    } else {
        // 长度在当前块，中间用 0 补满
        let need_fill_block_count = BYTE_LENGTH_OF_LATEST_BLOCK - rest_bytes_length - 1;
        for _ in 0..need_fill_block_count {
            next_block.push(0);
        }
        // 长度部分
        let mark: usize = (2 << 8) - 1;
        let bit_length = bytes_length * 8;
        for bit in (0..64).step_by(8) {
            let right_bit = BYTE_LENGTH_OF_PEER_BLOCK - bit - 8;
            let cur_u8 = (bit_length >> right_bit) & mark;
            next_block.push(cur_u8 as u8);
        }
        blocks.push(next_block);
    };

    blocks
}

fn u8_blocks_to_u32_blocks(u8_blocks: Vec<Vec<u8>>) -> Vec<Vec<u32>> {
    let mut blocks: Vec<Vec<u32>> = Vec::new();

    for u8_block in u8_blocks {
        let mut block: Vec<u32> = Vec::new();
        for index in 0..16 {
            let letter = u32::from(u8_block[index * 4]) << 24
                | u32::from(u8_block[index * 4 + 1]) << 16
                | u32::from(u8_block[index * 4 + 2]) << 8
                | u32::from(u8_block[index * 4 + 3]);
            block.push(letter);
        }
        blocks.push(block);
    }

    blocks
}

fn expand_u32_blocks(blocks: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut blocks_after_expand: Vec<Vec<u32>> = Vec::new();
    for block in blocks.clone() {
        let mut block_after_expand: Vec<u32> = Vec::new();
        for i in 0..16 as usize {
            block_after_expand.push(block[i]);
        }

        for idx in 16..80 as usize {
            let letter = block_after_expand[idx - 3]
                ^ block_after_expand[idx - 8]
                ^ block_after_expand[idx - 14]
                ^ block_after_expand[idx - 16];
            block_after_expand.push(circular_left_shift(letter, 1));
        }
        blocks_after_expand.push(block_after_expand);
    }

    blocks_after_expand
}

fn transform_blocks(blocks: Vec<Vec<u32>>) -> (u32, u32, u32, u32, u32) {
    let mut state = STATE.clone();
    for block in blocks {
        let result = transform_block(block, state.clone());
        state = add_state(state, result);
    }

    state
}

fn transform_block(block: Vec<u32>, state: (u32, u32, u32, u32, u32)) -> (u32, u32, u32, u32, u32) {
    let (mut s1, mut s2, mut s3, mut s4, mut s5) = state.clone();

    for i in 0..20 as usize {
        let temp = circular_left_shift(s1, 5) + ((s2 & s3) | ((!s2) & s4)) + s5 + block[i] + K.0;
        s5 = s4;
        s4 = s3;
        s3 = circular_left_shift(s2, 30);
        s2 = s1;
        s1 = temp;
    }

    for i in 20..40 as usize {
        let temp = circular_left_shift(s1, 5) + (s2 ^ s3 ^ s4) + s5 + block[i] + K.1;
        s5 = s4;
        s4 = s3;
        s3 = circular_left_shift(s2, 30);
        s2 = s1;
        s1 = temp;
    }

    for i in 40..60 as usize {
        let temp =
            circular_left_shift(s1, 5) + ((s2 & s3) | (s2 & s4) | (s3 & s4)) + s5 + block[i] + K.2;
        s5 = s4;
        s4 = s3;
        s3 = circular_left_shift(s2, 30);
        s2 = s1;
        s1 = temp;
    }

    for i in 60..80 as usize {
        let temp = circular_left_shift(s1, 5) + (s2 ^ s3 ^ s4) + s5 + block[i] + K.3;
        s5 = s4;
        s4 = s3;
        s3 = circular_left_shift(s2, 30);
        s2 = s1;
        s1 = temp;
    }

    (s1, s2, s3, s4, s5)
}

fn add_state(
    a: (u32, u32, u32, u32, u32),
    b: (u32, u32, u32, u32, u32),
) -> (u32, u32, u32, u32, u32) {
    (
        u32::wrapping_add(a.0, b.0),
        u32::wrapping_add(a.1, b.1),
        u32::wrapping_add(a.2, b.2),
        u32::wrapping_add(a.3, b.3),
        u32::wrapping_add(a.4, b.4),
    )
}

fn circular_left_shift(input: u32, n: u32) -> u32 {
    (input << n) | (input >> (32 - n))
}

#[cfg(test)]
mod tests {
    #[test]
    fn circular_left_shift_test() {
        assert_eq!(super::circular_left_shift(0x67452301, 5), 0xE8A4602C);
    }

    #[test]
    fn add_state_test() {
        assert_eq!(
            super::add_state((1, 2, 3, 4, 5), (1, 2, 3, 4, 5)),
            (2, 4, 6, 8, 10)
        )
    }

    #[test]
    fn transform_block_test() {
        let block = vec![
            825307520, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 1650615040, 0, 48, 3301230080,
            0, 96, 2307492865, 48, 3301230272, 320018435, 0, 384, 640036870, 240, 3621248259,
            1280073820, 1296910849, 1536, 2560147672, 320019395, 1600091151, 825327857, 320018435,
            6336, 1903171681, 3840, 2105397309, 3301311908, 1566387221, 24768, 2594116938,
            572695826, 1229710577, 320343827, 825328625, 1600058575, 68328980, 62976, 1329972175,
            1281382796, 2324542810, 825724145, 3135392938, 573196770, 3855505269, 830537457,
            2104948525, 824789489, 2263767383, 983040, 2110796157, 4147389685, 1563170141, 6340608,
            2668972698, 581243810, 1841480015, 403644179, 11694592, 314090835, 3140965421,
            16124416, 3966403430, 1365777661,
        ];
        let initial_state = (0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0);
        let state = super::add_state(super::transform_block(block, initial_state), initial_state);
        assert_eq!(
            state,
            (0x6216F8A7, 0x5FD5BB3D, 0x5F22B6F9, 0x958CDEDE, 0x3FC086C2)
        )
    }

    #[test]
    fn sha1_test() {
        {
            let input = String::from("111");
            let result = super::sha1(&input.as_bytes());
            match result {
                Ok(hash) => {
                    assert_eq!(hash, "6216F8A75FD5BB3D5F22B6F9958CDEDE3FC086C2");
                }
                Err(e) => {
                    panic!(e);
                }
            }
        }
        {
            let input = String::from("1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890");
            let result = super::sha1(&input.as_bytes());
            match result {
                Ok(hash) => {
                    assert_eq!(hash, "FECFD28BBC9345891A66D7C1B8FF46E6192D284");
                }
                Err(e) => {
                    panic!(e);
                }
            }
        }
        {
            let input = String::from("12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890");
            let result = super::sha1(&input.as_bytes());
            match result {
                Ok(hash) => {
                    assert_eq!(hash, "F0026E50BEA5E90DD113D933312A8A6C7FB06F7B");
                }
                Err(e) => {
                    panic!(e);
                }
            }
        }
        {
            let input = String::from("你好");
            let result = super::sha1(&input.as_bytes());
            match result {
                Ok(hash) => {
                    assert_eq!(hash, "440EE0853AD1E99F962B63E459EF992D7C211722");
                }
                Err(e) => {
                    panic!(e);
                }
            }
        }
        {
            let input = String::from("1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm");
            let result = super::sha1(&input.as_bytes());
            match result {
                Ok(hash) => {
                    assert_eq!(hash, "F44A816E8E7FEDCF53145D11C2C1DCAF5543F8AF");
                }
                Err(e) => {
                    panic!(e);
                }
            }
        }
    }
}
