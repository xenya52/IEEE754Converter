use std::{env};

/*
From decimal to IEEE754
*/
fn decimal_to_ieee754(input: &String) ->Result <[u32;32], &'static str> {
    let decimal:f32 = input.parse().expect("Please type right datatype for serialsation");
    let numeric_input = decimal;
    let mut pre_decimal_place: i32 = decimal as i32;
    let mut decimal_place: f32 = decimal - pre_decimal_place as f32;
    let mut ieee_array:[u32;32] = [0;32]; //mantissa included
    let mut temp: u8 = 0; //get used for expoent and characteristic
    let mut i: usize = 0;

    //negative or positive?
    if pre_decimal_place < 0 {
        ieee_array[i] = 1;
    }
    i += 9;
    //conversion of the pre decimal place
    if pre_decimal_place != 0 {
        pre_decimal_place = pre_decimal_place / 2; //Ignore the first mantissa index
        while pre_decimal_place != 0 {
            if pre_decimal_place % 2 != 0 { // We have a residual
                ieee_array[i] = 1;
            }
            else { // We dont have a residual
                ieee_array[i] = 0;
            }
            pre_decimal_place = pre_decimal_place / 2;
            i += 1;
            temp += 1;
        }
    }
    if decimal_place != 0.0 {
        //conversion of the decimal place
        let mut loop_check: bool = true;
        while loop_check {
            decimal_place = decimal_place * 2.0;
            if decimal_place == 1.0 {
                loop_check = false;
            }
            if decimal_place * 2.0 > 1.0 {
                ieee_array[i] = 1;
            }
            else {
                ieee_array[i] = 0;
            }
            i += 1;
        }
    }
    //characteristic
    i = 8;
    if numeric_input != 0.0 {
        temp = temp + 127;
        while temp != 0 {
            if temp % 2 != 0 { // We have a residual
                ieee_array[i] = 1;
            }
            else { // We dont have a residual
                ieee_array[i] = 0;
            }
            temp = temp / 2;
            i -= 1;
        }
    }
    Ok(ieee_array)
}

fn ieee754_binary_to_float(binary: &str) -> Result<f32, &'static str> {
    // Ensure the binary string has 32 bits
    if binary.len() != 32 {
        return Err("Binary representation must be 32 bits long");
    }

    // Parse the binary string as u32
    let parsed_u32 = u32::from_str_radix(binary, 2).map_err(|_| "Invalid binary representation")?;

    // Reinterpret the binary representation as a floating-point number
    let float_value = f32::from_bits(parsed_u32);

    Ok(float_value)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let param1 = &args[1];
    let param2 = &args[2];

    loop {
        let option:Vec<_> = param1.chars().collect();
        match option[0] {
            's' => {
                println!("<=Serialize=>");
                let solution = decimal_to_ieee754(&param2);
                println!("Decimal: {:?}", solution);
                break;
            }
            'd' => {
                println!("<=Deserialize=>");
                let solution = ieee754_binary_to_float(&param2);
                println!("Float: {:?}", solution);
                break;
            }
            _ => {
                println!("Type: [execution] -s (serialize) || -d (desirialised) [value]");
                break;
            }
        }
    }
}