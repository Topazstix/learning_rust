use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

// Declare a mutable dictionary of type HashMap with the key 1 and value 'A'.
let mut my_dict: HashMap<i32, &str> = HashMap::new();
my_dict.insert(1, "A");

// Declare a mutable dictionary of type HashMap with the key 2 and value 123.
let mut my_dict: HashMap<i32, i32> = HashMap::new();
my_dict.insert(2, 123);

// Declare a mutable dictionary of type HashMap with the key "this string" and value 456.
let mut my_dict: HashMap<&str, i32> = HashMap::new();
my_dict.insert("this string", 456);

println!("{:?}", my_dict);

// Declare a mutable integer with the value 456.
let mut an_integer = 456;

// Declare a mutable set with the value 456.
let mut a_set: HashSet<i32> = HashSet::new();
a_set.insert(456);

// Add the value 1 to the set.
a_set.insert(1);

println!("Dictionary keys: {:?}\nDictionary values: {:?}\n", my_dict.keys(), my_dict.values());

// Iterate over the keys in the dictionary and print the key and value.
for key in my_dict.keys() {
    println!("the key is: {:?}.\t the value is: {:?}", key, my_dict[key]);
}

// Declare a mutable dictionary of sets with the key 0 and the set {1, 2, 3}.
let mut dict_of_sets: HashMap<i32, HashSet<i32>> = HashMap::new();
dict_of_sets.insert(0, HashSet::from_iter(vec![1, 2, 3]));

// Declare a mutable dictionary of sets with the key 1 and the set {4, 5, 6}.
let mut dict_of_sets: HashMap<i32, HashSet<i32>> = HashMap::new();
dict_of_sets.insert(1, HashSet::from_iter(vec![4, 5, 6]));

// Declare a mutable dictionary of sets with the key 2 and the set {7, 8, 9}.
let mut dict_of_sets: HashMap<i32, HashSet<i32>> = HashMap::new();
dict_of_sets.insert(2, HashSet::from_iter(vec![7, 8, 9]));

println!("{:?}", dict_of_sets);

// Iterate over the keys in the dictionary and print the key and value.
println!("\n\nFor loop 1:");
for key in dict_of_sets.keys() {
    println!("the key is: {:?}.\t the value is: {:?}", key, dict_of_sets[key]);
}

// Iterate over the keys in the dictionary and print the key and the length of the
