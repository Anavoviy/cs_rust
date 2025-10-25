mod bit_operations;
mod arrays;

fn main() {
    println!("Hello, world!");

    //bit_operations::num_to_different_number_systems(15);
    bit_operations::present_bit_opoerations(15, 7);

    let number_systems = bit_operations::convert_number(15);
    println!(
        "Число {} (DEC) -> {} (HEX), {} (OCT), {} (BIN)",
        number_systems.dec,
        number_systems.hex,
        number_systems.oct,
        number_systems.bin
    );
    let x = 32;
    println!("Является ли число {x} степенью двойки -> {}", bit_operations::is_power_of_two(x));

    let a = 15;
    let b = 9;
    println!("Числа {a} ({a:b}) и {b} ({b:b}) имеют {} различный(ых) бит",
             bit_operations::count_different_bits(a, b))

}
