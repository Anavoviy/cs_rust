//Для удобства
pub struct NumberSystems {
    pub dec: String,
    pub hex: String,
    pub oct: String,
    pub bin: String
}

pub fn num_to_different_number_systems(num: i32){
    println!(
        "Число {} (DEC) в разных системах счисления: {:b} (BIN), {:o} (OCT), {:x} (HEX) \n",
        num,
        num,
        num,
        num
    )
}

pub fn present_bit_opoerations(x: i32, y: i32){
    println!("Битовые и побитовые операции:");

    let res_and: i32 = x & y;
    println!("AND: {x} ({x:b}) & {y} ({y:b}) = {res_and} ({res_and:b})");

    let res_or: i32 = x | y;
    println!("OR: {x} ({x:b}) | {y} ({y:b}) = {res_or} ({res_or:b})");

    let res_xor: i32 = x ^ y;
    println!("XOR: {x} ({x:b}) ^ {y} ({y:b}) = {res_xor} ({res_xor:b})");

    let res_not: i32 = !x;
    println!("NOT: !{x} ({x:b}) = {res_not} ({res_not:b})");

    let res_shift_left: i32 = x << 2;
    println!("Сдвиг влево: {x} ({x:b}) << 2 = {res_shift_left} ({res_shift_left:b})");

    let res_shift_right: i32 = x >> 2;
    println!("Сдвиг вправо: {x} ({x:b}) >> 2 = {res_shift_right} ({res_shift_right:b}) \n");
}

//HOMEWORK
pub fn convert_number(num: i32) -> NumberSystems {
    NumberSystems {
        dec: format!("{num}"),
        hex: format!("{num:x}"),
        oct: format!("{num:o}"),
        bin: format!("{num:b}"),
    }
}
pub fn convert_number2(num: i32) -> (String, String, String, String) {
    (format!("{num}"),
     format!("{num:x}"),
     format!("{num:o}"),
     format!("{num:b}"))
}
pub fn is_power_of_two(num: i32) -> bool {
    num > 0 && (num & (num - 1)) == 0
}

pub fn count_different_bits(a: i32, b: i32) -> i32 {
    let mut temp = a ^ b;
    let mut count = 0;
    while temp != 0 {
        temp &= temp - 1;
        count += 1;
    }

    count
}
