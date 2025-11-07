fn main() {
    let age: i32 = 25; // integer variable
    let name: String = String::from("Alice"); // string variable

    // let name1_gender: char = 'f'; //char variable

    let name_gender = String::from("female");

    let vars1: String = String::from("male"); // This will cause a compile-time error due to invalid variable name
    

    println!("Name: {}, Age: {}, name_gender: {}, vars1: {}", name, age, name_gender, vars1);
}


    