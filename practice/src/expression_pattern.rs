pub fn test_match_array(){
    let prices:[i32; 4]  = [30000,50000,90000,120000];

    match prices[0..=3]{
        [30000,50000]=> println!("You have some reasonable priced cars"),
        [30000,50000,..]=> println!("You have a variety of cars"),
        _ => println!("You dont have reasonable priced cars ")
    }
}




pub fn test_match_string()->u32{
    let car_manufacturer = "Toyota";


    match car_manufacturer{
        "Hyundai"=> 10000,
        "Toyota" => 100000,
        _ => 0
    }
}



pub fn test_match_int(){
    let myage: u16 = 33;
    let y: u8 = 5;

    match myage{
        1..=35 if y==5 => println!("Your age is 35, y is 5"),
        1..=35 if y!=5 => println!("Your age is 35, y is not 5"),
        1..=35 => println!("Your age is 35"),
        0 => println!("You are a newborn"),
        36..=149 => println!("Your age is between 36 to 149"),
        150.. => println!("Your age is over 150"),
    }
}