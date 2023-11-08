use core::num;
use std::{io, array};

/*
From decimal to IEEE754
*/
fn one_serialize() {
    //Take userInput as a string
    println!("<=Serialize=>");
    println!("Type number in");
    let mut user_input =  String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Fail to read from stdin");
    
    //dereverence the string
    let numeric_input = user_input.trim();
    match numeric_input.parse::<f32>() {
        Ok(decimal) => {
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
            if (pre_decimal_place != 0) {
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
            if numeric_input != "0" {
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
            
            
            println!("{:?}", ieee_array);
        }
        Err(..) => println!("Not a number: {}", numeric_input),
    };
}

fn two_deserialize() {
    println!("<=Deserialize=>");
    println!("Type number in");
    let mut user_input =  String::new();
    let mut i:usize = 0; //index
    let mut counter:usize = 0; //calculation index
    let mut exponent: u8 = 0;
    let base: u8 = 2;
    let mut solution:f32 = 0.0;
    io::stdin()
        .read_line(&mut user_input)
        .expect("Fail to read from stdin");
    
    let mut binary_array: [u8;32] = [0;32];

    for c in user_input.chars() {
        if c == '1' {
            binary_array[i] = 1;
        }
        i += 1;
    }
    //Calculate the characteristic
    i = 8;
    while i != 0 {
        exponent += binary_array[i] * base.pow(counter as u32);
        counter += 1;
        i -= 1;
    }
    exponent -= 127;
    //Calculate the pre decimal place
    i = exponent as usize + 8;
    counter = 0;
    while i != 8 {
        solution += binary_array[i] as f32 * base.pow(counter as u32) as f32;
        i -= 1;
        counter += 1;
    }
    solution += 1.0 * base.pow(counter as u32) as f32;
    //Calculate the decimal place
    i = 31;
    counter = 0;
    while i != exponent as usize + 8 {
        solution += binary_array[i] as f32 * base.pow(counter as u32) as f32;
        i -= 1;
        counter += 1;
    }


    println!("{}", solution);
    println!("{}", exponent);
    println!("{:?}", binary_array);
}
fn three_explanation() {
    println!("The IEEE 754 is a standard for representing floating-point numbers (floating-point numbers) in the binary system in computers.
It defines the representation of real numbers by sign, exponent and mantissa.");
}

fn main_menu() {
    loop {
        println!("<- Menu ->");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Fehler beim Einlesen");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ungültige Eingabe. Bitte wähle eine der verfügbaren Optionen.");
                continue;
            }
        };
        match choice {
            1 => {
                one_serialize();
            }
            2 => {
                two_deserialize();
            }   
            3 => {
                println!("<=Explanation=>");
                three_explanation();
            }
            4 => {
                four_help();
            }
            5 => {
                break;
            }
            _ => println!("Wrong input!")
        }
    }
}

fn four_help() {
    print!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("<=IEEE.754 Converter=>");
    println!("<=Choose a operation=>");
    println!("<-------------------->");
    println!("<-1.Serialize to 754->");
    println!("<-2.Deserialize     ->");
    println!("<-3.Explanation     ->");
    println!("<-4.Help            ->");
    println!("<-5.Quit            ->");
    println!("<-------------------->");
}
fn main() {
        four_help();
        main_menu();
}