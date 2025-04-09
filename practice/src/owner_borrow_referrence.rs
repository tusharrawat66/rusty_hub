

pub fn ownership_concept(){
    println!("\n***********Ownership***************");   

    // ownership
    let tushar:String= String::from("Car");   // Tushar gets a car
    println!("I own the {:?}", tushar);

    let friend = tushar; // Gives it to a friend (move)

    // println!("{:?}", me);  Error: I dont have it anymore
    println!("Friend has the {:?}",friend);


    // borrowing (sharing)

}

pub fn borrowing_concept(){
     println!("\n************Borrow**************\n➡️ Read-only borrowing\n");   
    // Read-only borrowing (sharing)

    let tushar:String = String::from("Car"); //Tushar owns the car
    let friend = &tushar; // Friend borrows it (read-only)

    println!("\tFriend sees {:?}", friend);   //Friend looks at it
    println!("\tTushar still has {:?}", tushar);   //Tushar still owns it

    println!("\n\n➡️ Mutable borrowing\n"); 

    // Mutable borrowing (changeable)
    let mut me = String::from("Beer");  // me owns a beer
    let gf = &mut me;     // gf borrows it to add "brings pizza" 

    gf.push_str(" with pizza");
    println!("{}",me);      // me still have beer, also pizza now
    //Rule: Only one &mut borrow at a time—no one else can borrow while friend has it.



}