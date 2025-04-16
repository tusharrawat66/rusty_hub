pub fn sum_numbers(input: &str) -> Result<i32,String>{
    if input.is_empty(){
        return Err(String::from("Empty Input"));
    }

    let numbers: Result<Vec<i32>,_> = input.split(',').map(|x| x.parse::<i32>().map_err(|_| "Invalid number!")).collect();

    match numbers{
        Ok(nums)=> {
            if nums.is_empty(){
                Err(String::from("Empty Input"))
            }
            else{
                Ok(nums.iter().sum())
            }
        }
        Err(e) => Err(String::from(e))
    }
    
}