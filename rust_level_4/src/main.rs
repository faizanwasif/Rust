fn main() {
    // Option
    let std1 = Student {
        name: "Faizan".to_string(),
        age: 22,
        phone: None,
    };

    match std1.print_info() {
        Ok(_) => println!("Student information printed successfully."),
        Err(e) => println!("Failed to print student information: {}", e),
    }
}

/* 
    1) Options
    2) Result
    3) Hashmaps
*/

// Option
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
