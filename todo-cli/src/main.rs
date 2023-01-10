// imports
use std::{collections::HashMap, io::Read, str::FromStr};



fn main() {
    // println!("Hello, world!");
    let action = std::env:: args().nth(1).expect("Please specificy an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    println!("{:?}, {:?}", action, item);

    // let mut todo = Todo {
    //     map: HashMap::new(),
    // };
    let mut todo = Todo::new().expect("Init of DB failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}",why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }//end elif
    
}//end main


// begin objects
struct Todo {
    // use rust built in HashMap to store key/val pairs
    map: HashMap<String, bool>,
}//end struct Todo

impl Todo {

    fn insert(&mut self, key: String) {
        // insert a new item into our map and pass bool
        // val true
        self.map.insert(key, true);
    }//end fn insert
    
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for(k, v) in self.map {
            let record = format!{"{}\t{}\n", k,v};
            content.push_str(&record)
        }
        std::fs::write("db.txt",content)
    }//end fn save

    fn new() -> Result<Todo, std::io::Error> {
        // this is a FUNCTIONAL implementation of
        // inserting data into a file
        // see the same code listed below for a for loop
        // variation of the same shit
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        
        let mut content = String::new();

        f.read_to_string(&mut content)?;
        
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line|line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        
            Ok(Todo {map})
    }//end fn new


    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }//end fn complete

    //ALTERNATIVE IMPLEMENTATION OF fn new() w/ for loop
    // fn new() -> Result<Todo, std::io::Error> {
    //     //open db
    //     let mut f = std::fs::OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .read(true)
    //         .open("db.txt")?;
    //     // read content into string
    //     let mut content = String::new();
    //     f.read_to_string(&mut content)?;

    //     // allocate an empty hashmap
    //     let mut map = HashMap::new();

    //     // loop over lines in file
    //     for entries in content.lines() {
    //         //split and bind
    //         let mut values = entries.split('\t');
    //         let key = values.next().expect("No Key");
    //         let val = values.next().expect("No Value");
    //         // insert into map
    //         map.insert(String::from(key), bool::from_str(val).unwrap());
    //     }
    //     // return Ok
    //     Ok(Todo {map})
    // } // end fn new for impl

}//end impl Todo