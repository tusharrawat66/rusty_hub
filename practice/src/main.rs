use std::collections::HashMap;
use std::fmt::{Display,Formatter, Result};
// mod vector_dtype; 
// mod owner_borrow_referrence;
// mod country_details;
mod tuples_dtype;


#[derive(Debug)]
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


    let result = tuples_dtype::even_pair("4", "2");
    match result {
        Ok((num1, num2)) => println!("{}, {}",num1, num2),
        Err(e) => println!("Error: {}", e)
    }
}

