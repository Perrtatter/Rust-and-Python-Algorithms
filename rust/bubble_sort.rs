fn bubble_sort(elements: &mut [i8]) -> &mut [i8]{

    for i in 0..elements.len(){

        for j in 0..elements.len() -i -1{

            if elements[j] > elements[j+1]{
                elements.swap(j, j+1);
            }
        }
    }

    return elements;
}


fn main(){
    let mut elements: [i8; 5] = [3,5,2,6,1];

    let sorted_elements = bubble_sort(&mut elements);
    println!("{:?}", sorted_elements);
}