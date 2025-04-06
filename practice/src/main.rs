use std::collections::HashMap;
use std::fmt::{Display,Formatter, Result};

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
    
    // println!("{:#?}", carinfo);

    for (k,v) in &carinfo{
        match carinfo.get(k) {
        Some(car)=>println!("Found: {}",car),
        None=>println!("Car with key: {} not found!",k),
    }
    println!("{}",v.is_make(&v.make));
    println!("{}",v.description())
}

}

