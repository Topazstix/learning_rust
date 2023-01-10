fn main() {
    println!("Hello, world!");

    // if else statements:
    let a = 5;
    
    if a < 10 {
        println!("a less than 10");
    } else {
        println!("a greather than 10");
    }

    // conditional match statements
    // similar to switch
    //EXAMPLE/PSEUDO CODE
    // let expression_result = match main_value {

    //     value_to_match => { execution_statements },
    //     value_to_match => { execution_statements },
    //     value_to_match => { execution_statements },
        
    //     _ => { default_execution_statements}
    // };

    // match working example
    let grade = "B";
    let _result = match grade {
        "A" => { println!("You got an A! Great!"); },
        "B" => { println!("You got an B! Good job!"); },
        "C" => { println!("You got an C! Eh, you did alright"); },
        "D" => { println!("You got an D! Passing"); },
        "F" => { println!("You got an F! Failure"); },
        
        _ => { println!("Unknown grade; see the teacher"); },
    };


    // if let statements
    let is_birthday = false;

    let message = if is_birthday {
        "Happy birthday"
    } else {
        "No cake for you!"
    };
    println!("Message: {}", message);


    //while loop
    let mut count = 0;

    while count < 10 {
        println!("Counter: {}", count);
        count +=1;
    }

    //for loop
    for x in 0..10 {
        println!("{}", x);
    }

    // for each // for loops for arrays/collections
    let employees = ["John", "Jane", "Jack", "jill"];

    for x in employees.iter() {
        println!("{}",x);
    }


    let number = 512;
    let freight = String::from("parrots");
    let destination = String::from("Washington DC");
    

    let testcar = RailCar{number, freight, destination};
    
    // {number:512,freight:String::from("Parrots"),destination:String::from("Washington DC")};

    println!("{}",testcar.to_string());

}

#[derive(Clone)]
struct RailCar {
    number:i16,
    freight:String,
    destination:String,
}//end railcar

struct Cursor<RailCar> {
    current: LinkedList<RailCar>
}//end cursor

enum LinkedList<RailCar> {
    None,
    Head { item: RailCar},
    LinkedList { item: RailCar, next: Box<LinkedList<RailCar>> },
}//end linked list

enum DoubleLinkedList<RailCar> {
    None,
    Head { item: RailCar},
    Tail { item: RailCar},
    DoubleLinkedList { item: RailCar,
         next: Box<DoubleLinkedList<RailCar>>, 
         previous: Box<DoubleLinkedList<RailCar>>, },
}//end double linked

// impl Copy for RailCar{}
impl RailCar  {
    // fn new(&mut self, number:i16, freight:String, destination:String) {
    //     self.number = number;
    //     self.freight = freight;
    //     self.destination = destination;
    // }//end constr

    fn get_destination(self) -> String {
        return self.destination;
    }

    fn get_freight(self) -> String {
        return self.freight;
    }

    fn to_string(self) -> String {
        return format!("{:<5} {:^5} {:>20}",self.number,self.freight,self.destination)
    }

    fn compare_to(self, other_destin:String) -> i8 {
        let one = String::from("West Palm Beach");
        let two = String::from("Orlando");
        let three = String::from("Charleston");
        let four = String::from("Washington DC");

        if self.get_destination().eq(&other_destin)  {
            return 0;
        } else if self.get_destination().eq(&four) {
            return -1;
        } else if self.get_destination().eq(&three) {
            if other_destin.eq(&one) || 
                    other_destin.eq(&two) {
                return -1;
            } else {
                return 1;
            }
        } else if self.get_destination().eq(&two) {
            if other_destin.eq(&one) {
                return -1
            } else {
                return 1;
            }
        } else {
            return 1;
        }

    }//end compare to



}