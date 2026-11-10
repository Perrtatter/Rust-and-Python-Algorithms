fn inseration_sort(elements: &mut [i8]) -> &mut [i8]{
    let mut i: usize = 1;

    while i < elements.len(){

        let mut j: usize = i;

        while j > 0 && elements[j-1] > elements[j]{
            elements.swap(j, j-1);
            j = j - 1;
        }

        i += 1;
    }

    return elements;
}


fn main(){
    let mut elements: [i8; 5] = [3, 5, 2, 6, 1];
    let mut sorted_elements = inseration_sort(&mut elements);

    println!("{:?}", sorted_elements);
}