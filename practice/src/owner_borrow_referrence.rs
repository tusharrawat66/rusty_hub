// Concepts with String

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
    let friend: &String = &tushar; // Friend borrows it (read-only)

    println!("\tFriend sees {:?}", friend);   //Friend looks at it
    println!("\tTushar still has {:?}", tushar);   //Tushar still owns it

    println!("\n\n➡️ Mutable borrowing\n"); 

    // Mutable borrowing (changeable)
    let mut me: String = String::from("Beer");  // me owns a beer
    let gf: &mut String = &mut me;     // gf borrows it to add "brings pizza" 

    gf.push_str(" with pizza");
    println!("{}",me);      // me still have beer, also pizza now
    //Rule: Only one &mut borrow at a time—no one else can borrow while friend has it.

}

pub fn reference(){
    println!("\n**************Reference***************\n");

    let mut tushar: String = String::from("Painting");   // Painting owned by tushar
    let friend1: &String = &tushar;
    println!("Friend sees {}", friend1);    //Friend1 can only see Painting
    
    let friend2: &mut String = &mut tushar;  // Friend2 given right to modify
    friend2.push_str(" is outstanding");    // Friend2 modifies the 'Painting' to 'Painting is outstanding'

    // println!("{}", friend1);  // Error!: Can't use by friend1 while friend2 changes it

    println!("{}\n",tushar);   // Painting modified for tushar as well

}


// Concept with Vector
pub fn ownership_concept_vec(){

    println!("************Concept with Vector************");
    println!("➡️ Read-only");
    let nums_var:Vec<i32> = vec![2,3,5,6,8,10,32];
    let borrower_var = &nums_var;
    // Borrow
    println!("Borrowed list: {:?}",borrower_var);
    println!("Original list: {:?}\n",nums_var);
    
    println!("➡️ Move");

    let transfer_var: Vec<i32> = nums_var;
    println!("{:?}\n",transfer_var);
    // println!("{:?}", nums_var);  // Error!: the vector's ownership has been changed so 'nums_var' would throw Error 

    println!("➡️ Move & Reference");
    let mut new_owner:Vec<i32> = transfer_var;
    println!("New owner: {:?}",new_owner);  // The new_owner has the vector's posession

    let modifier_friend:&mut Vec<i32> = &mut new_owner;

    modifier_friend.push(100);  // Vector modified by Modifier friend
    println!("Modifier friend: {:?}", modifier_friend);  // The Modifier friend has the vector's access

    println!("New owner with updated vector: {:?}",new_owner);   // new_owner now has the updated vector

    // modifier_friend.push(123);
    // println!("{:?}",modifier_friend);     // Error: cannot perform modification more than once
}




// Exercise to test Above concept knowledge

pub fn practice(){
    println!("➡️Exercise 1\n");
    let toy:String = String::from("Robot");
    let new_owner:String = toy;
    println!("New owner: {}\n",new_owner);

    println!("➡️Exercise 2\n");
    let a_vec:Vec<i32> = vec![10,20,30];

    let friend:&Vec<i32> = &a_vec; 
    println!("{}",friend[0]);
    println!("a_vec still owns: {:?}\n",a_vec);

    println!("➡️Exercise 3\n");
    let mut pet:String = String::from("Dog");
    let pet_modifier:&mut String = &mut pet;
    pet_modifier.push_str(" is cute");
    println!("Printing pet while borrower is active:{}\n",pet);

    println!("➡️Exercise 4\n");
    let new_vec:Vec<i32> = vec![1,2,3];
    let friend1:&Vec<i32> = &new_vec;     // Borrowed for read only
    println!("Length: {}",friend1.len());

    let new_owner:Vec<i32> = new_vec;  // Moved the ownership
    println!("New owner: {:?}\n", new_owner);

    println!("➡️Exercise 5\n");
    let mut game:String = String::from("Soccer");

    let friend1:&String = &game;  // read only
    let friend2:&String = &game;  // read only
    println!("Friend 1: {}", friend1);
    println!("Friend 2: {}", friend2);

    let friend3:&mut String = &mut game;

    println!("Friend 3: {}\n", friend3);
    

    println!("➡️Exercise 6\n");
    let mut a_vec:Vec<i32> = vec![5, 10, 15, 20];

    let b_vec:&Vec<i32> = &a_vec;
    println!("Element on index 2: {}", b_vec[2]);

    let c_vec:&mut Vec<i32> = &mut a_vec;
    c_vec.push(25);



}



