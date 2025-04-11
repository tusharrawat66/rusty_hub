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
//     let x_int = match x.parse(){
//         Ok(num) if num > 0 => num,
//         Ok(_)=>return Err(String::from("x doesn't have positive value")),
//         Err(_)=>return Err(String::from("x is not a number"))

//     };

//     let y_int = match y.parse(){
//         Ok(num) if num > 0 => num,
//         Ok(_) => return Err(String::from("y doesn't have positive value")),
//         Err(_) => return Err(String::from("y is not a number"))
//     };

//     Ok((x_int,y_int))

// }


pub fn measure_text(text: &str)-> Result<(usize,String), String>{
    if text.is_empty(){
        return Err(String::from("Text is Empty"));
    }
    Ok((text.len(),text.to_uppercase()))
}
