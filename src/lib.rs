mod actions;

pub mod libs{
    use crate::actions::{add_to_list, clean, mark_done, printlist, remove_from_list};

    pub fn process_input(args : Vec<String>){
        if args[1] == "list"{
            printlist();
        }
        else if args[1] == "add"{
            add_to_list(args[2].clone());
        }
        else if args[1] == "remove"{
            remove_from_list(args[2].parse::<i32>().expect("Wrong input..."));
        }
        else if args[1] == "done"{
            mark_done(args[2].parse::<i32>().expect("Wrong input..."));
        }else if args[1] == "clean"{
            clean();
        }
        else{
            println!("Invalied command detected...");
        }
    }
}
