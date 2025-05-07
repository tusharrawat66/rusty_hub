pub fn if_test(){
    let age_to_drive:u8 = 18u8;
    println!("Enter the person's age:");

    let my_input = &mut String::from("");
    std::io::stdin().read_line(my_input).unwrap();

    let age: u8 = my_input.replace("\n", "").parse::<u8>().unwrap();
    if age >= age_to_drive{
        println!("Issuing driver's license");   
    }
    else{println!("They can Fuck-off")}
}