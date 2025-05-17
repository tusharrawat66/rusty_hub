use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColor{
    Silver,
    Black,
    White,
    Blue,
}

// #[derive(Debug)]
#[allow(dead_code)]
struct VehicleTuple(String, String, u16);


#[derive(Debug)]
#[allow(dead_code)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

struct Person<'p>{
    first_name: Cell<&'p str>,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool
}

// Mutable Struct
// struct Person {
//     first_name: String,
//     last_name: String,
//     birth_year: u16,
//     birth_month: u8,
//     visited_europe: bool
// }



pub fn test_create_vehicletuple() {
    let myvehicle = new_vehicle_tuple();
    println!("Manufacturing: {0}, model: {1}, year: {2}",myvehicle.0, myvehicle.1, myvehicle.2);
}

fn new_vehicle_tuple() -> VehicleTuple {
    return VehicleTuple("Hyundai".to_string(), "Venue".to_string(), 2021);
}


fn new_vehicle() -> Vehicle {
    let v1:Vehicle = Vehicle {manufacturer: "Hyundai".to_string(), model: "Venue".to_string(), year: 2021, color: VehicleColor::Silver};
    return v1;
}

fn new_person() -> Person<'static> {
    let p1: Person = Person{first_name: Cell::from("Tushar"),
                    last_name: "Rawat".to_string(),
                    birth_year: 1992,
                    birth_month: 2,
                    visited_europe: false,    
    };
    p1.first_name.set("Chinku");
    return p1;
}


pub fn test_create_vehicle() {
    let myvehicle: Vehicle = new_vehicle();
    println!("{:#?}",myvehicle);
}


pub fn test_create_person() {
    let myperson = new_person();

    println!("First name: {}, Last name: {}, Birth month:{}, Birth year: {}, Ever visited Europe: {}", myperson.first_name.get(), myperson.last_name, myperson.birth_month, myperson.birth_year, myperson.visited_europe);

}