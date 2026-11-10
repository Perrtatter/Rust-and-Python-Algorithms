fn main(){
    // test all
    for i in ((0..10).step_by(2)).rev(){
        println!("{}", i);
    }
    
    println!("---------------------");
    



    // test step_by
    for i in (0..10).step_by(2){
        println!("{}", i);
    }
    
    println!("---------------------");
    
    
    // test rev 
    for i in (0..10).rev(){
        println!("{}", i);
    }
    
    println!("---------------------");
    
    // test nothing
    for i in 0..10{
        println!("{}", i);
    }
}