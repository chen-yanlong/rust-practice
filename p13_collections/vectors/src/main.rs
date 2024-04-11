 fn main() {
    // asigning vectors
    {    
        let mut v1: Vec<i32> = Vec::new();
        v1.push(1);
        let v2 = vec![1, 2, 3];
    }
    // access to elements
    {
        // First method
        let v = vec![1, 2, 3, 4, 5];
        let third_element = &v[2];
        println!("{}", third_element);

        // Second safer method, prevent querying value doesn't exist
        match v.get(2) {
            Some(third) => println!("{}", third),
            None => println!("No third element")
        }
    }
    // print elements
    {
        let v = vec![1, 2, 3, 4, 5];
        for i in &v {
            println!("{}", i);
        }
    }
    // store different type in vectors using enums 
    {
        enum Type {
            Int(i32),
            Float(f64),
            Text(String)
        }

        let v = vec![
            Type::Int(3),
            Type::Float(3.5),
            Type::Text(String::from("alan")),
        ];

        match &v[1] {
            Type::Int(i) => println!("{}", i),
            _ => println!("Not an interger")
        };
    }




}
