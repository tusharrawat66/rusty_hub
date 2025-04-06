use super::Car;


pub fn vec_func(){
    // Creating a new Vector and populating it.
    let mut cars:Vec<Car> = Vec::new(); 
    cars.push(Car { id: 1, make: String::from("Volkswagen"), model: String::from("Golf"), color: String::from("Red") });
    cars.push(Car { id: 2, make: String::from("Toyota"), model: String::from("Camry"), color: String::from("White") });
    cars.push(Car { id: 3, make: String::from("Ford"), model: String::from("Ecosport"), color: String::from("Grey") });


    // Filtering car from the Vectors
    let car_search:Vec<&Car> = cars.iter().filter(|car| car.make == "Toyota").collect();

    // Fetching the name of the car after filtering.
    match car_search.get(0) {Some(car) => println!("\n{}", car.description()), None => println!("No search found")}

    println!("{:#?}",cars);       
}
