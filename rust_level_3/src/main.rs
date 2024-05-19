fn main() {
    let mut my_car = Vechicle{
        make : String::from("Toyota"),
        model : String::from("Corolla"),
        year : 2015,
        fuel: 50
    };
    // println!("My car is a {} {} {}", my_car.year, my_car.make, my_car.model);
    // let model = my_car.model;
    // println!("{:?}", my_car.model); // this will give us an error because owenrship moved
    // To fix add clone() method
    // let model = my_car.model.clone();
    // println!("{:?}", model);

    // Structs with methods
    my_car.print_details();

    my_car.refuel(10);

    my_car.print_details();


    // Enums
    let up = Direction::Up;
    let down = Direction::Down;
    let left = Direction::Left;
    let right = Direction::Right;

    // Enums with values
    let up = Movement::Up(10);
    let down = Movement::Down(20);
    let left = Movement::Left(30);
    let right = Movement::Right(40);

    let up:u32 = match up{
        Movement::Up(value) => 10,
        _ => 0
    }; 
    println!("{}", up);
}

// Structs

struct Vechicle{
    make : String,
    model : String,
    year : i32,
    fuel: u32
}

// Structs with methods

impl Vechicle{
    fn print_details(&self){
        println!("year: {} , make: {} , model: {}, fuel: {}", self.year, self.make, self.model, self.fuel);
    }

    fn refuel(&mut self, fuel: u32){
        self.fuel += fuel;
    }
}

// Enums
enum Direction{
    Up,
    Down,
    Left,
    Right
}

// Enums with values
enum Movement{
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32)
}
