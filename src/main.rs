 /*
  what are struct 
  - structs are a way to create a custom data type
  - structs are similar to classes in other languages
  - structs are defined using the struct keyword
  - structs are used to group together related data
  - structs are used to create new data types
  - structs are used to represent real-world entities
  - structs are used to represent data in a more structured way
 
 */

 struct Points { x: f64, y:f64}

 #[derive(Debug)]
 struct Color(u8,u8,u8);


  #[derive(Debug)]
 struct Owner {name:String}


 struct Car {
    owner: String,
    year:u32,
    fuel_level:f32,
    price : u32
 }





impl  Car {
    fn display_car(&self){
        println!(
            "Owner: {}, Year: {}, Price: {}", self.owner, self.year, self.price
        )
    }    
}

fn main() {
    // named-field struct

    let p = Points {y:1.0, x:2.0};
    println!("your point is, {}", p.y);

    // Tuple struct
    let c:Color= Color(255, 0, 128);

    println!("Tuple Struct = {:?}", c);

    let mut owner= Owner {name: String::from("James")};
    let secondOwner= owner;

    println!("{:?}", secondOwner); 

    //  
    let my_car= Car{ owner: String::from("James"), year: 2023, fuel_level:0.0, price: 10_000 };

     my_car.display_car();

   
    let car_owner=my_car.owner;

    let another_car= Car{
        owner:String::from("victor"),
        ..my_car
    };

    println!("car owner is  {}", car_owner);
    
    // Tuple struct
    let points_2D: (i32, i32) = (1,3);
    let points2_3D= (4,10, 13);

    //  functionality to structs





}
