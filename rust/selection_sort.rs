fn selection_sort(elements: &mut [i8]) -> &mut [i8]{
    for i in 0..elements.len(){
        let mut min = i;

        for j in i+1..elements.len(){

            if elements[min] > elements[j]{
                min = j;
            }
        }

        elements.swap(i, min);
    }

    return elements;
}


fn main(){
    let mut elements: [i8; 5] = [3,5,2,6,1];

    let sorted_elements = selection_sort(&mut elements);
    println!("{:?}", sorted_elements);
}
