fn cocktail_sort(elements: &mut [i8]) -> &mut [i8]{
    let mut swap = true;
  
    while swap{
        swap = false;
        
        for i in 0..elements.len() -1 {
            if elements[i] > elements[i + 1]{
                elements.swap(i, i+1); 
                swap = true;
            }
        }
        
        for i in (0..elements.len() -2).rev(){
            if elements[i] > elements[i + 1]{
                elements.swap(i, i+1); 
                swap = true;
            }
        }
    }
    
    return elements; 
}


fn main(){
    let mut elements: [i8; 5] = [3,5,4,2,1];
    
    let sorted_elements = cocktail_sort(&mut elements);
    println!("{:?}", sorted_elements);
}