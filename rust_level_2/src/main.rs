fn main() {
    owenership();

    let s = String::from("faizan");
    takes_ownership(s);

    // println!("{}", s); this will give an error because s is moved to takes_ownership function

    let take_ownership = gives_ownership();
    println!("{:?}", take_ownership);

    // Borrowing
    borrowing_immutable_refs();
    borrowing_mutable_refs();

    // Dereferencing
    println!("Dereferencing");
    dereferencing();
}

/* 
    1) Owenership
    2) Borrowing
    3) Dereferencing
*/

// onwership in rust is a unique feature that makes rust a safe language.
// In rust, each value has a variable that is called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
// The ownership rules apply to non-primitive types like String, Vec, etc.

fn owenership(){
    let s = String::from("faizan");
    // let s1 = s; this will give an error because s is moved to s1
    let s1 = s.clone(); // this will create a deep copy of s
    println!("{:?}, {:?}", s, s1);
}

// ownership in functions

fn takes_ownership(s: String){
    println!("{:?}", s);
}

fn gives_ownership()-> Vec<i32>{
    let v = vec![1,2,3];
    v
}

// References must always be valid
// References are immutable by default
// References are passed by borrowing

// Borrowing

// Borrowing is a way to pass a reference
// Borrowing allows you to use a value without taking ownership of it
// Borrowing is done by using & symbol
// One mutable reference or multiple immutable references can be created at a time

fn borrowing_immutable_refs(){
    let s = String::from("faizan");
    let s1 = &s;
    let s2 = &s;
    println!("{:?}, {:?}, {:?}", s, s1, s2);
}

fn borrowing_mutable_refs(){
    let mut s = String::from("faizan");
    let s1 = &mut s;
    // let s2 = &mut s;
    // println!("{:?}, {:?}, {:?}", s, s1, s2); // this will give an error because we can't have multiple mutable references
    println!("{:?}", s1);
}

// Dereferencing
fn dereferencing(){
    let mut s = 1;
    let s1 = &mut s;
    let mut s2 = *s1;
    println!("s1 : {:?}, s2 : {:?}", s1, s2);
    *s1=444;

    println!("s1 after {:?}", s1);
    println!("s after {:?}", s);

}