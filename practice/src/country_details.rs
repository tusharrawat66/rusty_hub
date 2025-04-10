pub fn country(){

    let countries:Vec<String> = vec![String::from("India"),String::from("USA"),String::from("Australia"),String::from("Japan")];
    let maybe_countries: Option<&String> = (&countries).get(1);
    match maybe_countries{
        Some(nation) => println!("Found: {}",nation),
        None => println!("No country here!") 
    }

    let id_result:Result<i32,_> = "42".parse();
    match id_result{
        Ok(x)=>println!("ID {}",x),
        Err(_)=>println!("Bad Number")
    }
}


pub fn describe_car(make:String, id:&str)-> Result<String,String>{
    let id_int:Result<i32,_> = id.parse();
    match id_int{
        Ok(x)=>{
            if x > 0 {
                let description:String = format!("The ID of {} is {}",make, x);
                Ok(description)
            
            } else {
                Err(String::from("The ID must be positive!"))
            }
        }
        Err(_)=>Err(String::from("Invalid ID - not a number"))

    }
}
