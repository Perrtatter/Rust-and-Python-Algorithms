struct Dog{
    name: String,
    age: i8
}

impl Dog{
    fn adopt(name: String, age: i8) -> Dog{
        Dog {name: name, age: age}
    }

    fn read_tag(&self) -> String{
        let tag_content = format!("Name: {}, Age: {}", self.name, self.age);

        return tag_content;
    }
}


fn main(){
    let rex = Dog::adopt(String::from("Rex"), 4);

    let tag_content = rex.read_tag();
    println!("{}", tag_content);
}