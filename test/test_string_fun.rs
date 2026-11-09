fn lower(nom: String) -> String{
    return nom; 
}

fn main(){
    let nom:String = String::from("TEST");

    let nom_lower = lower(nom);
    println!("{}", nom_lower)

    let nom: [String; 3] = [
        String::from("Hello"),
        String::from("TEST"),
        String::from ("HH")
    ];

    for i in &nom{
        println!("{}", i);
    }
}