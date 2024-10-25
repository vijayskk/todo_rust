use std::{fs::File, io::{Read, Write}, path::Path};

const FILENAME : &str = "data.csv";


pub fn printlist(){
    let path = Path::new(FILENAME);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("File opening failed...{} {}",display,why),
        Ok(file) => file
    };   

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!(""),
    }
    let mut todos : Vec<&str> =  s.split('\n').collect();
    todos.remove(0);
    let mut index = 1;
    for i in todos{
        let messages : Vec<&str> = i.split(',').collect();
        let status = {
            if messages[1] == "0"{
                "Not Completed"
            }else{
                "Completed"
            }
        };
        println!("{}. {} ({})",index,messages[0],status);
        index+= 1;
    }
}
pub fn add_to_list(item:String){
    let path = Path::new(FILENAME);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("File opening failed...{} {}",display,why),
        Ok(file) => file
    };   

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!(""),
    }
    drop(file);

    let mut file = match File::create(&path) {
        Err(why) => panic!("File opening failed...{} {}",display,why),
        Ok(file) => file
    };  
    s.push('\n');
    s.push_str(&item);
    s.push_str(",0");
    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("{item} is added."),
    }
}