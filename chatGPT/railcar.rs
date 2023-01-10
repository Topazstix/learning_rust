// TRANSLATED FROM CS 1450 ASSIGNMENT 9 
// Chatbot/openai

// Here is a possible Rust translation of the provided Java code:

struct RailCar {
    // RailCar

    number: i32,
    freight: String,
    destination: String,
}

impl RailCar {
    pub fn new(number: i32, freight: String, destination: String) -> RailCar {
        RailCar { number, freight, destination }
    }

    pub fn get_freight(&self) -> &str {
        &self.freight
    }

    pub fn get_destination(&self) -> &str {
        &self.destination
    }

    pub fn to_string(&self) -> String {
        format!("{}\t{:15}\t{:10}", self.number, self.freight, self.destination)
    }
}

impl PartialEq for RailCar {
    fn eq(&self, other: &Self) -> bool {
        self.destination == other.destination
    }
}

impl Eq for RailCar {}

impl PartialOrd for RailCar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // set strings for comparison
        const ONE: &str = "West Palm Beach"; // last stop
        const TWO: &str = "Orlando";         // second last stop
        const THREE: &str = "Charleston";    // second stop
        const FOUR: &str = "Washington DC";  // first stop

        if self.destination == other.destination {
            // if both cars are equal; return 0
            Some(std::cmp::Ordering::Equal)
        } else if self.destination == FOUR {
            // else if DC, return -1
            Some(std::cmp::Ordering::Less)
        } else if self.destination == THREE {
            // else if Charl:
            if other.destination == ONE || other.destination == TWO {
                // if other is >, return -1
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Greater)
            }
        } else if self.destination == TWO {
            // if orl.
            if other.destination == ONE {
                // if other >, return -1
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Greater)
            }
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

impl Ord for RailCar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// In Rust, structs are used to define custom data types, and their methods are defined using the impl keyword. In the provided code, RailCar implements the Comparable interface in Java, which means it defines a compareTo method. In Rust, this functionality is provided by the Ord and PartialOrd traits, which are implemented by the RailCar struct.

// Note that I've made a few changes to the code to make it more idiomatic Rust, such