fn main(){
    // Variables and Mutability
    let a = 2;
    let mut b = a;
    b = 4;

    // Constants
    const MAX_POINTS : u32 =500_000;

    // Shadowing
    let x = 1;
    let x = x + 3;
    print!("The value of x is: {}\n", x);

    // Primitive Data Types
    let a : u16 = 1; // unsigned 16-bit integer
    let b : i32 = -1; // signed 32-bit integer

    let c : f32 = 1.9; // 16-bit floating point

    let d : bool = true; // boolean

    let e : char = 'a'; // character

    

    // Compound Data Types

    // &str and string
    let sentence = "Hello, World!"; // fixed sized string
    let mut sentence_1 = String::from("hello"); // dynamic sized string
    sentence_1.push_str(" world!");

    print!("{:?}\n", sentence_1);

    // Tuple
    let tup = (1, "hello",4.5);
    let (x,y,z) = tup;
    print!("{:?}\n", tup);

    // Unit
    let unit = ();

    // Arrays
    let mut arr = [1,2,3,4,5];
    let first = arr[0];
    print!("{:?}\n", first);
    arr[1] = 10;
    print!("{:?}\n", arr);

    // Vectors
    let mut vec: Vec<&str> =vec!["hello", "world"];
    print!("vec before pushing: {:?}\n", vec);
    vec.push(" -> pushing after world");
    print!("vec after pushing: {:?}\n", vec);

    // Aliasing
    type MyInt = i32;
    let f : MyInt = 1;

    type MyTuple = (i32, f32, char);
    let g : MyTuple = (1, 1.0, 'a');

    type MyArray = [i32; 5];
    let h : MyArray = [1,2,3,4,5];

    type MyVec = Vec<i32>;
    let mut i : MyVec = vec![1,2,3,4,5];

}