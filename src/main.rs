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
            let mut negativeInput: f32 =(givenFloat as f32 - positiveInput as f32) as f32;
            let mut tempArray = [0;32];
            let mut index:usize = 31;
            let mut tempNameStellenzähler: u8 = 0;
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
                tempNameStellenzähler += 1;
                positiveInput = positiveInput / 2;
            }
            tempNameStellenzähler -= 1;
            //Separate the decimal part from the full part
            tempArray[index] = 2;
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