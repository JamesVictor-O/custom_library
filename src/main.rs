mod rust_options;

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

 #[derive(Debug)]
struct Car {
    owner: String,
    year:u32,
    fuel_level:f32,
    price : u32
 }



 // three forms of self a method could take
    // - First form: An immutable reference to self(&self);
    // - Second form : A mutable reference to self(&mut self)
    // - Third form: An owned form of self(self )


impl  Car {
    // - first form: An immutable reference to self(&self);
    fn display_car(&self){
        println!(
            "Owner: {}, Year: {}, Price: {}", self.owner, self.year, self.price
        )
    }   

    // - Second form : A mutable reference to self(&mut self)
    
    fn refule(&mut self, gallons:f32){
        self.fuel_level + gallons;
    }


    // - Third form: An owned form of self(self )

    fn  sell(self) -> Car{
        self
    }

    //  Associated functions 
    //  associate functions are simply functions define inside an impl block, they are accoiated with the type not instances
    // associate functions do not take self because it's creating a new instance , not acting on exsisting onr

    fn new(  owner: String,
    year:u32,
    fuel_level:f32,
    price : u32) -> Car{
        Car {
            owner, year, fuel_level, price
        }
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
    let mut my_car= Car{ owner: String::from("James"), year: 2023, fuel_level:0.0, price: 10_000 };

    //  my_car.display_car()

    my_car.refule(3.0);

    println!("car fuel level is {}", my_car.fuel_level);
    
    // Tuple struct
    let points_2D: (i32, i32) = (1,3);
    let points2_3D= (4,10, 13);

    //  functionality to structs
    println!("call my car, {:?}", my_car.display_car());

    //  Two requirements for a function to be considered as a method.
    //  - Must be inside an implementation block
    //  - first parameter must be self

   let latest_car= Car::new(String::from("ochula"), 2000, 3.0, 8_000);

   println!("the lates model, {:?}", latest_car);
   


 rust_options::rust_options();
}
