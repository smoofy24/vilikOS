use heapless::String;
use crate::printk;

pub fn test() {
    let int8: i8 = -8;
    printk!("{:04}", int8);

    let int16: i16 = -16;
    printk!("{}", int16);

    let int32: i32 = -32;
    printk!("{}", int32);

    let int64: i64 = -64;
    printk!("{}", int64);

    let int128: i128 = -128;
    printk!("{}", int128);

    let int: isize = -1;
    printk!("{}", int);

    let uint8: u8 = 8;
    printk!("{}", uint8);

    let uint16: u16 = 16;
    printk!("{}", uint16);

    let uint32: u32 = 32;
    printk!("{}", uint32);

    let uint64: u64 = 64;
    printk!("{}", uint64);

    let uint128: u128 = 128;
    printk!("{}", uint128);

    let uint: usize = 1;
    printk!("{}", uint);

    let float: f32 = 3.141;
    printk!("{}", float);

    let double: f64 = -6.282;
    printk!("{}", double);

    let t: bool = true;
    printk!("{}", t);

    let f: bool = false;
    printk!("{}", f);

    let z: char = 'ℤ';
    printk!("{}", z);

    let crab = "\u{1F980}";
    printk!("{}", crab);

    let string = String::<64>::try_from("From kernel with love!").unwrap();
    printk!("{}", string);

    let months = ["January", "February", "March"];
    printk!("{:?}", months);

    let tupple: (i32, f64, u8) = (500, 6.4, 1);
    printk!("{:?}", tupple);
}
