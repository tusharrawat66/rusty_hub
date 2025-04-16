use super::Car;


pub fn filter_cars_by_make(cars: &Vec<Car>, make: &str)->Vec<Car>{
    let car_vec:Vec<Car> = cars.iter().filter(|x| x.make==make).cloned().collect();
    car_vec
}