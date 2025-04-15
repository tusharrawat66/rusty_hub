// tuples

// pub fn practice(){
//     let a_var = (String::from("Cherry"),2,true);
//     println!("A tuple: {:?}",a_var.2);  // Output: true. An element is accessed using Var.index  

//     let (fruit,number,in_stock) = a_var;
//     println!("ID: {}, Make: {}, In stock: {}", number, fruit, in_stock); //Break it apart into variables:
// }


// pub fn get_car_info(id:&str,make:&str) -> Result<(i32,String), String>{
//     let id_num:Result<i32,_> = id.parse();
//     match id_num{
//         Ok(num)=>{
//             if num > 0 {
//                 let desc = format!("{} with ID {}",make,num);
//                 Ok((num, desc))
//             }
//             else {
//                 Err(String::from("The ID should be positive"))
//             }
//         }
//         Err(_)=>Err(String::from("The ID should be a number"))
//     }
// }


/////////////////////////////////////////////////////////
// Parse Two Numbers into a Tuple
// Method 1: Nested match
// pub fn parse_coordinates(x:&str, y:&str)-> Result<(i32,i32), String>{
//     let x_int:Result<i32,_> = x.parse();
//     let y_int: Result<i32, _> = y.parse();

   
//     match x_int{
//         Ok(x_num) => {
//             if x_num >0{
//                 match y_int {
//                     Ok(y_num) =>{
//                         if y_num >0{
//                         Ok((x_num,y_num))
//                         }
//                         else{
//                             Err(String::from("y doesn't have a positive value"))
//                         }
//                     }
//                     Err(_) => Err(String::from("y should be a number"))   
//                 }
//             }else{
//                 Err(String::from("x doesn't have positive value"))
//             }
//         }
//         Err(_)=>Err(String::from("x should be a number"))
//     }   

// }

// Method 2: Early Returns

// pub fn parse_coordinates(x:&str, y:&str)-> Result<(i32,i32), String>{
//     let x_int: i32 = match x.parse(){
//         Ok(num) if num > 0 => num,
//         Ok(_)=>return Err(String::from("x doesn't have positive value")),
//         Err(_)=>return Err(String::from("x is not a number"))

//     };

//     let y_int: i32 = match y.parse(){
//         Ok(num) if num > 0 => num,
//         Ok(_) => return Err(String::from("y doesn't have positive value")),
//         Err(_) => return Err(String::from("y is not a number"))
//     };

//     Ok((x_int,y_int))

// }


// pub fn measure_text(text: &str)-> Result<(usize,String), String>{
//     if text.is_empty(){
//         return Err(String::from("Text is Empty"));
//     }
//     Ok((text.len(),text.to_uppercase()))
// }



// pub fn validate_car_tuple(id: &str, speed: &str)-> Result<(i32,i32), String>{
//     let id_var: i32 = match id.parse(){
//                                 Ok(num) if num > 0 => num,
//                                 Ok(_) => return Err(String::from("id is less than zero")),
//                                 Err(_) => return Err(String::from("id is not a number"))
//     };

//     let speed_var: i32 = match speed.parse(){
//         Ok(num) if num > 0 => num,
//         Ok(_) => return Err(String::from("speed is less than zero")),
//         Err(_) => return Err(String::from("speed is not a number"))
//     };

//     Ok((id_var,speed_var))
// }


// pub fn full_name(first: &str, last: &str)-> Result<(String,usize),String>{
//     if first.len() < 2 || last.len() < 2{ 
//         return Err(String::from("Name part too short!"));
//     }

//     let full_name: String = format!("{} {}",first,last);
//     let char_length: usize = full_name.len();
//     Ok((full_name,char_length))

// }

pub fn even_pair(num1: &str, num2: &str)-> Result<(i32,i32), String>{
    let num_one = match num1.parse(){
        Ok(num) if num%2 == 0 => num,
        Ok(_) => return Err(String::from("Numbers must be even!")), 
        Err(_) => return Err(String::from("Invalid number!"))
    };

    let num_two = match num2.parse(){
        Ok(num) if num%2 == 0 => num,
        Ok(_) => return Err(String::from("Numbers must be even!")), 
        Err(_) => return Err(String::from("Invalid number!"))
    };

    Ok((num_one, num_two))
}