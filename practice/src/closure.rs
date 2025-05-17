struct Person{
    first_name: String,
    last_name: String

}


pub fn closure_test(){
    let add = |x:i32, y:i32| {
        println!("x: {} y: {}",x,y);
        x + y
    };
    let result = add(5,10);

    let print_result = |x:i32| println!("Result is {}",(result+x));
    print_result(100);

    let mut p1: Person = Person{first_name: "Tushar".to_string(), last_name:"Rawat".to_string()};
    let mut change_name = |x:&str| p1.last_name= x.to_string();
    change_name("Singh");
    println!("{} {}",p1.first_name,p1.last_name);
}