// Enums 
// its a way to define a type that can be one of several possible variants
#[derive(Debug)]
enum TrafficLight {
     Red,
     Yellow,
     Green,
}

enum Message{
    Quit, 
    Move { x:i32, y:i32},
    Write(String),
    ChangeColor(u8, u8, u8)
}
#[derive(Debug)]
enum TravelType {
    Car(f32),
    Train(f32),
    Areoplan(f32)
}



impl TravelType {
    fn TraveAllawoncs(&self) -> f32{
     let allowance=    match  self {
             TravelType::Car(miles) => miles * 2.0,
             TravelType::Train(miles) => miles * 3.0,
             TravelType::Areoplan(miles) => miles * 4.0
         };

       allowance
    }

    fn print_self(&self){
        println!("Hello from {:?}", self);
    }
}

pub fn fn_enum(){
    let light= TrafficLight::Green;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Green => println!("Go!"),
        TrafficLight::Yellow => println!("Slow down")
    }
    let msg=Message::Move { x: 20, y: 30 };
    process_message(msg);

    let traveled_by= TravelType::Car(100.0);

    let allowance=traveled_by.TraveAllawoncs();
    println!("Allowance for trip: {}", allowance);
}

fn process_message(msg:Message){
    match msg {
        Message::Quit => println!("this program will quit"),
        Message::Move { x, y } => println!("the move cordinate are ({} adnd {})", x,y),
        Message::ChangeColor(r, g, b) => println!("Color = RGB({}, {}, {})", r, g, b),
        Message::Write(text) => println!("text: {}", text)
    }
}
