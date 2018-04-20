
use common::*;

fn add_one_to_buffer(buffer: &mut [u32], mut i: usize) {
    loop {
        buffer[i] = match buffer[i] {
            BLUE => GREEN,
            GREEN => RED,
            RED => YELLOW,
            YELLOW => PURPLE,
            PURPLE => GREY,
            GREY => WHITE,
            WHITE => BLACK,
            BLACK => BLUE,
            other => other,
        };

        if buffer[i] != BLUE || i == 0 {
            break;
        }

        i -= 1;
    }
}

fn add_n_to_buffer(buffer: &mut [u32], mut n: u32) {
    let len = buffer.len();
    for i in (0..len).rev() {
        for _ in 0..(n & 0b111) {
            add_one_to_buffer(buffer, i);
        }

        n >>= 3;

        if n == 0 {
            break;
        }
    }
}

fn nor(b1: bool, b2: bool) -> bool {
    !(b1 || b2)
}

#[inline]
pub fn update_and_render(state: &mut Framebuffer, input: Input) {
    let buffer = &mut state.buffer;
    if input.pressed_this_frame(Button::Left) {
        if nor(nor(nor(nor(nor(false, false), nor(nor(match buffer[14] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[16] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), match buffer[15] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(false, false), nor(false, false))), nor(nor(nor(match buffer[2] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[10] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), nor(false, match buffer[18] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(false, false), nor(nor(match buffer[5] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[7] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }), false)))), nor(nor(nor(nor(false, match buffer[3] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), nor(nor(match buffer[19] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[11] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), match buffer[9] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(false, false), nor(match buffer[21] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, nor(match buffer[8] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[17] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })))), nor(nor(nor(match buffer[20] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[6] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(false, match buffer[0] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, })), nor(nor(match buffer[12] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }, nor(match buffer[4] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[1] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, })), nor(false, false))))) {
	let n = match buffer[24] {
            BLUE => 204,
            GREEN => 162,
            RED => 113,
            YELLOW => 194,
            PURPLE => 149,
            GREY => 95,
            WHITE => 40,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 238,
            GREEN => 28,
            RED => 136,
            YELLOW => 248,
            PURPLE => 199,
            GREY => 7,
            WHITE => 251,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 147,
            GREEN => 45,
            RED => 188,
            YELLOW => 88,
            PURPLE => 145,
            GREY => 223,
            WHITE => 0,
            BLACK => 208,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 69,
            GREEN => 54,
            RED => 251,
            YELLOW => 35,
            PURPLE => 2,
            GREY => 210,
            WHITE => 234,
            BLACK => 233,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 83,
            GREEN => 204,
            RED => 245,
            YELLOW => 78,
            PURPLE => 44,
            GREY => 115,
            WHITE => 11,
            BLACK => 167,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 17,
            GREEN => 107,
            RED => 175,
            YELLOW => 66,
            PURPLE => 117,
            GREY => 181,
            WHITE => 59,
            BLACK => 143,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 134,
            GREEN => 131,
            RED => 118,
            YELLOW => 93,
            PURPLE => 14,
            GREY => 205,
            WHITE => 19,
            BLACK => 167,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 46,
            GREEN => 50,
            RED => 158,
            YELLOW => 99,
            PURPLE => 133,
            GREY => 16,
            WHITE => 166,
            BLACK => 241,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 35,
            GREEN => 190,
            RED => 87,
            YELLOW => 61,
            PURPLE => 171,
            GREY => 110,
            WHITE => 42,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 252,
            GREEN => 26,
            RED => 183,
            YELLOW => 180,
            PURPLE => 30,
            GREY => 162,
            WHITE => 188,
            BLACK => 10,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 21,
            GREEN => 66,
            RED => 213,
            YELLOW => 193,
            PURPLE => 243,
            GREY => 48,
            WHITE => 137,
            BLACK => 142,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 162,
            GREEN => 192,
            RED => 127,
            YELLOW => 53,
            PURPLE => 119,
            GREY => 15,
            WHITE => 246,
            BLACK => 165,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 208,
            GREEN => 6,
            RED => 151,
            YELLOW => 40,
            PURPLE => 34,
            GREY => 223,
            WHITE => 67,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 139,
            GREEN => 101,
            RED => 172,
            YELLOW => 80,
            PURPLE => 111,
            GREY => 132,
            WHITE => 98,
            BLACK => 153,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 225,
            GREEN => 127,
            RED => 118,
            YELLOW => 96,
            PURPLE => 219,
            GREY => 31,
            WHITE => 87,
            BLACK => 146,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 61,
            GREEN => 173,
            RED => 14,
            YELLOW => 130,
            PURPLE => 233,
            GREY => 14,
            WHITE => 87,
            BLACK => 250,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 56,
            GREEN => 24,
            RED => 172,
            YELLOW => 163,
            PURPLE => 82,
            GREY => 36,
            WHITE => 55,
            BLACK => 184,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 19,
            GREEN => 64,
            RED => 159,
            YELLOW => 59,
            PURPLE => 119,
            GREY => 89,
            WHITE => 200,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 119,
            GREEN => 95,
            RED => 12,
            YELLOW => 235,
            PURPLE => 104,
            GREY => 38,
            WHITE => 187,
            BLACK => 206,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 227,
            GREEN => 247,
            RED => 194,
            YELLOW => 82,
            PURPLE => 155,
            GREY => 2,
            WHITE => 218,
            BLACK => 222,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 46,
            GREEN => 136,
            RED => 171,
            YELLOW => 216,
            PURPLE => 249,
            GREY => 121,
            WHITE => 228,
            BLACK => 55,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 169,
            GREEN => 203,
            RED => 173,
            YELLOW => 222,
            PURPLE => 149,
            GREY => 109,
            WHITE => 38,
            BLACK => 239,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 183,
            GREEN => 218,
            RED => 50,
            YELLOW => 164,
            PURPLE => 212,
            GREY => 124,
            WHITE => 211,
            BLACK => 219,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 119,
            GREEN => 19,
            RED => 80,
            YELLOW => 60,
            PURPLE => 123,
            GREY => 206,
            WHITE => 125,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 240,
            GREEN => 80,
            RED => 200,
            YELLOW => 160,
            PURPLE => 33,
            GREY => 90,
            WHITE => 119,
            BLACK => 146,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 112,
            GREEN => 197,
            RED => 241,
            YELLOW => 164,
            PURPLE => 239,
            GREY => 147,
            WHITE => 123,
            BLACK => 106,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 28,
            GREEN => 253,
            RED => 163,
            YELLOW => 182,
            PURPLE => 209,
            GREY => 127,
            WHITE => 123,
            BLACK => 251,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 55,
            GREEN => 148,
            RED => 173,
            YELLOW => 97,
            PURPLE => 218,
            GREY => 170,
            WHITE => 155,
            BLACK => 198,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 143,
            GREEN => 251,
            RED => 67,
            YELLOW => 110,
            PURPLE => 13,
            GREY => 235,
            WHITE => 215,
            BLACK => 218,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 5,
            GREEN => 159,
            RED => 125,
            YELLOW => 5,
            PURPLE => 135,
            GREY => 157,
            WHITE => 215,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 59,
            GREEN => 30,
            RED => 116,
            YELLOW => 94,
            PURPLE => 71,
            GREY => 38,
            WHITE => 235,
            BLACK => 136,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 163,
            GREEN => 105,
            RED => 67,
            YELLOW => 172,
            PURPLE => 208,
            GREY => 18,
            WHITE => 55,
            BLACK => 38,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 139,
            GREEN => 53,
            RED => 106,
            YELLOW => 197,
            PURPLE => 42,
            GREY => 79,
            WHITE => 189,
            BLACK => 27,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 152,
            GREEN => 76,
            RED => 92,
            YELLOW => 49,
            PURPLE => 16,
            GREY => 130,
            WHITE => 23,
            BLACK => 196,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 40,
            GREEN => 118,
            RED => 94,
            YELLOW => 109,
            PURPLE => 70,
            GREY => 109,
            WHITE => 167,
            BLACK => 64,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 218,
            GREEN => 181,
            RED => 15,
            YELLOW => 218,
            PURPLE => 170,
            GREY => 198,
            WHITE => 193,
            BLACK => 89,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 215,
            GREEN => 58,
            RED => 43,
            YELLOW => 39,
            PURPLE => 12,
            GREY => 115,
            WHITE => 83,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 29,
            GREEN => 201,
            RED => 92,
            YELLOW => 226,
            PURPLE => 104,
            GREY => 161,
            WHITE => 80,
            BLACK => 79,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 204,
            GREEN => 143,
            RED => 121,
            YELLOW => 1,
            PURPLE => 68,
            GREY => 183,
            WHITE => 209,
            BLACK => 206,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 118,
            GREEN => 171,
            RED => 95,
            YELLOW => 90,
            PURPLE => 250,
            GREY => 246,
            WHITE => 135,
            BLACK => 166,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 206,
            GREEN => 211,
            RED => 197,
            YELLOW => 46,
            PURPLE => 233,
            GREY => 63,
            WHITE => 247,
            BLACK => 13,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 93,
            GREEN => 126,
            RED => 211,
            YELLOW => 99,
            PURPLE => 157,
            GREY => 57,
            WHITE => 72,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 186,
            GREEN => 45,
            RED => 183,
            YELLOW => 122,
            PURPLE => 169,
            GREY => 225,
            WHITE => 34,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 22,
            GREEN => 182,
            RED => 6,
            YELLOW => 55,
            PURPLE => 168,
            GREY => 236,
            WHITE => 243,
            BLACK => 223,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else {
	let n = match buffer[19] {
            BLUE => 224,
            GREEN => 173,
            RED => 128,
            YELLOW => 2,
            PURPLE => 3,
            GREY => 126,
            WHITE => 180,
            BLACK => 47,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[88] {
            BLUE => 132,
            GREEN => 225,
            RED => 208,
            YELLOW => 99,
            PURPLE => 43,
            GREY => 137,
            WHITE => 224,
            BLACK => 92,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 78,
            GREEN => 44,
            RED => 70,
            YELLOW => 29,
            PURPLE => 227,
            GREY => 174,
            WHITE => 112,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[75] {
            BLUE => 189,
            GREEN => 145,
            RED => 178,
            YELLOW => 82,
            PURPLE => 140,
            GREY => 131,
            WHITE => 210,
            BLACK => 236,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 186,
            GREEN => 38,
            RED => 92,
            YELLOW => 76,
            PURPLE => 203,
            GREY => 76,
            WHITE => 63,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 170,
            GREEN => 195,
            RED => 81,
            YELLOW => 77,
            PURPLE => 183,
            GREY => 201,
            WHITE => 100,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 25,
            GREEN => 188,
            RED => 142,
            YELLOW => 178,
            PURPLE => 181,
            GREY => 234,
            WHITE => 162,
            BLACK => 145,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 220,
            GREEN => 45,
            RED => 119,
            YELLOW => 204,
            PURPLE => 208,
            GREY => 147,
            WHITE => 4,
            BLACK => 242,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[82] {
            BLUE => 37,
            GREEN => 225,
            RED => 200,
            YELLOW => 117,
            PURPLE => 222,
            GREY => 171,
            WHITE => 149,
            BLACK => 111,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 104,
            GREEN => 118,
            RED => 153,
            YELLOW => 238,
            PURPLE => 85,
            GREY => 1,
            WHITE => 245,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 52,
            GREEN => 114,
            RED => 168,
            YELLOW => 61,
            PURPLE => 178,
            GREY => 144,
            WHITE => 122,
            BLACK => 196,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 146,
            GREEN => 83,
            RED => 150,
            YELLOW => 229,
            PURPLE => 185,
            GREY => 150,
            WHITE => 239,
            BLACK => 180,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[86] {
            BLUE => 219,
            GREEN => 42,
            RED => 75,
            YELLOW => 176,
            PURPLE => 167,
            GREY => 153,
            WHITE => 91,
            BLACK => 26,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[73] {
            BLUE => 51,
            GREEN => 20,
            RED => 65,
            YELLOW => 225,
            PURPLE => 80,
            GREY => 173,
            WHITE => 160,
            BLACK => 82,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[76] {
            BLUE => 75,
            GREEN => 104,
            RED => 83,
            YELLOW => 16,
            PURPLE => 134,
            GREY => 14,
            WHITE => 198,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 213,
            GREEN => 147,
            RED => 21,
            YELLOW => 190,
            PURPLE => 39,
            GREY => 41,
            WHITE => 120,
            BLACK => 44,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 70,
            GREEN => 165,
            RED => 75,
            YELLOW => 35,
            PURPLE => 163,
            GREY => 1,
            WHITE => 139,
            BLACK => 172,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 136,
            GREEN => 155,
            RED => 48,
            YELLOW => 114,
            PURPLE => 105,
            GREY => 239,
            WHITE => 130,
            BLACK => 72,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 86,
            GREEN => 6,
            RED => 54,
            YELLOW => 124,
            PURPLE => 70,
            GREY => 233,
            WHITE => 43,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[72] {
            BLUE => 12,
            GREEN => 255,
            RED => 101,
            YELLOW => 131,
            PURPLE => 170,
            GREY => 44,
            WHITE => 13,
            BLACK => 52,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 213,
            GREEN => 199,
            RED => 102,
            YELLOW => 82,
            PURPLE => 57,
            GREY => 34,
            WHITE => 249,
            BLACK => 116,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 8,
            GREEN => 107,
            RED => 143,
            YELLOW => 38,
            PURPLE => 18,
            GREY => 106,
            WHITE => 223,
            BLACK => 234,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 52,
            GREEN => 152,
            RED => 100,
            YELLOW => 170,
            PURPLE => 202,
            GREY => 2,
            WHITE => 114,
            BLACK => 14,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 27,
            GREEN => 61,
            RED => 194,
            YELLOW => 177,
            PURPLE => 43,
            GREY => 117,
            WHITE => 22,
            BLACK => 1,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 37,
            GREEN => 147,
            RED => 52,
            YELLOW => 52,
            PURPLE => 179,
            GREY => 121,
            WHITE => 70,
            BLACK => 47,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 118,
            GREEN => 38,
            RED => 85,
            YELLOW => 190,
            PURPLE => 252,
            GREY => 38,
            WHITE => 118,
            BLACK => 123,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 71,
            GREEN => 35,
            RED => 86,
            YELLOW => 119,
            PURPLE => 129,
            GREY => 52,
            WHITE => 206,
            BLACK => 210,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 253,
            GREEN => 79,
            RED => 17,
            YELLOW => 6,
            PURPLE => 174,
            GREY => 9,
            WHITE => 242,
            BLACK => 10,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 97,
            GREEN => 43,
            RED => 170,
            YELLOW => 194,
            PURPLE => 161,
            GREY => 70,
            WHITE => 2,
            BLACK => 215,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[79] {
            BLUE => 255,
            GREEN => 102,
            RED => 77,
            YELLOW => 136,
            PURPLE => 234,
            GREY => 94,
            WHITE => 207,
            BLACK => 70,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 102,
            GREEN => 155,
            RED => 126,
            YELLOW => 57,
            PURPLE => 21,
            GREY => 84,
            WHITE => 249,
            BLACK => 28,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 47,
            GREEN => 26,
            RED => 11,
            YELLOW => 44,
            PURPLE => 169,
            GREY => 65,
            WHITE => 124,
            BLACK => 246,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 86,
            GREEN => 102,
            RED => 73,
            YELLOW => 182,
            PURPLE => 242,
            GREY => 27,
            WHITE => 86,
            BLACK => 209,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[81] {
            BLUE => 86,
            GREEN => 11,
            RED => 224,
            YELLOW => 58,
            PURPLE => 120,
            GREY => 216,
            WHITE => 235,
            BLACK => 246,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 1,
            GREEN => 166,
            RED => 77,
            YELLOW => 156,
            PURPLE => 68,
            GREY => 219,
            WHITE => 55,
            BLACK => 166,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[65] {
            BLUE => 66,
            GREEN => 235,
            RED => 253,
            YELLOW => 128,
            PURPLE => 194,
            GREY => 2,
            WHITE => 225,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 128,
            GREEN => 16,
            RED => 36,
            YELLOW => 187,
            PURPLE => 85,
            GREY => 163,
            WHITE => 248,
            BLACK => 33,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[92] {
            BLUE => 251,
            GREEN => 255,
            RED => 8,
            YELLOW => 99,
            PURPLE => 251,
            GREY => 61,
            WHITE => 48,
            BLACK => 200,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 140,
            GREEN => 36,
            RED => 21,
            YELLOW => 224,
            PURPLE => 187,
            GREY => 30,
            WHITE => 243,
            BLACK => 231,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 86,
            GREEN => 242,
            RED => 102,
            YELLOW => 219,
            PURPLE => 62,
            GREY => 212,
            WHITE => 1,
            BLACK => 219,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[78] {
            BLUE => 4,
            GREEN => 240,
            RED => 138,
            YELLOW => 210,
            PURPLE => 11,
            GREY => 39,
            WHITE => 218,
            BLACK => 159,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 163,
            GREEN => 36,
            RED => 241,
            YELLOW => 65,
            PURPLE => 240,
            GREY => 111,
            WHITE => 92,
            BLACK => 28,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[89] {
            BLUE => 126,
            GREEN => 77,
            RED => 232,
            YELLOW => 246,
            PURPLE => 180,
            GREY => 64,
            WHITE => 37,
            BLACK => 42,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 106,
            GREEN => 17,
            RED => 3,
            YELLOW => 145,
            PURPLE => 2,
            GREY => 14,
            WHITE => 171,
            BLACK => 111,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 170,
            GREEN => 49,
            RED => 158,
            YELLOW => 39,
            PURPLE => 136,
            GREY => 148,
            WHITE => 188,
            BLACK => 198,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[80] {
            BLUE => 137,
            GREEN => 124,
            RED => 181,
            YELLOW => 113,
            PURPLE => 5,
            GREY => 47,
            WHITE => 71,
            BLACK => 185,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[69] {
            BLUE => 135,
            GREEN => 37,
            RED => 137,
            YELLOW => 90,
            PURPLE => 199,
            GREY => 136,
            WHITE => 28,
            BLACK => 111,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[70] {
            BLUE => 157,
            GREEN => 30,
            RED => 22,
            YELLOW => 209,
            PURPLE => 33,
            GREY => 187,
            WHITE => 247,
            BLACK => 112,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 225,
            GREEN => 159,
            RED => 203,
            YELLOW => 124,
            PURPLE => 194,
            GREY => 84,
            WHITE => 189,
            BLACK => 24,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 82,
            GREEN => 133,
            RED => 169,
            YELLOW => 54,
            PURPLE => 56,
            GREY => 70,
            WHITE => 151,
            BLACK => 236,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 60,
            GREEN => 99,
            RED => 52,
            YELLOW => 18,
            PURPLE => 114,
            GREY => 124,
            WHITE => 140,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 85,
            GREEN => 146,
            RED => 236,
            YELLOW => 227,
            PURPLE => 129,
            GREY => 20,
            WHITE => 183,
            BLACK => 60,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 143,
            GREEN => 112,
            RED => 84,
            YELLOW => 123,
            PURPLE => 128,
            GREY => 64,
            WHITE => 98,
            BLACK => 182,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 128,
            GREEN => 191,
            RED => 247,
            YELLOW => 217,
            PURPLE => 138,
            GREY => 37,
            WHITE => 241,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 179,
            GREEN => 32,
            RED => 188,
            YELLOW => 151,
            PURPLE => 196,
            GREY => 182,
            WHITE => 52,
            BLACK => 196,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[90] {
            BLUE => 201,
            GREEN => 0,
            RED => 18,
            YELLOW => 82,
            PURPLE => 193,
            GREY => 219,
            WHITE => 52,
            BLACK => 48,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[74] {
            BLUE => 188,
            GREEN => 6,
            RED => 63,
            YELLOW => 240,
            PURPLE => 99,
            GREY => 145,
            WHITE => 63,
            BLACK => 121,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 1,
            GREEN => 53,
            RED => 187,
            YELLOW => 197,
            PURPLE => 222,
            GREY => 209,
            WHITE => 176,
            BLACK => 123,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 33,
            GREEN => 162,
            RED => 31,
            YELLOW => 199,
            PURPLE => 127,
            GREY => 109,
            WHITE => 154,
            BLACK => 53,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[95] {
            BLUE => 171,
            GREEN => 232,
            RED => 171,
            YELLOW => 199,
            PURPLE => 138,
            GREY => 151,
            WHITE => 126,
            BLACK => 244,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 61,
            GREEN => 224,
            RED => 116,
            YELLOW => 39,
            PURPLE => 114,
            GREY => 3,
            WHITE => 97,
            BLACK => 111,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 191,
            GREEN => 193,
            RED => 24,
            YELLOW => 252,
            PURPLE => 16,
            GREY => 101,
            WHITE => 205,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 33,
            GREEN => 110,
            RED => 150,
            YELLOW => 58,
            PURPLE => 117,
            GREY => 161,
            WHITE => 246,
            BLACK => 216,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[94] {
            BLUE => 168,
            GREEN => 212,
            RED => 233,
            YELLOW => 249,
            PURPLE => 129,
            GREY => 244,
            WHITE => 164,
            BLACK => 138,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 1,
            GREEN => 118,
            RED => 66,
            YELLOW => 72,
            PURPLE => 180,
            GREY => 106,
            WHITE => 189,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[91] {
            BLUE => 239,
            GREEN => 77,
            RED => 15,
            YELLOW => 198,
            PURPLE => 95,
            GREY => 28,
            WHITE => 174,
            BLACK => 212,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 13,
            GREEN => 17,
            RED => 141,
            YELLOW => 69,
            PURPLE => 154,
            GREY => 233,
            WHITE => 122,
            BLACK => 108,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 153,
            GREEN => 51,
            RED => 70,
            YELLOW => 170,
            PURPLE => 39,
            GREY => 222,
            WHITE => 103,
            BLACK => 143,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 91,
            GREEN => 154,
            RED => 247,
            YELLOW => 135,
            PURPLE => 222,
            GREY => 93,
            WHITE => 150,
            BLACK => 1,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 121,
            GREEN => 95,
            RED => 102,
            YELLOW => 247,
            PURPLE => 97,
            GREY => 188,
            WHITE => 201,
            BLACK => 151,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 44,
            GREEN => 30,
            RED => 210,
            YELLOW => 120,
            PURPLE => 179,
            GREY => 14,
            WHITE => 188,
            BLACK => 60,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[87] {
            BLUE => 7,
            GREEN => 46,
            RED => 98,
            YELLOW => 152,
            PURPLE => 221,
            GREY => 13,
            WHITE => 250,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 149,
            GREEN => 238,
            RED => 183,
            YELLOW => 81,
            PURPLE => 86,
            GREY => 138,
            WHITE => 84,
            BLACK => 74,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 83,
            GREEN => 194,
            RED => 114,
            YELLOW => 150,
            PURPLE => 14,
            GREY => 66,
            WHITE => 250,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[77] {
            BLUE => 167,
            GREEN => 146,
            RED => 19,
            YELLOW => 229,
            PURPLE => 137,
            GREY => 106,
            WHITE => 54,
            BLACK => 222,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 160,
            GREEN => 2,
            RED => 77,
            YELLOW => 109,
            PURPLE => 16,
            GREY => 5,
            WHITE => 81,
            BLACK => 182,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[71] {
            BLUE => 57,
            GREEN => 102,
            RED => 102,
            YELLOW => 163,
            PURPLE => 156,
            GREY => 169,
            WHITE => 122,
            BLACK => 16,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 42,
            GREEN => 210,
            RED => 150,
            YELLOW => 252,
            PURPLE => 51,
            GREY => 63,
            WHITE => 16,
            BLACK => 130,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 136,
            GREEN => 217,
            RED => 55,
            YELLOW => 35,
            PURPLE => 70,
            GREY => 90,
            WHITE => 202,
            BLACK => 235,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 179,
            GREEN => 93,
            RED => 142,
            YELLOW => 233,
            PURPLE => 134,
            GREY => 129,
            WHITE => 251,
            BLACK => 15,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[83] {
            BLUE => 234,
            GREEN => 219,
            RED => 47,
            YELLOW => 105,
            PURPLE => 179,
            GREY => 160,
            WHITE => 5,
            BLACK => 164,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 208,
            GREEN => 89,
            RED => 196,
            YELLOW => 172,
            PURPLE => 99,
            GREY => 218,
            WHITE => 3,
            BLACK => 74,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 147,
            GREEN => 37,
            RED => 98,
            YELLOW => 79,
            PURPLE => 5,
            GREY => 37,
            WHITE => 125,
            BLACK => 113,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 16,
            GREEN => 60,
            RED => 163,
            YELLOW => 12,
            PURPLE => 74,
            GREY => 250,
            WHITE => 193,
            BLACK => 33,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 83,
            GREEN => 181,
            RED => 129,
            YELLOW => 77,
            PURPLE => 127,
            GREY => 74,
            WHITE => 48,
            BLACK => 152,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 188,
            GREEN => 231,
            RED => 171,
            YELLOW => 53,
            PURPLE => 193,
            GREY => 181,
            WHITE => 43,
            BLACK => 120,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 178,
            GREEN => 160,
            RED => 18,
            YELLOW => 106,
            PURPLE => 112,
            GREY => 87,
            WHITE => 60,
            BLACK => 101,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[84] {
            BLUE => 183,
            GREEN => 30,
            RED => 222,
            YELLOW => 73,
            PURPLE => 208,
            GREY => 66,
            WHITE => 93,
            BLACK => 255,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 160,
            GREEN => 102,
            RED => 171,
            YELLOW => 102,
            PURPLE => 147,
            GREY => 90,
            WHITE => 204,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 108,
            GREEN => 68,
            RED => 218,
            YELLOW => 45,
            PURPLE => 22,
            GREY => 39,
            WHITE => 188,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 230,
            GREEN => 110,
            RED => 153,
            YELLOW => 74,
            PURPLE => 114,
            GREY => 168,
            WHITE => 6,
            BLACK => 146,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[85] {
            BLUE => 87,
            GREEN => 185,
            RED => 254,
            YELLOW => 241,
            PURPLE => 230,
            GREY => 50,
            WHITE => 206,
            BLACK => 251,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[96] {
            BLUE => 29,
            GREEN => 90,
            RED => 194,
            YELLOW => 24,
            PURPLE => 48,
            GREY => 115,
            WHITE => 195,
            BLACK => 110,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[93] {
            BLUE => 223,
            GREEN => 69,
            RED => 87,
            YELLOW => 155,
            PURPLE => 60,
            GREY => 39,
            WHITE => 69,
            BLACK => 177,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 119,
            GREEN => 11,
            RED => 125,
            YELLOW => 41,
            PURPLE => 151,
            GREY => 50,
            WHITE => 177,
            BLACK => 47,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 24,
            GREEN => 171,
            RED => 46,
            YELLOW => 229,
            PURPLE => 168,
            GREY => 84,
            WHITE => 199,
            BLACK => 222,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}

    }

    if input.pressed_this_frame(Button::Right) {
        if nor(nor(nor(match buffer[2] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, false), nor(match buffer[5] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, false)), nor(nor(false, nor(match buffer[0] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[1] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, })), nor(false, match buffer[3] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }))) {
	let n = match buffer[24] {
            BLUE => 237,
            GREEN => 253,
            RED => 162,
            YELLOW => 245,
            PURPLE => 95,
            GREY => 176,
            WHITE => 189,
            BLACK => 121,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 131,
            GREEN => 223,
            RED => 189,
            YELLOW => 118,
            PURPLE => 22,
            GREY => 31,
            WHITE => 51,
            BLACK => 119,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 26,
            GREEN => 159,
            RED => 232,
            YELLOW => 175,
            PURPLE => 63,
            GREY => 111,
            WHITE => 45,
            BLACK => 61,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 58,
            GREEN => 24,
            RED => 124,
            YELLOW => 206,
            PURPLE => 40,
            GREY => 78,
            WHITE => 249,
            BLACK => 139,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 250,
            GREEN => 83,
            RED => 21,
            YELLOW => 92,
            PURPLE => 211,
            GREY => 54,
            WHITE => 156,
            BLACK => 9,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 47,
            GREEN => 79,
            RED => 19,
            YELLOW => 76,
            PURPLE => 116,
            GREY => 163,
            WHITE => 136,
            BLACK => 138,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 29,
            GREEN => 221,
            RED => 109,
            YELLOW => 196,
            PURPLE => 21,
            GREY => 188,
            WHITE => 74,
            BLACK => 17,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 214,
            GREEN => 239,
            RED => 22,
            YELLOW => 169,
            PURPLE => 58,
            GREY => 196,
            WHITE => 155,
            BLACK => 189,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 5,
            GREEN => 168,
            RED => 239,
            YELLOW => 203,
            PURPLE => 61,
            GREY => 161,
            WHITE => 7,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 212,
            GREEN => 27,
            RED => 130,
            YELLOW => 175,
            PURPLE => 122,
            GREY => 156,
            WHITE => 93,
            BLACK => 172,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 85,
            GREEN => 242,
            RED => 10,
            YELLOW => 247,
            PURPLE => 169,
            GREY => 179,
            WHITE => 185,
            BLACK => 95,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 164,
            GREEN => 189,
            RED => 94,
            YELLOW => 250,
            PURPLE => 178,
            GREY => 83,
            WHITE => 236,
            BLACK => 36,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 120,
            GREEN => 198,
            RED => 65,
            YELLOW => 42,
            PURPLE => 201,
            GREY => 126,
            WHITE => 134,
            BLACK => 168,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 70,
            GREEN => 130,
            RED => 193,
            YELLOW => 92,
            PURPLE => 190,
            GREY => 75,
            WHITE => 149,
            BLACK => 47,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 7,
            GREEN => 96,
            RED => 85,
            YELLOW => 174,
            PURPLE => 47,
            GREY => 185,
            WHITE => 121,
            BLACK => 163,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 155,
            GREEN => 56,
            RED => 9,
            YELLOW => 132,
            PURPLE => 43,
            GREY => 37,
            WHITE => 121,
            BLACK => 230,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 198,
            GREEN => 81,
            RED => 157,
            YELLOW => 116,
            PURPLE => 77,
            GREY => 233,
            WHITE => 79,
            BLACK => 16,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 151,
            GREEN => 114,
            RED => 56,
            YELLOW => 68,
            PURPLE => 235,
            GREY => 60,
            WHITE => 240,
            BLACK => 127,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 145,
            GREEN => 149,
            RED => 148,
            YELLOW => 204,
            PURPLE => 89,
            GREY => 10,
            WHITE => 168,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 79,
            GREEN => 170,
            RED => 147,
            YELLOW => 212,
            PURPLE => 50,
            GREY => 111,
            WHITE => 127,
            BLACK => 132,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 86,
            GREEN => 140,
            RED => 181,
            YELLOW => 1,
            PURPLE => 191,
            GREY => 89,
            WHITE => 12,
            BLACK => 60,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 230,
            GREEN => 209,
            RED => 134,
            YELLOW => 205,
            PURPLE => 187,
            GREY => 161,
            WHITE => 71,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 215,
            GREEN => 198,
            RED => 107,
            YELLOW => 68,
            PURPLE => 174,
            GREY => 85,
            WHITE => 205,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 208,
            GREEN => 128,
            RED => 146,
            YELLOW => 220,
            PURPLE => 122,
            GREY => 203,
            WHITE => 21,
            BLACK => 75,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if nor(nor(nor(nor(nor(nor(match buffer[3] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[15] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), match buffer[13] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }), nor(match buffer[7] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, nor(match buffer[27] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[4] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }))), nor(nor(nor(match buffer[6] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[20] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }), nor(match buffer[0] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[8] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, })), nor(false, nor(match buffer[29] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[26] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })))), nor(nor(nor(match buffer[25] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[21] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), nor(nor(match buffer[11] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[17] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }), match buffer[1] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(match buffer[22] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, false), nor(nor(match buffer[12] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[5] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }), false)))), nor(nor(nor(nor(match buffer[16] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, nor(match buffer[19] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[28] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, })), nor(match buffer[18] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, false)), nor(nor(match buffer[24] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, false), nor(false, match buffer[2] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }))), nor(nor(nor(match buffer[14] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[10] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), nor(match buffer[9] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, false)), nor(nor(false, false), nor(false, false))))) {
	let n = match buffer[40] {
            BLUE => 240,
            GREEN => 4,
            RED => 206,
            YELLOW => 243,
            PURPLE => 132,
            GREY => 180,
            WHITE => 13,
            BLACK => 211,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 11,
            GREEN => 162,
            RED => 27,
            YELLOW => 9,
            PURPLE => 192,
            GREY => 90,
            WHITE => 23,
            BLACK => 35,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 165,
            GREEN => 81,
            RED => 244,
            YELLOW => 76,
            PURPLE => 207,
            GREY => 175,
            WHITE => 188,
            BLACK => 197,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 67,
            GREEN => 12,
            RED => 180,
            YELLOW => 63,
            PURPLE => 66,
            GREY => 134,
            WHITE => 49,
            BLACK => 160,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 13,
            GREEN => 54,
            RED => 23,
            YELLOW => 57,
            PURPLE => 160,
            GREY => 45,
            WHITE => 40,
            BLACK => 190,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 46,
            GREEN => 118,
            RED => 230,
            YELLOW => 205,
            PURPLE => 46,
            GREY => 135,
            WHITE => 30,
            BLACK => 26,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 107,
            GREEN => 239,
            RED => 208,
            YELLOW => 19,
            PURPLE => 255,
            GREY => 246,
            WHITE => 205,
            BLACK => 120,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 26,
            GREEN => 100,
            RED => 201,
            YELLOW => 241,
            PURPLE => 97,
            GREY => 184,
            WHITE => 44,
            BLACK => 11,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 68,
            GREEN => 248,
            RED => 7,
            YELLOW => 140,
            PURPLE => 107,
            GREY => 53,
            WHITE => 108,
            BLACK => 22,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 192,
            GREEN => 33,
            RED => 2,
            YELLOW => 152,
            PURPLE => 27,
            GREY => 188,
            WHITE => 202,
            BLACK => 163,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 66,
            GREEN => 111,
            RED => 240,
            YELLOW => 3,
            PURPLE => 61,
            GREY => 55,
            WHITE => 122,
            BLACK => 105,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 179,
            GREEN => 250,
            RED => 190,
            YELLOW => 228,
            PURPLE => 245,
            GREY => 90,
            WHITE => 192,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 71,
            GREEN => 27,
            RED => 117,
            YELLOW => 172,
            PURPLE => 255,
            GREY => 218,
            WHITE => 181,
            BLACK => 90,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 99,
            GREEN => 30,
            RED => 212,
            YELLOW => 215,
            PURPLE => 200,
            GREY => 9,
            WHITE => 255,
            BLACK => 52,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 205,
            GREEN => 236,
            RED => 55,
            YELLOW => 103,
            PURPLE => 3,
            GREY => 130,
            WHITE => 54,
            BLACK => 155,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 71,
            GREEN => 32,
            RED => 44,
            YELLOW => 149,
            PURPLE => 192,
            GREY => 240,
            WHITE => 90,
            BLACK => 222,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 215,
            GREEN => 14,
            RED => 8,
            YELLOW => 57,
            PURPLE => 208,
            GREY => 239,
            WHITE => 114,
            BLACK => 136,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 202,
            GREEN => 80,
            RED => 220,
            YELLOW => 39,
            PURPLE => 20,
            GREY => 89,
            WHITE => 204,
            BLACK => 172,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 137,
            GREEN => 200,
            RED => 74,
            YELLOW => 111,
            PURPLE => 137,
            GREY => 252,
            WHITE => 224,
            BLACK => 216,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 191,
            GREEN => 136,
            RED => 70,
            YELLOW => 35,
            PURPLE => 140,
            GREY => 159,
            WHITE => 120,
            BLACK => 116,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 168,
            GREEN => 10,
            RED => 51,
            YELLOW => 202,
            PURPLE => 242,
            GREY => 18,
            WHITE => 16,
            BLACK => 247,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 98,
            GREEN => 72,
            RED => 21,
            YELLOW => 32,
            PURPLE => 158,
            GREY => 233,
            WHITE => 107,
            BLACK => 119,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 7,
            GREEN => 168,
            RED => 135,
            YELLOW => 191,
            PURPLE => 207,
            GREY => 208,
            WHITE => 142,
            BLACK => 73,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 33,
            GREEN => 23,
            RED => 99,
            YELLOW => 215,
            PURPLE => 169,
            GREY => 96,
            WHITE => 30,
            BLACK => 190,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 0,
            GREEN => 15,
            RED => 240,
            YELLOW => 208,
            PURPLE => 214,
            GREY => 233,
            WHITE => 29,
            BLACK => 101,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 80,
            GREEN => 96,
            RED => 151,
            YELLOW => 215,
            PURPLE => 78,
            GREY => 64,
            WHITE => 95,
            BLACK => 196,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 112,
            GREEN => 193,
            RED => 85,
            YELLOW => 247,
            PURPLE => 30,
            GREY => 29,
            WHITE => 252,
            BLACK => 49,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 17,
            GREEN => 170,
            RED => 249,
            YELLOW => 233,
            PURPLE => 6,
            GREY => 69,
            WHITE => 17,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 64,
            GREEN => 52,
            RED => 215,
            YELLOW => 254,
            PURPLE => 68,
            GREY => 30,
            WHITE => 78,
            BLACK => 211,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 57,
            GREEN => 95,
            RED => 149,
            YELLOW => 240,
            PURPLE => 39,
            GREY => 102,
            WHITE => 239,
            BLACK => 76,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 42,
            GREEN => 100,
            RED => 34,
            YELLOW => 235,
            PURPLE => 178,
            GREY => 230,
            WHITE => 75,
            BLACK => 104,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 24,
            GREEN => 15,
            RED => 192,
            YELLOW => 15,
            PURPLE => 70,
            GREY => 120,
            WHITE => 82,
            BLACK => 137,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 179,
            GREEN => 160,
            RED => 144,
            YELLOW => 2,
            PURPLE => 195,
            GREY => 176,
            WHITE => 120,
            BLACK => 70,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 113,
            GREEN => 185,
            RED => 19,
            YELLOW => 125,
            PURPLE => 165,
            GREY => 9,
            WHITE => 106,
            BLACK => 255,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 30,
            GREEN => 225,
            RED => 251,
            YELLOW => 219,
            PURPLE => 21,
            GREY => 238,
            WHITE => 82,
            BLACK => 43,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 104,
            GREEN => 243,
            RED => 203,
            YELLOW => 159,
            PURPLE => 116,
            GREY => 80,
            WHITE => 12,
            BLACK => 6,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 225,
            GREEN => 49,
            RED => 74,
            YELLOW => 218,
            PURPLE => 20,
            GREY => 103,
            WHITE => 53,
            BLACK => 144,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 67,
            GREEN => 108,
            RED => 244,
            YELLOW => 153,
            PURPLE => 171,
            GREY => 125,
            WHITE => 19,
            BLACK => 88,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 108,
            GREEN => 20,
            RED => 14,
            YELLOW => 96,
            PURPLE => 168,
            GREY => 132,
            WHITE => 197,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 169,
            GREEN => 83,
            RED => 59,
            YELLOW => 19,
            PURPLE => 69,
            GREY => 244,
            WHITE => 152,
            BLACK => 28,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 67,
            GREEN => 34,
            RED => 102,
            YELLOW => 132,
            PURPLE => 208,
            GREY => 184,
            WHITE => 18,
            BLACK => 249,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 231,
            GREEN => 220,
            RED => 47,
            YELLOW => 203,
            PURPLE => 6,
            GREY => 115,
            WHITE => 58,
            BLACK => 212,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 143,
            GREEN => 184,
            RED => 83,
            YELLOW => 65,
            PURPLE => 238,
            GREY => 110,
            WHITE => 0,
            BLACK => 177,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 84,
            GREEN => 157,
            RED => 189,
            YELLOW => 248,
            PURPLE => 239,
            GREY => 73,
            WHITE => 164,
            BLACK => 209,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 173,
            GREEN => 118,
            RED => 74,
            YELLOW => 111,
            PURPLE => 163,
            GREY => 32,
            WHITE => 235,
            BLACK => 142,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 180,
            GREEN => 189,
            RED => 189,
            YELLOW => 91,
            PURPLE => 33,
            GREY => 87,
            WHITE => 98,
            BLACK => 174,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 41,
            GREEN => 68,
            RED => 65,
            YELLOW => 58,
            PURPLE => 48,
            GREY => 217,
            WHITE => 117,
            BLACK => 243,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 130,
            GREEN => 21,
            RED => 108,
            YELLOW => 224,
            PURPLE => 101,
            GREY => 48,
            WHITE => 228,
            BLACK => 254,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 206,
            GREEN => 229,
            RED => 212,
            YELLOW => 131,
            PURPLE => 132,
            GREY => 82,
            WHITE => 204,
            BLACK => 141,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 134,
            GREEN => 146,
            RED => 158,
            YELLOW => 182,
            PURPLE => 76,
            GREY => 5,
            WHITE => 73,
            BLACK => 112,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 157,
            GREEN => 169,
            RED => 214,
            YELLOW => 193,
            PURPLE => 38,
            GREY => 34,
            WHITE => 138,
            BLACK => 70,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if nor(nor(nor(nor(nor(nor(match buffer[14] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[0] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }), match buffer[3] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), nor(match buffer[18] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[5] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(match buffer[11] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[8] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }), nor(nor(match buffer[10] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[20] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }), nor(match buffer[25] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[26] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })))), nor(nor(nor(match buffer[24] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, false), nor(match buffer[17] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, nor(match buffer[16] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[21] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }))), nor(nor(match buffer[1] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, false), nor(match buffer[4] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[19] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, })))), nor(nor(nor(nor(match buffer[2] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, false), nor(nor(match buffer[27] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[13] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), match buffer[9] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(nor(match buffer[7] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[6] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), false), nor(false, false))), nor(nor(nor(false, match buffer[23] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }), nor(false, false)), nor(nor(match buffer[22] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, nor(match buffer[29] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[30] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(match buffer[15] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[28] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), false))))) {
	let n = match buffer[103] {
            BLUE => 96,
            GREEN => 104,
            RED => 87,
            YELLOW => 225,
            PURPLE => 21,
            GREY => 73,
            WHITE => 200,
            BLACK => 122,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[88] {
            BLUE => 255,
            GREEN => 191,
            RED => 87,
            YELLOW => 72,
            PURPLE => 188,
            GREY => 27,
            WHITE => 227,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 84,
            GREEN => 158,
            RED => 192,
            YELLOW => 237,
            PURPLE => 31,
            GREY => 129,
            WHITE => 10,
            BLACK => 147,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 7,
            GREEN => 27,
            RED => 200,
            YELLOW => 141,
            PURPLE => 230,
            GREY => 128,
            WHITE => 139,
            BLACK => 36,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[95] {
            BLUE => 214,
            GREEN => 228,
            RED => 41,
            YELLOW => 205,
            PURPLE => 236,
            GREY => 213,
            WHITE => 4,
            BLACK => 47,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[94] {
            BLUE => 140,
            GREEN => 219,
            RED => 184,
            YELLOW => 105,
            PURPLE => 50,
            GREY => 161,
            WHITE => 193,
            BLACK => 76,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 195,
            GREEN => 102,
            RED => 89,
            YELLOW => 220,
            PURPLE => 191,
            GREY => 134,
            WHITE => 7,
            BLACK => 176,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[96] {
            BLUE => 93,
            GREEN => 11,
            RED => 250,
            YELLOW => 177,
            PURPLE => 90,
            GREY => 3,
            WHITE => 185,
            BLACK => 167,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 31,
            GREEN => 193,
            RED => 216,
            YELLOW => 136,
            PURPLE => 181,
            GREY => 94,
            WHITE => 243,
            BLACK => 207,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 128,
            GREEN => 64,
            RED => 222,
            YELLOW => 153,
            PURPLE => 96,
            GREY => 122,
            WHITE => 237,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 182,
            GREEN => 1,
            RED => 13,
            YELLOW => 30,
            PURPLE => 125,
            GREY => 95,
            WHITE => 154,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 93,
            GREEN => 231,
            RED => 155,
            YELLOW => 31,
            PURPLE => 118,
            GREY => 198,
            WHITE => 118,
            BLACK => 231,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 69,
            GREEN => 207,
            RED => 187,
            YELLOW => 111,
            PURPLE => 50,
            GREY => 148,
            WHITE => 110,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 199,
            GREEN => 173,
            RED => 220,
            YELLOW => 29,
            PURPLE => 82,
            GREY => 155,
            WHITE => 60,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 29,
            GREEN => 35,
            RED => 215,
            YELLOW => 221,
            PURPLE => 69,
            GREY => 100,
            WHITE => 101,
            BLACK => 9,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[99] {
            BLUE => 217,
            GREEN => 156,
            RED => 118,
            YELLOW => 59,
            PURPLE => 239,
            GREY => 172,
            WHITE => 11,
            BLACK => 90,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 82,
            GREEN => 70,
            RED => 48,
            YELLOW => 111,
            PURPLE => 48,
            GREY => 143,
            WHITE => 165,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[83] {
            BLUE => 88,
            GREEN => 104,
            RED => 83,
            YELLOW => 159,
            PURPLE => 110,
            GREY => 131,
            WHITE => 236,
            BLACK => 226,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[85] {
            BLUE => 198,
            GREEN => 47,
            RED => 200,
            YELLOW => 32,
            PURPLE => 70,
            GREY => 138,
            WHITE => 221,
            BLACK => 62,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 183,
            GREEN => 177,
            RED => 19,
            YELLOW => 64,
            PURPLE => 237,
            GREY => 21,
            WHITE => 56,
            BLACK => 34,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 145,
            GREEN => 149,
            RED => 42,
            YELLOW => 24,
            PURPLE => 83,
            GREY => 84,
            WHITE => 126,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 173,
            GREEN => 81,
            RED => 147,
            YELLOW => 141,
            PURPLE => 95,
            GREY => 0,
            WHITE => 222,
            BLACK => 164,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 151,
            GREEN => 243,
            RED => 33,
            YELLOW => 5,
            PURPLE => 253,
            GREY => 5,
            WHITE => 34,
            BLACK => 220,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 211,
            GREEN => 152,
            RED => 17,
            YELLOW => 39,
            PURPLE => 195,
            GREY => 214,
            WHITE => 174,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 208,
            GREEN => 211,
            RED => 13,
            YELLOW => 169,
            PURPLE => 253,
            GREY => 200,
            WHITE => 12,
            BLACK => 160,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 22,
            GREEN => 62,
            RED => 18,
            YELLOW => 9,
            PURPLE => 255,
            GREY => 73,
            WHITE => 254,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 145,
            GREEN => 89,
            RED => 12,
            YELLOW => 141,
            PURPLE => 177,
            GREY => 8,
            WHITE => 245,
            BLACK => 250,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 146,
            GREEN => 182,
            RED => 252,
            YELLOW => 65,
            PURPLE => 150,
            GREY => 92,
            WHITE => 117,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 212,
            GREEN => 176,
            RED => 13,
            YELLOW => 174,
            PURPLE => 184,
            GREY => 184,
            WHITE => 58,
            BLACK => 17,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[89] {
            BLUE => 144,
            GREEN => 23,
            RED => 165,
            YELLOW => 235,
            PURPLE => 36,
            GREY => 187,
            WHITE => 213,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 76,
            GREEN => 65,
            RED => 123,
            YELLOW => 148,
            PURPLE => 142,
            GREY => 69,
            WHITE => 143,
            BLACK => 225,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 251,
            GREEN => 223,
            RED => 125,
            YELLOW => 197,
            PURPLE => 240,
            GREY => 79,
            WHITE => 153,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[71] {
            BLUE => 88,
            GREEN => 68,
            RED => 129,
            YELLOW => 42,
            PURPLE => 3,
            GREY => 78,
            WHITE => 59,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[84] {
            BLUE => 228,
            GREEN => 135,
            RED => 90,
            YELLOW => 245,
            PURPLE => 106,
            GREY => 47,
            WHITE => 227,
            BLACK => 100,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 124,
            GREEN => 4,
            RED => 47,
            YELLOW => 52,
            PURPLE => 94,
            GREY => 147,
            WHITE => 182,
            BLACK => 50,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 2,
            GREEN => 52,
            RED => 28,
            YELLOW => 188,
            PURPLE => 122,
            GREY => 253,
            WHITE => 167,
            BLACK => 47,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 77,
            GREEN => 224,
            RED => 179,
            YELLOW => 251,
            PURPLE => 40,
            GREY => 177,
            WHITE => 254,
            BLACK => 36,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[72] {
            BLUE => 20,
            GREEN => 102,
            RED => 71,
            YELLOW => 183,
            PURPLE => 237,
            GREY => 91,
            WHITE => 201,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 239,
            GREEN => 105,
            RED => 172,
            YELLOW => 124,
            PURPLE => 162,
            GREY => 121,
            WHITE => 203,
            BLACK => 94,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 25,
            GREEN => 5,
            RED => 131,
            YELLOW => 182,
            PURPLE => 80,
            GREY => 204,
            WHITE => 77,
            BLACK => 173,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[73] {
            BLUE => 189,
            GREEN => 202,
            RED => 176,
            YELLOW => 148,
            PURPLE => 141,
            GREY => 37,
            WHITE => 60,
            BLACK => 207,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[86] {
            BLUE => 122,
            GREEN => 76,
            RED => 165,
            YELLOW => 60,
            PURPLE => 222,
            GREY => 158,
            WHITE => 118,
            BLACK => 75,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[78] {
            BLUE => 195,
            GREEN => 26,
            RED => 84,
            YELLOW => 23,
            PURPLE => 18,
            GREY => 106,
            WHITE => 27,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 128,
            GREEN => 213,
            RED => 156,
            YELLOW => 124,
            PURPLE => 234,
            GREY => 230,
            WHITE => 71,
            BLACK => 251,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 188,
            GREEN => 97,
            RED => 84,
            YELLOW => 194,
            PURPLE => 217,
            GREY => 146,
            WHITE => 223,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 92,
            GREEN => 167,
            RED => 94,
            YELLOW => 231,
            PURPLE => 91,
            GREY => 144,
            WHITE => 61,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[91] {
            BLUE => 112,
            GREEN => 203,
            RED => 196,
            YELLOW => 253,
            PURPLE => 118,
            GREY => 147,
            WHITE => 206,
            BLACK => 131,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 74,
            GREEN => 68,
            RED => 174,
            YELLOW => 71,
            PURPLE => 119,
            GREY => 51,
            WHITE => 145,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[98] {
            BLUE => 9,
            GREEN => 169,
            RED => 116,
            YELLOW => 130,
            PURPLE => 182,
            GREY => 119,
            WHITE => 118,
            BLACK => 133,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[70] {
            BLUE => 19,
            GREEN => 152,
            RED => 23,
            YELLOW => 95,
            PURPLE => 199,
            GREY => 207,
            WHITE => 213,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 162,
            GREEN => 156,
            RED => 136,
            YELLOW => 224,
            PURPLE => 169,
            GREY => 64,
            WHITE => 232,
            BLACK => 109,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 63,
            GREEN => 172,
            RED => 199,
            YELLOW => 85,
            PURPLE => 14,
            GREY => 165,
            WHITE => 64,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[74] {
            BLUE => 67,
            GREEN => 22,
            RED => 26,
            YELLOW => 187,
            PURPLE => 12,
            GREY => 119,
            WHITE => 12,
            BLACK => 98,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[87] {
            BLUE => 131,
            GREEN => 77,
            RED => 223,
            YELLOW => 30,
            PURPLE => 25,
            GREY => 245,
            WHITE => 203,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 253,
            GREEN => 220,
            RED => 212,
            YELLOW => 233,
            PURPLE => 14,
            GREY => 118,
            WHITE => 183,
            BLACK => 40,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 111,
            GREEN => 166,
            RED => 121,
            YELLOW => 96,
            PURPLE => 96,
            GREY => 103,
            WHITE => 239,
            BLACK => 11,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 2,
            GREEN => 43,
            RED => 63,
            YELLOW => 203,
            PURPLE => 123,
            GREY => 51,
            WHITE => 61,
            BLACK => 252,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 147,
            GREEN => 58,
            RED => 76,
            YELLOW => 225,
            PURPLE => 66,
            GREY => 154,
            WHITE => 36,
            BLACK => 252,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 249,
            GREEN => 46,
            RED => 113,
            YELLOW => 225,
            PURPLE => 133,
            GREY => 226,
            WHITE => 224,
            BLACK => 166,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[81] {
            BLUE => 99,
            GREEN => 98,
            RED => 178,
            YELLOW => 232,
            PURPLE => 133,
            GREY => 235,
            WHITE => 144,
            BLACK => 150,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 180,
            GREEN => 157,
            RED => 240,
            YELLOW => 203,
            PURPLE => 13,
            GREY => 230,
            WHITE => 162,
            BLACK => 95,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 248,
            GREEN => 140,
            RED => 158,
            YELLOW => 81,
            PURPLE => 103,
            GREY => 179,
            WHITE => 173,
            BLACK => 118,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 21,
            GREEN => 102,
            RED => 254,
            YELLOW => 229,
            PURPLE => 223,
            GREY => 170,
            WHITE => 41,
            BLACK => 15,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 9,
            GREEN => 214,
            RED => 168,
            YELLOW => 130,
            PURPLE => 235,
            GREY => 30,
            WHITE => 248,
            BLACK => 78,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 217,
            GREEN => 142,
            RED => 95,
            YELLOW => 35,
            PURPLE => 251,
            GREY => 149,
            WHITE => 145,
            BLACK => 218,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 70,
            GREEN => 100,
            RED => 235,
            YELLOW => 215,
            PURPLE => 171,
            GREY => 192,
            WHITE => 133,
            BLACK => 229,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 152,
            GREEN => 48,
            RED => 91,
            YELLOW => 19,
            PURPLE => 127,
            GREY => 112,
            WHITE => 209,
            BLACK => 241,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 243,
            GREEN => 255,
            RED => 181,
            YELLOW => 236,
            PURPLE => 218,
            GREY => 27,
            WHITE => 16,
            BLACK => 57,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 141,
            GREEN => 74,
            RED => 239,
            YELLOW => 197,
            PURPLE => 84,
            GREY => 78,
            WHITE => 230,
            BLACK => 197,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 211,
            GREEN => 152,
            RED => 40,
            YELLOW => 248,
            PURPLE => 26,
            GREY => 179,
            WHITE => 67,
            BLACK => 102,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[79] {
            BLUE => 106,
            GREEN => 165,
            RED => 53,
            YELLOW => 196,
            PURPLE => 254,
            GREY => 134,
            WHITE => 21,
            BLACK => 197,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 242,
            GREEN => 164,
            RED => 76,
            YELLOW => 234,
            PURPLE => 71,
            GREY => 138,
            WHITE => 249,
            BLACK => 98,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 209,
            GREEN => 94,
            RED => 85,
            YELLOW => 215,
            PURPLE => 2,
            GREY => 108,
            WHITE => 109,
            BLACK => 2,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 167,
            GREEN => 220,
            RED => 205,
            YELLOW => 45,
            PURPLE => 53,
            GREY => 132,
            WHITE => 68,
            BLACK => 224,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[97] {
            BLUE => 254,
            GREEN => 12,
            RED => 115,
            YELLOW => 14,
            PURPLE => 251,
            GREY => 157,
            WHITE => 174,
            BLACK => 253,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[76] {
            BLUE => 68,
            GREEN => 74,
            RED => 174,
            YELLOW => 246,
            PURPLE => 46,
            GREY => 98,
            WHITE => 34,
            BLACK => 18,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[80] {
            BLUE => 5,
            GREEN => 168,
            RED => 105,
            YELLOW => 158,
            PURPLE => 128,
            GREY => 142,
            WHITE => 24,
            BLACK => 243,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[102] {
            BLUE => 74,
            GREEN => 200,
            RED => 122,
            YELLOW => 245,
            PURPLE => 238,
            GREY => 95,
            WHITE => 13,
            BLACK => 14,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[92] {
            BLUE => 20,
            GREEN => 121,
            RED => 239,
            YELLOW => 109,
            PURPLE => 93,
            GREY => 251,
            WHITE => 45,
            BLACK => 182,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 46,
            GREEN => 75,
            RED => 181,
            YELLOW => 244,
            PURPLE => 79,
            GREY => 190,
            WHITE => 232,
            BLACK => 52,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[104] {
            BLUE => 166,
            GREEN => 51,
            RED => 171,
            YELLOW => 68,
            PURPLE => 215,
            GREY => 238,
            WHITE => 231,
            BLACK => 131,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[90] {
            BLUE => 128,
            GREEN => 225,
            RED => 86,
            YELLOW => 178,
            PURPLE => 221,
            GREY => 86,
            WHITE => 112,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 193,
            GREEN => 70,
            RED => 72,
            YELLOW => 234,
            PURPLE => 178,
            GREY => 122,
            WHITE => 19,
            BLACK => 236,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 159,
            GREEN => 18,
            RED => 60,
            YELLOW => 8,
            PURPLE => 114,
            GREY => 5,
            WHITE => 206,
            BLACK => 184,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 159,
            GREEN => 202,
            RED => 43,
            YELLOW => 250,
            PURPLE => 70,
            GREY => 111,
            WHITE => 129,
            BLACK => 181,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 182,
            GREEN => 236,
            RED => 165,
            YELLOW => 244,
            PURPLE => 59,
            GREY => 10,
            WHITE => 115,
            BLACK => 212,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[77] {
            BLUE => 220,
            GREEN => 161,
            RED => 17,
            YELLOW => 176,
            PURPLE => 124,
            GREY => 17,
            WHITE => 71,
            BLACK => 223,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[69] {
            BLUE => 168,
            GREEN => 183,
            RED => 230,
            YELLOW => 166,
            PURPLE => 126,
            GREY => 23,
            WHITE => 37,
            BLACK => 37,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 107,
            GREEN => 89,
            RED => 56,
            YELLOW => 235,
            PURPLE => 53,
            GREY => 246,
            WHITE => 159,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 93,
            GREEN => 140,
            RED => 146,
            YELLOW => 200,
            PURPLE => 182,
            GREY => 45,
            WHITE => 35,
            BLACK => 35,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[82] {
            BLUE => 179,
            GREEN => 40,
            RED => 34,
            YELLOW => 253,
            PURPLE => 45,
            GREY => 178,
            WHITE => 184,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 145,
            GREEN => 239,
            RED => 98,
            YELLOW => 238,
            PURPLE => 213,
            GREY => 145,
            WHITE => 129,
            BLACK => 180,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 107,
            GREEN => 144,
            RED => 78,
            YELLOW => 213,
            PURPLE => 191,
            GREY => 177,
            WHITE => 146,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 244,
            GREEN => 222,
            RED => 219,
            YELLOW => 48,
            PURPLE => 33,
            GREY => 166,
            WHITE => 223,
            BLACK => 85,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 183,
            GREEN => 125,
            RED => 142,
            YELLOW => 125,
            PURPLE => 40,
            GREY => 250,
            WHITE => 134,
            BLACK => 210,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 40,
            GREEN => 29,
            RED => 37,
            YELLOW => 50,
            PURPLE => 82,
            GREY => 219,
            WHITE => 189,
            BLACK => 214,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 189,
            GREEN => 79,
            RED => 50,
            YELLOW => 18,
            PURPLE => 46,
            GREY => 226,
            WHITE => 24,
            BLACK => 250,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 33,
            GREEN => 205,
            RED => 62,
            YELLOW => 187,
            PURPLE => 114,
            GREY => 108,
            WHITE => 40,
            BLACK => 133,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[93] {
            BLUE => 173,
            GREEN => 198,
            RED => 99,
            YELLOW => 9,
            PURPLE => 200,
            GREY => 74,
            WHITE => 197,
            BLACK => 232,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 149,
            GREEN => 37,
            RED => 169,
            YELLOW => 56,
            PURPLE => 89,
            GREY => 43,
            WHITE => 74,
            BLACK => 38,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[101] {
            BLUE => 2,
            GREEN => 239,
            RED => 36,
            YELLOW => 147,
            PURPLE => 172,
            GREY => 39,
            WHITE => 99,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[75] {
            BLUE => 155,
            GREEN => 3,
            RED => 165,
            YELLOW => 167,
            PURPLE => 90,
            GREY => 103,
            WHITE => 121,
            BLACK => 231,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[100] {
            BLUE => 155,
            GREEN => 150,
            RED => 123,
            YELLOW => 211,
            PURPLE => 236,
            GREY => 239,
            WHITE => 105,
            BLACK => 135,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[56] {
            BLUE => 214,
            GREEN => 65,
            RED => 97,
            YELLOW => 202,
            PURPLE => 157,
            GREY => 146,
            WHITE => 235,
            BLACK => 184,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else {
	let n = match buffer[2] {
            BLUE => 163,
            GREEN => 191,
            RED => 118,
            YELLOW => 12,
            PURPLE => 56,
            GREY => 164,
            WHITE => 171,
            BLACK => 198,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[90] {
            BLUE => 68,
            GREEN => 11,
            RED => 239,
            YELLOW => 107,
            PURPLE => 254,
            GREY => 232,
            WHITE => 247,
            BLACK => 219,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 192,
            GREEN => 153,
            RED => 37,
            YELLOW => 79,
            PURPLE => 213,
            GREY => 113,
            WHITE => 44,
            BLACK => 157,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[82] {
            BLUE => 75,
            GREEN => 32,
            RED => 112,
            YELLOW => 238,
            PURPLE => 193,
            GREY => 46,
            WHITE => 72,
            BLACK => 157,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 1,
            GREEN => 144,
            RED => 152,
            YELLOW => 166,
            PURPLE => 144,
            GREY => 45,
            WHITE => 155,
            BLACK => 111,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 109,
            GREEN => 104,
            RED => 183,
            YELLOW => 203,
            PURPLE => 123,
            GREY => 248,
            WHITE => 205,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 99,
            GREEN => 243,
            RED => 3,
            YELLOW => 28,
            PURPLE => 76,
            GREY => 189,
            WHITE => 87,
            BLACK => 100,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 63,
            GREEN => 108,
            RED => 3,
            YELLOW => 172,
            PURPLE => 107,
            GREY => 28,
            WHITE => 55,
            BLACK => 216,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[72] {
            BLUE => 72,
            GREEN => 153,
            RED => 92,
            YELLOW => 148,
            PURPLE => 223,
            GREY => 56,
            WHITE => 66,
            BLACK => 211,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 1,
            GREEN => 98,
            RED => 66,
            YELLOW => 11,
            PURPLE => 106,
            GREY => 219,
            WHITE => 53,
            BLACK => 226,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[89] {
            BLUE => 235,
            GREEN => 31,
            RED => 91,
            YELLOW => 151,
            PURPLE => 102,
            GREY => 46,
            WHITE => 158,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 178,
            GREEN => 133,
            RED => 24,
            YELLOW => 164,
            PURPLE => 98,
            GREY => 217,
            WHITE => 190,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 233,
            GREEN => 218,
            RED => 58,
            YELLOW => 93,
            PURPLE => 139,
            GREY => 78,
            WHITE => 156,
            BLACK => 136,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[94] {
            BLUE => 101,
            GREEN => 160,
            RED => 101,
            YELLOW => 137,
            PURPLE => 112,
            GREY => 51,
            WHITE => 77,
            BLACK => 212,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 119,
            GREEN => 208,
            RED => 16,
            YELLOW => 142,
            PURPLE => 171,
            GREY => 234,
            WHITE => 8,
            BLACK => 137,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 34,
            GREEN => 147,
            RED => 43,
            YELLOW => 90,
            PURPLE => 132,
            GREY => 89,
            WHITE => 121,
            BLACK => 155,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 167,
            GREEN => 73,
            RED => 156,
            YELLOW => 83,
            PURPLE => 176,
            GREY => 156,
            WHITE => 29,
            BLACK => 143,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[77] {
            BLUE => 90,
            GREEN => 243,
            RED => 95,
            YELLOW => 26,
            PURPLE => 89,
            GREY => 180,
            WHITE => 214,
            BLACK => 166,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 155,
            GREEN => 16,
            RED => 65,
            YELLOW => 242,
            PURPLE => 27,
            GREY => 240,
            WHITE => 51,
            BLACK => 97,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 24,
            GREEN => 251,
            RED => 161,
            YELLOW => 97,
            PURPLE => 154,
            GREY => 82,
            WHITE => 171,
            BLACK => 67,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 1,
            GREEN => 152,
            RED => 180,
            YELLOW => 239,
            PURPLE => 35,
            GREY => 65,
            WHITE => 225,
            BLACK => 153,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 52,
            GREEN => 33,
            RED => 1,
            YELLOW => 178,
            PURPLE => 173,
            GREY => 148,
            WHITE => 67,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[91] {
            BLUE => 82,
            GREEN => 28,
            RED => 173,
            YELLOW => 197,
            PURPLE => 18,
            GREY => 79,
            WHITE => 203,
            BLACK => 205,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 106,
            GREEN => 40,
            RED => 233,
            YELLOW => 204,
            PURPLE => 216,
            GREY => 203,
            WHITE => 120,
            BLACK => 255,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[93] {
            BLUE => 62,
            GREEN => 116,
            RED => 11,
            YELLOW => 248,
            PURPLE => 66,
            GREY => 237,
            WHITE => 232,
            BLACK => 33,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 56,
            GREEN => 196,
            RED => 132,
            YELLOW => 182,
            PURPLE => 41,
            GREY => 134,
            WHITE => 53,
            BLACK => 78,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 41,
            GREEN => 170,
            RED => 207,
            YELLOW => 49,
            PURPLE => 0,
            GREY => 211,
            WHITE => 168,
            BLACK => 66,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 159,
            GREEN => 26,
            RED => 53,
            YELLOW => 0,
            PURPLE => 189,
            GREY => 242,
            WHITE => 203,
            BLACK => 229,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 27,
            GREEN => 161,
            RED => 169,
            YELLOW => 253,
            PURPLE => 128,
            GREY => 101,
            WHITE => 43,
            BLACK => 106,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 233,
            GREEN => 141,
            RED => 111,
            YELLOW => 137,
            PURPLE => 148,
            GREY => 236,
            WHITE => 239,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 125,
            GREEN => 138,
            RED => 70,
            YELLOW => 75,
            PURPLE => 127,
            GREY => 95,
            WHITE => 89,
            BLACK => 219,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 200,
            GREEN => 133,
            RED => 130,
            YELLOW => 126,
            PURPLE => 40,
            GREY => 146,
            WHITE => 126,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 136,
            GREEN => 81,
            RED => 94,
            YELLOW => 147,
            PURPLE => 198,
            GREY => 91,
            WHITE => 38,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 227,
            GREEN => 215,
            RED => 203,
            YELLOW => 248,
            PURPLE => 59,
            GREY => 161,
            WHITE => 220,
            BLACK => 175,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[81] {
            BLUE => 189,
            GREEN => 36,
            RED => 140,
            YELLOW => 157,
            PURPLE => 7,
            GREY => 183,
            WHITE => 182,
            BLACK => 149,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 211,
            GREEN => 238,
            RED => 29,
            YELLOW => 169,
            PURPLE => 26,
            GREY => 155,
            WHITE => 163,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 112,
            GREEN => 12,
            RED => 24,
            YELLOW => 246,
            PURPLE => 60,
            GREY => 128,
            WHITE => 147,
            BLACK => 0,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[65] {
            BLUE => 209,
            GREEN => 54,
            RED => 252,
            YELLOW => 166,
            PURPLE => 100,
            GREY => 140,
            WHITE => 26,
            BLACK => 206,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 10,
            GREEN => 34,
            RED => 16,
            YELLOW => 226,
            PURPLE => 162,
            GREY => 181,
            WHITE => 85,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 23,
            GREEN => 26,
            RED => 208,
            YELLOW => 47,
            PURPLE => 104,
            GREY => 134,
            WHITE => 110,
            BLACK => 145,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[83] {
            BLUE => 254,
            GREEN => 164,
            RED => 50,
            YELLOW => 107,
            PURPLE => 92,
            GREY => 7,
            WHITE => 227,
            BLACK => 252,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 208,
            GREEN => 238,
            RED => 134,
            YELLOW => 115,
            PURPLE => 141,
            GREY => 171,
            WHITE => 158,
            BLACK => 38,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[95] {
            BLUE => 255,
            GREEN => 152,
            RED => 26,
            YELLOW => 148,
            PURPLE => 145,
            GREY => 163,
            WHITE => 53,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[73] {
            BLUE => 51,
            GREEN => 36,
            RED => 151,
            YELLOW => 167,
            PURPLE => 21,
            GREY => 158,
            WHITE => 19,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[92] {
            BLUE => 19,
            GREEN => 157,
            RED => 253,
            YELLOW => 136,
            PURPLE => 120,
            GREY => 175,
            WHITE => 97,
            BLACK => 127,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 174,
            GREEN => 216,
            RED => 44,
            YELLOW => 167,
            PURPLE => 208,
            GREY => 7,
            WHITE => 109,
            BLACK => 16,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[84] {
            BLUE => 198,
            GREEN => 148,
            RED => 38,
            YELLOW => 186,
            PURPLE => 109,
            GREY => 45,
            WHITE => 167,
            BLACK => 233,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[80] {
            BLUE => 18,
            GREEN => 71,
            RED => 165,
            YELLOW => 182,
            PURPLE => 35,
            GREY => 126,
            WHITE => 199,
            BLACK => 118,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[70] {
            BLUE => 231,
            GREEN => 20,
            RED => 171,
            YELLOW => 121,
            PURPLE => 201,
            GREY => 100,
            WHITE => 222,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 135,
            GREEN => 38,
            RED => 65,
            YELLOW => 125,
            PURPLE => 168,
            GREY => 40,
            WHITE => 231,
            BLACK => 27,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[79] {
            BLUE => 185,
            GREEN => 11,
            RED => 72,
            YELLOW => 211,
            PURPLE => 247,
            GREY => 159,
            WHITE => 193,
            BLACK => 180,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[85] {
            BLUE => 142,
            GREEN => 71,
            RED => 116,
            YELLOW => 99,
            PURPLE => 111,
            GREY => 0,
            WHITE => 129,
            BLACK => 51,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 38,
            GREEN => 248,
            RED => 18,
            YELLOW => 255,
            PURPLE => 136,
            GREY => 157,
            WHITE => 8,
            BLACK => 94,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 214,
            GREEN => 148,
            RED => 21,
            YELLOW => 199,
            PURPLE => 73,
            GREY => 32,
            WHITE => 64,
            BLACK => 105,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 16,
            GREEN => 252,
            RED => 161,
            YELLOW => 75,
            PURPLE => 140,
            GREY => 157,
            WHITE => 248,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 197,
            GREEN => 69,
            RED => 111,
            YELLOW => 11,
            PURPLE => 227,
            GREY => 134,
            WHITE => 186,
            BLACK => 122,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[76] {
            BLUE => 92,
            GREEN => 11,
            RED => 118,
            YELLOW => 236,
            PURPLE => 39,
            GREY => 88,
            WHITE => 43,
            BLACK => 145,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 172,
            GREEN => 104,
            RED => 211,
            YELLOW => 42,
            PURPLE => 185,
            GREY => 183,
            WHITE => 89,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[88] {
            BLUE => 100,
            GREEN => 26,
            RED => 158,
            YELLOW => 161,
            PURPLE => 100,
            GREY => 22,
            WHITE => 52,
            BLACK => 147,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[96] {
            BLUE => 51,
            GREEN => 221,
            RED => 227,
            YELLOW => 232,
            PURPLE => 231,
            GREY => 95,
            WHITE => 144,
            BLACK => 9,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 165,
            GREEN => 99,
            RED => 204,
            YELLOW => 243,
            PURPLE => 179,
            GREY => 24,
            WHITE => 45,
            BLACK => 228,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[71] {
            BLUE => 66,
            GREEN => 224,
            RED => 5,
            YELLOW => 89,
            PURPLE => 34,
            GREY => 202,
            WHITE => 49,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 27,
            GREEN => 231,
            RED => 101,
            YELLOW => 83,
            PURPLE => 196,
            GREY => 140,
            WHITE => 26,
            BLACK => 155,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 99,
            GREEN => 130,
            RED => 104,
            YELLOW => 99,
            PURPLE => 176,
            GREY => 62,
            WHITE => 45,
            BLACK => 177,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 162,
            GREEN => 53,
            RED => 205,
            YELLOW => 182,
            PURPLE => 177,
            GREY => 200,
            WHITE => 80,
            BLACK => 247,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[87] {
            BLUE => 192,
            GREEN => 23,
            RED => 124,
            YELLOW => 189,
            PURPLE => 71,
            GREY => 174,
            WHITE => 174,
            BLACK => 62,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 161,
            GREEN => 233,
            RED => 229,
            YELLOW => 58,
            PURPLE => 41,
            GREY => 219,
            WHITE => 60,
            BLACK => 42,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 95,
            GREEN => 34,
            RED => 217,
            YELLOW => 220,
            PURPLE => 65,
            GREY => 142,
            WHITE => 109,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 149,
            GREEN => 169,
            RED => 190,
            YELLOW => 38,
            PURPLE => 1,
            GREY => 132,
            WHITE => 73,
            BLACK => 170,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[86] {
            BLUE => 231,
            GREEN => 165,
            RED => 168,
            YELLOW => 106,
            PURPLE => 175,
            GREY => 103,
            WHITE => 142,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[69] {
            BLUE => 15,
            GREEN => 207,
            RED => 52,
            YELLOW => 229,
            PURPLE => 93,
            GREY => 181,
            WHITE => 130,
            BLACK => 135,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[75] {
            BLUE => 149,
            GREEN => 181,
            RED => 70,
            YELLOW => 128,
            PURPLE => 60,
            GREY => 222,
            WHITE => 5,
            BLACK => 252,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 1,
            GREEN => 96,
            RED => 228,
            YELLOW => 66,
            PURPLE => 80,
            GREY => 130,
            WHITE => 47,
            BLACK => 129,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 163,
            GREEN => 232,
            RED => 186,
            YELLOW => 27,
            PURPLE => 169,
            GREY => 57,
            WHITE => 121,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 221,
            GREEN => 138,
            RED => 125,
            YELLOW => 134,
            PURPLE => 48,
            GREY => 2,
            WHITE => 127,
            BLACK => 185,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 198,
            GREEN => 153,
            RED => 18,
            YELLOW => 3,
            PURPLE => 237,
            GREY => 112,
            WHITE => 183,
            BLACK => 216,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 221,
            GREEN => 206,
            RED => 118,
            YELLOW => 51,
            PURPLE => 47,
            GREY => 111,
            WHITE => 255,
            BLACK => 88,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 205,
            GREEN => 177,
            RED => 252,
            YELLOW => 172,
            PURPLE => 121,
            GREY => 71,
            WHITE => 133,
            BLACK => 102,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 120,
            GREEN => 72,
            RED => 34,
            YELLOW => 177,
            PURPLE => 205,
            GREY => 163,
            WHITE => 89,
            BLACK => 226,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 124,
            GREEN => 235,
            RED => 173,
            YELLOW => 220,
            PURPLE => 230,
            GREY => 176,
            WHITE => 48,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[97] {
            BLUE => 143,
            GREEN => 103,
            RED => 61,
            YELLOW => 125,
            PURPLE => 195,
            GREY => 211,
            WHITE => 23,
            BLACK => 176,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[74] {
            BLUE => 106,
            GREEN => 239,
            RED => 75,
            YELLOW => 96,
            PURPLE => 140,
            GREY => 82,
            WHITE => 17,
            BLACK => 44,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 166,
            GREEN => 144,
            RED => 33,
            YELLOW => 74,
            PURPLE => 38,
            GREY => 43,
            WHITE => 52,
            BLACK => 57,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[78] {
            BLUE => 8,
            GREEN => 162,
            RED => 132,
            YELLOW => 196,
            PURPLE => 248,
            GREY => 209,
            WHITE => 104,
            BLACK => 103,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 205,
            GREEN => 57,
            RED => 92,
            YELLOW => 186,
            PURPLE => 117,
            GREY => 208,
            WHITE => 6,
            BLACK => 35,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 35,
            GREEN => 207,
            RED => 66,
            YELLOW => 7,
            PURPLE => 253,
            GREY => 160,
            WHITE => 194,
            BLACK => 157,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 190,
            GREEN => 119,
            RED => 67,
            YELLOW => 149,
            PURPLE => 150,
            GREY => 82,
            WHITE => 12,
            BLACK => 138,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 245,
            GREEN => 52,
            RED => 128,
            YELLOW => 55,
            PURPLE => 119,
            GREY => 4,
            WHITE => 85,
            BLACK => 106,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 156,
            GREEN => 86,
            RED => 133,
            YELLOW => 232,
            PURPLE => 7,
            GREY => 233,
            WHITE => 223,
            BLACK => 244,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 213,
            GREEN => 159,
            RED => 76,
            YELLOW => 216,
            PURPLE => 27,
            GREY => 124,
            WHITE => 156,
            BLACK => 80,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 206,
            GREEN => 211,
            RED => 49,
            YELLOW => 206,
            PURPLE => 20,
            GREY => 216,
            WHITE => 24,
            BLACK => 160,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 53,
            GREEN => 119,
            RED => 76,
            YELLOW => 199,
            PURPLE => 57,
            GREY => 223,
            WHITE => 140,
            BLACK => 166,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 86,
            GREEN => 169,
            RED => 228,
            YELLOW => 63,
            PURPLE => 212,
            GREY => 47,
            WHITE => 79,
            BLACK => 167,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 109,
            GREEN => 220,
            RED => 30,
            YELLOW => 205,
            PURPLE => 115,
            GREY => 157,
            WHITE => 16,
            BLACK => 186,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 160,
            GREEN => 112,
            RED => 240,
            YELLOW => 61,
            PURPLE => 224,
            GREY => 4,
            WHITE => 36,
            BLACK => 104,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[56] {
            BLUE => 208,
            GREEN => 221,
            RED => 195,
            YELLOW => 182,
            PURPLE => 30,
            GREY => 154,
            WHITE => 44,
            BLACK => 183,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 114,
            GREEN => 129,
            RED => 42,
            YELLOW => 95,
            PURPLE => 130,
            GREY => 63,
            WHITE => 194,
            BLACK => 79,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}

    }

    if input.pressed_this_frame(Button::Up) {
        if nor(nor(nor(nor(match buffer[8] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, false), nor(match buffer[11] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, false)), nor(nor(false, false), nor(match buffer[1] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[2] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }))), nor(nor(nor(nor(match buffer[5] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[10] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), nor(match buffer[13] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[14] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, })), nor(nor(match buffer[7] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[12] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), match buffer[6] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(match buffer[3] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[9] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), nor(false, match buffer[0] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, })))) {
	let n = match buffer[25] {
            BLUE => 148,
            GREEN => 197,
            RED => 228,
            YELLOW => 154,
            PURPLE => 161,
            GREY => 28,
            WHITE => 120,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 38,
            GREEN => 54,
            RED => 23,
            YELLOW => 51,
            PURPLE => 236,
            GREY => 8,
            WHITE => 88,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 219,
            GREEN => 231,
            RED => 101,
            YELLOW => 51,
            PURPLE => 208,
            GREY => 67,
            WHITE => 40,
            BLACK => 95,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 164,
            GREEN => 24,
            RED => 126,
            YELLOW => 235,
            PURPLE => 34,
            GREY => 29,
            WHITE => 227,
            BLACK => 53,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 95,
            GREEN => 54,
            RED => 213,
            YELLOW => 70,
            PURPLE => 5,
            GREY => 58,
            WHITE => 211,
            BLACK => 191,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 83,
            GREEN => 127,
            RED => 79,
            YELLOW => 6,
            PURPLE => 169,
            GREY => 215,
            WHITE => 173,
            BLACK => 137,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 123,
            GREEN => 69,
            RED => 35,
            YELLOW => 149,
            PURPLE => 49,
            GREY => 93,
            WHITE => 39,
            BLACK => 73,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 93,
            GREEN => 21,
            RED => 188,
            YELLOW => 230,
            PURPLE => 23,
            GREY => 56,
            WHITE => 170,
            BLACK => 183,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 193,
            GREEN => 183,
            RED => 52,
            YELLOW => 136,
            PURPLE => 83,
            GREY => 7,
            WHITE => 28,
            BLACK => 212,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 171,
            GREEN => 30,
            RED => 210,
            YELLOW => 73,
            PURPLE => 27,
            GREY => 55,
            WHITE => 135,
            BLACK => 218,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 237,
            GREEN => 112,
            RED => 70,
            YELLOW => 33,
            PURPLE => 223,
            GREY => 114,
            WHITE => 244,
            BLACK => 185,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 60,
            GREEN => 182,
            RED => 199,
            YELLOW => 18,
            PURPLE => 112,
            GREY => 211,
            WHITE => 67,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 149,
            GREEN => 28,
            RED => 87,
            YELLOW => 216,
            PURPLE => 137,
            GREY => 26,
            WHITE => 138,
            BLACK => 252,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 176,
            GREEN => 215,
            RED => 168,
            YELLOW => 221,
            PURPLE => 170,
            GREY => 17,
            WHITE => 126,
            BLACK => 214,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 140,
            GREEN => 77,
            RED => 197,
            YELLOW => 239,
            PURPLE => 105,
            GREY => 19,
            WHITE => 144,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 165,
            GREEN => 113,
            RED => 81,
            YELLOW => 251,
            PURPLE => 45,
            GREY => 64,
            WHITE => 203,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 174,
            GREEN => 72,
            RED => 255,
            YELLOW => 218,
            PURPLE => 114,
            GREY => 161,
            WHITE => 220,
            BLACK => 216,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 144,
            GREEN => 7,
            RED => 39,
            YELLOW => 197,
            PURPLE => 118,
            GREY => 2,
            WHITE => 81,
            BLACK => 92,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 28,
            GREEN => 153,
            RED => 228,
            YELLOW => 136,
            PURPLE => 15,
            GREY => 139,
            WHITE => 223,
            BLACK => 71,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 242,
            GREEN => 229,
            RED => 161,
            YELLOW => 221,
            PURPLE => 100,
            GREY => 119,
            WHITE => 216,
            BLACK => 45,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 159,
            GREEN => 94,
            RED => 28,
            YELLOW => 24,
            PURPLE => 37,
            GREY => 32,
            WHITE => 49,
            BLACK => 159,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 165,
            GREEN => 7,
            RED => 29,
            YELLOW => 65,
            PURPLE => 83,
            GREY => 112,
            WHITE => 104,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 165,
            GREEN => 147,
            RED => 176,
            YELLOW => 242,
            PURPLE => 105,
            GREY => 152,
            WHITE => 151,
            BLACK => 101,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 48,
            GREEN => 222,
            RED => 207,
            YELLOW => 16,
            PURPLE => 42,
            GREY => 222,
            WHITE => 190,
            BLACK => 112,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 199,
            GREEN => 40,
            RED => 78,
            YELLOW => 73,
            PURPLE => 253,
            GREY => 215,
            WHITE => 146,
            BLACK => 0,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 98,
            GREEN => 88,
            RED => 160,
            YELLOW => 233,
            PURPLE => 58,
            GREY => 204,
            WHITE => 54,
            BLACK => 248,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 24,
            GREEN => 105,
            RED => 121,
            YELLOW => 177,
            PURPLE => 128,
            GREY => 140,
            WHITE => 75,
            BLACK => 121,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 203,
            GREEN => 180,
            RED => 204,
            YELLOW => 215,
            PURPLE => 220,
            GREY => 139,
            WHITE => 116,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 202,
            GREEN => 145,
            RED => 151,
            YELLOW => 103,
            PURPLE => 136,
            GREY => 101,
            WHITE => 228,
            BLACK => 66,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 36,
            GREEN => 250,
            RED => 9,
            YELLOW => 124,
            PURPLE => 242,
            GREY => 7,
            WHITE => 13,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 61,
            GREEN => 188,
            RED => 90,
            YELLOW => 156,
            PURPLE => 209,
            GREY => 107,
            WHITE => 237,
            BLACK => 232,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 21,
            GREEN => 75,
            RED => 25,
            YELLOW => 147,
            PURPLE => 202,
            GREY => 156,
            WHITE => 188,
            BLACK => 128,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 198,
            GREEN => 1,
            RED => 146,
            YELLOW => 152,
            PURPLE => 251,
            GREY => 58,
            WHITE => 13,
            BLACK => 169,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 111,
            GREEN => 26,
            RED => 213,
            YELLOW => 241,
            PURPLE => 127,
            GREY => 116,
            WHITE => 25,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 16,
            GREEN => 232,
            RED => 114,
            YELLOW => 71,
            PURPLE => 230,
            GREY => 9,
            WHITE => 82,
            BLACK => 103,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 35,
            GREEN => 175,
            RED => 65,
            YELLOW => 75,
            PURPLE => 178,
            GREY => 255,
            WHITE => 20,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 67,
            GREEN => 208,
            RED => 201,
            YELLOW => 47,
            PURPLE => 251,
            GREY => 229,
            WHITE => 147,
            BLACK => 124,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 211,
            GREEN => 56,
            RED => 103,
            YELLOW => 159,
            PURPLE => 206,
            GREY => 158,
            WHITE => 235,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 188,
            GREEN => 0,
            RED => 89,
            YELLOW => 109,
            PURPLE => 145,
            GREY => 64,
            WHITE => 220,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 53,
            GREEN => 240,
            RED => 98,
            YELLOW => 120,
            PURPLE => 216,
            GREY => 145,
            WHITE => 116,
            BLACK => 123,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 130,
            GREEN => 218,
            RED => 202,
            YELLOW => 223,
            PURPLE => 212,
            GREY => 49,
            WHITE => 218,
            BLACK => 197,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 231,
            GREEN => 6,
            RED => 174,
            YELLOW => 78,
            PURPLE => 97,
            GREY => 199,
            WHITE => 128,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 20,
            GREEN => 152,
            RED => 225,
            YELLOW => 185,
            PURPLE => 234,
            GREY => 6,
            WHITE => 33,
            BLACK => 138,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if nor(nor(nor(match buffer[5] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, false), nor(false, false)), nor(nor(false, nor(match buffer[1] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[0] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })), nor(nor(match buffer[2] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[4] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), false))) {
	let n = match buffer[22] {
            BLUE => 146,
            GREEN => 96,
            RED => 65,
            YELLOW => 91,
            PURPLE => 179,
            GREY => 156,
            WHITE => 44,
            BLACK => 145,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 27,
            GREEN => 57,
            RED => 54,
            YELLOW => 88,
            PURPLE => 253,
            GREY => 54,
            WHITE => 126,
            BLACK => 159,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 70,
            GREEN => 91,
            RED => 201,
            YELLOW => 76,
            PURPLE => 108,
            GREY => 90,
            WHITE => 18,
            BLACK => 239,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 114,
            GREEN => 243,
            RED => 250,
            YELLOW => 131,
            PURPLE => 2,
            GREY => 2,
            WHITE => 227,
            BLACK => 50,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 44,
            GREEN => 38,
            RED => 128,
            YELLOW => 76,
            PURPLE => 95,
            GREY => 123,
            WHITE => 49,
            BLACK => 247,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 1,
            GREEN => 3,
            RED => 130,
            YELLOW => 114,
            PURPLE => 93,
            GREY => 75,
            WHITE => 27,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 38,
            GREEN => 7,
            RED => 201,
            YELLOW => 31,
            PURPLE => 20,
            GREY => 137,
            WHITE => 73,
            BLACK => 141,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 5,
            GREEN => 211,
            RED => 164,
            YELLOW => 61,
            PURPLE => 48,
            GREY => 41,
            WHITE => 195,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 228,
            GREEN => 24,
            RED => 60,
            YELLOW => 115,
            PURPLE => 171,
            GREY => 121,
            WHITE => 57,
            BLACK => 186,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 81,
            GREEN => 182,
            RED => 15,
            YELLOW => 188,
            PURPLE => 86,
            GREY => 220,
            WHITE => 238,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 129,
            GREEN => 168,
            RED => 7,
            YELLOW => 120,
            PURPLE => 153,
            GREY => 54,
            WHITE => 148,
            BLACK => 94,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 78,
            GREEN => 215,
            RED => 209,
            YELLOW => 251,
            PURPLE => 192,
            GREY => 5,
            WHITE => 17,
            BLACK => 59,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 146,
            GREEN => 75,
            RED => 251,
            YELLOW => 201,
            PURPLE => 68,
            GREY => 194,
            WHITE => 36,
            BLACK => 121,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 1,
            GREEN => 199,
            RED => 104,
            YELLOW => 40,
            PURPLE => 140,
            GREY => 63,
            WHITE => 223,
            BLACK => 152,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 22,
            GREEN => 48,
            RED => 86,
            YELLOW => 183,
            PURPLE => 192,
            GREY => 10,
            WHITE => 95,
            BLACK => 170,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 248,
            GREEN => 152,
            RED => 165,
            YELLOW => 213,
            PURPLE => 212,
            GREY => 19,
            WHITE => 218,
            BLACK => 204,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 218,
            GREEN => 188,
            RED => 210,
            YELLOW => 129,
            PURPLE => 84,
            GREY => 113,
            WHITE => 16,
            BLACK => 168,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 80,
            GREEN => 123,
            RED => 129,
            YELLOW => 129,
            PURPLE => 131,
            GREY => 74,
            WHITE => 224,
            BLACK => 52,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 178,
            GREEN => 14,
            RED => 0,
            YELLOW => 253,
            PURPLE => 185,
            GREY => 131,
            WHITE => 9,
            BLACK => 223,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 18,
            GREEN => 19,
            RED => 88,
            YELLOW => 170,
            PURPLE => 52,
            GREY => 74,
            WHITE => 120,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 236,
            GREEN => 118,
            RED => 77,
            YELLOW => 39,
            PURPLE => 115,
            GREY => 97,
            WHITE => 240,
            BLACK => 65,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 130,
            GREEN => 195,
            RED => 133,
            YELLOW => 104,
            PURPLE => 149,
            GREY => 67,
            WHITE => 68,
            BLACK => 147,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 96,
            GREEN => 131,
            RED => 177,
            YELLOW => 12,
            PURPLE => 175,
            GREY => 180,
            WHITE => 186,
            BLACK => 73,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 54,
            GREEN => 172,
            RED => 20,
            YELLOW => 41,
            PURPLE => 185,
            GREY => 24,
            WHITE => 128,
            BLACK => 163,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if nor(nor(nor(nor(nor(false, match buffer[6] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }), nor(nor(match buffer[11] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[14] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }), false)), nor(nor(false, match buffer[17] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), nor(false, nor(match buffer[13] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[4] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })))), nor(nor(nor(false, false), nor(match buffer[16] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[9] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(nor(match buffer[19] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[2] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), match buffer[3] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }), nor(false, match buffer[8] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, })))), nor(nor(nor(nor(match buffer[7] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[10] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }), nor(false, false)), nor(nor(false, match buffer[5] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), nor(match buffer[15] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[18] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }))), nor(nor(nor(false, false), nor(match buffer[12] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[0] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, })), nor(nor(false, false), nor(false, false))))) {
	let n = match buffer[4] {
            BLUE => 101,
            GREEN => 178,
            RED => 68,
            YELLOW => 128,
            PURPLE => 232,
            GREY => 246,
            WHITE => 162,
            BLACK => 93,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 189,
            GREEN => 168,
            RED => 153,
            YELLOW => 1,
            PURPLE => 162,
            GREY => 29,
            WHITE => 22,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 139,
            GREEN => 44,
            RED => 48,
            YELLOW => 145,
            PURPLE => 88,
            GREY => 6,
            WHITE => 235,
            BLACK => 227,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 138,
            GREEN => 182,
            RED => 12,
            YELLOW => 51,
            PURPLE => 210,
            GREY => 63,
            WHITE => 249,
            BLACK => 74,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else {
	let n = match buffer[11] {
            BLUE => 93,
            GREEN => 141,
            RED => 18,
            YELLOW => 47,
            PURPLE => 244,
            GREY => 96,
            WHITE => 59,
            BLACK => 38,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 10,
            GREEN => 165,
            RED => 183,
            YELLOW => 209,
            PURPLE => 56,
            GREY => 225,
            WHITE => 124,
            BLACK => 114,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 49,
            GREEN => 236,
            RED => 163,
            YELLOW => 240,
            PURPLE => 2,
            GREY => 165,
            WHITE => 60,
            BLACK => 210,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 213,
            GREEN => 75,
            RED => 119,
            YELLOW => 61,
            PURPLE => 60,
            GREY => 187,
            WHITE => 2,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 27,
            GREEN => 147,
            RED => 6,
            YELLOW => 121,
            PURPLE => 237,
            GREY => 223,
            WHITE => 36,
            BLACK => 11,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 166,
            GREEN => 66,
            RED => 86,
            YELLOW => 129,
            PURPLE => 66,
            GREY => 54,
            WHITE => 140,
            BLACK => 159,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 231,
            GREEN => 234,
            RED => 37,
            YELLOW => 112,
            PURPLE => 189,
            GREY => 38,
            WHITE => 144,
            BLACK => 7,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 181,
            GREEN => 4,
            RED => 143,
            YELLOW => 30,
            PURPLE => 24,
            GREY => 14,
            WHITE => 64,
            BLACK => 36,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 112,
            GREEN => 85,
            RED => 16,
            YELLOW => 201,
            PURPLE => 70,
            GREY => 77,
            WHITE => 189,
            BLACK => 224,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 27,
            GREEN => 250,
            RED => 3,
            YELLOW => 211,
            PURPLE => 52,
            GREY => 242,
            WHITE => 178,
            BLACK => 170,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 153,
            GREEN => 194,
            RED => 214,
            YELLOW => 201,
            PURPLE => 168,
            GREY => 4,
            WHITE => 183,
            BLACK => 180,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 211,
            GREEN => 233,
            RED => 23,
            YELLOW => 105,
            PURPLE => 245,
            GREY => 177,
            WHITE => 173,
            BLACK => 57,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 60,
            GREEN => 196,
            RED => 130,
            YELLOW => 61,
            PURPLE => 94,
            GREY => 56,
            WHITE => 202,
            BLACK => 206,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 99,
            GREEN => 40,
            RED => 228,
            YELLOW => 102,
            PURPLE => 46,
            GREY => 9,
            WHITE => 136,
            BLACK => 180,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 70,
            GREEN => 145,
            RED => 81,
            YELLOW => 197,
            PURPLE => 233,
            GREY => 165,
            WHITE => 138,
            BLACK => 184,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}

    }

    if input.pressed_this_frame(Button::Down) {
        if nor(nor(nor(nor(nor(false, false), nor(false, false)), nor(nor(false, false), nor(nor(match buffer[12] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[2] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), false))), nor(nor(nor(match buffer[7] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, false), nor(false, match buffer[13] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(nor(match buffer[3] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[17] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(match buffer[16] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[14] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, })), nor(match buffer[8] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, nor(match buffer[0] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[15] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }))))), nor(nor(nor(nor(match buffer[19] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, false), nor(false, false)), nor(nor(false, nor(match buffer[18] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[10] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, })), nor(false, false))), nor(nor(nor(match buffer[9] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, nor(match buffer[20] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[4] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(match buffer[11] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[5] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), false)), nor(nor(false, match buffer[6] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), nor(false, false))))) {
	let n = match buffer[5] {
            BLUE => 171,
            GREEN => 184,
            RED => 12,
            YELLOW => 209,
            PURPLE => 54,
            GREY => 69,
            WHITE => 167,
            BLACK => 26,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 94,
            GREEN => 162,
            RED => 62,
            YELLOW => 68,
            PURPLE => 229,
            GREY => 199,
            WHITE => 109,
            BLACK => 199,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 127,
            GREEN => 214,
            RED => 65,
            YELLOW => 164,
            PURPLE => 188,
            GREY => 113,
            WHITE => 127,
            BLACK => 210,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 223,
            GREEN => 166,
            RED => 163,
            YELLOW => 132,
            PURPLE => 7,
            GREY => 117,
            WHITE => 142,
            BLACK => 236,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 147,
            GREEN => 37,
            RED => 9,
            YELLOW => 226,
            PURPLE => 10,
            GREY => 96,
            WHITE => 42,
            BLACK => 98,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 203,
            GREEN => 40,
            RED => 112,
            YELLOW => 14,
            PURPLE => 217,
            GREY => 24,
            WHITE => 35,
            BLACK => 123,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 55,
            GREEN => 241,
            RED => 7,
            YELLOW => 157,
            PURPLE => 172,
            GREY => 117,
            WHITE => 196,
            BLACK => 161,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 217,
            GREEN => 32,
            RED => 232,
            YELLOW => 222,
            PURPLE => 93,
            GREY => 42,
            WHITE => 61,
            BLACK => 229,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 156,
            GREEN => 180,
            RED => 155,
            YELLOW => 247,
            PURPLE => 169,
            GREY => 155,
            WHITE => 133,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 10,
            GREEN => 9,
            RED => 190,
            YELLOW => 61,
            PURPLE => 69,
            GREY => 190,
            WHITE => 124,
            BLACK => 110,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 95,
            GREEN => 179,
            RED => 212,
            YELLOW => 13,
            PURPLE => 50,
            GREY => 146,
            WHITE => 220,
            BLACK => 170,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 21,
            GREEN => 170,
            RED => 189,
            YELLOW => 183,
            PURPLE => 245,
            GREY => 229,
            WHITE => 138,
            BLACK => 45,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 41,
            GREEN => 125,
            RED => 212,
            YELLOW => 141,
            PURPLE => 216,
            GREY => 101,
            WHITE => 135,
            BLACK => 242,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 48,
            GREEN => 233,
            RED => 156,
            YELLOW => 234,
            PURPLE => 131,
            GREY => 239,
            WHITE => 164,
            BLACK => 168,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 24,
            GREEN => 29,
            RED => 104,
            YELLOW => 59,
            PURPLE => 102,
            GREY => 3,
            WHITE => 126,
            BLACK => 34,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 173,
            GREEN => 152,
            RED => 156,
            YELLOW => 78,
            PURPLE => 30,
            GREY => 170,
            WHITE => 27,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 110,
            GREEN => 8,
            RED => 114,
            YELLOW => 193,
            PURPLE => 204,
            GREY => 143,
            WHITE => 28,
            BLACK => 128,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 71,
            GREEN => 70,
            RED => 224,
            YELLOW => 220,
            PURPLE => 125,
            GREY => 138,
            WHITE => 40,
            BLACK => 101,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 211,
            GREEN => 26,
            RED => 128,
            YELLOW => 136,
            PURPLE => 109,
            GREY => 106,
            WHITE => 251,
            BLACK => 191,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 190,
            GREEN => 130,
            RED => 139,
            YELLOW => 36,
            PURPLE => 133,
            GREY => 103,
            WHITE => 96,
            BLACK => 22,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 231,
            GREEN => 81,
            RED => 135,
            YELLOW => 7,
            PURPLE => 22,
            GREY => 84,
            WHITE => 76,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 22,
            GREEN => 120,
            RED => 190,
            YELLOW => 234,
            PURPLE => 170,
            GREY => 224,
            WHITE => 124,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 29,
            GREEN => 57,
            RED => 58,
            YELLOW => 231,
            PURPLE => 79,
            GREY => 41,
            WHITE => 201,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 217,
            GREEN => 33,
            RED => 98,
            YELLOW => 54,
            PURPLE => 223,
            GREY => 231,
            WHITE => 121,
            BLACK => 242,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 174,
            GREEN => 88,
            RED => 64,
            YELLOW => 203,
            PURPLE => 223,
            GREY => 201,
            WHITE => 29,
            BLACK => 244,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 87,
            GREEN => 2,
            RED => 56,
            YELLOW => 52,
            PURPLE => 229,
            GREY => 46,
            WHITE => 5,
            BLACK => 114,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 190,
            GREEN => 2,
            RED => 4,
            YELLOW => 236,
            PURPLE => 211,
            GREY => 233,
            WHITE => 229,
            BLACK => 145,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 193,
            GREEN => 173,
            RED => 245,
            YELLOW => 94,
            PURPLE => 194,
            GREY => 70,
            WHITE => 25,
            BLACK => 24,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 232,
            GREEN => 201,
            RED => 216,
            YELLOW => 190,
            PURPLE => 139,
            GREY => 222,
            WHITE => 94,
            BLACK => 99,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 107,
            GREEN => 84,
            RED => 227,
            YELLOW => 55,
            PURPLE => 147,
            GREY => 38,
            WHITE => 79,
            BLACK => 55,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 191,
            GREEN => 33,
            RED => 4,
            YELLOW => 113,
            PURPLE => 99,
            GREY => 83,
            WHITE => 213,
            BLACK => 105,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 58,
            GREEN => 192,
            RED => 135,
            YELLOW => 148,
            PURPLE => 143,
            GREY => 33,
            WHITE => 212,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 71,
            GREEN => 50,
            RED => 143,
            YELLOW => 166,
            PURPLE => 114,
            GREY => 63,
            WHITE => 214,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 243,
            GREEN => 64,
            RED => 247,
            YELLOW => 74,
            PURPLE => 245,
            GREY => 51,
            WHITE => 132,
            BLACK => 16,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 29,
            GREEN => 16,
            RED => 47,
            YELLOW => 143,
            PURPLE => 141,
            GREY => 2,
            WHITE => 116,
            BLACK => 54,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 115,
            GREEN => 239,
            RED => 96,
            YELLOW => 46,
            PURPLE => 196,
            GREY => 59,
            WHITE => 242,
            BLACK => 131,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 126,
            GREEN => 16,
            RED => 112,
            YELLOW => 36,
            PURPLE => 75,
            GREY => 77,
            WHITE => 51,
            BLACK => 3,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 76,
            GREEN => 246,
            RED => 192,
            YELLOW => 125,
            PURPLE => 121,
            GREY => 149,
            WHITE => 116,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 122,
            GREEN => 218,
            RED => 153,
            YELLOW => 176,
            PURPLE => 21,
            GREY => 41,
            WHITE => 1,
            BLACK => 45,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 139,
            GREEN => 203,
            RED => 211,
            YELLOW => 29,
            PURPLE => 177,
            GREY => 128,
            WHITE => 180,
            BLACK => 50,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 212,
            GREEN => 122,
            RED => 58,
            YELLOW => 68,
            PURPLE => 66,
            GREY => 195,
            WHITE => 69,
            BLACK => 81,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 44,
            GREEN => 144,
            RED => 92,
            YELLOW => 184,
            PURPLE => 33,
            GREY => 88,
            WHITE => 19,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 214,
            GREEN => 122,
            RED => 141,
            YELLOW => 11,
            PURPLE => 231,
            GREY => 196,
            WHITE => 225,
            BLACK => 153,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 45,
            GREEN => 8,
            RED => 109,
            YELLOW => 114,
            PURPLE => 147,
            GREY => 42,
            WHITE => 123,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 247,
            GREEN => 3,
            RED => 36,
            YELLOW => 110,
            PURPLE => 24,
            GREY => 0,
            WHITE => 213,
            BLACK => 181,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 253,
            GREEN => 82,
            RED => 46,
            YELLOW => 94,
            PURPLE => 69,
            GREY => 50,
            WHITE => 49,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 223,
            GREEN => 150,
            RED => 4,
            YELLOW => 215,
            PURPLE => 250,
            GREY => 54,
            WHITE => 60,
            BLACK => 43,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 189,
            GREEN => 201,
            RED => 218,
            YELLOW => 27,
            PURPLE => 227,
            GREY => 25,
            WHITE => 65,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 69,
            GREEN => 39,
            RED => 225,
            YELLOW => 233,
            PURPLE => 171,
            GREY => 245,
            WHITE => 195,
            BLACK => 205,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 117,
            GREEN => 112,
            RED => 211,
            YELLOW => 15,
            PURPLE => 33,
            GREY => 213,
            WHITE => 225,
            BLACK => 188,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 252,
            GREEN => 203,
            RED => 222,
            YELLOW => 13,
            PURPLE => 185,
            GREY => 13,
            WHITE => 59,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if false {
	let n = match buffer[25] {
            BLUE => 239,
            GREEN => 70,
            RED => 185,
            YELLOW => 160,
            PURPLE => 201,
            GREY => 176,
            WHITE => 176,
            BLACK => 73,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 199,
            GREEN => 142,
            RED => 150,
            YELLOW => 159,
            PURPLE => 233,
            GREY => 168,
            WHITE => 26,
            BLACK => 74,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 213,
            GREEN => 174,
            RED => 50,
            YELLOW => 175,
            PURPLE => 83,
            GREY => 98,
            WHITE => 160,
            BLACK => 160,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 143,
            GREEN => 146,
            RED => 177,
            YELLOW => 71,
            PURPLE => 65,
            GREY => 195,
            WHITE => 101,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 33,
            GREEN => 168,
            RED => 101,
            YELLOW => 144,
            PURPLE => 101,
            GREY => 218,
            WHITE => 111,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 30,
            GREEN => 203,
            RED => 94,
            YELLOW => 238,
            PURPLE => 185,
            GREY => 154,
            WHITE => 50,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 164,
            GREEN => 17,
            RED => 200,
            YELLOW => 255,
            PURPLE => 209,
            GREY => 126,
            WHITE => 177,
            BLACK => 3,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 20,
            GREEN => 41,
            RED => 137,
            YELLOW => 127,
            PURPLE => 66,
            GREY => 166,
            WHITE => 100,
            BLACK => 186,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 118,
            GREEN => 172,
            RED => 122,
            YELLOW => 195,
            PURPLE => 241,
            GREY => 234,
            WHITE => 220,
            BLACK => 75,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 217,
            GREEN => 29,
            RED => 25,
            YELLOW => 107,
            PURPLE => 5,
            GREY => 13,
            WHITE => 79,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 154,
            GREEN => 201,
            RED => 65,
            YELLOW => 101,
            PURPLE => 77,
            GREY => 186,
            WHITE => 130,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 25,
            GREEN => 218,
            RED => 212,
            YELLOW => 183,
            PURPLE => 58,
            GREY => 154,
            WHITE => 34,
            BLACK => 30,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 120,
            GREEN => 162,
            RED => 12,
            YELLOW => 92,
            PURPLE => 124,
            GREY => 199,
            WHITE => 159,
            BLACK => 190,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 121,
            GREEN => 80,
            RED => 14,
            YELLOW => 104,
            PURPLE => 228,
            GREY => 43,
            WHITE => 159,
            BLACK => 234,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 168,
            GREEN => 119,
            RED => 174,
            YELLOW => 29,
            PURPLE => 191,
            GREY => 198,
            WHITE => 234,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 233,
            GREEN => 95,
            RED => 37,
            YELLOW => 66,
            PURPLE => 210,
            GREY => 41,
            WHITE => 5,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 146,
            GREEN => 230,
            RED => 194,
            YELLOW => 20,
            PURPLE => 126,
            GREY => 145,
            WHITE => 87,
            BLACK => 41,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 162,
            GREEN => 12,
            RED => 58,
            YELLOW => 139,
            PURPLE => 170,
            GREY => 177,
            WHITE => 55,
            BLACK => 122,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 150,
            GREEN => 230,
            RED => 87,
            YELLOW => 252,
            PURPLE => 85,
            GREY => 73,
            WHITE => 30,
            BLACK => 152,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 154,
            GREEN => 245,
            RED => 240,
            YELLOW => 77,
            PURPLE => 147,
            GREY => 224,
            WHITE => 110,
            BLACK => 95,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 52,
            GREEN => 47,
            RED => 47,
            YELLOW => 187,
            PURPLE => 121,
            GREY => 125,
            WHITE => 161,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 39,
            GREEN => 143,
            RED => 154,
            YELLOW => 122,
            PURPLE => 126,
            GREY => 222,
            WHITE => 76,
            BLACK => 158,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 225,
            GREEN => 73,
            RED => 37,
            YELLOW => 87,
            PURPLE => 62,
            GREY => 226,
            WHITE => 39,
            BLACK => 28,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 110,
            GREEN => 32,
            RED => 12,
            YELLOW => 30,
            PURPLE => 51,
            GREY => 156,
            WHITE => 69,
            BLACK => 118,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 126,
            GREEN => 171,
            RED => 63,
            YELLOW => 84,
            PURPLE => 228,
            GREY => 102,
            WHITE => 228,
            BLACK => 204,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 187,
            GREEN => 223,
            RED => 116,
            YELLOW => 188,
            PURPLE => 39,
            GREY => 10,
            WHITE => 87,
            BLACK => 137,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 108,
            GREEN => 103,
            RED => 84,
            YELLOW => 229,
            PURPLE => 107,
            GREY => 54,
            WHITE => 150,
            BLACK => 67,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 191,
            GREEN => 250,
            RED => 207,
            YELLOW => 210,
            PURPLE => 196,
            GREY => 15,
            WHITE => 79,
            BLACK => 101,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 178,
            GREEN => 247,
            RED => 85,
            YELLOW => 69,
            PURPLE => 15,
            GREY => 184,
            WHITE => 138,
            BLACK => 46,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 18,
            GREEN => 123,
            RED => 118,
            YELLOW => 31,
            PURPLE => 199,
            GREY => 39,
            WHITE => 94,
            BLACK => 116,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 177,
            GREEN => 83,
            RED => 51,
            YELLOW => 150,
            PURPLE => 64,
            GREY => 134,
            WHITE => 21,
            BLACK => 249,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 87,
            GREEN => 44,
            RED => 125,
            YELLOW => 37,
            PURPLE => 58,
            GREY => 3,
            WHITE => 148,
            BLACK => 114,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 78,
            GREEN => 162,
            RED => 58,
            YELLOW => 120,
            PURPLE => 239,
            GREY => 125,
            WHITE => 181,
            BLACK => 232,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 84,
            GREEN => 105,
            RED => 118,
            YELLOW => 108,
            PURPLE => 195,
            GREY => 9,
            WHITE => 125,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 126,
            GREEN => 9,
            RED => 97,
            YELLOW => 249,
            PURPLE => 188,
            GREY => 116,
            WHITE => 19,
            BLACK => 254,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 104,
            GREEN => 57,
            RED => 47,
            YELLOW => 205,
            PURPLE => 221,
            GREY => 50,
            WHITE => 247,
            BLACK => 20,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if nor(nor(nor(nor(nor(false, false), nor(false, match buffer[17] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(false, match buffer[9] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), nor(false, match buffer[16] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }))), nor(nor(nor(false, match buffer[13] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), nor(match buffer[18] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, false)), nor(nor(match buffer[5] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, nor(match buffer[12] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[2] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })), nor(match buffer[15] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[8] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, })))), nor(nor(nor(nor(match buffer[6] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, false), nor(false, match buffer[10] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })), nor(nor(false, match buffer[14] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), nor(nor(match buffer[11] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[7] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), nor(match buffer[1] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[4] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })))), nor(nor(nor(false, false), nor(false, false)), nor(nor(false, match buffer[0] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), nor(false, false))))) {
	let n = match buffer[44] {
            BLUE => 91,
            GREEN => 104,
            RED => 65,
            YELLOW => 75,
            PURPLE => 111,
            GREY => 247,
            WHITE => 112,
            BLACK => 255,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 106,
            GREEN => 181,
            RED => 118,
            YELLOW => 170,
            PURPLE => 162,
            GREY => 233,
            WHITE => 219,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 12,
            GREEN => 36,
            RED => 174,
            YELLOW => 178,
            PURPLE => 120,
            GREY => 247,
            WHITE => 165,
            BLACK => 173,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 246,
            GREEN => 12,
            RED => 145,
            YELLOW => 72,
            PURPLE => 73,
            GREY => 188,
            WHITE => 156,
            BLACK => 123,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 219,
            GREEN => 138,
            RED => 223,
            YELLOW => 191,
            PURPLE => 221,
            GREY => 76,
            WHITE => 56,
            BLACK => 149,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 52,
            GREEN => 181,
            RED => 96,
            YELLOW => 37,
            PURPLE => 95,
            GREY => 218,
            WHITE => 86,
            BLACK => 151,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 220,
            GREEN => 97,
            RED => 148,
            YELLOW => 30,
            PURPLE => 138,
            GREY => 211,
            WHITE => 127,
            BLACK => 89,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 243,
            GREEN => 251,
            RED => 143,
            YELLOW => 217,
            PURPLE => 13,
            GREY => 12,
            WHITE => 128,
            BLACK => 216,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 37,
            GREEN => 4,
            RED => 1,
            YELLOW => 136,
            PURPLE => 164,
            GREY => 192,
            WHITE => 84,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 55,
            GREEN => 212,
            RED => 30,
            YELLOW => 116,
            PURPLE => 115,
            GREY => 52,
            WHITE => 166,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 236,
            GREEN => 164,
            RED => 4,
            YELLOW => 186,
            PURPLE => 93,
            GREY => 50,
            WHITE => 13,
            BLACK => 211,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 132,
            GREEN => 0,
            RED => 42,
            YELLOW => 143,
            PURPLE => 22,
            GREY => 138,
            WHITE => 125,
            BLACK => 54,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 98,
            GREEN => 133,
            RED => 179,
            YELLOW => 238,
            PURPLE => 173,
            GREY => 130,
            WHITE => 100,
            BLACK => 49,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 227,
            GREEN => 246,
            RED => 198,
            YELLOW => 151,
            PURPLE => 47,
            GREY => 0,
            WHITE => 239,
            BLACK => 100,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 6,
            GREEN => 156,
            RED => 204,
            YELLOW => 128,
            PURPLE => 119,
            GREY => 229,
            WHITE => 175,
            BLACK => 217,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 77,
            GREEN => 83,
            RED => 117,
            YELLOW => 214,
            PURPLE => 73,
            GREY => 140,
            WHITE => 1,
            BLACK => 145,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 176,
            GREEN => 139,
            RED => 64,
            YELLOW => 171,
            PURPLE => 3,
            GREY => 168,
            WHITE => 97,
            BLACK => 186,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 99,
            GREEN => 79,
            RED => 152,
            YELLOW => 91,
            PURPLE => 207,
            GREY => 53,
            WHITE => 239,
            BLACK => 34,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 137,
            GREEN => 69,
            RED => 202,
            YELLOW => 29,
            PURPLE => 172,
            GREY => 156,
            WHITE => 43,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 207,
            GREEN => 239,
            RED => 245,
            YELLOW => 148,
            PURPLE => 33,
            GREY => 224,
            WHITE => 14,
            BLACK => 80,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 35,
            GREEN => 217,
            RED => 6,
            YELLOW => 234,
            PURPLE => 34,
            GREY => 160,
            WHITE => 104,
            BLACK => 248,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 155,
            GREEN => 40,
            RED => 58,
            YELLOW => 180,
            PURPLE => 68,
            GREY => 220,
            WHITE => 116,
            BLACK => 250,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 169,
            GREEN => 7,
            RED => 27,
            YELLOW => 198,
            PURPLE => 121,
            GREY => 186,
            WHITE => 243,
            BLACK => 22,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 164,
            GREEN => 153,
            RED => 110,
            YELLOW => 155,
            PURPLE => 65,
            GREY => 80,
            WHITE => 144,
            BLACK => 170,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 110,
            GREEN => 78,
            RED => 99,
            YELLOW => 1,
            PURPLE => 12,
            GREY => 27,
            WHITE => 7,
            BLACK => 174,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 127,
            GREEN => 52,
            RED => 69,
            YELLOW => 56,
            PURPLE => 222,
            GREY => 217,
            WHITE => 246,
            BLACK => 37,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 105,
            GREEN => 172,
            RED => 139,
            YELLOW => 222,
            PURPLE => 189,
            GREY => 40,
            WHITE => 178,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 189,
            GREEN => 53,
            RED => 189,
            YELLOW => 250,
            PURPLE => 169,
            GREY => 202,
            WHITE => 194,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 221,
            GREEN => 92,
            RED => 216,
            YELLOW => 180,
            PURPLE => 38,
            GREY => 215,
            WHITE => 66,
            BLACK => 90,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 161,
            GREEN => 165,
            RED => 255,
            YELLOW => 216,
            PURPLE => 26,
            GREY => 173,
            WHITE => 247,
            BLACK => 60,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 162,
            GREEN => 52,
            RED => 162,
            YELLOW => 68,
            PURPLE => 224,
            GREY => 59,
            WHITE => 56,
            BLACK => 137,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 74,
            GREEN => 33,
            RED => 81,
            YELLOW => 65,
            PURPLE => 179,
            GREY => 9,
            WHITE => 222,
            BLACK => 234,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 30,
            GREEN => 115,
            RED => 91,
            YELLOW => 191,
            PURPLE => 168,
            GREY => 17,
            WHITE => 203,
            BLACK => 61,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 29,
            GREEN => 116,
            RED => 40,
            YELLOW => 107,
            PURPLE => 126,
            GREY => 180,
            WHITE => 32,
            BLACK => 227,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 94,
            GREEN => 72,
            RED => 212,
            YELLOW => 240,
            PURPLE => 149,
            GREY => 199,
            WHITE => 8,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 190,
            GREEN => 216,
            RED => 121,
            YELLOW => 36,
            PURPLE => 77,
            GREY => 74,
            WHITE => 237,
            BLACK => 55,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 159,
            GREEN => 102,
            RED => 218,
            YELLOW => 151,
            PURPLE => 61,
            GREY => 1,
            WHITE => 135,
            BLACK => 168,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 83,
            GREEN => 175,
            RED => 221,
            YELLOW => 110,
            PURPLE => 31,
            GREY => 27,
            WHITE => 234,
            BLACK => 125,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 186,
            GREEN => 93,
            RED => 140,
            YELLOW => 35,
            PURPLE => 157,
            GREY => 74,
            WHITE => 221,
            BLACK => 124,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 230,
            GREEN => 155,
            RED => 184,
            YELLOW => 227,
            PURPLE => 87,
            GREY => 196,
            WHITE => 135,
            BLACK => 178,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 18,
            GREEN => 11,
            RED => 91,
            YELLOW => 178,
            PURPLE => 189,
            GREY => 111,
            WHITE => 47,
            BLACK => 48,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 150,
            GREEN => 14,
            RED => 165,
            YELLOW => 222,
            PURPLE => 9,
            GREY => 216,
            WHITE => 17,
            BLACK => 22,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 220,
            GREEN => 209,
            RED => 122,
            YELLOW => 97,
            PURPLE => 47,
            GREY => 83,
            WHITE => 193,
            BLACK => 186,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 12,
            GREEN => 225,
            RED => 40,
            YELLOW => 167,
            PURPLE => 183,
            GREY => 22,
            WHITE => 41,
            BLACK => 22,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else {
	let n = match buffer[32] {
            BLUE => 18,
            GREEN => 201,
            RED => 96,
            YELLOW => 154,
            PURPLE => 172,
            GREY => 107,
            WHITE => 190,
            BLACK => 248,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 35,
            GREEN => 191,
            RED => 70,
            YELLOW => 22,
            PURPLE => 118,
            GREY => 32,
            WHITE => 99,
            BLACK => 18,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 71,
            GREEN => 252,
            RED => 185,
            YELLOW => 182,
            PURPLE => 238,
            GREY => 169,
            WHITE => 136,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 7,
            GREEN => 118,
            RED => 227,
            YELLOW => 184,
            PURPLE => 218,
            GREY => 29,
            WHITE => 137,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 183,
            GREEN => 250,
            RED => 176,
            YELLOW => 186,
            PURPLE => 234,
            GREY => 231,
            WHITE => 245,
            BLACK => 123,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 2,
            GREEN => 167,
            RED => 17,
            YELLOW => 59,
            PURPLE => 223,
            GREY => 150,
            WHITE => 219,
            BLACK => 199,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 16,
            GREEN => 26,
            RED => 205,
            YELLOW => 230,
            PURPLE => 101,
            GREY => 147,
            WHITE => 239,
            BLACK => 168,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 29,
            GREEN => 103,
            RED => 34,
            YELLOW => 75,
            PURPLE => 247,
            GREY => 113,
            WHITE => 182,
            BLACK => 46,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 10,
            GREEN => 155,
            RED => 140,
            YELLOW => 62,
            PURPLE => 50,
            GREY => 1,
            WHITE => 105,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 220,
            GREEN => 153,
            RED => 105,
            YELLOW => 206,
            PURPLE => 62,
            GREY => 126,
            WHITE => 2,
            BLACK => 241,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 117,
            GREEN => 166,
            RED => 69,
            YELLOW => 133,
            PURPLE => 127,
            GREY => 225,
            WHITE => 24,
            BLACK => 138,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 132,
            GREEN => 209,
            RED => 228,
            YELLOW => 9,
            PURPLE => 73,
            GREY => 3,
            WHITE => 253,
            BLACK => 62,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 191,
            GREEN => 215,
            RED => 112,
            YELLOW => 232,
            PURPLE => 232,
            GREY => 224,
            WHITE => 184,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 183,
            GREEN => 212,
            RED => 102,
            YELLOW => 24,
            PURPLE => 49,
            GREY => 202,
            WHITE => 250,
            BLACK => 247,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 198,
            GREEN => 22,
            RED => 163,
            YELLOW => 93,
            PURPLE => 89,
            GREY => 201,
            WHITE => 245,
            BLACK => 190,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[70] {
            BLUE => 237,
            GREEN => 13,
            RED => 58,
            YELLOW => 171,
            PURPLE => 55,
            GREY => 33,
            WHITE => 189,
            BLACK => 45,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 128,
            GREEN => 247,
            RED => 43,
            YELLOW => 97,
            PURPLE => 39,
            GREY => 129,
            WHITE => 242,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 152,
            GREEN => 178,
            RED => 78,
            YELLOW => 70,
            PURPLE => 128,
            GREY => 100,
            WHITE => 5,
            BLACK => 12,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 123,
            GREEN => 50,
            RED => 214,
            YELLOW => 171,
            PURPLE => 129,
            GREY => 233,
            WHITE => 170,
            BLACK => 113,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 225,
            GREEN => 143,
            RED => 39,
            YELLOW => 121,
            PURPLE => 43,
            GREY => 253,
            WHITE => 163,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 74,
            GREEN => 93,
            RED => 181,
            YELLOW => 28,
            PURPLE => 111,
            GREY => 65,
            WHITE => 55,
            BLACK => 27,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 227,
            GREEN => 221,
            RED => 0,
            YELLOW => 223,
            PURPLE => 60,
            GREY => 199,
            WHITE => 193,
            BLACK => 141,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 148,
            GREEN => 202,
            RED => 70,
            YELLOW => 100,
            PURPLE => 145,
            GREY => 142,
            WHITE => 8,
            BLACK => 18,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 18,
            GREEN => 245,
            RED => 104,
            YELLOW => 87,
            PURPLE => 108,
            GREY => 203,
            WHITE => 87,
            BLACK => 185,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 44,
            GREEN => 173,
            RED => 106,
            YELLOW => 33,
            PURPLE => 106,
            GREY => 170,
            WHITE => 235,
            BLACK => 59,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 92,
            GREEN => 83,
            RED => 243,
            YELLOW => 232,
            PURPLE => 170,
            GREY => 11,
            WHITE => 160,
            BLACK => 252,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 126,
            GREEN => 47,
            RED => 245,
            YELLOW => 188,
            PURPLE => 186,
            GREY => 157,
            WHITE => 140,
            BLACK => 163,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 228,
            GREEN => 148,
            RED => 37,
            YELLOW => 182,
            PURPLE => 148,
            GREY => 113,
            WHITE => 17,
            BLACK => 135,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 146,
            GREEN => 76,
            RED => 190,
            YELLOW => 67,
            PURPLE => 2,
            GREY => 6,
            WHITE => 183,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 146,
            GREEN => 2,
            RED => 15,
            YELLOW => 180,
            PURPLE => 103,
            GREY => 65,
            WHITE => 162,
            BLACK => 223,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 37,
            GREEN => 91,
            RED => 187,
            YELLOW => 123,
            PURPLE => 185,
            GREY => 233,
            WHITE => 154,
            BLACK => 33,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 12,
            GREEN => 29,
            RED => 5,
            YELLOW => 99,
            PURPLE => 95,
            GREY => 49,
            WHITE => 12,
            BLACK => 19,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 215,
            GREEN => 116,
            RED => 239,
            YELLOW => 254,
            PURPLE => 129,
            GREY => 236,
            WHITE => 246,
            BLACK => 81,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 21,
            GREEN => 172,
            RED => 71,
            YELLOW => 194,
            PURPLE => 93,
            GREY => 35,
            WHITE => 141,
            BLACK => 214,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 20,
            GREEN => 31,
            RED => 226,
            YELLOW => 51,
            PURPLE => 227,
            GREY => 46,
            WHITE => 167,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[73] {
            BLUE => 141,
            GREEN => 185,
            RED => 138,
            YELLOW => 88,
            PURPLE => 207,
            GREY => 230,
            WHITE => 51,
            BLACK => 64,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 118,
            GREEN => 228,
            RED => 199,
            YELLOW => 238,
            PURPLE => 199,
            GREY => 201,
            WHITE => 146,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 10,
            GREEN => 117,
            RED => 95,
            YELLOW => 167,
            PURPLE => 196,
            GREY => 69,
            WHITE => 235,
            BLACK => 83,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[65] {
            BLUE => 153,
            GREEN => 99,
            RED => 177,
            YELLOW => 179,
            PURPLE => 176,
            GREY => 244,
            WHITE => 192,
            BLACK => 82,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 117,
            GREEN => 158,
            RED => 161,
            YELLOW => 198,
            PURPLE => 246,
            GREY => 114,
            WHITE => 172,
            BLACK => 106,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 55,
            GREEN => 0,
            RED => 136,
            YELLOW => 171,
            PURPLE => 237,
            GREY => 14,
            WHITE => 222,
            BLACK => 217,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 159,
            GREEN => 60,
            RED => 228,
            YELLOW => 173,
            PURPLE => 19,
            GREY => 58,
            WHITE => 64,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 79,
            GREEN => 146,
            RED => 166,
            YELLOW => 96,
            PURPLE => 250,
            GREY => 114,
            WHITE => 53,
            BLACK => 53,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 124,
            GREEN => 192,
            RED => 115,
            YELLOW => 1,
            PURPLE => 123,
            GREY => 18,
            WHITE => 237,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 251,
            GREEN => 238,
            RED => 142,
            YELLOW => 179,
            PURPLE => 66,
            GREY => 65,
            WHITE => 66,
            BLACK => 209,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 124,
            GREEN => 34,
            RED => 144,
            YELLOW => 96,
            PURPLE => 211,
            GREY => 129,
            WHITE => 62,
            BLACK => 66,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 233,
            GREEN => 88,
            RED => 81,
            YELLOW => 75,
            PURPLE => 142,
            GREY => 201,
            WHITE => 241,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 13,
            GREEN => 152,
            RED => 126,
            YELLOW => 66,
            PURPLE => 234,
            GREY => 54,
            WHITE => 150,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 113,
            GREEN => 60,
            RED => 161,
            YELLOW => 188,
            PURPLE => 8,
            GREY => 116,
            WHITE => 128,
            BLACK => 222,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[72] {
            BLUE => 27,
            GREEN => 13,
            RED => 88,
            YELLOW => 244,
            PURPLE => 158,
            GREY => 77,
            WHITE => 184,
            BLACK => 46,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 197,
            GREEN => 58,
            RED => 86,
            YELLOW => 178,
            PURPLE => 131,
            GREY => 117,
            WHITE => 157,
            BLACK => 82,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 111,
            GREEN => 100,
            RED => 55,
            YELLOW => 7,
            PURPLE => 92,
            GREY => 62,
            WHITE => 125,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 109,
            GREEN => 48,
            RED => 38,
            YELLOW => 7,
            PURPLE => 20,
            GREY => 66,
            WHITE => 34,
            BLACK => 146,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 178,
            GREEN => 48,
            RED => 99,
            YELLOW => 16,
            PURPLE => 219,
            GREY => 241,
            WHITE => 80,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 6,
            GREEN => 233,
            RED => 121,
            YELLOW => 73,
            PURPLE => 193,
            GREY => 2,
            WHITE => 91,
            BLACK => 79,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 149,
            GREEN => 156,
            RED => 46,
            YELLOW => 213,
            PURPLE => 144,
            GREY => 107,
            WHITE => 117,
            BLACK => 89,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 199,
            GREEN => 23,
            RED => 232,
            YELLOW => 69,
            PURPLE => 29,
            GREY => 38,
            WHITE => 189,
            BLACK => 51,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 230,
            GREEN => 49,
            RED => 225,
            YELLOW => 241,
            PURPLE => 215,
            GREY => 23,
            WHITE => 184,
            BLACK => 117,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 231,
            GREEN => 145,
            RED => 135,
            YELLOW => 83,
            PURPLE => 149,
            GREY => 243,
            WHITE => 35,
            BLACK => 243,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 221,
            GREEN => 67,
            RED => 4,
            YELLOW => 68,
            PURPLE => 184,
            GREY => 65,
            WHITE => 5,
            BLACK => 55,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 199,
            GREEN => 197,
            RED => 154,
            YELLOW => 195,
            PURPLE => 74,
            GREY => 62,
            WHITE => 119,
            BLACK => 57,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 120,
            GREEN => 36,
            RED => 37,
            YELLOW => 217,
            PURPLE => 94,
            GREY => 141,
            WHITE => 1,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[71] {
            BLUE => 21,
            GREEN => 13,
            RED => 249,
            YELLOW => 224,
            PURPLE => 70,
            GREY => 127,
            WHITE => 182,
            BLACK => 171,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[69] {
            BLUE => 99,
            GREEN => 53,
            RED => 54,
            YELLOW => 198,
            PURPLE => 142,
            GREY => 36,
            WHITE => 34,
            BLACK => 129,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 192,
            GREEN => 231,
            RED => 67,
            YELLOW => 207,
            PURPLE => 31,
            GREY => 4,
            WHITE => 123,
            BLACK => 249,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 13,
            GREEN => 127,
            RED => 96,
            YELLOW => 136,
            PURPLE => 111,
            GREY => 62,
            WHITE => 98,
            BLACK => 80,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 198,
            GREEN => 154,
            RED => 230,
            YELLOW => 239,
            PURPLE => 117,
            GREY => 202,
            WHITE => 156,
            BLACK => 9,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 248,
            GREEN => 189,
            RED => 134,
            YELLOW => 252,
            PURPLE => 220,
            GREY => 63,
            WHITE => 172,
            BLACK => 143,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 186,
            GREEN => 84,
            RED => 170,
            YELLOW => 166,
            PURPLE => 239,
            GREY => 243,
            WHITE => 7,
            BLACK => 50,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[56] {
            BLUE => 14,
            GREEN => 246,
            RED => 163,
            YELLOW => 118,
            PURPLE => 64,
            GREY => 127,
            WHITE => 226,
            BLACK => 228,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 184,
            GREEN => 83,
            RED => 45,
            YELLOW => 252,
            PURPLE => 222,
            GREY => 59,
            WHITE => 48,
            BLACK => 83,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 95,
            GREEN => 99,
            RED => 246,
            YELLOW => 101,
            PURPLE => 63,
            GREY => 156,
            WHITE => 25,
            BLACK => 38,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 186,
            GREEN => 222,
            RED => 134,
            YELLOW => 113,
            PURPLE => 160,
            GREY => 63,
            WHITE => 1,
            BLACK => 173,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}

    }

    if input.pressed_this_frame(Button::Select) {
        if nor(nor(nor(nor(nor(match buffer[19] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[12] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(nor(match buffer[4] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[22] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), false)), nor(nor(false, false), nor(match buffer[26] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, nor(match buffer[3] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[10] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })))), nor(nor(nor(match buffer[24] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, false), nor(false, nor(match buffer[7] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[15] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }))), nor(nor(nor(match buffer[13] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[18] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), match buffer[25] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), nor(false, nor(match buffer[6] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[17] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }))))), nor(nor(nor(nor(match buffer[14] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[2] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), nor(false, match buffer[1] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(match buffer[16] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[0] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), nor(match buffer[11] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, nor(match buffer[8] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[20] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })))), nor(nor(nor(false, false), nor(match buffer[9] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }, false)), nor(nor(false, false), nor(nor(match buffer[23] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[5] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), match buffer[21] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }))))) {
	let n = match buffer[21] {
            BLUE => 193,
            GREEN => 98,
            RED => 1,
            YELLOW => 240,
            PURPLE => 195,
            GREY => 47,
            WHITE => 195,
            BLACK => 15,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 24,
            GREEN => 155,
            RED => 209,
            YELLOW => 40,
            PURPLE => 168,
            GREY => 118,
            WHITE => 195,
            BLACK => 132,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 87,
            GREEN => 83,
            RED => 105,
            YELLOW => 105,
            PURPLE => 153,
            GREY => 112,
            WHITE => 217,
            BLACK => 183,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 16,
            GREEN => 252,
            RED => 57,
            YELLOW => 192,
            PURPLE => 26,
            GREY => 249,
            WHITE => 100,
            BLACK => 104,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 29,
            GREEN => 26,
            RED => 152,
            YELLOW => 65,
            PURPLE => 97,
            GREY => 136,
            WHITE => 122,
            BLACK => 36,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 180,
            GREEN => 142,
            RED => 124,
            YELLOW => 182,
            PURPLE => 149,
            GREY => 7,
            WHITE => 13,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 188,
            GREEN => 236,
            RED => 22,
            YELLOW => 212,
            PURPLE => 72,
            GREY => 169,
            WHITE => 149,
            BLACK => 118,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 237,
            GREEN => 158,
            RED => 216,
            YELLOW => 206,
            PURPLE => 136,
            GREY => 87,
            WHITE => 226,
            BLACK => 152,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 91,
            GREEN => 54,
            RED => 190,
            YELLOW => 235,
            PURPLE => 90,
            GREY => 30,
            WHITE => 228,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 224,
            GREEN => 191,
            RED => 237,
            YELLOW => 29,
            PURPLE => 185,
            GREY => 159,
            WHITE => 251,
            BLACK => 0,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 63,
            GREEN => 36,
            RED => 201,
            YELLOW => 240,
            PURPLE => 190,
            GREY => 68,
            WHITE => 56,
            BLACK => 121,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 82,
            GREEN => 2,
            RED => 87,
            YELLOW => 48,
            PURPLE => 156,
            GREY => 46,
            WHITE => 202,
            BLACK => 106,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 239,
            GREEN => 65,
            RED => 153,
            YELLOW => 62,
            PURPLE => 111,
            GREY => 234,
            WHITE => 110,
            BLACK => 72,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 25,
            GREEN => 63,
            RED => 60,
            YELLOW => 187,
            PURPLE => 106,
            GREY => 25,
            WHITE => 15,
            BLACK => 121,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 167,
            GREEN => 180,
            RED => 252,
            YELLOW => 205,
            PURPLE => 70,
            GREY => 47,
            WHITE => 1,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 103,
            GREEN => 16,
            RED => 193,
            YELLOW => 220,
            PURPLE => 161,
            GREY => 202,
            WHITE => 179,
            BLACK => 143,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 159,
            GREEN => 248,
            RED => 56,
            YELLOW => 217,
            PURPLE => 194,
            GREY => 57,
            WHITE => 44,
            BLACK => 242,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 44,
            GREEN => 47,
            RED => 85,
            YELLOW => 82,
            PURPLE => 45,
            GREY => 120,
            WHITE => 69,
            BLACK => 54,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 196,
            GREEN => 168,
            RED => 20,
            YELLOW => 209,
            PURPLE => 45,
            GREY => 13,
            WHITE => 222,
            BLACK => 229,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 178,
            GREEN => 107,
            RED => 191,
            YELLOW => 12,
            PURPLE => 176,
            GREY => 142,
            WHITE => 56,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 233,
            GREEN => 187,
            RED => 226,
            YELLOW => 162,
            PURPLE => 100,
            GREY => 245,
            WHITE => 70,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 120,
            GREEN => 204,
            RED => 138,
            YELLOW => 189,
            PURPLE => 68,
            GREY => 171,
            WHITE => 234,
            BLACK => 50,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 161,
            GREEN => 73,
            RED => 245,
            YELLOW => 247,
            PURPLE => 130,
            GREY => 247,
            WHITE => 233,
            BLACK => 248,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 222,
            GREEN => 203,
            RED => 231,
            YELLOW => 160,
            PURPLE => 52,
            GREY => 80,
            WHITE => 168,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 168,
            GREEN => 58,
            RED => 186,
            YELLOW => 119,
            PURPLE => 39,
            GREY => 42,
            WHITE => 18,
            BLACK => 103,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 0,
            GREEN => 241,
            RED => 21,
            YELLOW => 197,
            PURPLE => 107,
            GREY => 35,
            WHITE => 184,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 184,
            GREEN => 214,
            RED => 234,
            YELLOW => 84,
            PURPLE => 26,
            GREY => 222,
            WHITE => 68,
            BLACK => 73,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 113,
            GREEN => 187,
            RED => 173,
            YELLOW => 132,
            PURPLE => 252,
            GREY => 14,
            WHITE => 42,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 187,
            GREEN => 117,
            RED => 40,
            YELLOW => 144,
            PURPLE => 193,
            GREY => 65,
            WHITE => 133,
            BLACK => 86,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 242,
            GREEN => 254,
            RED => 68,
            YELLOW => 92,
            PURPLE => 147,
            GREY => 104,
            WHITE => 114,
            BLACK => 84,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 196,
            GREEN => 15,
            RED => 245,
            YELLOW => 0,
            PURPLE => 148,
            GREY => 24,
            WHITE => 41,
            BLACK => 102,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 221,
            GREEN => 164,
            RED => 249,
            YELLOW => 38,
            PURPLE => 105,
            GREY => 43,
            WHITE => 160,
            BLACK => 141,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 161,
            GREEN => 149,
            RED => 193,
            YELLOW => 10,
            PURPLE => 111,
            GREY => 136,
            WHITE => 215,
            BLACK => 197,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 139,
            GREEN => 93,
            RED => 35,
            YELLOW => 174,
            PURPLE => 81,
            GREY => 227,
            WHITE => 83,
            BLACK => 144,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else {
	let n = match buffer[14] {
            BLUE => 109,
            GREEN => 72,
            RED => 210,
            YELLOW => 55,
            PURPLE => 141,
            GREY => 73,
            WHITE => 231,
            BLACK => 234,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 72,
            GREEN => 228,
            RED => 185,
            YELLOW => 251,
            PURPLE => 147,
            GREY => 63,
            WHITE => 240,
            BLACK => 19,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 157,
            GREEN => 66,
            RED => 217,
            YELLOW => 133,
            PURPLE => 10,
            GREY => 232,
            WHITE => 162,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 57,
            GREEN => 43,
            RED => 57,
            YELLOW => 43,
            PURPLE => 77,
            GREY => 198,
            WHITE => 199,
            BLACK => 204,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 219,
            GREEN => 180,
            RED => 95,
            YELLOW => 238,
            PURPLE => 15,
            GREY => 37,
            WHITE => 112,
            BLACK => 184,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 35,
            GREEN => 200,
            RED => 151,
            YELLOW => 242,
            PURPLE => 78,
            GREY => 183,
            WHITE => 219,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 122,
            GREEN => 53,
            RED => 197,
            YELLOW => 206,
            PURPLE => 144,
            GREY => 38,
            WHITE => 147,
            BLACK => 222,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 235,
            GREEN => 100,
            RED => 176,
            YELLOW => 199,
            PURPLE => 190,
            GREY => 45,
            WHITE => 166,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 3,
            GREEN => 89,
            RED => 52,
            YELLOW => 26,
            PURPLE => 231,
            GREY => 229,
            WHITE => 71,
            BLACK => 235,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 133,
            GREEN => 58,
            RED => 179,
            YELLOW => 49,
            PURPLE => 60,
            GREY => 81,
            WHITE => 4,
            BLACK => 252,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 216,
            GREEN => 89,
            RED => 217,
            YELLOW => 153,
            PURPLE => 68,
            GREY => 45,
            WHITE => 132,
            BLACK => 27,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 23,
            GREEN => 235,
            RED => 5,
            YELLOW => 199,
            PURPLE => 67,
            GREY => 118,
            WHITE => 95,
            BLACK => 39,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 19,
            GREEN => 21,
            RED => 104,
            YELLOW => 179,
            PURPLE => 6,
            GREY => 61,
            WHITE => 191,
            BLACK => 132,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 152,
            GREEN => 47,
            RED => 252,
            YELLOW => 147,
            PURPLE => 199,
            GREY => 117,
            WHITE => 135,
            BLACK => 76,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 31,
            GREEN => 172,
            RED => 165,
            YELLOW => 217,
            PURPLE => 132,
            GREY => 193,
            WHITE => 57,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 228,
            GREEN => 35,
            RED => 174,
            YELLOW => 41,
            PURPLE => 103,
            GREY => 91,
            WHITE => 107,
            BLACK => 255,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}

    }

    if input.pressed_this_frame(Button::Start) {
        if nor(nor(nor(nor(nor(match buffer[12] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[19] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), nor(false, false)), nor(nor(match buffer[0] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[7] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(match buffer[1] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[20] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }))), nor(nor(nor(match buffer[3] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, false), nor(match buffer[4] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, false)), nor(nor(match buffer[8] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, false), nor(nor(match buffer[11] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[18] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), match buffer[5] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })))), nor(nor(nor(nor(false, false), nor(false, match buffer[9] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(nor(match buffer[14] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[2] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), nor(match buffer[6] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[13] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, })), nor(false, false))), nor(nor(nor(false, false), nor(match buffer[16] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[15] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(match buffer[10] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, false), nor(false, false))))) {
	let n = match buffer[56] {
            BLUE => 28,
            GREEN => 141,
            RED => 56,
            YELLOW => 108,
            PURPLE => 117,
            GREY => 235,
            WHITE => 229,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 215,
            GREEN => 41,
            RED => 248,
            YELLOW => 126,
            PURPLE => 113,
            GREY => 59,
            WHITE => 70,
            BLACK => 203,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 95,
            GREEN => 240,
            RED => 128,
            YELLOW => 186,
            PURPLE => 160,
            GREY => 135,
            WHITE => 192,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 16,
            GREEN => 237,
            RED => 69,
            YELLOW => 246,
            PURPLE => 241,
            GREY => 205,
            WHITE => 81,
            BLACK => 89,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 235,
            GREEN => 129,
            RED => 107,
            YELLOW => 20,
            PURPLE => 121,
            GREY => 71,
            WHITE => 76,
            BLACK => 211,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 106,
            GREEN => 141,
            RED => 57,
            YELLOW => 191,
            PURPLE => 230,
            GREY => 166,
            WHITE => 34,
            BLACK => 22,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 202,
            GREEN => 7,
            RED => 148,
            YELLOW => 187,
            PURPLE => 169,
            GREY => 147,
            WHITE => 124,
            BLACK => 151,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 208,
            GREEN => 106,
            RED => 60,
            YELLOW => 40,
            PURPLE => 87,
            GREY => 204,
            WHITE => 175,
            BLACK => 169,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 55,
            GREEN => 100,
            RED => 217,
            YELLOW => 3,
            PURPLE => 50,
            GREY => 144,
            WHITE => 226,
            BLACK => 124,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 237,
            GREEN => 123,
            RED => 92,
            YELLOW => 222,
            PURPLE => 72,
            GREY => 233,
            WHITE => 225,
            BLACK => 244,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 158,
            GREEN => 72,
            RED => 93,
            YELLOW => 139,
            PURPLE => 62,
            GREY => 138,
            WHITE => 34,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 39,
            GREEN => 211,
            RED => 12,
            YELLOW => 235,
            PURPLE => 132,
            GREY => 83,
            WHITE => 167,
            BLACK => 173,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 233,
            GREEN => 181,
            RED => 31,
            YELLOW => 88,
            PURPLE => 219,
            GREY => 243,
            WHITE => 100,
            BLACK => 89,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 91,
            GREEN => 233,
            RED => 218,
            YELLOW => 114,
            PURPLE => 184,
            GREY => 78,
            WHITE => 165,
            BLACK => 164,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 121,
            GREEN => 186,
            RED => 190,
            YELLOW => 189,
            PURPLE => 2,
            GREY => 57,
            WHITE => 31,
            BLACK => 198,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 22,
            GREEN => 62,
            RED => 67,
            YELLOW => 10,
            PURPLE => 181,
            GREY => 33,
            WHITE => 146,
            BLACK => 163,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 90,
            GREEN => 40,
            RED => 248,
            YELLOW => 19,
            PURPLE => 57,
            GREY => 144,
            WHITE => 211,
            BLACK => 153,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 15,
            GREEN => 133,
            RED => 20,
            YELLOW => 214,
            PURPLE => 20,
            GREY => 6,
            WHITE => 253,
            BLACK => 11,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 110,
            GREEN => 99,
            RED => 109,
            YELLOW => 174,
            PURPLE => 52,
            GREY => 35,
            WHITE => 198,
            BLACK => 27,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 133,
            GREEN => 144,
            RED => 175,
            YELLOW => 155,
            PURPLE => 18,
            GREY => 112,
            WHITE => 2,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 247,
            GREEN => 224,
            RED => 220,
            YELLOW => 11,
            PURPLE => 197,
            GREY => 79,
            WHITE => 107,
            BLACK => 216,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 46,
            GREEN => 38,
            RED => 218,
            YELLOW => 203,
            PURPLE => 119,
            GREY => 110,
            WHITE => 13,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 84,
            GREEN => 170,
            RED => 75,
            YELLOW => 4,
            PURPLE => 117,
            GREY => 160,
            WHITE => 18,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 108,
            GREEN => 231,
            RED => 197,
            YELLOW => 18,
            PURPLE => 222,
            GREY => 61,
            WHITE => 139,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 104,
            GREEN => 240,
            RED => 2,
            YELLOW => 14,
            PURPLE => 122,
            GREY => 128,
            WHITE => 174,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 226,
            GREEN => 84,
            RED => 153,
            YELLOW => 248,
            PURPLE => 19,
            GREY => 199,
            WHITE => 97,
            BLACK => 246,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 193,
            GREEN => 170,
            RED => 245,
            YELLOW => 81,
            PURPLE => 111,
            GREY => 157,
            WHITE => 84,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 237,
            GREEN => 253,
            RED => 48,
            YELLOW => 184,
            PURPLE => 231,
            GREY => 52,
            WHITE => 50,
            BLACK => 200,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 66,
            GREEN => 94,
            RED => 140,
            YELLOW => 134,
            PURPLE => 14,
            GREY => 110,
            WHITE => 186,
            BLACK => 227,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 24,
            GREEN => 84,
            RED => 239,
            YELLOW => 159,
            PURPLE => 198,
            GREY => 84,
            WHITE => 177,
            BLACK => 100,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 123,
            GREEN => 51,
            RED => 95,
            YELLOW => 81,
            PURPLE => 123,
            GREY => 182,
            WHITE => 0,
            BLACK => 6,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 3,
            GREEN => 74,
            RED => 57,
            YELLOW => 134,
            PURPLE => 160,
            GREY => 228,
            WHITE => 39,
            BLACK => 199,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 3,
            GREEN => 107,
            RED => 226,
            YELLOW => 62,
            PURPLE => 149,
            GREY => 210,
            WHITE => 7,
            BLACK => 129,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 210,
            GREEN => 58,
            RED => 39,
            YELLOW => 84,
            PURPLE => 60,
            GREY => 154,
            WHITE => 213,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 174,
            GREEN => 8,
            RED => 98,
            YELLOW => 238,
            PURPLE => 139,
            GREY => 154,
            WHITE => 174,
            BLACK => 217,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 58,
            GREEN => 45,
            RED => 248,
            YELLOW => 7,
            PURPLE => 199,
            GREY => 54,
            WHITE => 232,
            BLACK => 173,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 244,
            GREEN => 193,
            RED => 110,
            YELLOW => 138,
            PURPLE => 189,
            GREY => 70,
            WHITE => 225,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 0,
            GREEN => 130,
            RED => 202,
            YELLOW => 128,
            PURPLE => 67,
            GREY => 250,
            WHITE => 20,
            BLACK => 153,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 173,
            GREEN => 47,
            RED => 233,
            YELLOW => 121,
            PURPLE => 227,
            GREY => 200,
            WHITE => 131,
            BLACK => 135,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 75,
            GREEN => 139,
            RED => 89,
            YELLOW => 191,
            PURPLE => 214,
            GREY => 165,
            WHITE => 107,
            BLACK => 81,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 10,
            GREEN => 191,
            RED => 166,
            YELLOW => 118,
            PURPLE => 70,
            GREY => 9,
            WHITE => 247,
            BLACK => 59,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 183,
            GREEN => 243,
            RED => 115,
            YELLOW => 248,
            PURPLE => 168,
            GREY => 193,
            WHITE => 179,
            BLACK => 129,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 33,
            GREEN => 23,
            RED => 189,
            YELLOW => 38,
            PURPLE => 107,
            GREY => 177,
            WHITE => 145,
            BLACK => 116,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 203,
            GREEN => 3,
            RED => 225,
            YELLOW => 27,
            PURPLE => 143,
            GREY => 50,
            WHITE => 9,
            BLACK => 15,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 198,
            GREEN => 215,
            RED => 208,
            YELLOW => 87,
            PURPLE => 59,
            GREY => 172,
            WHITE => 146,
            BLACK => 108,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 67,
            GREEN => 103,
            RED => 156,
            YELLOW => 220,
            PURPLE => 233,
            GREY => 65,
            WHITE => 44,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 136,
            GREEN => 60,
            RED => 27,
            YELLOW => 91,
            PURPLE => 125,
            GREY => 101,
            WHITE => 114,
            BLACK => 65,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 38,
            GREEN => 151,
            RED => 24,
            YELLOW => 249,
            PURPLE => 147,
            GREY => 239,
            WHITE => 64,
            BLACK => 19,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 139,
            GREEN => 49,
            RED => 227,
            YELLOW => 74,
            PURPLE => 242,
            GREY => 99,
            WHITE => 238,
            BLACK => 73,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 212,
            GREEN => 96,
            RED => 197,
            YELLOW => 216,
            PURPLE => 183,
            GREY => 168,
            WHITE => 169,
            BLACK => 98,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 3,
            GREEN => 46,
            RED => 224,
            YELLOW => 111,
            PURPLE => 129,
            GREY => 62,
            WHITE => 52,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 76,
            GREEN => 96,
            RED => 33,
            YELLOW => 39,
            PURPLE => 238,
            GREY => 38,
            WHITE => 112,
            BLACK => 83,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 151,
            GREEN => 180,
            RED => 188,
            YELLOW => 129,
            PURPLE => 27,
            GREY => 234,
            WHITE => 227,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 42,
            GREEN => 167,
            RED => 219,
            YELLOW => 211,
            PURPLE => 25,
            GREY => 230,
            WHITE => 65,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 215,
            GREEN => 43,
            RED => 72,
            YELLOW => 78,
            PURPLE => 56,
            GREY => 151,
            WHITE => 30,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 243,
            GREEN => 78,
            RED => 154,
            YELLOW => 119,
            PURPLE => 208,
            GREY => 23,
            WHITE => 205,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 104,
            GREEN => 207,
            RED => 21,
            YELLOW => 63,
            PURPLE => 248,
            GREY => 232,
            WHITE => 39,
            BLACK => 252,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 68,
            GREEN => 111,
            RED => 249,
            YELLOW => 150,
            PURPLE => 60,
            GREY => 48,
            WHITE => 214,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 92,
            GREEN => 181,
            RED => 37,
            YELLOW => 224,
            PURPLE => 157,
            GREY => 21,
            WHITE => 43,
            BLACK => 89,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 207,
            GREEN => 255,
            RED => 14,
            YELLOW => 189,
            PURPLE => 152,
            GREY => 147,
            WHITE => 158,
            BLACK => 99,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 239,
            GREEN => 43,
            RED => 136,
            YELLOW => 104,
            PURPLE => 101,
            GREY => 68,
            WHITE => 86,
            BLACK => 215,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 217,
            GREEN => 119,
            RED => 181,
            YELLOW => 83,
            PURPLE => 102,
            GREY => 192,
            WHITE => 126,
            BLACK => 59,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 132,
            GREEN => 254,
            RED => 196,
            YELLOW => 41,
            PURPLE => 56,
            GREY => 80,
            WHITE => 115,
            BLACK => 2,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if nor(nor(nor(nor(false, false), nor(match buffer[11] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }, false)), nor(nor(false, nor(match buffer[7] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[1] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, })), nor(match buffer[5] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[4] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }))), nor(nor(nor(false, false), nor(false, match buffer[2] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, })), nor(nor(nor(match buffer[3] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[10] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), match buffer[9] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(match buffer[8] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[0] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })))) {
	let n = match buffer[17] {
            BLUE => 9,
            GREEN => 237,
            RED => 82,
            YELLOW => 33,
            PURPLE => 198,
            GREY => 132,
            WHITE => 250,
            BLACK => 146,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 167,
            GREEN => 204,
            RED => 36,
            YELLOW => 178,
            PURPLE => 61,
            GREY => 36,
            WHITE => 216,
            BLACK => 204,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 23,
            GREEN => 54,
            RED => 1,
            YELLOW => 170,
            PURPLE => 160,
            GREY => 152,
            WHITE => 133,
            BLACK => 225,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 75,
            GREEN => 139,
            RED => 45,
            YELLOW => 32,
            PURPLE => 61,
            GREY => 101,
            WHITE => 52,
            BLACK => 143,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 1,
            GREEN => 101,
            RED => 40,
            YELLOW => 65,
            PURPLE => 230,
            GREY => 165,
            WHITE => 14,
            BLACK => 147,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 47,
            GREEN => 99,
            RED => 232,
            YELLOW => 147,
            PURPLE => 153,
            GREY => 38,
            WHITE => 59,
            BLACK => 222,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 24,
            GREEN => 76,
            RED => 159,
            YELLOW => 218,
            PURPLE => 89,
            GREY => 50,
            WHITE => 8,
            BLACK => 92,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 154,
            GREEN => 4,
            RED => 29,
            YELLOW => 176,
            PURPLE => 166,
            GREY => 147,
            WHITE => 205,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 61,
            GREEN => 110,
            RED => 88,
            YELLOW => 76,
            PURPLE => 198,
            GREY => 196,
            WHITE => 87,
            BLACK => 104,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 118,
            GREEN => 242,
            RED => 251,
            YELLOW => 231,
            PURPLE => 42,
            GREY => 207,
            WHITE => 4,
            BLACK => 136,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 252,
            GREEN => 53,
            RED => 80,
            YELLOW => 1,
            PURPLE => 216,
            GREY => 113,
            WHITE => 86,
            BLACK => 84,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 75,
            GREEN => 27,
            RED => 228,
            YELLOW => 26,
            PURPLE => 97,
            GREY => 29,
            WHITE => 249,
            BLACK => 95,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 203,
            GREEN => 45,
            RED => 31,
            YELLOW => 209,
            PURPLE => 10,
            GREY => 243,
            WHITE => 162,
            BLACK => 104,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 54,
            GREEN => 239,
            RED => 165,
            YELLOW => 146,
            PURPLE => 215,
            GREY => 63,
            WHITE => 3,
            BLACK => 93,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 140,
            GREEN => 73,
            RED => 40,
            YELLOW => 50,
            PURPLE => 222,
            GREY => 122,
            WHITE => 198,
            BLACK => 122,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 163,
            GREEN => 253,
            RED => 85,
            YELLOW => 56,
            PURPLE => 107,
            GREY => 56,
            WHITE => 30,
            BLACK => 12,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 181,
            GREEN => 157,
            RED => 44,
            YELLOW => 190,
            PURPLE => 26,
            GREY => 99,
            WHITE => 163,
            BLACK => 22,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 20,
            GREEN => 82,
            RED => 166,
            YELLOW => 132,
            PURPLE => 10,
            GREY => 228,
            WHITE => 88,
            BLACK => 116,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 216,
            GREEN => 174,
            RED => 61,
            YELLOW => 121,
            PURPLE => 52,
            GREY => 94,
            WHITE => 18,
            BLACK => 39,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 15,
            GREEN => 100,
            RED => 103,
            YELLOW => 184,
            PURPLE => 112,
            GREY => 22,
            WHITE => 207,
            BLACK => 19,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 103,
            GREEN => 225,
            RED => 132,
            YELLOW => 190,
            PURPLE => 100,
            GREY => 195,
            WHITE => 120,
            BLACK => 209,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 156,
            GREEN => 156,
            RED => 254,
            YELLOW => 42,
            PURPLE => 15,
            GREY => 24,
            WHITE => 223,
            BLACK => 31,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 3,
            GREEN => 146,
            RED => 47,
            YELLOW => 173,
            PURPLE => 66,
            GREY => 186,
            WHITE => 21,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 46,
            GREEN => 217,
            RED => 204,
            YELLOW => 166,
            PURPLE => 82,
            GREY => 251,
            WHITE => 36,
            BLACK => 159,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 212,
            GREEN => 83,
            RED => 80,
            YELLOW => 126,
            PURPLE => 229,
            GREY => 115,
            WHITE => 146,
            BLACK => 158,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 50,
            GREEN => 138,
            RED => 155,
            YELLOW => 134,
            PURPLE => 223,
            GREY => 132,
            WHITE => 191,
            BLACK => 14,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 213,
            GREEN => 185,
            RED => 137,
            YELLOW => 130,
            PURPLE => 97,
            GREY => 168,
            WHITE => 65,
            BLACK => 239,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 151,
            GREEN => 186,
            RED => 76,
            YELLOW => 208,
            PURPLE => 24,
            GREY => 60,
            WHITE => 223,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 177,
            GREEN => 69,
            RED => 190,
            YELLOW => 41,
            PURPLE => 81,
            GREY => 246,
            WHITE => 143,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 167,
            GREEN => 135,
            RED => 184,
            YELLOW => 189,
            PURPLE => 93,
            GREY => 219,
            WHITE => 53,
            BLACK => 30,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 226,
            GREEN => 149,
            RED => 71,
            YELLOW => 174,
            PURPLE => 37,
            GREY => 213,
            WHITE => 90,
            BLACK => 63,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 143,
            GREEN => 250,
            RED => 40,
            YELLOW => 34,
            PURPLE => 194,
            GREY => 6,
            WHITE => 103,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 1,
            GREEN => 147,
            RED => 213,
            YELLOW => 11,
            PURPLE => 235,
            GREY => 211,
            WHITE => 178,
            BLACK => 209,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 78,
            GREEN => 61,
            RED => 86,
            YELLOW => 181,
            PURPLE => 229,
            GREY => 236,
            WHITE => 124,
            BLACK => 9,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 218,
            GREEN => 3,
            RED => 246,
            YELLOW => 13,
            PURPLE => 218,
            GREY => 237,
            WHITE => 69,
            BLACK => 178,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 204,
            GREEN => 119,
            RED => 46,
            YELLOW => 206,
            PURPLE => 0,
            GREY => 81,
            WHITE => 12,
            BLACK => 155,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 31,
            GREEN => 160,
            RED => 214,
            YELLOW => 182,
            PURPLE => 132,
            GREY => 243,
            WHITE => 4,
            BLACK => 184,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 62,
            GREEN => 159,
            RED => 32,
            YELLOW => 101,
            PURPLE => 28,
            GREY => 201,
            WHITE => 235,
            BLACK => 219,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 219,
            GREEN => 241,
            RED => 152,
            YELLOW => 106,
            PURPLE => 95,
            GREY => 124,
            WHITE => 35,
            BLACK => 183,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 25,
            GREEN => 233,
            RED => 141,
            YELLOW => 92,
            PURPLE => 24,
            GREY => 247,
            WHITE => 75,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 151,
            GREEN => 213,
            RED => 193,
            YELLOW => 246,
            PURPLE => 24,
            GREY => 89,
            WHITE => 144,
            BLACK => 230,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 227,
            GREEN => 190,
            RED => 11,
            YELLOW => 144,
            PURPLE => 60,
            GREY => 90,
            WHITE => 35,
            BLACK => 248,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 163,
            GREEN => 184,
            RED => 44,
            YELLOW => 141,
            PURPLE => 172,
            GREY => 60,
            WHITE => 140,
            BLACK => 190,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[65] {
            BLUE => 17,
            GREEN => 234,
            RED => 255,
            YELLOW => 213,
            PURPLE => 93,
            GREY => 233,
            WHITE => 215,
            BLACK => 192,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 199,
            GREEN => 152,
            RED => 50,
            YELLOW => 157,
            PURPLE => 222,
            GREY => 76,
            WHITE => 34,
            BLACK => 92,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 53,
            GREEN => 89,
            RED => 42,
            YELLOW => 122,
            PURPLE => 82,
            GREY => 164,
            WHITE => 199,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 85,
            GREEN => 92,
            RED => 94,
            YELLOW => 242,
            PURPLE => 90,
            GREY => 25,
            WHITE => 220,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[56] {
            BLUE => 130,
            GREEN => 65,
            RED => 241,
            YELLOW => 226,
            PURPLE => 36,
            GREY => 87,
            WHITE => 19,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 219,
            GREEN => 82,
            RED => 4,
            YELLOW => 211,
            PURPLE => 251,
            GREY => 164,
            WHITE => 57,
            BLACK => 30,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 60,
            GREEN => 128,
            RED => 135,
            YELLOW => 162,
            PURPLE => 169,
            GREY => 116,
            WHITE => 35,
            BLACK => 60,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 14,
            GREEN => 198,
            RED => 25,
            YELLOW => 17,
            PURPLE => 234,
            GREY => 82,
            WHITE => 63,
            BLACK => 81,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 186,
            GREEN => 85,
            RED => 232,
            YELLOW => 114,
            PURPLE => 11,
            GREY => 48,
            WHITE => 90,
            BLACK => 169,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 138,
            GREEN => 17,
            RED => 221,
            YELLOW => 10,
            PURPLE => 100,
            GREY => 7,
            WHITE => 73,
            BLACK => 132,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 195,
            GREEN => 157,
            RED => 64,
            YELLOW => 187,
            PURPLE => 93,
            GREY => 199,
            WHITE => 187,
            BLACK => 36,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 94,
            GREEN => 81,
            RED => 213,
            YELLOW => 55,
            PURPLE => 94,
            GREY => 4,
            WHITE => 63,
            BLACK => 130,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 35,
            GREEN => 220,
            RED => 201,
            YELLOW => 45,
            PURPLE => 231,
            GREY => 159,
            WHITE => 151,
            BLACK => 55,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 202,
            GREEN => 5,
            RED => 112,
            YELLOW => 155,
            PURPLE => 74,
            GREY => 94,
            WHITE => 132,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 48,
            GREEN => 197,
            RED => 147,
            YELLOW => 190,
            PURPLE => 44,
            GREY => 140,
            WHITE => 251,
            BLACK => 31,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 59,
            GREEN => 67,
            RED => 167,
            YELLOW => 23,
            PURPLE => 70,
            GREY => 141,
            WHITE => 239,
            BLACK => 28,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 170,
            GREEN => 253,
            RED => 119,
            YELLOW => 241,
            PURPLE => 138,
            GREY => 142,
            WHITE => 250,
            BLACK => 41,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 26,
            GREEN => 104,
            RED => 142,
            YELLOW => 126,
            PURPLE => 169,
            GREY => 119,
            WHITE => 156,
            BLACK => 215,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 45,
            GREEN => 239,
            RED => 156,
            YELLOW => 130,
            PURPLE => 129,
            GREY => 9,
            WHITE => 12,
            BLACK => 178,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 217,
            GREEN => 102,
            RED => 79,
            YELLOW => 201,
            PURPLE => 180,
            GREY => 143,
            WHITE => 123,
            BLACK => 46,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 237,
            GREEN => 252,
            RED => 167,
            YELLOW => 35,
            PURPLE => 39,
            GREY => 238,
            WHITE => 213,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 157,
            GREEN => 35,
            RED => 232,
            YELLOW => 148,
            PURPLE => 20,
            GREY => 253,
            WHITE => 174,
            BLACK => 122,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 31,
            GREEN => 101,
            RED => 2,
            YELLOW => 128,
            PURPLE => 163,
            GREY => 159,
            WHITE => 250,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 206,
            GREEN => 151,
            RED => 171,
            YELLOW => 188,
            PURPLE => 122,
            GREY => 173,
            WHITE => 202,
            BLACK => 45,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 12,
            GREEN => 17,
            RED => 210,
            YELLOW => 133,
            PURPLE => 208,
            GREY => 226,
            WHITE => 182,
            BLACK => 235,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else {
	let n = match buffer[65] {
            BLUE => 61,
            GREEN => 41,
            RED => 94,
            YELLOW => 124,
            PURPLE => 85,
            GREY => 41,
            WHITE => 137,
            BLACK => 111,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 37,
            GREEN => 3,
            RED => 67,
            YELLOW => 185,
            PURPLE => 252,
            GREY => 110,
            WHITE => 32,
            BLACK => 71,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 237,
            GREEN => 19,
            RED => 93,
            YELLOW => 118,
            PURPLE => 70,
            GREY => 142,
            WHITE => 112,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 85,
            GREEN => 228,
            RED => 27,
            YELLOW => 10,
            PURPLE => 81,
            GREY => 0,
            WHITE => 90,
            BLACK => 253,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[92] {
            BLUE => 236,
            GREEN => 243,
            RED => 204,
            YELLOW => 222,
            PURPLE => 193,
            GREY => 56,
            WHITE => 117,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 193,
            GREEN => 16,
            RED => 91,
            YELLOW => 120,
            PURPLE => 185,
            GREY => 160,
            WHITE => 179,
            BLACK => 161,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[87] {
            BLUE => 80,
            GREEN => 178,
            RED => 24,
            YELLOW => 42,
            PURPLE => 38,
            GREY => 107,
            WHITE => 164,
            BLACK => 155,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[69] {
            BLUE => 161,
            GREEN => 160,
            RED => 4,
            YELLOW => 144,
            PURPLE => 252,
            GREY => 201,
            WHITE => 150,
            BLACK => 247,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 134,
            GREEN => 137,
            RED => 176,
            YELLOW => 138,
            PURPLE => 78,
            GREY => 102,
            WHITE => 39,
            BLACK => 186,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 123,
            GREEN => 230,
            RED => 113,
            YELLOW => 161,
            PURPLE => 70,
            GREY => 35,
            WHITE => 62,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 128,
            GREEN => 247,
            RED => 161,
            YELLOW => 145,
            PURPLE => 48,
            GREY => 125,
            WHITE => 73,
            BLACK => 112,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 19,
            GREEN => 143,
            RED => 172,
            YELLOW => 125,
            PURPLE => 144,
            GREY => 181,
            WHITE => 159,
            BLACK => 137,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 120,
            GREEN => 222,
            RED => 89,
            YELLOW => 250,
            PURPLE => 106,
            GREY => 129,
            WHITE => 30,
            BLACK => 20,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 141,
            GREEN => 241,
            RED => 113,
            YELLOW => 66,
            PURPLE => 220,
            GREY => 66,
            WHITE => 19,
            BLACK => 48,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[70] {
            BLUE => 247,
            GREEN => 234,
            RED => 146,
            YELLOW => 187,
            PURPLE => 114,
            GREY => 214,
            WHITE => 0,
            BLACK => 182,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 110,
            GREEN => 26,
            RED => 227,
            YELLOW => 9,
            PURPLE => 99,
            GREY => 202,
            WHITE => 217,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 239,
            GREEN => 7,
            RED => 111,
            YELLOW => 47,
            PURPLE => 142,
            GREY => 237,
            WHITE => 252,
            BLACK => 144,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 3,
            GREEN => 205,
            RED => 36,
            YELLOW => 53,
            PURPLE => 85,
            GREY => 189,
            WHITE => 22,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 19,
            GREEN => 161,
            RED => 43,
            YELLOW => 216,
            PURPLE => 191,
            GREY => 173,
            WHITE => 207,
            BLACK => 43,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 170,
            GREEN => 72,
            RED => 2,
            YELLOW => 120,
            PURPLE => 139,
            GREY => 150,
            WHITE => 136,
            BLACK => 173,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 9,
            GREEN => 170,
            RED => 231,
            YELLOW => 176,
            PURPLE => 115,
            GREY => 72,
            WHITE => 152,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 165,
            GREEN => 139,
            RED => 122,
            YELLOW => 45,
            PURPLE => 199,
            GREY => 194,
            WHITE => 75,
            BLACK => 178,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 56,
            GREEN => 32,
            RED => 155,
            YELLOW => 176,
            PURPLE => 212,
            GREY => 234,
            WHITE => 125,
            BLACK => 105,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[108] {
            BLUE => 180,
            GREEN => 227,
            RED => 34,
            YELLOW => 184,
            PURPLE => 72,
            GREY => 21,
            WHITE => 72,
            BLACK => 98,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[82] {
            BLUE => 4,
            GREEN => 119,
            RED => 245,
            YELLOW => 162,
            PURPLE => 20,
            GREY => 232,
            WHITE => 11,
            BLACK => 113,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[75] {
            BLUE => 56,
            GREEN => 220,
            RED => 81,
            YELLOW => 84,
            PURPLE => 7,
            GREY => 8,
            WHITE => 250,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[74] {
            BLUE => 140,
            GREEN => 32,
            RED => 24,
            YELLOW => 227,
            PURPLE => 170,
            GREY => 4,
            WHITE => 170,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 185,
            GREEN => 8,
            RED => 79,
            YELLOW => 175,
            PURPLE => 135,
            GREY => 3,
            WHITE => 223,
            BLACK => 246,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 9,
            GREEN => 201,
            RED => 221,
            YELLOW => 55,
            PURPLE => 178,
            GREY => 252,
            WHITE => 107,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 149,
            GREEN => 134,
            RED => 50,
            YELLOW => 177,
            PURPLE => 16,
            GREY => 19,
            WHITE => 6,
            BLACK => 129,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 170,
            GREEN => 10,
            RED => 121,
            YELLOW => 34,
            PURPLE => 61,
            GREY => 18,
            WHITE => 70,
            BLACK => 117,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[110] {
            BLUE => 222,
            GREEN => 196,
            RED => 179,
            YELLOW => 234,
            PURPLE => 67,
            GREY => 207,
            WHITE => 156,
            BLACK => 219,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 130,
            GREEN => 254,
            RED => 126,
            YELLOW => 180,
            PURPLE => 41,
            GREY => 2,
            WHITE => 42,
            BLACK => 55,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[104] {
            BLUE => 3,
            GREEN => 81,
            RED => 46,
            YELLOW => 98,
            PURPLE => 226,
            GREY => 1,
            WHITE => 241,
            BLACK => 92,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 82,
            GREEN => 133,
            RED => 123,
            YELLOW => 237,
            PURPLE => 209,
            GREY => 121,
            WHITE => 137,
            BLACK => 110,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 73,
            GREEN => 130,
            RED => 28,
            YELLOW => 207,
            PURPLE => 226,
            GREY => 102,
            WHITE => 178,
            BLACK => 239,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 26,
            GREEN => 203,
            RED => 66,
            YELLOW => 162,
            PURPLE => 35,
            GREY => 24,
            WHITE => 185,
            BLACK => 222,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[56] {
            BLUE => 133,
            GREEN => 145,
            RED => 227,
            YELLOW => 72,
            PURPLE => 23,
            GREY => 229,
            WHITE => 97,
            BLACK => 160,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 101,
            GREEN => 41,
            RED => 231,
            YELLOW => 220,
            PURPLE => 143,
            GREY => 200,
            WHITE => 44,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[88] {
            BLUE => 65,
            GREEN => 219,
            RED => 68,
            YELLOW => 103,
            PURPLE => 22,
            GREY => 167,
            WHITE => 234,
            BLACK => 115,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 233,
            GREEN => 251,
            RED => 120,
            YELLOW => 161,
            PURPLE => 150,
            GREY => 225,
            WHITE => 219,
            BLACK => 106,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[85] {
            BLUE => 241,
            GREEN => 138,
            RED => 153,
            YELLOW => 37,
            PURPLE => 215,
            GREY => 125,
            WHITE => 51,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[100] {
            BLUE => 75,
            GREEN => 225,
            RED => 6,
            YELLOW => 34,
            PURPLE => 153,
            GREY => 42,
            WHITE => 171,
            BLACK => 133,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 168,
            GREEN => 9,
            RED => 116,
            YELLOW => 151,
            PURPLE => 57,
            GREY => 204,
            WHITE => 98,
            BLACK => 104,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[109] {
            BLUE => 73,
            GREEN => 3,
            RED => 106,
            YELLOW => 12,
            PURPLE => 239,
            GREY => 233,
            WHITE => 158,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 66,
            GREEN => 114,
            RED => 80,
            YELLOW => 93,
            PURPLE => 144,
            GREY => 4,
            WHITE => 115,
            BLACK => 46,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[80] {
            BLUE => 83,
            GREEN => 74,
            RED => 42,
            YELLOW => 234,
            PURPLE => 89,
            GREY => 105,
            WHITE => 206,
            BLACK => 114,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 184,
            GREEN => 36,
            RED => 222,
            YELLOW => 119,
            PURPLE => 73,
            GREY => 232,
            WHITE => 234,
            BLACK => 21,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[93] {
            BLUE => 20,
            GREEN => 27,
            RED => 35,
            YELLOW => 90,
            PURPLE => 243,
            GREY => 59,
            WHITE => 98,
            BLACK => 159,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 33,
            GREEN => 140,
            RED => 237,
            YELLOW => 244,
            PURPLE => 136,
            GREY => 73,
            WHITE => 137,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 155,
            GREEN => 131,
            RED => 98,
            YELLOW => 215,
            PURPLE => 190,
            GREY => 55,
            WHITE => 176,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[86] {
            BLUE => 81,
            GREEN => 204,
            RED => 172,
            YELLOW => 74,
            PURPLE => 129,
            GREY => 16,
            WHITE => 193,
            BLACK => 122,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[102] {
            BLUE => 132,
            GREEN => 94,
            RED => 76,
            YELLOW => 74,
            PURPLE => 231,
            GREY => 92,
            WHITE => 129,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[95] {
            BLUE => 128,
            GREEN => 165,
            RED => 227,
            YELLOW => 26,
            PURPLE => 138,
            GREY => 112,
            WHITE => 125,
            BLACK => 150,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 161,
            GREEN => 48,
            RED => 134,
            YELLOW => 188,
            PURPLE => 68,
            GREY => 167,
            WHITE => 9,
            BLACK => 129,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 164,
            GREEN => 189,
            RED => 207,
            YELLOW => 35,
            PURPLE => 237,
            GREY => 61,
            WHITE => 156,
            BLACK => 118,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[77] {
            BLUE => 193,
            GREEN => 124,
            RED => 144,
            YELLOW => 173,
            PURPLE => 202,
            GREY => 13,
            WHITE => 14,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 197,
            GREEN => 207,
            RED => 80,
            YELLOW => 125,
            PURPLE => 253,
            GREY => 164,
            WHITE => 253,
            BLACK => 39,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 42,
            GREEN => 22,
            RED => 89,
            YELLOW => 55,
            PURPLE => 178,
            GREY => 45,
            WHITE => 214,
            BLACK => 220,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[72] {
            BLUE => 148,
            GREEN => 55,
            RED => 239,
            YELLOW => 128,
            PURPLE => 148,
            GREY => 243,
            WHITE => 165,
            BLACK => 142,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 33,
            GREEN => 46,
            RED => 175,
            YELLOW => 243,
            PURPLE => 77,
            GREY => 237,
            WHITE => 60,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[90] {
            BLUE => 173,
            GREEN => 88,
            RED => 108,
            YELLOW => 189,
            PURPLE => 86,
            GREY => 111,
            WHITE => 43,
            BLACK => 12,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[73] {
            BLUE => 220,
            GREEN => 149,
            RED => 7,
            YELLOW => 74,
            PURPLE => 113,
            GREY => 188,
            WHITE => 65,
            BLACK => 110,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 93,
            GREEN => 233,
            RED => 104,
            YELLOW => 182,
            PURPLE => 196,
            GREY => 246,
            WHITE => 120,
            BLACK => 78,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 99,
            GREEN => 60,
            RED => 88,
            YELLOW => 217,
            PURPLE => 51,
            GREY => 28,
            WHITE => 55,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 123,
            GREEN => 42,
            RED => 102,
            YELLOW => 11,
            PURPLE => 59,
            GREY => 121,
            WHITE => 104,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[96] {
            BLUE => 241,
            GREEN => 244,
            RED => 8,
            YELLOW => 80,
            PURPLE => 39,
            GREY => 200,
            WHITE => 167,
            BLACK => 80,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 48,
            GREEN => 8,
            RED => 134,
            YELLOW => 19,
            PURPLE => 64,
            GREY => 43,
            WHITE => 184,
            BLACK => 217,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[105] {
            BLUE => 38,
            GREEN => 34,
            RED => 211,
            YELLOW => 189,
            PURPLE => 205,
            GREY => 225,
            WHITE => 6,
            BLACK => 181,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 163,
            GREEN => 82,
            RED => 233,
            YELLOW => 45,
            PURPLE => 51,
            GREY => 114,
            WHITE => 255,
            BLACK => 178,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 234,
            GREEN => 248,
            RED => 128,
            YELLOW => 159,
            PURPLE => 41,
            GREY => 26,
            WHITE => 85,
            BLACK => 49,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[79] {
            BLUE => 36,
            GREEN => 134,
            RED => 184,
            YELLOW => 220,
            PURPLE => 112,
            GREY => 206,
            WHITE => 87,
            BLACK => 163,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 254,
            GREEN => 119,
            RED => 102,
            YELLOW => 49,
            PURPLE => 154,
            GREY => 191,
            WHITE => 111,
            BLACK => 110,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 229,
            GREEN => 204,
            RED => 66,
            YELLOW => 51,
            PURPLE => 125,
            GREY => 197,
            WHITE => 53,
            BLACK => 167,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[83] {
            BLUE => 212,
            GREEN => 161,
            RED => 139,
            YELLOW => 7,
            PURPLE => 127,
            GREY => 228,
            WHITE => 128,
            BLACK => 10,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[111] {
            BLUE => 11,
            GREEN => 157,
            RED => 25,
            YELLOW => 213,
            PURPLE => 132,
            GREY => 155,
            WHITE => 61,
            BLACK => 233,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[103] {
            BLUE => 10,
            GREEN => 43,
            RED => 174,
            YELLOW => 29,
            PURPLE => 131,
            GREY => 236,
            WHITE => 95,
            BLACK => 129,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[101] {
            BLUE => 22,
            GREEN => 214,
            RED => 253,
            YELLOW => 222,
            PURPLE => 233,
            GREY => 161,
            WHITE => 225,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 192,
            GREEN => 193,
            RED => 149,
            YELLOW => 121,
            PURPLE => 8,
            GREY => 149,
            WHITE => 29,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 70,
            GREEN => 238,
            RED => 236,
            YELLOW => 203,
            PURPLE => 11,
            GREY => 41,
            WHITE => 7,
            BLACK => 242,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 14,
            GREEN => 234,
            RED => 201,
            YELLOW => 189,
            PURPLE => 114,
            GREY => 13,
            WHITE => 82,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 144,
            GREEN => 36,
            RED => 48,
            YELLOW => 232,
            PURPLE => 179,
            GREY => 93,
            WHITE => 122,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 214,
            GREEN => 116,
            RED => 232,
            YELLOW => 217,
            PURPLE => 244,
            GREY => 102,
            WHITE => 112,
            BLACK => 215,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[99] {
            BLUE => 25,
            GREEN => 220,
            RED => 175,
            YELLOW => 155,
            PURPLE => 180,
            GREY => 204,
            WHITE => 171,
            BLACK => 88,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[106] {
            BLUE => 180,
            GREEN => 105,
            RED => 131,
            YELLOW => 168,
            PURPLE => 240,
            GREY => 234,
            WHITE => 213,
            BLACK => 113,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 92,
            GREEN => 173,
            RED => 201,
            YELLOW => 237,
            PURPLE => 236,
            GREY => 92,
            WHITE => 230,
            BLACK => 74,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 71,
            GREEN => 161,
            RED => 28,
            YELLOW => 209,
            PURPLE => 251,
            GREY => 136,
            WHITE => 136,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[78] {
            BLUE => 73,
            GREEN => 254,
            RED => 122,
            YELLOW => 129,
            PURPLE => 158,
            GREY => 8,
            WHITE => 206,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 189,
            GREEN => 25,
            RED => 118,
            YELLOW => 44,
            PURPLE => 40,
            GREY => 163,
            WHITE => 107,
            BLACK => 161,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[94] {
            BLUE => 239,
            GREEN => 30,
            RED => 251,
            YELLOW => 226,
            PURPLE => 116,
            GREY => 212,
            WHITE => 50,
            BLACK => 211,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[98] {
            BLUE => 63,
            GREEN => 146,
            RED => 189,
            YELLOW => 63,
            PURPLE => 181,
            GREY => 218,
            WHITE => 72,
            BLACK => 93,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 173,
            GREEN => 49,
            RED => 28,
            YELLOW => 62,
            PURPLE => 179,
            GREY => 127,
            WHITE => 126,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 245,
            GREEN => 213,
            RED => 103,
            YELLOW => 188,
            PURPLE => 34,
            GREY => 209,
            WHITE => 24,
            BLACK => 3,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 117,
            GREEN => 129,
            RED => 23,
            YELLOW => 144,
            PURPLE => 173,
            GREY => 118,
            WHITE => 24,
            BLACK => 170,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 216,
            GREEN => 21,
            RED => 7,
            YELLOW => 170,
            PURPLE => 178,
            GREY => 132,
            WHITE => 237,
            BLACK => 85,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[81] {
            BLUE => 253,
            GREEN => 175,
            RED => 88,
            YELLOW => 92,
            PURPLE => 183,
            GREY => 238,
            WHITE => 180,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[76] {
            BLUE => 212,
            GREEN => 128,
            RED => 87,
            YELLOW => 83,
            PURPLE => 93,
            GREY => 225,
            WHITE => 213,
            BLACK => 182,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 199,
            GREEN => 176,
            RED => 132,
            YELLOW => 9,
            PURPLE => 67,
            GREY => 44,
            WHITE => 174,
            BLACK => 170,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[107] {
            BLUE => 121,
            GREEN => 11,
            RED => 112,
            YELLOW => 216,
            PURPLE => 8,
            GREY => 228,
            WHITE => 237,
            BLACK => 35,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[71] {
            BLUE => 242,
            GREEN => 219,
            RED => 45,
            YELLOW => 224,
            PURPLE => 203,
            GREY => 220,
            WHITE => 128,
            BLACK => 42,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 107,
            GREEN => 45,
            RED => 243,
            YELLOW => 134,
            PURPLE => 35,
            GREY => 42,
            WHITE => 203,
            BLACK => 237,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 67,
            GREEN => 53,
            RED => 160,
            YELLOW => 173,
            PURPLE => 80,
            GREY => 173,
            WHITE => 142,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[84] {
            BLUE => 105,
            GREEN => 61,
            RED => 203,
            YELLOW => 91,
            PURPLE => 115,
            GREY => 234,
            WHITE => 242,
            BLACK => 144,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 21,
            GREEN => 79,
            RED => 113,
            YELLOW => 147,
            PURPLE => 153,
            GREY => 241,
            WHITE => 179,
            BLACK => 135,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 172,
            GREEN => 95,
            RED => 183,
            YELLOW => 168,
            PURPLE => 172,
            GREY => 128,
            WHITE => 63,
            BLACK => 167,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[97] {
            BLUE => 93,
            GREEN => 114,
            RED => 196,
            YELLOW => 26,
            PURPLE => 30,
            GREY => 131,
            WHITE => 23,
            BLACK => 134,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 84,
            GREEN => 10,
            RED => 213,
            YELLOW => 66,
            PURPLE => 213,
            GREY => 7,
            WHITE => 238,
            BLACK => 131,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[91] {
            BLUE => 22,
            GREEN => 73,
            RED => 143,
            YELLOW => 253,
            PURPLE => 91,
            GREY => 203,
            WHITE => 173,
            BLACK => 50,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 171,
            GREEN => 160,
            RED => 17,
            YELLOW => 205,
            PURPLE => 88,
            GREY => 152,
            WHITE => 33,
            BLACK => 249,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 240,
            GREEN => 12,
            RED => 57,
            YELLOW => 181,
            PURPLE => 206,
            GREY => 103,
            WHITE => 121,
            BLACK => 160,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 28,
            GREEN => 190,
            RED => 180,
            YELLOW => 15,
            PURPLE => 108,
            GREY => 128,
            WHITE => 110,
            BLACK => 251,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}

    }

    if input.pressed_this_frame(Button::A) {
        if nor(nor(nor(nor(nor(match buffer[11] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[17] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }), nor(match buffer[13] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[10] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(nor(match buffer[27] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[28] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(match buffer[20] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[4] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, })), nor(nor(match buffer[25] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[8] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }), false))), nor(nor(nor(nor(match buffer[19] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[23] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }), match buffer[14] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }), nor(false, match buffer[0] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })), nor(nor(match buffer[5] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[2] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }), nor(match buffer[22] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, nor(match buffer[7] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[21] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }))))), nor(nor(nor(nor(false, match buffer[1] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(match buffer[12] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[16] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(nor(match buffer[3] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }, match buffer[15] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => true, _ => false, }), match buffer[24] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), nor(false, match buffer[18] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }))), nor(nor(nor(false, false), nor(false, false)), nor(nor(false, false), nor(match buffer[6] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => true, _ => false, }, match buffer[26] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }))))) {
	let n = match buffer[90] {
            BLUE => 138,
            GREEN => 186,
            RED => 151,
            YELLOW => 112,
            PURPLE => 186,
            GREY => 252,
            WHITE => 97,
            BLACK => 139,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[107] {
            BLUE => 190,
            GREEN => 178,
            RED => 250,
            YELLOW => 39,
            PURPLE => 202,
            GREY => 23,
            WHITE => 210,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[56] {
            BLUE => 94,
            GREEN => 17,
            RED => 122,
            YELLOW => 174,
            PURPLE => 120,
            GREY => 21,
            WHITE => 190,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[110] {
            BLUE => 102,
            GREEN => 111,
            RED => 21,
            YELLOW => 28,
            PURPLE => 149,
            GREY => 62,
            WHITE => 129,
            BLACK => 103,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 120,
            GREEN => 57,
            RED => 66,
            YELLOW => 192,
            PURPLE => 65,
            GREY => 130,
            WHITE => 178,
            BLACK => 151,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 23,
            GREEN => 87,
            RED => 197,
            YELLOW => 144,
            PURPLE => 53,
            GREY => 88,
            WHITE => 61,
            BLACK => 168,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[69] {
            BLUE => 91,
            GREEN => 237,
            RED => 81,
            YELLOW => 180,
            PURPLE => 251,
            GREY => 90,
            WHITE => 80,
            BLACK => 171,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 159,
            GREEN => 206,
            RED => 70,
            YELLOW => 212,
            PURPLE => 218,
            GREY => 80,
            WHITE => 125,
            BLACK => 61,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 159,
            GREEN => 145,
            RED => 164,
            YELLOW => 110,
            PURPLE => 158,
            GREY => 13,
            WHITE => 96,
            BLACK => 132,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[88] {
            BLUE => 249,
            GREEN => 246,
            RED => 39,
            YELLOW => 126,
            PURPLE => 238,
            GREY => 113,
            WHITE => 231,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 147,
            GREEN => 172,
            RED => 29,
            YELLOW => 72,
            PURPLE => 22,
            GREY => 72,
            WHITE => 93,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[91] {
            BLUE => 65,
            GREEN => 57,
            RED => 197,
            YELLOW => 32,
            PURPLE => 127,
            GREY => 124,
            WHITE => 234,
            BLACK => 33,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[98] {
            BLUE => 31,
            GREEN => 91,
            RED => 36,
            YELLOW => 31,
            PURPLE => 25,
            GREY => 91,
            WHITE => 3,
            BLACK => 30,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 222,
            GREEN => 192,
            RED => 205,
            YELLOW => 47,
            PURPLE => 99,
            GREY => 230,
            WHITE => 180,
            BLACK => 192,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[100] {
            BLUE => 2,
            GREEN => 147,
            RED => 97,
            YELLOW => 47,
            PURPLE => 61,
            GREY => 208,
            WHITE => 252,
            BLACK => 5,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 11,
            GREEN => 176,
            RED => 58,
            YELLOW => 172,
            PURPLE => 192,
            GREY => 242,
            WHITE => 200,
            BLACK => 30,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[106] {
            BLUE => 209,
            GREEN => 95,
            RED => 51,
            YELLOW => 213,
            PURPLE => 0,
            GREY => 188,
            WHITE => 245,
            BLACK => 117,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 242,
            GREEN => 238,
            RED => 82,
            YELLOW => 66,
            PURPLE => 156,
            GREY => 255,
            WHITE => 73,
            BLACK => 168,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 143,
            GREEN => 27,
            RED => 49,
            YELLOW => 10,
            PURPLE => 3,
            GREY => 91,
            WHITE => 188,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 242,
            GREEN => 34,
            RED => 178,
            YELLOW => 223,
            PURPLE => 237,
            GREY => 149,
            WHITE => 176,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 124,
            GREEN => 230,
            RED => 238,
            YELLOW => 205,
            PURPLE => 86,
            GREY => 126,
            WHITE => 97,
            BLACK => 205,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[86] {
            BLUE => 114,
            GREEN => 141,
            RED => 154,
            YELLOW => 8,
            PURPLE => 24,
            GREY => 59,
            WHITE => 209,
            BLACK => 178,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 73,
            GREEN => 2,
            RED => 90,
            YELLOW => 30,
            PURPLE => 195,
            GREY => 169,
            WHITE => 106,
            BLACK => 76,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 186,
            GREEN => 118,
            RED => 198,
            YELLOW => 72,
            PURPLE => 174,
            GREY => 157,
            WHITE => 114,
            BLACK => 120,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 49,
            GREEN => 154,
            RED => 150,
            YELLOW => 223,
            PURPLE => 189,
            GREY => 199,
            WHITE => 76,
            BLACK => 216,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 103,
            GREEN => 124,
            RED => 228,
            YELLOW => 202,
            PURPLE => 212,
            GREY => 39,
            WHITE => 33,
            BLACK => 116,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[109] {
            BLUE => 230,
            GREEN => 39,
            RED => 236,
            YELLOW => 235,
            PURPLE => 49,
            GREY => 139,
            WHITE => 163,
            BLACK => 120,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[95] {
            BLUE => 110,
            GREEN => 155,
            RED => 237,
            YELLOW => 208,
            PURPLE => 52,
            GREY => 89,
            WHITE => 183,
            BLACK => 214,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 171,
            GREEN => 251,
            RED => 125,
            YELLOW => 249,
            PURPLE => 5,
            GREY => 254,
            WHITE => 53,
            BLACK => 227,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 169,
            GREEN => 201,
            RED => 42,
            YELLOW => 72,
            PURPLE => 205,
            GREY => 44,
            WHITE => 1,
            BLACK => 124,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[108] {
            BLUE => 208,
            GREEN => 144,
            RED => 150,
            YELLOW => 62,
            PURPLE => 76,
            GREY => 5,
            WHITE => 63,
            BLACK => 149,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 125,
            GREEN => 218,
            RED => 114,
            YELLOW => 138,
            PURPLE => 193,
            GREY => 31,
            WHITE => 56,
            BLACK => 44,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[89] {
            BLUE => 30,
            GREEN => 39,
            RED => 119,
            YELLOW => 150,
            PURPLE => 151,
            GREY => 55,
            WHITE => 93,
            BLACK => 204,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 21,
            GREEN => 64,
            RED => 98,
            YELLOW => 236,
            PURPLE => 203,
            GREY => 100,
            WHITE => 191,
            BLACK => 116,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 214,
            GREEN => 39,
            RED => 27,
            YELLOW => 8,
            PURPLE => 81,
            GREY => 184,
            WHITE => 114,
            BLACK => 150,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[94] {
            BLUE => 71,
            GREEN => 137,
            RED => 4,
            YELLOW => 132,
            PURPLE => 52,
            GREY => 62,
            WHITE => 90,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 197,
            GREEN => 35,
            RED => 2,
            YELLOW => 92,
            PURPLE => 143,
            GREY => 183,
            WHITE => 130,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 81,
            GREEN => 55,
            RED => 139,
            YELLOW => 24,
            PURPLE => 254,
            GREY => 182,
            WHITE => 160,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[70] {
            BLUE => 22,
            GREEN => 27,
            RED => 206,
            YELLOW => 36,
            PURPLE => 86,
            GREY => 83,
            WHITE => 191,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[73] {
            BLUE => 27,
            GREEN => 252,
            RED => 117,
            YELLOW => 66,
            PURPLE => 182,
            GREY => 37,
            WHITE => 212,
            BLACK => 200,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[80] {
            BLUE => 206,
            GREEN => 146,
            RED => 181,
            YELLOW => 89,
            PURPLE => 103,
            GREY => 95,
            WHITE => 152,
            BLACK => 198,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[87] {
            BLUE => 121,
            GREEN => 93,
            RED => 159,
            YELLOW => 47,
            PURPLE => 185,
            GREY => 193,
            WHITE => 87,
            BLACK => 54,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 188,
            GREEN => 24,
            RED => 42,
            YELLOW => 191,
            PURPLE => 223,
            GREY => 235,
            WHITE => 10,
            BLACK => 209,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 193,
            GREEN => 235,
            RED => 8,
            YELLOW => 91,
            PURPLE => 174,
            GREY => 9,
            WHITE => 208,
            BLACK => 99,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 163,
            GREEN => 223,
            RED => 243,
            YELLOW => 96,
            PURPLE => 103,
            GREY => 111,
            WHITE => 138,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 0,
            GREEN => 68,
            RED => 135,
            YELLOW => 194,
            PURPLE => 25,
            GREY => 216,
            WHITE => 19,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[79] {
            BLUE => 118,
            GREEN => 55,
            RED => 208,
            YELLOW => 179,
            PURPLE => 54,
            GREY => 32,
            WHITE => 185,
            BLACK => 72,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 172,
            GREEN => 129,
            RED => 114,
            YELLOW => 135,
            PURPLE => 87,
            GREY => 253,
            WHITE => 54,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 121,
            GREEN => 102,
            RED => 85,
            YELLOW => 112,
            PURPLE => 169,
            GREY => 59,
            WHITE => 229,
            BLACK => 82,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 168,
            GREEN => 65,
            RED => 183,
            YELLOW => 85,
            PURPLE => 43,
            GREY => 137,
            WHITE => 123,
            BLACK => 46,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[81] {
            BLUE => 158,
            GREEN => 115,
            RED => 231,
            YELLOW => 64,
            PURPLE => 209,
            GREY => 194,
            WHITE => 182,
            BLACK => 115,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 215,
            GREEN => 75,
            RED => 161,
            YELLOW => 79,
            PURPLE => 233,
            GREY => 46,
            WHITE => 103,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 59,
            GREEN => 26,
            RED => 33,
            YELLOW => 2,
            PURPLE => 225,
            GREY => 230,
            WHITE => 216,
            BLACK => 101,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 83,
            GREEN => 69,
            RED => 231,
            YELLOW => 15,
            PURPLE => 52,
            GREY => 157,
            WHITE => 229,
            BLACK => 28,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 43,
            GREEN => 85,
            RED => 229,
            YELLOW => 183,
            PURPLE => 31,
            GREY => 238,
            WHITE => 96,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[93] {
            BLUE => 201,
            GREEN => 182,
            RED => 120,
            YELLOW => 231,
            PURPLE => 57,
            GREY => 83,
            WHITE => 167,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[84] {
            BLUE => 127,
            GREEN => 55,
            RED => 169,
            YELLOW => 29,
            PURPLE => 245,
            GREY => 108,
            WHITE => 205,
            BLACK => 255,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 58,
            GREEN => 147,
            RED => 56,
            YELLOW => 205,
            PURPLE => 94,
            GREY => 204,
            WHITE => 200,
            BLACK => 53,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 69,
            GREEN => 130,
            RED => 131,
            YELLOW => 28,
            PURPLE => 181,
            GREY => 250,
            WHITE => 129,
            BLACK => 101,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 61,
            GREEN => 117,
            RED => 146,
            YELLOW => 254,
            PURPLE => 207,
            GREY => 19,
            WHITE => 93,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[77] {
            BLUE => 200,
            GREEN => 195,
            RED => 56,
            YELLOW => 28,
            PURPLE => 23,
            GREY => 28,
            WHITE => 103,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[99] {
            BLUE => 210,
            GREEN => 169,
            RED => 66,
            YELLOW => 56,
            PURPLE => 116,
            GREY => 43,
            WHITE => 120,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 99,
            GREEN => 9,
            RED => 55,
            YELLOW => 107,
            PURPLE => 129,
            GREY => 174,
            WHITE => 51,
            BLACK => 186,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[74] {
            BLUE => 87,
            GREEN => 233,
            RED => 84,
            YELLOW => 108,
            PURPLE => 112,
            GREY => 31,
            WHITE => 66,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 19,
            GREEN => 7,
            RED => 95,
            YELLOW => 12,
            PURPLE => 168,
            GREY => 15,
            WHITE => 225,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[105] {
            BLUE => 255,
            GREEN => 241,
            RED => 156,
            YELLOW => 113,
            PURPLE => 203,
            GREY => 28,
            WHITE => 212,
            BLACK => 82,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[72] {
            BLUE => 32,
            GREEN => 36,
            RED => 43,
            YELLOW => 185,
            PURPLE => 91,
            GREY => 61,
            WHITE => 103,
            BLACK => 91,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 81,
            GREEN => 18,
            RED => 157,
            YELLOW => 12,
            PURPLE => 5,
            GREY => 151,
            WHITE => 49,
            BLACK => 75,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 1,
            GREEN => 112,
            RED => 10,
            YELLOW => 221,
            PURPLE => 118,
            GREY => 17,
            WHITE => 194,
            BLACK => 106,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[76] {
            BLUE => 75,
            GREEN => 183,
            RED => 192,
            YELLOW => 134,
            PURPLE => 254,
            GREY => 121,
            WHITE => 241,
            BLACK => 98,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 15,
            GREEN => 98,
            RED => 121,
            YELLOW => 63,
            PURPLE => 47,
            GREY => 122,
            WHITE => 80,
            BLACK => 225,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 78,
            GREEN => 221,
            RED => 77,
            YELLOW => 255,
            PURPLE => 73,
            GREY => 195,
            WHITE => 85,
            BLACK => 125,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[65] {
            BLUE => 63,
            GREEN => 201,
            RED => 162,
            YELLOW => 111,
            PURPLE => 15,
            GREY => 83,
            WHITE => 141,
            BLACK => 78,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[92] {
            BLUE => 163,
            GREEN => 169,
            RED => 30,
            YELLOW => 160,
            PURPLE => 68,
            GREY => 142,
            WHITE => 50,
            BLACK => 149,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 107,
            GREEN => 46,
            RED => 254,
            YELLOW => 200,
            PURPLE => 133,
            GREY => 235,
            WHITE => 227,
            BLACK => 221,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 107,
            GREEN => 234,
            RED => 138,
            YELLOW => 204,
            PURPLE => 148,
            GREY => 227,
            WHITE => 190,
            BLACK => 171,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[82] {
            BLUE => 121,
            GREEN => 159,
            RED => 197,
            YELLOW => 98,
            PURPLE => 105,
            GREY => 223,
            WHITE => 107,
            BLACK => 45,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 45,
            GREEN => 206,
            RED => 103,
            YELLOW => 21,
            PURPLE => 51,
            GREY => 102,
            WHITE => 227,
            BLACK => 248,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 124,
            GREEN => 35,
            RED => 129,
            YELLOW => 213,
            PURPLE => 106,
            GREY => 57,
            WHITE => 158,
            BLACK => 6,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 18,
            GREEN => 117,
            RED => 178,
            YELLOW => 160,
            PURPLE => 135,
            GREY => 76,
            WHITE => 152,
            BLACK => 117,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 254,
            GREEN => 62,
            RED => 31,
            YELLOW => 92,
            PURPLE => 13,
            GREY => 179,
            WHITE => 79,
            BLACK => 130,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 4,
            GREEN => 26,
            RED => 18,
            YELLOW => 119,
            PURPLE => 95,
            GREY => 42,
            WHITE => 230,
            BLACK => 153,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 84,
            GREEN => 23,
            RED => 251,
            YELLOW => 240,
            PURPLE => 78,
            GREY => 182,
            WHITE => 213,
            BLACK => 181,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[75] {
            BLUE => 226,
            GREEN => 234,
            RED => 40,
            YELLOW => 107,
            PURPLE => 51,
            GREY => 115,
            WHITE => 192,
            BLACK => 59,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 142,
            GREEN => 59,
            RED => 189,
            YELLOW => 253,
            PURPLE => 251,
            GREY => 71,
            WHITE => 3,
            BLACK => 132,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[104] {
            BLUE => 75,
            GREEN => 129,
            RED => 181,
            YELLOW => 153,
            PURPLE => 59,
            GREY => 99,
            WHITE => 151,
            BLACK => 118,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 48,
            GREEN => 204,
            RED => 184,
            YELLOW => 72,
            PURPLE => 95,
            GREY => 103,
            WHITE => 201,
            BLACK => 94,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 65,
            GREEN => 106,
            RED => 138,
            YELLOW => 225,
            PURPLE => 111,
            GREY => 26,
            WHITE => 200,
            BLACK => 17,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 157,
            GREEN => 187,
            RED => 196,
            YELLOW => 82,
            PURPLE => 27,
            GREY => 71,
            WHITE => 164,
            BLACK => 251,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[103] {
            BLUE => 2,
            GREEN => 41,
            RED => 5,
            YELLOW => 60,
            PURPLE => 160,
            GREY => 163,
            WHITE => 171,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[71] {
            BLUE => 215,
            GREEN => 178,
            RED => 188,
            YELLOW => 123,
            PURPLE => 152,
            GREY => 58,
            WHITE => 67,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 204,
            GREEN => 34,
            RED => 193,
            YELLOW => 113,
            PURPLE => 16,
            GREY => 118,
            WHITE => 60,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 153,
            GREEN => 46,
            RED => 247,
            YELLOW => 125,
            PURPLE => 208,
            GREY => 183,
            WHITE => 166,
            BLACK => 164,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 109,
            GREEN => 120,
            RED => 186,
            YELLOW => 189,
            PURPLE => 95,
            GREY => 201,
            WHITE => 111,
            BLACK => 44,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[96] {
            BLUE => 152,
            GREEN => 152,
            RED => 191,
            YELLOW => 154,
            PURPLE => 124,
            GREY => 78,
            WHITE => 195,
            BLACK => 123,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 189,
            GREEN => 121,
            RED => 57,
            YELLOW => 32,
            PURPLE => 185,
            GREY => 228,
            WHITE => 161,
            BLACK => 121,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 74,
            GREEN => 172,
            RED => 6,
            YELLOW => 169,
            PURPLE => 201,
            GREY => 34,
            WHITE => 5,
            BLACK => 228,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 91,
            GREEN => 217,
            RED => 9,
            YELLOW => 22,
            PURPLE => 155,
            GREY => 179,
            WHITE => 118,
            BLACK => 115,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[97] {
            BLUE => 12,
            GREEN => 126,
            RED => 144,
            YELLOW => 103,
            PURPLE => 238,
            GREY => 84,
            WHITE => 95,
            BLACK => 214,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 145,
            GREEN => 11,
            RED => 172,
            YELLOW => 11,
            PURPLE => 130,
            GREY => 77,
            WHITE => 202,
            BLACK => 84,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 40,
            GREEN => 154,
            RED => 64,
            YELLOW => 11,
            PURPLE => 169,
            GREY => 123,
            WHITE => 255,
            BLACK => 102,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[83] {
            BLUE => 217,
            GREEN => 194,
            RED => 129,
            YELLOW => 52,
            PURPLE => 85,
            GREY => 68,
            WHITE => 27,
            BLACK => 196,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 239,
            GREEN => 215,
            RED => 213,
            YELLOW => 199,
            PURPLE => 107,
            GREY => 23,
            WHITE => 204,
            BLACK => 122,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 72,
            GREEN => 110,
            RED => 201,
            YELLOW => 118,
            PURPLE => 10,
            GREY => 36,
            WHITE => 221,
            BLACK => 81,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[85] {
            BLUE => 145,
            GREEN => 147,
            RED => 84,
            YELLOW => 130,
            PURPLE => 10,
            GREY => 8,
            WHITE => 182,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 67,
            GREEN => 58,
            RED => 130,
            YELLOW => 105,
            PURPLE => 53,
            GREY => 44,
            WHITE => 183,
            BLACK => 69,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 215,
            GREEN => 147,
            RED => 247,
            YELLOW => 112,
            PURPLE => 97,
            GREY => 48,
            WHITE => 235,
            BLACK => 166,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[101] {
            BLUE => 1,
            GREEN => 78,
            RED => 71,
            YELLOW => 205,
            PURPLE => 24,
            GREY => 67,
            WHITE => 36,
            BLACK => 180,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[102] {
            BLUE => 12,
            GREEN => 215,
            RED => 108,
            YELLOW => 216,
            PURPLE => 138,
            GREY => 224,
            WHITE => 81,
            BLACK => 177,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[78] {
            BLUE => 132,
            GREEN => 85,
            RED => 108,
            YELLOW => 192,
            PURPLE => 22,
            GREY => 151,
            WHITE => 153,
            BLACK => 142,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if nor(nor(nor(nor(nor(match buffer[6] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[8] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), match buffer[0] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(match buffer[7] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[9] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })), nor(nor(match buffer[5] { 4292956723 => true, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, }, nor(match buffer[4] { 4292956723 => true, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[3] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })), nor(false, false))), nor(nor(nor(false, match buffer[1] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(false, match buffer[10] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => true, 4280427042 => false, _ => false, })), nor(nor(false, match buffer[11] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), nor(false, false)))) {
	let n = match buffer[13] {
            BLUE => 96,
            GREEN => 161,
            RED => 179,
            YELLOW => 205,
            PURPLE => 172,
            GREY => 91,
            WHITE => 80,
            BLACK => 61,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 112,
            GREEN => 71,
            RED => 227,
            YELLOW => 68,
            PURPLE => 6,
            GREY => 186,
            WHITE => 63,
            BLACK => 204,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 111,
            GREEN => 187,
            RED => 92,
            YELLOW => 156,
            PURPLE => 75,
            GREY => 196,
            WHITE => 188,
            BLACK => 2,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 62,
            GREEN => 230,
            RED => 76,
            YELLOW => 89,
            PURPLE => 197,
            GREY => 192,
            WHITE => 192,
            BLACK => 206,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 101,
            GREEN => 203,
            RED => 198,
            YELLOW => 74,
            PURPLE => 33,
            GREY => 86,
            WHITE => 96,
            BLACK => 103,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 34,
            GREEN => 117,
            RED => 184,
            YELLOW => 237,
            PURPLE => 209,
            GREY => 52,
            WHITE => 151,
            BLACK => 48,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 75,
            GREEN => 178,
            RED => 204,
            YELLOW => 124,
            PURPLE => 179,
            GREY => 160,
            WHITE => 235,
            BLACK => 253,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 149,
            GREEN => 244,
            RED => 5,
            YELLOW => 205,
            PURPLE => 217,
            GREY => 33,
            WHITE => 150,
            BLACK => 76,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 106,
            GREEN => 72,
            RED => 152,
            YELLOW => 63,
            PURPLE => 43,
            GREY => 55,
            WHITE => 201,
            BLACK => 18,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 36,
            GREEN => 72,
            RED => 194,
            YELLOW => 227,
            PURPLE => 118,
            GREY => 105,
            WHITE => 99,
            BLACK => 73,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 35,
            GREEN => 246,
            RED => 134,
            YELLOW => 37,
            PURPLE => 100,
            GREY => 51,
            WHITE => 252,
            BLACK => 90,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 185,
            GREEN => 195,
            RED => 213,
            YELLOW => 179,
            PURPLE => 147,
            GREY => 122,
            WHITE => 162,
            BLACK => 229,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 12,
            GREEN => 174,
            RED => 32,
            YELLOW => 69,
            PURPLE => 193,
            GREY => 160,
            WHITE => 130,
            BLACK => 14,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 74,
            GREEN => 37,
            RED => 238,
            YELLOW => 51,
            PURPLE => 25,
            GREY => 244,
            WHITE => 100,
            BLACK => 247,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 148,
            GREEN => 212,
            RED => 66,
            YELLOW => 111,
            PURPLE => 245,
            GREY => 13,
            WHITE => 82,
            BLACK => 182,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 5,
            GREEN => 202,
            RED => 130,
            YELLOW => 27,
            PURPLE => 26,
            GREY => 249,
            WHITE => 226,
            BLACK => 220,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 43,
            GREEN => 71,
            RED => 157,
            YELLOW => 25,
            PURPLE => 171,
            GREY => 106,
            WHITE => 216,
            BLACK => 14,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 46,
            GREEN => 98,
            RED => 59,
            YELLOW => 137,
            PURPLE => 159,
            GREY => 145,
            WHITE => 212,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 183,
            GREEN => 116,
            RED => 131,
            YELLOW => 140,
            PURPLE => 65,
            GREY => 189,
            WHITE => 16,
            BLACK => 178,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 226,
            GREEN => 181,
            RED => 188,
            YELLOW => 59,
            PURPLE => 56,
            GREY => 164,
            WHITE => 33,
            BLACK => 114,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 30,
            GREEN => 171,
            RED => 132,
            YELLOW => 122,
            PURPLE => 131,
            GREY => 204,
            WHITE => 250,
            BLACK => 40,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 53,
            GREEN => 137,
            RED => 93,
            YELLOW => 67,
            PURPLE => 190,
            GREY => 208,
            WHITE => 201,
            BLACK => 115,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 166,
            GREEN => 82,
            RED => 219,
            YELLOW => 29,
            PURPLE => 119,
            GREY => 33,
            WHITE => 184,
            BLACK => 22,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 228,
            GREEN => 37,
            RED => 233,
            YELLOW => 190,
            PURPLE => 162,
            GREY => 221,
            WHITE => 127,
            BLACK => 48,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 230,
            GREEN => 86,
            RED => 3,
            YELLOW => 102,
            PURPLE => 166,
            GREY => 227,
            WHITE => 125,
            BLACK => 79,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 60,
            GREEN => 19,
            RED => 86,
            YELLOW => 196,
            PURPLE => 41,
            GREY => 35,
            WHITE => 107,
            BLACK => 82,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 68,
            GREEN => 6,
            RED => 72,
            YELLOW => 53,
            PURPLE => 248,
            GREY => 18,
            WHITE => 49,
            BLACK => 228,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 33,
            GREEN => 11,
            RED => 39,
            YELLOW => 3,
            PURPLE => 193,
            GREY => 151,
            WHITE => 62,
            BLACK => 54,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 28,
            GREEN => 27,
            RED => 28,
            YELLOW => 2,
            PURPLE => 243,
            GREY => 23,
            WHITE => 143,
            BLACK => 199,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 121,
            GREEN => 89,
            RED => 137,
            YELLOW => 6,
            PURPLE => 45,
            GREY => 119,
            WHITE => 6,
            BLACK => 61,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 31,
            GREEN => 40,
            RED => 227,
            YELLOW => 69,
            PURPLE => 30,
            GREY => 19,
            WHITE => 58,
            BLACK => 70,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 93,
            GREEN => 39,
            RED => 42,
            YELLOW => 246,
            PURPLE => 140,
            GREY => 231,
            WHITE => 12,
            BLACK => 49,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 3,
            GREEN => 188,
            RED => 22,
            YELLOW => 246,
            PURPLE => 55,
            GREY => 254,
            WHITE => 160,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 24,
            GREEN => 145,
            RED => 234,
            YELLOW => 26,
            PURPLE => 56,
            GREY => 120,
            WHITE => 160,
            BLACK => 149,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 20,
            GREEN => 246,
            RED => 182,
            YELLOW => 97,
            PURPLE => 187,
            GREY => 177,
            WHITE => 125,
            BLACK => 27,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 50,
            GREEN => 98,
            RED => 238,
            YELLOW => 65,
            PURPLE => 235,
            GREY => 18,
            WHITE => 199,
            BLACK => 155,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 98,
            GREEN => 161,
            RED => 45,
            YELLOW => 219,
            PURPLE => 56,
            GREY => 152,
            WHITE => 113,
            BLACK => 103,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else if nor(nor(nor(match buffer[3] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[0] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => true, _ => false, }), match buffer[4] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }), nor(false, match buffer[1] { 4292956723 => true, 4285444144 => false, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, })) {
	
}else {
	let n = match buffer[52] {
            BLUE => 79,
            GREEN => 180,
            RED => 235,
            YELLOW => 3,
            PURPLE => 73,
            GREY => 18,
            WHITE => 195,
            BLACK => 143,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[70] {
            BLUE => 128,
            GREEN => 82,
            RED => 41,
            YELLOW => 6,
            PURPLE => 107,
            GREY => 92,
            WHITE => 0,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[77] {
            BLUE => 49,
            GREEN => 247,
            RED => 70,
            YELLOW => 122,
            PURPLE => 45,
            GREY => 95,
            WHITE => 92,
            BLACK => 236,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 40,
            GREEN => 217,
            RED => 127,
            YELLOW => 126,
            PURPLE => 0,
            GREY => 161,
            WHITE => 71,
            BLACK => 40,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 11,
            GREEN => 55,
            RED => 71,
            YELLOW => 105,
            PURPLE => 180,
            GREY => 189,
            WHITE => 196,
            BLACK => 158,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 202,
            GREEN => 26,
            RED => 90,
            YELLOW => 30,
            PURPLE => 6,
            GREY => 50,
            WHITE => 236,
            BLACK => 179,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[104] {
            BLUE => 45,
            GREEN => 225,
            RED => 246,
            YELLOW => 125,
            PURPLE => 62,
            GREY => 149,
            WHITE => 158,
            BLACK => 138,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 63,
            GREEN => 179,
            RED => 10,
            YELLOW => 254,
            PURPLE => 247,
            GREY => 64,
            WHITE => 99,
            BLACK => 250,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 132,
            GREEN => 178,
            RED => 123,
            YELLOW => 196,
            PURPLE => 104,
            GREY => 167,
            WHITE => 250,
            BLACK => 158,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 142,
            GREEN => 122,
            RED => 80,
            YELLOW => 8,
            PURPLE => 25,
            GREY => 155,
            WHITE => 252,
            BLACK => 186,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 114,
            GREEN => 168,
            RED => 130,
            YELLOW => 41,
            PURPLE => 18,
            GREY => 13,
            WHITE => 66,
            BLACK => 52,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[98] {
            BLUE => 220,
            GREEN => 242,
            RED => 65,
            YELLOW => 238,
            PURPLE => 112,
            GREY => 160,
            WHITE => 39,
            BLACK => 85,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[80] {
            BLUE => 158,
            GREEN => 126,
            RED => 167,
            YELLOW => 154,
            PURPLE => 188,
            GREY => 69,
            WHITE => 220,
            BLACK => 68,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 27,
            GREEN => 115,
            RED => 66,
            YELLOW => 104,
            PURPLE => 211,
            GREY => 92,
            WHITE => 20,
            BLACK => 159,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[103] {
            BLUE => 132,
            GREEN => 70,
            RED => 241,
            YELLOW => 135,
            PURPLE => 241,
            GREY => 45,
            WHITE => 15,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 116,
            GREEN => 98,
            RED => 241,
            YELLOW => 221,
            PURPLE => 147,
            GREY => 98,
            WHITE => 79,
            BLACK => 130,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 181,
            GREEN => 128,
            RED => 253,
            YELLOW => 34,
            PURPLE => 174,
            GREY => 24,
            WHITE => 42,
            BLACK => 181,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 122,
            GREEN => 93,
            RED => 241,
            YELLOW => 131,
            PURPLE => 51,
            GREY => 241,
            WHITE => 124,
            BLACK => 125,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 153,
            GREEN => 229,
            RED => 211,
            YELLOW => 144,
            PURPLE => 133,
            GREY => 112,
            WHITE => 16,
            BLACK => 6,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 92,
            GREEN => 204,
            RED => 227,
            YELLOW => 177,
            PURPLE => 34,
            GREY => 69,
            WHITE => 164,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 106,
            GREEN => 162,
            RED => 21,
            YELLOW => 148,
            PURPLE => 85,
            GREY => 86,
            WHITE => 161,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[88] {
            BLUE => 134,
            GREEN => 88,
            RED => 111,
            YELLOW => 55,
            PURPLE => 223,
            GREY => 191,
            WHITE => 76,
            BLACK => 74,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[102] {
            BLUE => 172,
            GREEN => 46,
            RED => 177,
            YELLOW => 34,
            PURPLE => 220,
            GREY => 217,
            WHITE => 159,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[101] {
            BLUE => 181,
            GREEN => 207,
            RED => 51,
            YELLOW => 9,
            PURPLE => 55,
            GREY => 180,
            WHITE => 71,
            BLACK => 172,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 153,
            GREEN => 242,
            RED => 131,
            YELLOW => 217,
            PURPLE => 119,
            GREY => 224,
            WHITE => 233,
            BLACK => 245,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[91] {
            BLUE => 19,
            GREEN => 241,
            RED => 102,
            YELLOW => 174,
            PURPLE => 67,
            GREY => 127,
            WHITE => 6,
            BLACK => 153,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 242,
            GREEN => 88,
            RED => 237,
            YELLOW => 183,
            PURPLE => 123,
            GREY => 209,
            WHITE => 99,
            BLACK => 11,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 223,
            GREEN => 28,
            RED => 227,
            YELLOW => 148,
            PURPLE => 80,
            GREY => 208,
            WHITE => 223,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 131,
            GREEN => 110,
            RED => 25,
            YELLOW => 54,
            PURPLE => 127,
            GREY => 189,
            WHITE => 37,
            BLACK => 90,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 84,
            GREEN => 138,
            RED => 116,
            YELLOW => 233,
            PURPLE => 251,
            GREY => 244,
            WHITE => 237,
            BLACK => 158,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 27,
            GREEN => 207,
            RED => 31,
            YELLOW => 252,
            PURPLE => 100,
            GREY => 156,
            WHITE => 33,
            BLACK => 52,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[84] {
            BLUE => 1,
            GREEN => 210,
            RED => 166,
            YELLOW => 32,
            PURPLE => 251,
            GREY => 21,
            WHITE => 101,
            BLACK => 28,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 27,
            GREEN => 30,
            RED => 222,
            YELLOW => 86,
            PURPLE => 80,
            GREY => 18,
            WHITE => 215,
            BLACK => 34,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 245,
            GREEN => 185,
            RED => 241,
            YELLOW => 99,
            PURPLE => 146,
            GREY => 167,
            WHITE => 243,
            BLACK => 102,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 35,
            GREEN => 109,
            RED => 212,
            YELLOW => 185,
            PURPLE => 210,
            GREY => 92,
            WHITE => 116,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[94] {
            BLUE => 80,
            GREEN => 79,
            RED => 38,
            YELLOW => 97,
            PURPLE => 33,
            GREY => 63,
            WHITE => 158,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[83] {
            BLUE => 36,
            GREEN => 168,
            RED => 176,
            YELLOW => 139,
            PURPLE => 82,
            GREY => 215,
            WHITE => 205,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[99] {
            BLUE => 70,
            GREEN => 7,
            RED => 78,
            YELLOW => 111,
            PURPLE => 118,
            GREY => 139,
            WHITE => 206,
            BLACK => 51,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[93] {
            BLUE => 216,
            GREEN => 226,
            RED => 54,
            YELLOW => 221,
            PURPLE => 224,
            GREY => 16,
            WHITE => 75,
            BLACK => 229,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 217,
            GREEN => 134,
            RED => 66,
            YELLOW => 117,
            PURPLE => 199,
            GREY => 54,
            WHITE => 22,
            BLACK => 74,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[97] {
            BLUE => 247,
            GREEN => 4,
            RED => 49,
            YELLOW => 198,
            PURPLE => 205,
            GREY => 78,
            WHITE => 68,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 205,
            GREEN => 82,
            RED => 21,
            YELLOW => 242,
            PURPLE => 233,
            GREY => 46,
            WHITE => 66,
            BLACK => 61,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 170,
            GREEN => 165,
            RED => 166,
            YELLOW => 156,
            PURPLE => 42,
            GREY => 23,
            WHITE => 127,
            BLACK => 192,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[75] {
            BLUE => 73,
            GREEN => 106,
            RED => 245,
            YELLOW => 230,
            PURPLE => 42,
            GREY => 201,
            WHITE => 4,
            BLACK => 173,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[89] {
            BLUE => 119,
            GREEN => 254,
            RED => 94,
            YELLOW => 78,
            PURPLE => 28,
            GREY => 65,
            WHITE => 57,
            BLACK => 53,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 235,
            GREEN => 23,
            RED => 221,
            YELLOW => 249,
            PURPLE => 83,
            GREY => 175,
            WHITE => 94,
            BLACK => 136,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[85] {
            BLUE => 35,
            GREEN => 134,
            RED => 156,
            YELLOW => 242,
            PURPLE => 140,
            GREY => 2,
            WHITE => 132,
            BLACK => 128,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 112,
            GREEN => 1,
            RED => 5,
            YELLOW => 110,
            PURPLE => 255,
            GREY => 26,
            WHITE => 184,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 36,
            GREEN => 50,
            RED => 200,
            YELLOW => 53,
            PURPLE => 32,
            GREY => 167,
            WHITE => 42,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 163,
            GREEN => 240,
            RED => 15,
            YELLOW => 21,
            PURPLE => 190,
            GREY => 190,
            WHITE => 236,
            BLACK => 4,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 1,
            GREEN => 137,
            RED => 168,
            YELLOW => 172,
            PURPLE => 34,
            GREY => 231,
            WHITE => 109,
            BLACK => 55,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 10,
            GREEN => 167,
            RED => 89,
            YELLOW => 231,
            PURPLE => 241,
            GREY => 11,
            WHITE => 229,
            BLACK => 26,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 67,
            GREEN => 191,
            RED => 238,
            YELLOW => 87,
            PURPLE => 191,
            GREY => 179,
            WHITE => 190,
            BLACK => 21,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 245,
            GREEN => 126,
            RED => 5,
            YELLOW => 193,
            PURPLE => 1,
            GREY => 81,
            WHITE => 249,
            BLACK => 149,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[92] {
            BLUE => 167,
            GREEN => 138,
            RED => 246,
            YELLOW => 251,
            PURPLE => 86,
            GREY => 124,
            WHITE => 137,
            BLACK => 16,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 188,
            GREEN => 64,
            RED => 64,
            YELLOW => 232,
            PURPLE => 47,
            GREY => 164,
            WHITE => 153,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[65] {
            BLUE => 173,
            GREEN => 165,
            RED => 218,
            YELLOW => 226,
            PURPLE => 35,
            GREY => 105,
            WHITE => 174,
            BLACK => 10,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 17,
            GREEN => 124,
            RED => 135,
            YELLOW => 5,
            PURPLE => 197,
            GREY => 4,
            WHITE => 110,
            BLACK => 12,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 130,
            GREEN => 59,
            RED => 98,
            YELLOW => 5,
            PURPLE => 58,
            GREY => 109,
            WHITE => 19,
            BLACK => 199,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 72,
            GREEN => 51,
            RED => 192,
            YELLOW => 221,
            PURPLE => 183,
            GREY => 183,
            WHITE => 80,
            BLACK => 246,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[107] {
            BLUE => 109,
            GREEN => 150,
            RED => 165,
            YELLOW => 237,
            PURPLE => 219,
            GREY => 127,
            WHITE => 68,
            BLACK => 124,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 92,
            GREEN => 127,
            RED => 30,
            YELLOW => 146,
            PURPLE => 238,
            GREY => 171,
            WHITE => 243,
            BLACK => 190,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 219,
            GREEN => 240,
            RED => 1,
            YELLOW => 153,
            PURPLE => 99,
            GREY => 45,
            WHITE => 124,
            BLACK => 102,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[76] {
            BLUE => 133,
            GREEN => 251,
            RED => 77,
            YELLOW => 9,
            PURPLE => 218,
            GREY => 214,
            WHITE => 133,
            BLACK => 172,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[82] {
            BLUE => 105,
            GREEN => 65,
            RED => 200,
            YELLOW => 179,
            PURPLE => 74,
            GREY => 53,
            WHITE => 8,
            BLACK => 134,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 95,
            GREEN => 161,
            RED => 103,
            YELLOW => 145,
            PURPLE => 187,
            GREY => 131,
            WHITE => 148,
            BLACK => 253,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[95] {
            BLUE => 162,
            GREEN => 166,
            RED => 35,
            YELLOW => 107,
            PURPLE => 179,
            GREY => 53,
            WHITE => 54,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[108] {
            BLUE => 229,
            GREEN => 92,
            RED => 241,
            YELLOW => 173,
            PURPLE => 7,
            GREY => 200,
            WHITE => 92,
            BLACK => 190,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[79] {
            BLUE => 33,
            GREEN => 152,
            RED => 14,
            YELLOW => 139,
            PURPLE => 76,
            GREY => 0,
            WHITE => 19,
            BLACK => 128,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[71] {
            BLUE => 151,
            GREEN => 188,
            RED => 154,
            YELLOW => 91,
            PURPLE => 4,
            GREY => 42,
            WHITE => 149,
            BLACK => 246,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[100] {
            BLUE => 250,
            GREEN => 215,
            RED => 222,
            YELLOW => 120,
            PURPLE => 94,
            GREY => 18,
            WHITE => 138,
            BLACK => 237,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 103,
            GREEN => 79,
            RED => 196,
            YELLOW => 130,
            PURPLE => 42,
            GREY => 119,
            WHITE => 71,
            BLACK => 24,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 144,
            GREEN => 156,
            RED => 73,
            YELLOW => 189,
            PURPLE => 79,
            GREY => 28,
            WHITE => 49,
            BLACK => 184,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 1,
            GREEN => 36,
            RED => 51,
            YELLOW => 198,
            PURPLE => 122,
            GREY => 116,
            WHITE => 62,
            BLACK => 60,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 124,
            GREEN => 45,
            RED => 235,
            YELLOW => 106,
            PURPLE => 106,
            GREY => 237,
            WHITE => 106,
            BLACK => 218,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[73] {
            BLUE => 199,
            GREEN => 48,
            RED => 195,
            YELLOW => 10,
            PURPLE => 106,
            GREY => 131,
            WHITE => 205,
            BLACK => 137,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 23,
            GREEN => 146,
            RED => 161,
            YELLOW => 96,
            PURPLE => 241,
            GREY => 2,
            WHITE => 199,
            BLACK => 81,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 110,
            GREEN => 6,
            RED => 188,
            YELLOW => 74,
            PURPLE => 28,
            GREY => 75,
            WHITE => 46,
            BLACK => 189,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[72] {
            BLUE => 183,
            GREEN => 121,
            RED => 230,
            YELLOW => 239,
            PURPLE => 58,
            GREY => 250,
            WHITE => 175,
            BLACK => 202,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 54,
            GREEN => 239,
            RED => 192,
            YELLOW => 29,
            PURPLE => 124,
            GREY => 150,
            WHITE => 92,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[96] {
            BLUE => 103,
            GREEN => 48,
            RED => 34,
            YELLOW => 229,
            PURPLE => 3,
            GREY => 229,
            WHITE => 65,
            BLACK => 99,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 250,
            GREEN => 25,
            RED => 109,
            YELLOW => 120,
            PURPLE => 120,
            GREY => 144,
            WHITE => 51,
            BLACK => 103,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 173,
            GREEN => 101,
            RED => 102,
            YELLOW => 39,
            PURPLE => 221,
            GREY => 19,
            WHITE => 164,
            BLACK => 89,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[87] {
            BLUE => 183,
            GREEN => 99,
            RED => 112,
            YELLOW => 199,
            PURPLE => 220,
            GREY => 72,
            WHITE => 139,
            BLACK => 208,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[86] {
            BLUE => 15,
            GREEN => 169,
            RED => 198,
            YELLOW => 41,
            PURPLE => 31,
            GREY => 235,
            WHITE => 117,
            BLACK => 127,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[105] {
            BLUE => 124,
            GREEN => 97,
            RED => 222,
            YELLOW => 254,
            PURPLE => 47,
            GREY => 168,
            WHITE => 120,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[81] {
            BLUE => 187,
            GREEN => 159,
            RED => 168,
            YELLOW => 182,
            PURPLE => 180,
            GREY => 93,
            WHITE => 214,
            BLACK => 156,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[56] {
            BLUE => 148,
            GREEN => 193,
            RED => 99,
            YELLOW => 154,
            PURPLE => 42,
            GREY => 181,
            WHITE => 59,
            BLACK => 3,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 79,
            GREEN => 43,
            RED => 221,
            YELLOW => 44,
            PURPLE => 180,
            GREY => 15,
            WHITE => 232,
            BLACK => 100,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 101,
            GREEN => 222,
            RED => 231,
            YELLOW => 121,
            PURPLE => 171,
            GREY => 201,
            WHITE => 178,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[90] {
            BLUE => 121,
            GREEN => 33,
            RED => 177,
            YELLOW => 213,
            PURPLE => 173,
            GREY => 0,
            WHITE => 58,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 43,
            GREEN => 173,
            RED => 120,
            YELLOW => 93,
            PURPLE => 145,
            GREY => 106,
            WHITE => 188,
            BLACK => 162,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 158,
            GREEN => 13,
            RED => 147,
            YELLOW => 176,
            PURPLE => 65,
            GREY => 251,
            WHITE => 164,
            BLACK => 251,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 246,
            GREEN => 180,
            RED => 152,
            YELLOW => 23,
            PURPLE => 37,
            GREY => 165,
            WHITE => 213,
            BLACK => 19,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 216,
            GREEN => 18,
            RED => 59,
            YELLOW => 46,
            PURPLE => 20,
            GREY => 188,
            WHITE => 0,
            BLACK => 185,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 252,
            GREEN => 5,
            RED => 85,
            YELLOW => 164,
            PURPLE => 157,
            GREY => 221,
            WHITE => 175,
            BLACK => 13,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 198,
            GREEN => 111,
            RED => 153,
            YELLOW => 193,
            PURPLE => 223,
            GREY => 230,
            WHITE => 166,
            BLACK => 89,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 157,
            GREEN => 223,
            RED => 197,
            YELLOW => 6,
            PURPLE => 111,
            GREY => 87,
            WHITE => 206,
            BLACK => 86,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 208,
            GREEN => 49,
            RED => 43,
            YELLOW => 19,
            PURPLE => 64,
            GREY => 5,
            WHITE => 131,
            BLACK => 21,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 105,
            GREEN => 168,
            RED => 159,
            YELLOW => 175,
            PURPLE => 93,
            GREY => 245,
            WHITE => 108,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[69] {
            BLUE => 126,
            GREEN => 238,
            RED => 47,
            YELLOW => 192,
            PURPLE => 208,
            GREY => 246,
            WHITE => 109,
            BLACK => 154,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 144,
            GREEN => 242,
            RED => 213,
            YELLOW => 227,
            PURPLE => 179,
            GREY => 130,
            WHITE => 29,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 184,
            GREEN => 118,
            RED => 233,
            YELLOW => 3,
            PURPLE => 234,
            GREY => 62,
            WHITE => 53,
            BLACK => 112,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[74] {
            BLUE => 32,
            GREEN => 15,
            RED => 92,
            YELLOW => 76,
            PURPLE => 56,
            GREY => 188,
            WHITE => 75,
            BLACK => 135,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[106] {
            BLUE => 255,
            GREEN => 8,
            RED => 56,
            YELLOW => 100,
            PURPLE => 242,
            GREY => 143,
            WHITE => 12,
            BLACK => 212,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[78] {
            BLUE => 36,
            GREEN => 134,
            RED => 160,
            YELLOW => 46,
            PURPLE => 235,
            GREY => 211,
            WHITE => 222,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 177,
            GREEN => 242,
            RED => 252,
            YELLOW => 5,
            PURPLE => 252,
            GREY => 191,
            WHITE => 197,
            BLACK => 254,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 159,
            GREEN => 243,
            RED => 22,
            YELLOW => 89,
            PURPLE => 248,
            GREY => 68,
            WHITE => 112,
            BLACK => 46,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}

    }

    if input.pressed_this_frame(Button::B) {
        if nor(nor(nor(false, match buffer[5] { 4292956723 => false, 4285444144 => false, 4282993118 => true, 4281842175 => false, 4283708243 => true, 4287331674 => true, 4293848814 => false, 4280427042 => true, _ => false, }), nor(false, nor(match buffer[0] { 4292956723 => false, 4285444144 => false, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }, match buffer[3] { 4292956723 => true, 4285444144 => true, 4282993118 => false, 4281842175 => true, 4283708243 => true, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }))), nor(nor(nor(match buffer[4] { 4292956723 => false, 4285444144 => true, 4282993118 => false, 4281842175 => false, 4283708243 => false, 4287331674 => false, 4293848814 => false, 4280427042 => false, _ => false, }, match buffer[1] { 4292956723 => false, 4285444144 => true, 4282993118 => true, 4281842175 => true, 4283708243 => true, 4287331674 => true, 4293848814 => true, 4280427042 => false, _ => false, }), false), nor(false, false))) {
	let n = match buffer[7] {
            BLUE => 78,
            GREEN => 72,
            RED => 70,
            YELLOW => 255,
            PURPLE => 254,
            GREY => 0,
            WHITE => 50,
            BLACK => 254,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 184,
            GREEN => 155,
            RED => 211,
            YELLOW => 5,
            PURPLE => 53,
            GREY => 17,
            WHITE => 38,
            BLACK => 95,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[73] {
            BLUE => 233,
            GREEN => 199,
            RED => 174,
            YELLOW => 226,
            PURPLE => 45,
            GREY => 107,
            WHITE => 220,
            BLACK => 72,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[88] {
            BLUE => 72,
            GREEN => 108,
            RED => 241,
            YELLOW => 34,
            PURPLE => 94,
            GREY => 224,
            WHITE => 39,
            BLACK => 107,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[71] {
            BLUE => 214,
            GREEN => 118,
            RED => 124,
            YELLOW => 21,
            PURPLE => 12,
            GREY => 217,
            WHITE => 8,
            BLACK => 20,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 202,
            GREEN => 96,
            RED => 105,
            YELLOW => 254,
            PURPLE => 27,
            GREY => 173,
            WHITE => 6,
            BLACK => 113,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 229,
            GREEN => 207,
            RED => 136,
            YELLOW => 116,
            PURPLE => 153,
            GREY => 217,
            WHITE => 152,
            BLACK => 247,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 23,
            GREEN => 125,
            RED => 55,
            YELLOW => 228,
            PURPLE => 165,
            GREY => 160,
            WHITE => 94,
            BLACK => 12,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[94] {
            BLUE => 7,
            GREEN => 88,
            RED => 223,
            YELLOW => 50,
            PURPLE => 46,
            GREY => 203,
            WHITE => 255,
            BLACK => 39,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[63] {
            BLUE => 3,
            GREEN => 25,
            RED => 241,
            YELLOW => 252,
            PURPLE => 199,
            GREY => 188,
            WHITE => 129,
            BLACK => 43,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 250,
            GREEN => 184,
            RED => 131,
            YELLOW => 34,
            PURPLE => 199,
            GREY => 40,
            WHITE => 226,
            BLACK => 18,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[68] {
            BLUE => 225,
            GREEN => 53,
            RED => 83,
            YELLOW => 123,
            PURPLE => 118,
            GREY => 120,
            WHITE => 63,
            BLACK => 151,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 209,
            GREEN => 132,
            RED => 40,
            YELLOW => 162,
            PURPLE => 155,
            GREY => 40,
            WHITE => 147,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 218,
            GREEN => 93,
            RED => 61,
            YELLOW => 249,
            PURPLE => 184,
            GREY => 77,
            WHITE => 26,
            BLACK => 152,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 84,
            GREEN => 197,
            RED => 163,
            YELLOW => 230,
            PURPLE => 132,
            GREY => 182,
            WHITE => 2,
            BLACK => 62,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[99] {
            BLUE => 70,
            GREEN => 201,
            RED => 237,
            YELLOW => 243,
            PURPLE => 179,
            GREY => 127,
            WHITE => 124,
            BLACK => 229,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 160,
            GREEN => 240,
            RED => 238,
            YELLOW => 149,
            PURPLE => 97,
            GREY => 3,
            WHITE => 66,
            BLACK => 104,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[61] {
            BLUE => 107,
            GREEN => 36,
            RED => 189,
            YELLOW => 199,
            PURPLE => 180,
            GREY => 85,
            WHITE => 225,
            BLACK => 27,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 1,
            GREEN => 62,
            RED => 235,
            YELLOW => 23,
            PURPLE => 247,
            GREY => 38,
            WHITE => 145,
            BLACK => 198,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 198,
            GREEN => 182,
            RED => 111,
            YELLOW => 153,
            PURPLE => 210,
            GREY => 9,
            WHITE => 95,
            BLACK => 183,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[106] {
            BLUE => 56,
            GREEN => 24,
            RED => 198,
            YELLOW => 49,
            PURPLE => 125,
            GREY => 249,
            WHITE => 158,
            BLACK => 45,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[87] {
            BLUE => 89,
            GREEN => 183,
            RED => 58,
            YELLOW => 72,
            PURPLE => 152,
            GREY => 74,
            WHITE => 224,
            BLACK => 19,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 251,
            GREEN => 52,
            RED => 127,
            YELLOW => 129,
            PURPLE => 217,
            GREY => 27,
            WHITE => 32,
            BLACK => 29,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[79] {
            BLUE => 95,
            GREEN => 34,
            RED => 177,
            YELLOW => 36,
            PURPLE => 58,
            GREY => 254,
            WHITE => 132,
            BLACK => 218,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 93,
            GREEN => 123,
            RED => 98,
            YELLOW => 141,
            PURPLE => 65,
            GREY => 181,
            WHITE => 160,
            BLACK => 67,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 251,
            GREEN => 224,
            RED => 235,
            YELLOW => 207,
            PURPLE => 63,
            GREY => 15,
            WHITE => 195,
            BLACK => 155,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 150,
            GREEN => 237,
            RED => 132,
            YELLOW => 23,
            PURPLE => 179,
            GREY => 23,
            WHITE => 70,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 182,
            GREEN => 253,
            RED => 12,
            YELLOW => 174,
            PURPLE => 241,
            GREY => 143,
            WHITE => 129,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[92] {
            BLUE => 194,
            GREEN => 46,
            RED => 83,
            YELLOW => 12,
            PURPLE => 165,
            GREY => 96,
            WHITE => 242,
            BLACK => 161,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[104] {
            BLUE => 129,
            GREEN => 26,
            RED => 165,
            YELLOW => 106,
            PURPLE => 213,
            GREY => 168,
            WHITE => 84,
            BLACK => 235,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 5,
            GREEN => 50,
            RED => 66,
            YELLOW => 30,
            PURPLE => 233,
            GREY => 107,
            WHITE => 239,
            BLACK => 146,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[89] {
            BLUE => 148,
            GREEN => 245,
            RED => 158,
            YELLOW => 234,
            PURPLE => 43,
            GREY => 123,
            WHITE => 212,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[100] {
            BLUE => 144,
            GREEN => 180,
            RED => 223,
            YELLOW => 46,
            PURPLE => 114,
            GREY => 104,
            WHITE => 157,
            BLACK => 114,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 16,
            GREEN => 164,
            RED => 0,
            YELLOW => 2,
            PURPLE => 177,
            GREY => 208,
            WHITE => 171,
            BLACK => 65,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 230,
            GREEN => 217,
            RED => 26,
            YELLOW => 90,
            PURPLE => 42,
            GREY => 16,
            WHITE => 61,
            BLACK => 138,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 93,
            GREEN => 250,
            RED => 97,
            YELLOW => 141,
            PURPLE => 238,
            GREY => 171,
            WHITE => 234,
            BLACK => 215,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 167,
            GREEN => 77,
            RED => 233,
            YELLOW => 250,
            PURPLE => 218,
            GREY => 37,
            WHITE => 247,
            BLACK => 85,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 198,
            GREEN => 50,
            RED => 44,
            YELLOW => 101,
            PURPLE => 247,
            GREY => 250,
            WHITE => 205,
            BLACK => 9,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[56] {
            BLUE => 79,
            GREEN => 6,
            RED => 20,
            YELLOW => 96,
            PURPLE => 80,
            GREY => 194,
            WHITE => 22,
            BLACK => 215,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 179,
            GREEN => 115,
            RED => 125,
            YELLOW => 21,
            PURPLE => 27,
            GREY => 243,
            WHITE => 204,
            BLACK => 111,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[78] {
            BLUE => 211,
            GREEN => 210,
            RED => 55,
            YELLOW => 171,
            PURPLE => 245,
            GREY => 49,
            WHITE => 76,
            BLACK => 14,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[90] {
            BLUE => 119,
            GREEN => 194,
            RED => 162,
            YELLOW => 70,
            PURPLE => 216,
            GREY => 213,
            WHITE => 196,
            BLACK => 188,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 103,
            GREEN => 89,
            RED => 188,
            YELLOW => 111,
            PURPLE => 42,
            GREY => 48,
            WHITE => 109,
            BLACK => 92,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 134,
            GREEN => 140,
            RED => 225,
            YELLOW => 190,
            PURPLE => 110,
            GREY => 208,
            WHITE => 26,
            BLACK => 217,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[108] {
            BLUE => 87,
            GREEN => 100,
            RED => 84,
            YELLOW => 221,
            PURPLE => 207,
            GREY => 172,
            WHITE => 190,
            BLACK => 115,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 250,
            GREEN => 245,
            RED => 78,
            YELLOW => 126,
            PURPLE => 56,
            GREY => 165,
            WHITE => 93,
            BLACK => 11,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[86] {
            BLUE => 48,
            GREEN => 170,
            RED => 28,
            YELLOW => 243,
            PURPLE => 117,
            GREY => 185,
            WHITE => 44,
            BLACK => 227,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[97] {
            BLUE => 152,
            GREEN => 195,
            RED => 82,
            YELLOW => 221,
            PURPLE => 221,
            GREY => 122,
            WHITE => 216,
            BLACK => 250,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[5] {
            BLUE => 179,
            GREEN => 193,
            RED => 69,
            YELLOW => 7,
            PURPLE => 90,
            GREY => 187,
            WHITE => 138,
            BLACK => 159,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[55] {
            BLUE => 203,
            GREEN => 38,
            RED => 39,
            YELLOW => 153,
            PURPLE => 10,
            GREY => 99,
            WHITE => 98,
            BLACK => 149,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[66] {
            BLUE => 186,
            GREEN => 0,
            RED => 247,
            YELLOW => 93,
            PURPLE => 228,
            GREY => 21,
            WHITE => 36,
            BLACK => 250,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[81] {
            BLUE => 52,
            GREEN => 216,
            RED => 202,
            YELLOW => 182,
            PURPLE => 16,
            GREY => 108,
            WHITE => 9,
            BLACK => 255,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 8,
            GREEN => 11,
            RED => 74,
            YELLOW => 176,
            PURPLE => 185,
            GREY => 194,
            WHITE => 148,
            BLACK => 41,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 209,
            GREEN => 137,
            RED => 116,
            YELLOW => 62,
            PURPLE => 153,
            GREY => 173,
            WHITE => 15,
            BLACK => 61,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 199,
            GREEN => 68,
            RED => 237,
            YELLOW => 207,
            PURPLE => 31,
            GREY => 187,
            WHITE => 101,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[107] {
            BLUE => 196,
            GREEN => 216,
            RED => 147,
            YELLOW => 75,
            PURPLE => 20,
            GREY => 38,
            WHITE => 204,
            BLACK => 195,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[93] {
            BLUE => 74,
            GREEN => 41,
            RED => 16,
            YELLOW => 183,
            PURPLE => 235,
            GREY => 12,
            WHITE => 160,
            BLACK => 83,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[111] {
            BLUE => 197,
            GREEN => 168,
            RED => 238,
            YELLOW => 42,
            PURPLE => 144,
            GREY => 93,
            WHITE => 37,
            BLACK => 139,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[57] {
            BLUE => 243,
            GREEN => 79,
            RED => 28,
            YELLOW => 158,
            PURPLE => 71,
            GREY => 122,
            WHITE => 219,
            BLACK => 100,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[75] {
            BLUE => 62,
            GREEN => 253,
            RED => 9,
            YELLOW => 103,
            PURPLE => 83,
            GREY => 124,
            WHITE => 75,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[72] {
            BLUE => 239,
            GREEN => 48,
            RED => 17,
            YELLOW => 198,
            PURPLE => 210,
            GREY => 182,
            WHITE => 82,
            BLACK => 48,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[85] {
            BLUE => 90,
            GREEN => 208,
            RED => 147,
            YELLOW => 37,
            PURPLE => 52,
            GREY => 56,
            WHITE => 106,
            BLACK => 101,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 221,
            GREEN => 162,
            RED => 47,
            YELLOW => 17,
            PURPLE => 156,
            GREY => 151,
            WHITE => 12,
            BLACK => 225,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[91] {
            BLUE => 196,
            GREEN => 75,
            RED => 243,
            YELLOW => 118,
            PURPLE => 169,
            GREY => 213,
            WHITE => 193,
            BLACK => 51,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 254,
            GREEN => 96,
            RED => 4,
            YELLOW => 181,
            PURPLE => 8,
            GREY => 35,
            WHITE => 202,
            BLACK => 97,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[101] {
            BLUE => 36,
            GREEN => 97,
            RED => 125,
            YELLOW => 225,
            PURPLE => 93,
            GREY => 221,
            WHITE => 247,
            BLACK => 125,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[84] {
            BLUE => 193,
            GREEN => 74,
            RED => 26,
            YELLOW => 111,
            PURPLE => 16,
            GREY => 80,
            WHITE => 235,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[102] {
            BLUE => 253,
            GREEN => 173,
            RED => 252,
            YELLOW => 240,
            PURPLE => 244,
            GREY => 61,
            WHITE => 126,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 13,
            GREEN => 160,
            RED => 57,
            YELLOW => 179,
            PURPLE => 104,
            GREY => 112,
            WHITE => 66,
            BLACK => 23,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 155,
            GREEN => 194,
            RED => 42,
            YELLOW => 19,
            PURPLE => 175,
            GREY => 164,
            WHITE => 241,
            BLACK => 139,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 92,
            GREEN => 130,
            RED => 72,
            YELLOW => 234,
            PURPLE => 23,
            GREY => 203,
            WHITE => 208,
            BLACK => 208,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[82] {
            BLUE => 96,
            GREEN => 245,
            RED => 135,
            YELLOW => 40,
            PURPLE => 46,
            GREY => 73,
            WHITE => 233,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 171,
            GREEN => 81,
            RED => 208,
            YELLOW => 155,
            PURPLE => 197,
            GREY => 80,
            WHITE => 155,
            BLACK => 234,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 232,
            GREEN => 23,
            RED => 92,
            YELLOW => 220,
            PURPLE => 123,
            GREY => 229,
            WHITE => 14,
            BLACK => 144,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[80] {
            BLUE => 115,
            GREEN => 245,
            RED => 196,
            YELLOW => 40,
            PURPLE => 201,
            GREY => 235,
            WHITE => 237,
            BLACK => 132,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[103] {
            BLUE => 28,
            GREEN => 121,
            RED => 182,
            YELLOW => 154,
            PURPLE => 160,
            GREY => 208,
            WHITE => 205,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[25] {
            BLUE => 164,
            GREEN => 195,
            RED => 104,
            YELLOW => 10,
            PURPLE => 129,
            GREY => 158,
            WHITE => 87,
            BLACK => 147,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[59] {
            BLUE => 92,
            GREEN => 124,
            RED => 226,
            YELLOW => 64,
            PURPLE => 32,
            GREY => 205,
            WHITE => 42,
            BLACK => 96,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 114,
            GREEN => 84,
            RED => 68,
            YELLOW => 34,
            PURPLE => 128,
            GREY => 156,
            WHITE => 141,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[60] {
            BLUE => 119,
            GREEN => 253,
            RED => 177,
            YELLOW => 64,
            PURPLE => 46,
            GREY => 210,
            WHITE => 79,
            BLACK => 198,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 207,
            GREEN => 124,
            RED => 169,
            YELLOW => 219,
            PURPLE => 82,
            GREY => 243,
            WHITE => 104,
            BLACK => 62,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 243,
            GREEN => 251,
            RED => 227,
            YELLOW => 126,
            PURPLE => 22,
            GREY => 247,
            WHITE => 38,
            BLACK => 150,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 247,
            GREEN => 197,
            RED => 33,
            YELLOW => 54,
            PURPLE => 137,
            GREY => 192,
            WHITE => 64,
            BLACK => 81,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 53,
            GREEN => 2,
            RED => 44,
            YELLOW => 238,
            PURPLE => 117,
            GREY => 91,
            WHITE => 75,
            BLACK => 214,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[109] {
            BLUE => 61,
            GREEN => 172,
            RED => 107,
            YELLOW => 158,
            PURPLE => 52,
            GREY => 187,
            WHITE => 8,
            BLACK => 213,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 214,
            GREEN => 69,
            RED => 204,
            YELLOW => 75,
            PURPLE => 131,
            GREY => 176,
            WHITE => 237,
            BLACK => 144,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[95] {
            BLUE => 158,
            GREEN => 114,
            RED => 91,
            YELLOW => 31,
            PURPLE => 75,
            GREY => 161,
            WHITE => 56,
            BLACK => 239,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[77] {
            BLUE => 199,
            GREEN => 213,
            RED => 228,
            YELLOW => 241,
            PURPLE => 208,
            GREY => 56,
            WHITE => 9,
            BLACK => 230,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[58] {
            BLUE => 172,
            GREEN => 235,
            RED => 254,
            YELLOW => 139,
            PURPLE => 239,
            GREY => 95,
            WHITE => 165,
            BLACK => 12,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[62] {
            BLUE => 237,
            GREEN => 54,
            RED => 156,
            YELLOW => 238,
            PURPLE => 250,
            GREY => 225,
            WHITE => 65,
            BLACK => 170,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[105] {
            BLUE => 247,
            GREEN => 250,
            RED => 169,
            YELLOW => 247,
            PURPLE => 11,
            GREY => 242,
            WHITE => 46,
            BLACK => 206,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 225,
            GREEN => 165,
            RED => 245,
            YELLOW => 2,
            PURPLE => 41,
            GREY => 74,
            WHITE => 45,
            BLACK => 168,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[69] {
            BLUE => 193,
            GREEN => 218,
            RED => 155,
            YELLOW => 53,
            PURPLE => 210,
            GREY => 181,
            WHITE => 189,
            BLACK => 167,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 108,
            GREEN => 155,
            RED => 25,
            YELLOW => 230,
            PURPLE => 157,
            GREY => 56,
            WHITE => 187,
            BLACK => 56,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 165,
            GREEN => 17,
            RED => 127,
            YELLOW => 255,
            PURPLE => 234,
            GREY => 33,
            WHITE => 26,
            BLACK => 139,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[74] {
            BLUE => 46,
            GREEN => 165,
            RED => 137,
            YELLOW => 96,
            PURPLE => 196,
            GREY => 30,
            WHITE => 61,
            BLACK => 225,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[67] {
            BLUE => 150,
            GREEN => 21,
            RED => 14,
            YELLOW => 138,
            PURPLE => 91,
            GREY => 110,
            WHITE => 35,
            BLACK => 119,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[65] {
            BLUE => 24,
            GREEN => 195,
            RED => 176,
            YELLOW => 63,
            PURPLE => 128,
            GREY => 239,
            WHITE => 13,
            BLACK => 249,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 107,
            GREEN => 154,
            RED => 153,
            YELLOW => 42,
            PURPLE => 107,
            GREY => 225,
            WHITE => 163,
            BLACK => 35,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[64] {
            BLUE => 77,
            GREEN => 200,
            RED => 137,
            YELLOW => 2,
            PURPLE => 173,
            GREY => 118,
            WHITE => 104,
            BLACK => 194,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[76] {
            BLUE => 150,
            GREEN => 18,
            RED => 60,
            YELLOW => 239,
            PURPLE => 144,
            GREY => 134,
            WHITE => 196,
            BLACK => 15,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[110] {
            BLUE => 185,
            GREEN => 14,
            RED => 35,
            YELLOW => 97,
            PURPLE => 116,
            GREY => 220,
            WHITE => 47,
            BLACK => 190,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 75,
            GREEN => 205,
            RED => 75,
            YELLOW => 134,
            PURPLE => 127,
            GREY => 21,
            WHITE => 77,
            BLACK => 114,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 184,
            GREEN => 109,
            RED => 210,
            YELLOW => 199,
            PURPLE => 4,
            GREY => 250,
            WHITE => 61,
            BLACK => 32,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 84,
            GREEN => 166,
            RED => 20,
            YELLOW => 145,
            PURPLE => 173,
            GREY => 179,
            WHITE => 88,
            BLACK => 108,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[83] {
            BLUE => 68,
            GREEN => 7,
            RED => 16,
            YELLOW => 31,
            PURPLE => 59,
            GREY => 88,
            WHITE => 217,
            BLACK => 85,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[98] {
            BLUE => 245,
            GREEN => 169,
            RED => 137,
            YELLOW => 45,
            PURPLE => 182,
            GREY => 156,
            WHITE => 109,
            BLACK => 207,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[96] {
            BLUE => 46,
            GREEN => 178,
            RED => 134,
            YELLOW => 88,
            PURPLE => 63,
            GREY => 122,
            WHITE => 216,
            BLACK => 59,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 238,
            GREEN => 150,
            RED => 75,
            YELLOW => 251,
            PURPLE => 108,
            GREY => 190,
            WHITE => 34,
            BLACK => 130,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 4,
            GREEN => 75,
            RED => 175,
            YELLOW => 156,
            PURPLE => 147,
            GREY => 189,
            WHITE => 139,
            BLACK => 13,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[70] {
            BLUE => 65,
            GREEN => 33,
            RED => 164,
            YELLOW => 57,
            PURPLE => 248,
            GREY => 150,
            WHITE => 103,
            BLACK => 255,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}else {
	let n = match buffer[5] {
            BLUE => 58,
            GREEN => 108,
            RED => 168,
            YELLOW => 42,
            PURPLE => 248,
            GREY => 68,
            WHITE => 27,
            BLACK => 169,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[10] {
            BLUE => 140,
            GREEN => 114,
            RED => 251,
            YELLOW => 45,
            PURPLE => 8,
            GREY => 203,
            WHITE => 121,
            BLACK => 148,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[22] {
            BLUE => 56,
            GREEN => 137,
            RED => 125,
            YELLOW => 205,
            PURPLE => 168,
            GREY => 21,
            WHITE => 164,
            BLACK => 126,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[14] {
            BLUE => 102,
            GREEN => 78,
            RED => 120,
            YELLOW => 118,
            PURPLE => 65,
            GREY => 213,
            WHITE => 45,
            BLACK => 142,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[26] {
            BLUE => 93,
            GREEN => 254,
            RED => 46,
            YELLOW => 99,
            PURPLE => 103,
            GREY => 26,
            WHITE => 234,
            BLACK => 88,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[23] {
            BLUE => 7,
            GREEN => 66,
            RED => 27,
            YELLOW => 115,
            PURPLE => 214,
            GREY => 226,
            WHITE => 45,
            BLACK => 187,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[31] {
            BLUE => 139,
            GREEN => 97,
            RED => 205,
            YELLOW => 202,
            PURPLE => 140,
            GREY => 237,
            WHITE => 186,
            BLACK => 110,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[13] {
            BLUE => 133,
            GREEN => 38,
            RED => 97,
            YELLOW => 167,
            PURPLE => 253,
            GREY => 47,
            WHITE => 73,
            BLACK => 250,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[6] {
            BLUE => 70,
            GREEN => 155,
            RED => 176,
            YELLOW => 193,
            PURPLE => 179,
            GREY => 222,
            WHITE => 162,
            BLACK => 144,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[16] {
            BLUE => 176,
            GREEN => 177,
            RED => 25,
            YELLOW => 53,
            PURPLE => 213,
            GREY => 236,
            WHITE => 162,
            BLACK => 63,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[11] {
            BLUE => 84,
            GREEN => 29,
            RED => 62,
            YELLOW => 218,
            PURPLE => 64,
            GREY => 237,
            WHITE => 76,
            BLACK => 238,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[46] {
            BLUE => 13,
            GREEN => 228,
            RED => 219,
            YELLOW => 64,
            PURPLE => 170,
            GREY => 145,
            WHITE => 123,
            BLACK => 175,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[9] {
            BLUE => 62,
            GREEN => 43,
            RED => 62,
            YELLOW => 169,
            PURPLE => 7,
            GREY => 88,
            WHITE => 12,
            BLACK => 10,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[3] {
            BLUE => 188,
            GREEN => 35,
            RED => 141,
            YELLOW => 87,
            PURPLE => 65,
            GREY => 94,
            WHITE => 85,
            BLACK => 71,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[29] {
            BLUE => 169,
            GREEN => 197,
            RED => 105,
            YELLOW => 192,
            PURPLE => 143,
            GREY => 226,
            WHITE => 198,
            BLACK => 224,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[28] {
            BLUE => 132,
            GREEN => 54,
            RED => 92,
            YELLOW => 185,
            PURPLE => 235,
            GREY => 212,
            WHITE => 126,
            BLACK => 183,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[32] {
            BLUE => 254,
            GREEN => 221,
            RED => 142,
            YELLOW => 97,
            PURPLE => 60,
            GREY => 94,
            WHITE => 24,
            BLACK => 201,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[0] {
            BLUE => 8,
            GREEN => 2,
            RED => 176,
            YELLOW => 19,
            PURPLE => 33,
            GREY => 209,
            WHITE => 150,
            BLACK => 49,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[19] {
            BLUE => 1,
            GREEN => 70,
            RED => 123,
            YELLOW => 153,
            PURPLE => 221,
            GREY => 142,
            WHITE => 70,
            BLACK => 204,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[38] {
            BLUE => 190,
            GREEN => 9,
            RED => 124,
            YELLOW => 218,
            PURPLE => 44,
            GREY => 9,
            WHITE => 227,
            BLACK => 77,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[35] {
            BLUE => 110,
            GREEN => 79,
            RED => 73,
            YELLOW => 153,
            PURPLE => 77,
            GREY => 228,
            WHITE => 212,
            BLACK => 139,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[24] {
            BLUE => 42,
            GREEN => 74,
            RED => 226,
            YELLOW => 35,
            PURPLE => 175,
            GREY => 153,
            WHITE => 223,
            BLACK => 242,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[27] {
            BLUE => 132,
            GREEN => 120,
            RED => 177,
            YELLOW => 217,
            PURPLE => 255,
            GREY => 158,
            WHITE => 163,
            BLACK => 193,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[34] {
            BLUE => 176,
            GREEN => 43,
            RED => 109,
            YELLOW => 44,
            PURPLE => 171,
            GREY => 78,
            WHITE => 105,
            BLACK => 225,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[21] {
            BLUE => 209,
            GREEN => 10,
            RED => 63,
            YELLOW => 191,
            PURPLE => 11,
            GREY => 184,
            WHITE => 105,
            BLACK => 143,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[44] {
            BLUE => 114,
            GREEN => 243,
            RED => 169,
            YELLOW => 171,
            PURPLE => 71,
            GREY => 184,
            WHITE => 117,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[20] {
            BLUE => 138,
            GREEN => 202,
            RED => 173,
            YELLOW => 76,
            PURPLE => 36,
            GREY => 96,
            WHITE => 167,
            BLACK => 44,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[33] {
            BLUE => 125,
            GREEN => 207,
            RED => 110,
            YELLOW => 131,
            PURPLE => 49,
            GREY => 129,
            WHITE => 240,
            BLACK => 41,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[17] {
            BLUE => 152,
            GREEN => 237,
            RED => 101,
            YELLOW => 12,
            PURPLE => 19,
            GREY => 84,
            WHITE => 211,
            BLACK => 130,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[52] {
            BLUE => 3,
            GREEN => 114,
            RED => 55,
            YELLOW => 150,
            PURPLE => 150,
            GREY => 226,
            WHITE => 84,
            BLACK => 249,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[45] {
            BLUE => 251,
            GREEN => 156,
            RED => 222,
            YELLOW => 220,
            PURPLE => 231,
            GREY => 69,
            WHITE => 157,
            BLACK => 55,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[54] {
            BLUE => 24,
            GREEN => 251,
            RED => 215,
            YELLOW => 82,
            PURPLE => 84,
            GREY => 54,
            WHITE => 48,
            BLACK => 15,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[49] {
            BLUE => 31,
            GREEN => 192,
            RED => 238,
            YELLOW => 75,
            PURPLE => 83,
            GREY => 233,
            WHITE => 187,
            BLACK => 3,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[39] {
            BLUE => 91,
            GREEN => 249,
            RED => 43,
            YELLOW => 105,
            PURPLE => 100,
            GREY => 207,
            WHITE => 41,
            BLACK => 13,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[41] {
            BLUE => 18,
            GREEN => 58,
            RED => 109,
            YELLOW => 179,
            PURPLE => 57,
            GREY => 78,
            WHITE => 198,
            BLACK => 192,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[51] {
            BLUE => 39,
            GREEN => 147,
            RED => 65,
            YELLOW => 25,
            PURPLE => 169,
            GREY => 247,
            WHITE => 213,
            BLACK => 177,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[53] {
            BLUE => 119,
            GREEN => 127,
            RED => 51,
            YELLOW => 191,
            PURPLE => 249,
            GREY => 64,
            WHITE => 139,
            BLACK => 86,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[1] {
            BLUE => 127,
            GREEN => 186,
            RED => 207,
            YELLOW => 241,
            PURPLE => 119,
            GREY => 126,
            WHITE => 53,
            BLACK => 44,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[12] {
            BLUE => 60,
            GREEN => 209,
            RED => 96,
            YELLOW => 176,
            PURPLE => 10,
            GREY => 18,
            WHITE => 250,
            BLACK => 103,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[18] {
            BLUE => 89,
            GREEN => 181,
            RED => 107,
            YELLOW => 125,
            PURPLE => 75,
            GREY => 232,
            WHITE => 211,
            BLACK => 58,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[8] {
            BLUE => 56,
            GREEN => 239,
            RED => 110,
            YELLOW => 193,
            PURPLE => 140,
            GREY => 212,
            WHITE => 9,
            BLACK => 1,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[30] {
            BLUE => 214,
            GREEN => 77,
            RED => 135,
            YELLOW => 159,
            PURPLE => 27,
            GREY => 59,
            WHITE => 179,
            BLACK => 188,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[36] {
            BLUE => 161,
            GREEN => 11,
            RED => 1,
            YELLOW => 207,
            PURPLE => 102,
            GREY => 120,
            WHITE => 96,
            BLACK => 192,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[42] {
            BLUE => 232,
            GREEN => 129,
            RED => 146,
            YELLOW => 27,
            PURPLE => 147,
            GREY => 154,
            WHITE => 129,
            BLACK => 54,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[50] {
            BLUE => 108,
            GREEN => 101,
            RED => 77,
            YELLOW => 5,
            PURPLE => 169,
            GREY => 144,
            WHITE => 50,
            BLACK => 191,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[7] {
            BLUE => 52,
            GREEN => 246,
            RED => 115,
            YELLOW => 225,
            PURPLE => 107,
            GREY => 7,
            WHITE => 70,
            BLACK => 59,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[43] {
            BLUE => 73,
            GREEN => 199,
            RED => 124,
            YELLOW => 111,
            PURPLE => 56,
            GREY => 39,
            WHITE => 205,
            BLACK => 25,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[4] {
            BLUE => 242,
            GREEN => 45,
            RED => 171,
            YELLOW => 76,
            PURPLE => 43,
            GREY => 73,
            WHITE => 198,
            BLACK => 112,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[40] {
            BLUE => 240,
            GREEN => 129,
            RED => 124,
            YELLOW => 187,
            PURPLE => 139,
            GREY => 207,
            WHITE => 227,
            BLACK => 8,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[2] {
            BLUE => 189,
            GREEN => 235,
            RED => 3,
            YELLOW => 103,
            PURPLE => 248,
            GREY => 247,
            WHITE => 125,
            BLACK => 40,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[48] {
            BLUE => 67,
            GREEN => 147,
            RED => 248,
            YELLOW => 10,
            PURPLE => 43,
            GREY => 144,
            WHITE => 233,
            BLACK => 240,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[47] {
            BLUE => 126,
            GREEN => 37,
            RED => 134,
            YELLOW => 13,
            PURPLE => 120,
            GREY => 158,
            WHITE => 227,
            BLACK => 40,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[15] {
            BLUE => 255,
            GREEN => 9,
            RED => 167,
            YELLOW => 101,
            PURPLE => 196,
            GREY => 111,
            WHITE => 6,
            BLACK => 115,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        
let n = match buffer[37] {
            BLUE => 253,
            GREEN => 64,
            RED => 152,
            YELLOW => 169,
            PURPLE => 138,
            GREY => 58,
            WHITE => 222,
            BLACK => 249,
            _ => 0
        };

        add_n_to_buffer(&mut buffer[0..SCREEN_WIDTH], n);
        

}

    }

    for y in 1..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            buffer[y * SCREEN_WIDTH + x] = buffer[x];
        }
    }
}
