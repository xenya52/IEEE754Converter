use std::{io, env};

/*
From decimal to IEEE754
*/
fn one_serialize(decimal: f32) {
    let numeric_input = decimal;
    let mut pre_decimal_place: i32 = decimal as i32;
    let mut decimal_place: f32 = decimal - pre_decimal_place as f32;
    let mut ieee_array = [0;32]; //mantissa included
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
    println!("{:?}", ieee_array)
}

fn ieee754_to_decimal(binary: u32) -> f32 {
    // Reinterpretiere die binäre Darstellung als Gleitkommazahl
    let float_value = unsafe { std::mem::transmute::<u32, f32>(binary) };
    float_value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let param1 = &args[1];
    let param2 = &args[2];

    loop {
        let number_input: u32 = match param2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input in loop");
                continue;
            }
        };
        let option:Vec<_> = param1.chars().collect();
        match option[0] {
            's' => {
                println!("Type you float number");
                //Take userInput as a string
                println!("<=Serialize=>");
                println!("Type number in");
                let mut user_input =  String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Fail to read from stdin");
                //Converting the input to float
                let number: f32 = match user_input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input");
                        return;
                    }
                };
                one_serialize(number);
                break;
            }
            'd' => {
                //Take userInput as a string
                println!("<=Deserialize=>");
                println!("Type you IEEE754 standartisted number");
                let binary_value: u32 = 0b1000001011100100000000000000000;
                let decimal_value = ieee754_to_decimal(binary_value);
                println!("IEEE 754 Binär: {:032b}", binary_value);
                println!("Dezimal: {}", decimal_value);
                break;
            }
            _ => println!("Type: [execution] -s (serialize) || -d (desirialised) [value]")
        }
    }
}