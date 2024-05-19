fn main() {
    // Option
    let std1 = Student {
        name: "Faizan".to_string(),
        age: 22,
        phone: None,
    };

    match std1.print_info() {
        Ok(_) => println!("Student 1 information printed successfully."),
        Err(e) => println!("Failed to print student 1 information: {}", e),
    }
    let std2=Student{
        name:"".to_string(),
        age:22,
        phone:None,
    };

    match std2.print_info() {
        Ok(_) => println!("Student 2 information printed successfully."),
        Err(e) => println!("Failed to print student 2 information: {}", e),
    }

    // Hashmaps
    hashMaps();
}

/* 
    1) Options
    2) Result
    3) Hashmaps
*/

// Option and Result
struct Student {
    name: String,
    age: u8,
    phone: Option<String>,
}

impl Student {
    fn print_info(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name is empty".to_string());
        }

        let phone = match &self.phone {
            Some(phone) => phone,
            None => "No phone number",
        };
        println!("Name: {}, Age: {}, Phone: {}", self.name, self.age, phone);
        Ok(())
    }
}
use std::collections::HashMap;
// Hashmaps
fn hashMaps(){
    let mut marks = HashMap::new();
    marks.insert("Maths", "90");
    marks.insert("Science", "80");

    let maths = marks.get("Maths");

    match maths{
        Some(marks)=>println!("Marks in Maths: {}",marks),
        None=>println!("No marks found for Maths"),
    }

    marks.entry("English").or_insert("70");

    for i in marks{
        println!("{:?}",i);
    }
}
