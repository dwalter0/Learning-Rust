fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(test: &str) -> &str
{  
    let mut start_slice = 0;
    let mut end_slice = test.len();
    let mut found_non_space = false;

    for (i, char_byte) in test.as_bytes().iter().enumerate(){
        if *char_byte != b' ' {
            start_slice = i;
            found_non_space = true;
            break;
        }
    }

    for (i, char_byte) in test.as_bytes().iter().enumerate().rev(){
        if *char_byte != b' ' {
            end_slice = i+1;
            break;
        }
    }

    //This means the test string contained only spaces so return an empty string.
    if !found_non_space {return ""}

    &test[start_slice..end_slice]
    
}