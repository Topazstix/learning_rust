fn main() {

    //instantiate the Cylinder object
    let cylinder = Cylinder{
        radius:1.0,
        height:2.0,
        unit: "cm".to_string(),
    };
    println!("Radius of cylinder: {} {}", cylinder.radius,cylinder.unit);
    println!("Height of cylinder: {} {}", cylinder.height,cylinder.unit);
    // println!("Hello, world!");

    let newcylinder = Cylinder::new(1.0,2.0);
    let triangle = Triangle{base:1.0,height:2.0};
    newcylinder.get_summary();
}//end main






// TRAITS
/* Traits only hold the behavior or methods of an objct.
 * these are similar to interfaces in other languages, shared
 * behaviors are abstracted in the traits
 * 
*/
trait Formula {
    fn get_area(&self) -> f32;
    fn get_perimeter(&self) -> f32;
}//end trait Formula


// STRUCTS
/* Structs are the equivalent of java/cpp classes.
 * they are user defined types that store
 * attributes/states of an object.
 * rust is NOT strictly OOP like java or cpp
 * 
*/
struct Cylinder { // pub modifier sets object public/ always private otherwise
    radius: f32,
    height: f32,
    unit: String, // remove unit in place of setting thru method
}//end struct Cylinder

struct Triangle {
    height: f32,
    base: f32,
}//end triangle

// IMPLEMENT THE TRAIT IN THE CYLINDER OBJT
const PI: f32 = 3.1415;
impl Formula for Cylinder {
    fn get_area(&self) -> f32 {
        return 2.0 * PI * self.radius * (self.height + self.radius)
    }

    fn get_perimeter(&self) -> f32 {
        return 2.0 * PI * self.radius
    }
    
}//end impl Formula for Cylinder

impl Formula for Triangle {
    fn get_area(&self) -> f32 {
        return 0.5 * PI * (self.base + self.height)
    }

    fn get_perimeter(&self) -> f32 {
        return PI
    }
}//end impl Formula Triangle

// IMPLEMENTS
/* Impl allows a struct to be given functions
 * 
*/
impl Cylinder {
    pub fn new(radius: f32, height: f32) -> Cylinder {
        Cylinder { 
            radius: radius, 
            height: height, 
            unit: "cm".to_string() 
        }
    }//end constructor

    pub fn get_summary(&self) {
        println!("Summary:");
        println!("r: {}{} | h: {}{}", self.radius, self.unit, self.height, self.unit);
        println!("area: {}\nperimeter: {}", self.get_area(), self.get_perimeter());
    }//end get summary

    pub fn get_unit(&self) -> &String {
        return &self.unit
    }//end get unit

    pub fn set_unit(&mut self, unit: String) {
        self.unit = unit;
    }//end set unit

}//end impl cylinder


