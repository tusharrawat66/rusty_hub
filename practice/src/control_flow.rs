use std::io;


pub fn for_test(){
    let ages = [8,18,26,35,41];
    let age_to_drive = 18i32;

    for value in ages{
        // println!("The current age is {0}", value);
        if value >= age_to_drive{
            println!("You are old enough to drive - {0}",value);
        }
        else{
            println!("You need to wait a little bit more ...");
        }
    }

}


#[allow(dead_code)]
pub fn loop_test(){
    let mut x = 1;
    loop {
        println!("Hello from Rust! 🦀");
        if x>5{
            break;
        }
        x += 1;
    }
}



#[allow(dead_code)]
pub fn while_test(){
    let age_to_drive = 16u8;
    let mut current_age = 0u8;

    while current_age<age_to_drive {
        println!("Waiting ...");
        current_age +=1;
        if current_age == 6{
            break;
        }        
    }
}



#[allow(dead_code)]
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


    