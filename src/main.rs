use std::io;

//From decimal to IEEE754
fn one_serialize() {
    //Take userInput as a string
    println!("<=Serialize=>");
    println!("Type number in");
    let mut userInput =  String::new();
    io::stdin()
        .read_line(&mut userInput)
        .expect("Fail to read from stdin");
    
    //dereverence the string
    let numericInput = userInput.trim();
    match numericInput.parse::<f32>() {
        Ok(givenFloat) => {
            let mut positiveInput: u32 = givenFloat as u32;
            let mut tempArray = [0;32];
            let mut index:usize = 31;
            loop {
                //If the calculation is over
                if positiveInput <= 0 {
                    break;
                }
                //We had a residual
                else if positiveInput % 2 != 0 {
                    tempArray[index] = 1;
                }
                //We dont have a residual
                else {
                    tempArray[index] = 0;
                }
                //Jump to the next calculation step
                index -= 1;
                positiveInput = positiveInput / 2;
            }
            //"Testing" give array out
            println!("{:?}", tempArray);
            /*
            let mut result:u32 = 0;
            let mut num = givenFloat;
        
            // Bit set for the leader
            if num < 0.0 {
                result |= 0x80000000; // Set the sign bit (Bit 31) for negative numbers
                num = -num; // Make the number positive for further processing
            }
        
            let mut exponent = 127;
        
            // Normalize the decimal number
            while num >= 2.0 {
                num /= 2.0;
                exponent += 1;
            }
            while num < 1.0 && exponent > 0 {
                num *= 2.0;
                exponent -= 1;
            }
        
            // remove the implicit one leader (Bias: 127)
            let mut fraction = num - 1.0;
            let mut frac_bits = 0;
        
            // Calculate the mantisse (Bits 0-22)
            for i in 0..23 {
                fraction *= 2.0;
                if fraction >= 1.0 {
                    frac_bits |= 1 << (22 - i);
                    fraction -= 1.0;
                }
            }
        
            // Set the Exponent (Bits 23-30)
            exponent &= 0xFF; // Bond the exponent on 8 Bits
            exponent <<= 23;
        
            result |= exponent | frac_bits;
            
            println!("You result {}", result);
            */
        }
        Err(..) => println!("Not a number: {}", numericInput),
    };
}

fn two_deserialize() {
    println!("<=Deserialize=>");
    println!("Type number in");
    let mut userInput =  String::new();
    io::stdin()
        .read_line(&mut userInput)
        .expect("Fail to read from stdin");
    
    let numericInput = userInput.trim();
    match numericInput.parse::<u32>() {
        Ok(i) => {
            println!("You input: {}", i);
            println!("After the division: {}", i + i);
        }
        Err(..) => println!("Not a number: {}", numericInput),
    };
}

fn three_explanation() {
    println!("The IEEE 754 is a standard for representing floating-point numbers (floating-point numbers) in the binary system in computers.
It defines the representation of real numbers by sign, exponent and mantissa.");
}

fn mainMenu() {
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
        mainMenu();
}