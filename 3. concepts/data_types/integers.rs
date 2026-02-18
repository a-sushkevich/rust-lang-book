/*  Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive,
where n is the number of bits that variant uses. So, an i8 can store numbers
from −(27) to 27 − 1, which equals −128 to 127. Unsigned variants can store
numbers from 0 to 2n − 1, so a u8 can store numbers from 0 to 28 − 1, which
equals 0 to 255 */

fn main() {
    let _x: i8 = 1;
    let _y: i16 = 16;
    let _z: i32 = 32; // Default to use in Rust
    let _a: i64 = 64;
    let _b: i128 = 128;
    let _c: isize = 42; // Architecture-dependent

    /* Additionally, the isize and usize types depend on the architecture of
    the computer your program is running on: 64 bits if you’re on a 64-bit architecture
    and 32 bits if you’re on a 32-bit architecture. */

    let _d: u8 = 1;
    let _e: u16 = 16;
    let _f: u32 = 32;
    let _g: u64 = 64;
    let _h: u128 = 128;
    let _i: usize = 42; // Architecture-dependent
}

/*
    Decimal	98_222
    Hex	0xff
    Octal	0o77
    Binary	0b1111_0000
    Byte (u8 only)	b'A'
*/
