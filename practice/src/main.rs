use std::collections::HashMap;
use std::fmt::{Display,Formatter, Result};
// mod vector_dtype; 
// mod owner_borrow_referrence;
// mod country_details;
// mod tuples_dtype;
// mod fibonnaci;
// mod filter_cars;
// mod parse_nums_string;
mod str_manipulation;
mod control_flow;
mod closure;
mod expression_pattern;
mod option_enum;

#[derive(Debug,Clone)]
struct Car{
    id: i32,
    make: String,
    model: String,
    color: String
}

// Trait impl
impl Display for Car{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f,"Car #{}: {} {} in {}", self.id, self.make, self.model, self.color)
    }
    
}

impl Car{
    fn description(&self) -> String{
        format!("{} {} in {}", self.make, self.model,self.color)
    }

    fn is_make(&self, make: &str) -> bool{
        self.make == make
    }
}



fn main() {
    let mut carinfo: HashMap<i32, Car>=HashMap::new();
// Inserting values into HashMap
    carinfo.insert(1,Car{
        id: 1,
        make: String::from("Honda"),
        model: String::from("City"),
        color: String::from("White")
    });

    carinfo.insert(2,Car{
        id: 2,
        make: String::from("Hyundai"),
        model: String::from("Venue"),
        color: String::from("Silver")
    });

    carinfo.insert(3,Car{
        id: 3,
        make: String::from("Maruti"),
        model: String::from("Swift"),
        color: String::from("Black")
    });
    
    // Iterating over HashMap
    for (k,v) in &carinfo{
        match carinfo.get(k) {
            Some(hello) => println!("Found: {}",hello),
            None => println!("Car with key: {} not found!",k),
        }
        // Calling Implementations on struct Car (similar to Class Method in Python)
        println!("{}", v.is_make(&v.make));
        println!("{}", v.description());
    }

    // Removing on k:v on the basis of the key from HashMap
    if let Some(keya)= carinfo.remove(&2){
        println!("Removed: {} key", keya);
    }

    println!("{:#?}",carinfo);

    // Method in HashMap that only upates if the key is avaiable else doe nothing
    carinfo.entry(5).or_insert(Car { id: (4), make: String::from("Mercedes"), model: String::from("E class"), color: String::from("Black") });

    println!("Car fleet size: {}", carinfo.len());
    println!("Empty? {}", carinfo.is_empty());

    println!("{}", match carinfo.get(&4) {Some(new_car) => new_car.to_string(), None => String::from("Key 4 not found!")});


    // vector_dtype::vec_func();

    // owner_borrow_referrence::ownership_concept();
    // owner_borrow_referrence::borrowing_concept();
    // owner_borrow_referrence::reference();
    // owner_borrow_referrence::ownership_concept_vec();
    // owner_borrow_referrence::practice();

    // country_details::country();

    // let result = country_details::describe_car(String::from("Toyota"), "1");
    // match result{
    //     Ok(x)=> println!("Success {}", x),
    //     Err(e)=> println!("Error: {}", e)

    // tuples_dtype::practice();

    // let result = tuples_dtype::get_car_info("55", "BMW");
    // match result {
    //     Ok((id,desc))=> println!("ID: {}, Desc: {}", id, desc),
    //     Err(e)=> println!("Error: {}",e)
    // }

    // let result = tuples_dtype::parse_coordinates("1","5");
    // match result {
    //     Ok((id1,id2)) => println!("Coordinate x:{} & y:{}",id1,id2),
    //     Err(e) => println!("Error: {}", e)    
    // }


    // let result = tuples_dtype::measure_text("tushar");
    // match result {
    //     Ok((size,uppercae)) => println!("{}, {}",size,uppercae),
    //     Err(e) => println!("Error: {}",e)
    // }


    // let result = tuples_dtype::validate_car_tuple("0","333");
    // match result {
    //     Ok((id,speed)) => println!("{}, {}",id,speed),
    //     Err(e) => println!("Error: {}",e)
    // }

    // let result = tuples_dtype::full_name("Tushar", "Rawat");
    // match result {
    //     Ok((full_name, char_len)) => println!("{}, {}",full_name, char_len),
    //     Err(e) => println!("Error: {}", e)
    // }


    // let result = tuples_dtype::even_pair("4", "2");
    // match result {
    //     Ok((num1, num2)) => println!("{}, {}",num1, num2),
    //     Err(e) => println!("Error: {}", e)
    // }

    // let result =  fibonnaci::fibo(5);
    // println!("{:?}",result);

    // let mut car_list:Vec<Car> = Vec::new(); 
    // car_list.push(Car { id: 1, make: String::from("Volkswagen"), model: String::from("Golf"), color: String::from("Red") });
    // car_list.push(Car { id: 2, make: String::from("Toyota"), model: String::from("Camry"), color: String::from("White") });
    // car_list.push(Car { id: 3, make: String::from("Ford"), model: String::from("Ecosport"), color: String::from("Grey") });
    // let result = filter_cars::filter_cars_by_make(&car_list,"Toyota");
    // println!("\n{:#?}",result);

    // let result = parse_nums_string::sum_numbers("1,2,3");
    // match result {
    //     Ok(res)=> println!("Sum of nums: {:?}", res),
    //     Err(e)=> println!("Error: {}",e)
        
    // }

    // let hello: std::result::Result<(i32, i32), String> = parse_nums_string::min_max("1,8,80,332,134,11,45");
    // match &hello {
    //     Ok((res1, res2))=> println!("Max: {}, Min: {}", res1, res2),
    //     Err(e)=> println!("Error: {}",e)
        
    // }
    // println!("{:?}", hello);



//     let tests = [
//         "user@domain.com",
//         "user.name@sub.domain.co.uk",
//         "user@",
//         "@domain.com",
//         "user@domain",
//         "user@.com",
//         "",
//         "a@@b.com",
//         "user@domain..com",
//         "tushar.rawat@gmail.com"
//     ];    

//     for email in tests{
//         match parse_nums_string::validate_email(email){
//             Ok(valid) => println!("Valid: '{}'", valid),
//             Err(e) => println!("{}: {}", e,email),
//         }
// }

// let result = str_manipulation::reverse_string("Hello World");

// match result {
//     Ok(res) => println!("{}",res),
//     Err(e) => println!("{}", e)
    
// }

// let result = str_manipulation::reverse_string_order("Hello rust");
// println!("{}", result);


// let result = str_manipulation::rev_str_order_spaces("Hello   rust ");
// println!("{:?}", result);

// let result= str_manipulation::get_full_name("Tushar", "Rawat");
// println!("Hi from {}", result);

// control_flow::if_test();
// control_flow::while_test();
// control_flow::loop_test();
control_flow::for_test();
closure::closure_test();
expression_pattern::test_match_int();
println!("{}",expression_pattern::test_match_string());
expression_pattern::test_match_array();
let result = option_enum::test_option_type();
println!("{}",result.unwrap());

let result =option_enum::test_option_string();
println!("{}",result.unwrap());


let result = option_enum::test_option_chartype();

if result.is_some() {
    println!("User selected character type"); 
    println!("Characted selected: {}", result.unwrap().to_string())
} 
else{
    println!("Character is None");
}


// println!("{}",result.unwrap().to_string());



}

