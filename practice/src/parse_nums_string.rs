// pub fn sum_numbers(input: &str) -> Result<i32,String>{
//     if input.is_empty(){
//         return Err(String::from("Empty Input"));
//     }

//     let numbers: Result<Vec<i32>,_> = input.split(',').map(|x| x.parse::<i32>().map_err(|_| "Invalid number!")).collect();

//     match numbers{
//         Ok(nums)=> {
//             if nums.is_empty(){
//                 Err(String::from("Empty Input"))
//             }
//             else{
//                 Ok(nums.iter().sum())
//             }
//         }
//         Err(e) => Err(String::from(e))
//     }
    
// }


// pub fn min_max(input: &str)->Result<(i32,i32),String>{
//     if input.is_empty(){
//         return Err(String::from("Empty Input!"));
//     }

//     let numbers:Result<Vec<i32>,_> = input.split(',').map(|x| x.parse::<i32>().map_err(|_| "Invalid element")).collect();

//     match numbers {
//         Ok(nums)=> {
//             if nums.is_empty(){
//                 return Err(String::from("Empty Input!"));
//             }
//             let min = *nums.iter().min().unwrap(); // Safe: nums not empty
//             let max = *nums.iter().max().unwrap(); // Safe: nums not empty
//             Ok((max, min))
//         }

//         Err(e) => Err(String::from(e))
//     }
//     }


pub fn validate_email(email:&str)->Result<String,String>{
    if email.is_empty() {
        return Err(String::from("Invalid"));
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(String::from("Invalid"));

    }
    let user: &str = parts[0];
    let domain: &str = parts[1];

    if user.is_empty() || domain.is_empty() {
        return Err(String::from("Invalid"));
    }

    if !domain.contains('.') {
        return Err(String::from("Invalid"));
    }
    let domain_parts: Vec<&str> = domain.split('.').collect();
    if domain_parts.iter().any(|part| part.is_empty()) {
        return Err(String::from("Invalid"));
    }

    Ok(email.to_string())

}