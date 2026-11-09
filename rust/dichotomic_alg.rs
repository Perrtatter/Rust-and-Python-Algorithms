fn dichto(elements: &mut [i8], x: i8) -> Option<usize>{
    elements.sort();

    let mut left: usize = 0;
    let mut right: usize = elements.len() -1;

    while left <= right{
        let mut center: usize = (right + left)/2;

        if elements[center] < x{
            left = center + 1;
        }

        else if elements[center] > x{
            right = center - 1;
        }

        else{
            return Some(center);
        }
    }

    return None;
}

fn main() {
    let mut elements: [i8; 9] = [2, 4, 8, 7, 9, 15, 12, 1, 17];

    let x = dichto(&mut elements, 15);
    println!("{}", x.unwrap());

 
    // cprintln!("{}", elements[x.unwrap()]);
}