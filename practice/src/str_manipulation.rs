// pub fn reverse_string(word:&str)-> Result<String,String>{

//     if word.is_empty(){
//         return Err(String::from("String is Empty"));
//     }
//     let new_word = word.chars().rev().collect::<String>();

//     Ok(new_word)
// }

pub fn reverse_string_order(word:&str)-> String{
   

    let new_word = word.split_whitespace().rev().collect::<Vec<&str>>().join(" ");
    
    new_word
    

}