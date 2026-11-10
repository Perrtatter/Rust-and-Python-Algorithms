fn test(test: i8) -> Option<i8>{
    /*
    if test == 1{
        return None;
    }
    
    else{
        // Some() cause is Option
        return Some(test + 1);
    }
    */
    
    let result = match test{
        1 => None,
        _ => Some(test + 1)
    };
    
    return result;
    
}

fn main(){
  let num: i8 = 1;
  
  println!("{:?}", test(num));
  // None
  
  let num: i8 = 2;
  
  println!("{:?}", test(num).unwrap()); // unwrap = Some(3) -> 3
  // None
 
}