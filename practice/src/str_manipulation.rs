#![allow(dead_code)]

pub fn reverse_string(word:&str)-> Result<String,String>{

    if word.is_empty(){
        return Err(String::from("String is Empty"));
    }
    let new_word = word.chars().rev().collect::<String>();

    Ok(new_word)
}


pub fn reverse_string_order(word:&str)-> String{
    let new_word = word.split_whitespace().rev().collect::<Vec<&str>>().join(" ");
    new_word
}

pub fn rev_str_order_spaces(word:&str)-> String{
    // let new_word: Vec<&str> = word.split(' ').filter(|w| !w.is_empty()).rev().collect::<Vec<&str>>();
    let new_word = word.split(" ")
                                .fold(String::new(), |acc, wod| {
                                    if wod.is_empty(){
                                        " ".to_string() + &acc
                                    } else{
                                        wod.to_string()+" "+ &acc
                                    }
                                })
                                .trim_end()
                                .to_string();
    
    new_word
}

pub fn get_full_name(first: &str, last: &str)-> String{

    let full_name: String = format!("{0} {1}", first, last);
    full_name

}
