use std::io;


pub fn if_test(){
    let age_to_drive:u8 = 18u8;
    println!("Enter the person's age:");

    let my_input = &mut String::from("");
    io::stdin().read_line( my_input).expect("Failed to read input");
      
            match my_input.trim().parse::<u8>(){
                Ok(age) =>{
                    if age >= age_to_drive{
                        println!("Issuing driver's license");   
                    }else{
                        println!("Not eligible for a license")
                }
            } 
            Err(_) => println!("Invalid input! Please enter a valid number (0-255).")

        }
    }


    