fn main(){
    // array
    // better for ram
    let array: [i8; 5] = [1,2,3,4,5];

    let array = [1,2,3,4,5];

    // matrix
    let matrix = [
        [0,0,0,0],
        [0,0,1,0]
    ];

    for i in &matrix{
        println!("{:?}", i);
    }

    // select
    //println!("{}", matrix[1][2]);

    // all
    let matrix: [String; 2] = [
        String::from("Mathys"), 
        String::from("Quentyn")
    ];

    
    // or 
    let matrix: [String; 2] = [
        "Mathys".to_string(), 
        "Quentyn".to_string()
    ];

    for i in &matrix{
        println!("{}", i);
    }
}
