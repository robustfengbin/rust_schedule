fn testInstance() {

    for n in  0..2000{
        let s1 =  Student {name:  String::from("robust"),age:12};
        println!("n:{:?}",s1);
        println!("n:{}",n);
    }
 }
 #[derive(Debug)]
 struct Student {
     name: String,
     age: u32
 }