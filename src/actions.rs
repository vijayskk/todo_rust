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
    println!("TODO LIST:");
    for i in todos{
        let messages : Vec<&str> = i.split(',').collect();
        if (messages.len() as i32) < 2{
            break;
        }
        let status = {
            if messages[1] == "0"{
                "Not Completed"
            }else{
                "Completed"
            }
        };
        println!("\t{}. {} ({})",index,messages[0],status);
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
pub fn remove_from_list(slno : i32){
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
    let todos : Vec<&str> =  s.split('\n').collect();
    drop(file);
    if todos.len() as i32 <= (slno) || slno < 1{
        println!("Wrong number");
        return;
    }
    let mut removed = String::new();
    let mut index = 1;
    for i in &todos{
        if index != slno+1{
            removed.push_str(i);
            if index != todos.len() as i32{
                removed.push('\n');
            }
        }
        index+= 1;
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("File opening failed...{} {}",display,why),
        Ok(file) => file
    };  
    match file.write_all(removed.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {println!("Item ({slno}) is removed.\n\n");printlist()},
    }
}

pub fn mark_done(slno : i32){
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
    if todos.len() as i32 <= (slno) || slno < 1{
        println!("Wrong number");
        return;
    }
    let mut edited = String::from(todos[(slno) as usize]);
    edited.push('1');
    todos[(slno) as usize] = edited.as_str();
    drop(file);

    let mut editedstr = String::new();
    let mut index = 1;
    for i in &todos{
        editedstr.push_str(i);
        if index != (todos.len() as i32){
            editedstr.push('\n');
        }
        index+= 1;
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("File opening failed...{} {}",display,why),
        Ok(file) => file
    };  
    match file.write_all(editedstr.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {println!("Item ({slno}) is marked done.\n\n");printlist()},
    }
}
pub fn clean(){
    let path = Path::new(FILENAME);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all("Message,isDoing".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("List is clean..."),
    }
}