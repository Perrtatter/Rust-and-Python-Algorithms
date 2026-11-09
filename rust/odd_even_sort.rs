fn odd_even_sort(elements: &mut [i8]) -> &mut [i8]{
    let mut sorted_flag: bool = false;

    while !sorted_flag{
        sorted_flag = true;

        for i in (1..elements.len() - 1).step_by(2){

            if elements[i] > elements[i + 1]{
                elements.swap(i, i + 1);
                sorted_flag = false;
            }
        }


        for j in (0..elements.len() - 1).step_by(2){
            
            if elements[j] > elements[j + 1]{
                elements.swap(j, j + 1);
                sorted_flag = false;
            }
        }
    }

                
    return elements;
}


fn main(){

    let mut elements: [i8; 5] = [3, 5, 2, 6, 1];
    let sorted_elements = odd_even_sort(&mut elements);

    println!("{:?}", sorted_elements);

}