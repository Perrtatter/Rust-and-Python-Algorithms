fn main(){
    // the type `[i8]` cannot be indexed by `i8`
    // let index: i8 = 2;
    let index: usize = 2;
    
    let element: [i8; 4] = [1,2,3,4];
    
    println!("{}", element[index]);
}