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
        Ok(decimal) => {
            let mut pre_decimal_place: i32 = decimal as i32;
            let mut decimal_place: f32 = decimal - pre_decimal_place as f32;
            let mut ieeeArray = [0;32]; //mantissa included
            let mut temp: u8 = 0; //get used for expoent and characteristic
            let mut i: usize = 0;

            //negative or positive?
            if pre_decimal_place < 0 {
                ieeeArray[i] = 1;
            }
            i += 9;
            //conversion of the pre decimal place
            pre_decimal_place = pre_decimal_place / 2; //Ignore the first mantissa index
            while pre_decimal_place != 0 {
                if pre_decimal_place % 2 != 0 { // We have a residual
                    ieeeArray[i] = 1;
                }
                else { // We dont have a residual
                    ieeeArray[i] = 0;
                }
                pre_decimal_place = pre_decimal_place / 2;
                i += 1;
                temp += 1;
            }
            i += 1;
            //conversion of the decimal place
            while decimal_place == 1.0 {
                if decimal_place * 2.0 > 1.0 {
                    ieeeArray[i] = 1;
                }
                else {
                    ieeeArray[i] = 0;
                }
                decimal_place = decimal_place * 2.0;
                i += 1;
            }
            i += 1;
            ieeeArray[i] = 1;

            //characteristic
            i = i * 1 - i;
            temp = temp + 127;
            while temp != 0 {
                if temp % 2 != 0 { // We have a residual
                    ieeeArray[i] = 1;
                }
                else { // We dont have a residual
                    ieeeArray[i] = 0;
                }
                temp = temp / 2;
                i += 1;
            }
            
            println!("{:?}", ieeeArray);
            /*
            let mut positiveInput: u32 = givenFloat as u32;
            let mut negativeInput: f32 =(givenFloat as f32 - positiveInput as f32) as f32;
            let mut tempArray = [0;32];
            let mut index:usize = 31;
            let mut exponent: u8 = 0;
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
                exponent += 1;
                positiveInput = positiveInput / 2;
                if (index == 30) {
                    tempArray[index] = 2;
                    index -= 1;
                }
            }
            exponent -= 1;
            //Define Mantissa
            //Separate the decimal part from the full part
            index -= 1;
            loop {
                if negativeInput == 1 as f32 {
                    break;
                }
                else if negativeInput * 2 as f32 >= 1 as f32 {
                    tempArray[index] = 1;
                }
                else {
                    tempArray[index] = 0;
                }
                index -= 1;
                negativeInput = negativeInput * 2 as f32;
            }
            
            //"Testing" give array out
            println!("{:?}", tempArray);
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