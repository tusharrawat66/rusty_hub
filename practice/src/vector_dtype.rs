use super::Car;


pub fn vec_func(){
    // Creating a new Vector and populating it.
    let mut cars:Vec<Car> = Vec::new(); 
    cars.push(Car { id: 1, make: String::from("Volkswagen"), model: String::from("Golf"), color: String::from("Red") });
    cars.push(Car { id: 2, make: String::from("Toyota"), model: String::from("Camry"), color: String::from("White") });
    cars.push(Car { id: 3, make: String::from("Ford"), model: String::from("Ecosport"), color: String::from("Grey") });


    // Extend to add to the vector
    let  mut new_list:Vec<String> = Vec::new();
    new_list.extend(vec![String::from("sup"), String::from("mama")]);
    println!("\n{:?}", new_list);



    // Slicing
    // Method 1
    let num_vec:Vec<i32>=vec![1,2,3,4,5,6,7,8,9,10];
    println!("\nNormal slicing Exclusive: {:?}",&num_vec[2..6]);
    print!("\nReverse slicing Inclusive: {:?}", &num_vec[1..=6]);
    // A Vector with i32 values
    let rand_numbers: Vec<i32> = vec![234,45,1,23,65,90,-11];
    // A Vector cant be reverse sliced like python, but we can achieve it by 
    let reversed:Vec<i32> = rand_numbers.into_iter().rev().collect();
    println!("\nReverse slice: {:#?}", &reversed[..3]);

    // Method 2
    let new_rand_numbers: Vec<i32> = vec![54,727,234,45,1,23,65,90,-11];
    // Skip and take in rust is similar to [0:2:4] functionality
    let moved:Vec<i32> = new_rand_numbers.into_iter().collect();
    let straight_skip:Vec<&i32> = moved.iter().skip(2).take(4).collect();
    println!("\nStriaght slice: {:?}", straight_skip);


    // Filter out all the evem numbers 
    println!("Even numbers: {:?}",straight_skip.iter().filter(|&&x| x%2==0).collect::<Vec<&&i32>>());


    // Filtering car from the Vectors
    let car_search:Vec<&Car> = cars.iter().filter(|car| car.make == "Toyota").collect();

    // Fetching the name of the car after filtering.
    match car_search.get(0) {Some(car) => println!("\n{}", car.description()), None => println!("No search found")}

    println!("\n{:#?}",cars);       
}
