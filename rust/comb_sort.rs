fn comb_sort(elements: &mut [i8]) -> &mut [i8]{
    let mut between = elements.len();
    let mut swap: bool = false;
    
    while between > 1 || swap == true{
        between = (between as f64 / 1.3) as usize;
        
        if between < 1{
            between = 1;
        }
        
        let mut i:usize = 0;
        swap = false;
        
        while i < elements.len() - between{
            if elements[i] > elements[i + between]{
                elements.swap(i, i + between);
                swap = true;
            }
            
            i = i + 1
        }
    }
    
    return elements; 
}



fn main(){
    let mut elements: [i8; 5] = [3,5,4,2,1];
    
    let sorted_elements = comb_sort(&mut elements);
    println!("{:?}", sorted_elements);
}