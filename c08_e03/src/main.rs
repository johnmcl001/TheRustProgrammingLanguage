use std::collections::HashMap;
use std::io::stdin;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();

    loop {
        input.clear();
        println!("Enter dept:");
        let department = match stdin().read_line(&mut input) {
            Ok(_) => employees.entry(input.trim().to_string()).or_insert(vec![]),
            Err(e) => panic!("{}", e),
        };
        input.clear();
        println!("Enter employee:");
        match stdin().read_line(&mut input) {
            Ok(_) => department.push(input.trim().to_string()),
            Err(e) => panic!("{}", e),
        };
        println!("{:#?}", employees);
        input.clear();
        println!("Quit? (Y/N): ");
        match stdin().read_line(&mut input) {
            Ok(_) => if input.trim().to_string() == "Y" || input.trim().to_string() == "y" { break },
            Err(e) => panic!("{}", e), 
        }
    }

    println!("{:#?}", employees);
    Ok(())
}