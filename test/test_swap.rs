fn main(){
    let mut elements: [String; 3] = [
        String::from("Apple"),
        String::from("Mango"),
        String::from("Peer")
    ];
    
    // 0 = Apple, 2 = Peer
    elements.swap(0,2);
    
    println!("{:?}", elements);
    // ["Peer", "Mango", "Apple"]
    
    
    let mut elements2: [i8; 3] = [3,2,1];
    
    // 0 = 3, 2 = 1
    elements2.swap(0,2);
    
    println!("{:?}", elements2)
    // [1, 2, 3]
    
}