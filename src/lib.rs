mod actions;

pub mod libs{
    use crate::actions::{add_to_list, printlist};

    pub fn process_input(args : Vec<String>){
        if args[1] == "list"{
            printlist();
        }
        else if args[1] == "add"{
            add_to_list(args[2].clone());
        }
        else{
            println!("Another command detected...");
        }
    }
}
