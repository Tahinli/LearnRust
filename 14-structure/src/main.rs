//this is how we implement struct
struct Person
    {
        id: u8,
        tel: u8,
        name: String,
        alive: bool,
    }

//they're tuple struct
//they might seem like same but different
struct RGB(u8,u8,u8);
struct Coordinates(u8,u8,u8);

//this is unit-like struct
//i don't know what to do now, but seems like it's about "trait"
struct UnitLike;

fn add_person(id:u8, tel:u8, name:String, alive:bool) ->Person
    {
        //if struct variables and funciton variables has same name
        //no need to specify again
        Person { id, tel, name, alive }
    }
fn main() 
    {
        println!("Hello, world!");

        let mut person1 = Person 
            {
                id : 101,
                tel : 111,
                name : String::from("Ahmet"),
                alive : false,
            };
        
        person1.name = String::from(person1.name+" Kaan");
        println!("{}\n{}\n{}\n{}", person1.id, person1.tel, person1.name, person1.alive);

        let person2 = Person
            {
                tel : 112,
                ..person1
            };

        //person.name is going to be problem. because it's moved already
        //println!("{}\n{}\n{}\n{}", person1.id, person1.tel, person1.name, person1.alive);
        println!("{}\n{}\n{}\n{}", person2.id, person2.tel, person2.name, person2.alive);
        
        //we calls our function to assign values
        let person3 = add_person(113, 114, String::from("Duck"), true);

        println!("{}\n{}\n{}\n{}", person3.id, person3.tel, person3.name, person3.alive);
        
        let color1 = RGB(1,2,3);
        let location1 = Coordinates(4,5,6);

        println!("{}{}{}", color1.0, color1.1, color1.2);
        println!("{}{}{}", location1.0, location1.1, location1.2);
        
        //I don't know how to use
        //that's why i used underscore to ignore compiler complaints
        let _new_unit = UnitLike;

    }
