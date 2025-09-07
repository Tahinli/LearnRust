// this is kind of null implemetation
// but better
// because we have to specify type
// and compiler will make sure we can not be able to treat as a non null
// <T> is generic type parameter we will learn later
// for now it can accept any type of data
// "T" can be everyting
#[derive(Debug)]
enum Option <T>
    {
        None,
        Some(T),
    }

// this feels like event-driven programming
// in this code, we basically have led states
// that they have different type of associations
// On state just keeps rgb color
// Blink and Breath keeps rgb and time value
// Off keeps nothing
#[derive(Debug)]
enum LedState
    {
        On(u8,u8,u8),
        Blink(u8,u8,u8,u16),
        Breath(u8,u8,u8,u16),
        Off,
    }
impl LedState
    {
        // this is also method, like we did in structs
        fn getter(&self) -> &Self
            {
                self
            }
    }

fn main() 
    {
        println!("Hello, world!");

        let green_light = LedState::On(0,255,0);
        let red_breathing = LedState::Breath(255,0,0, 10);
        let blue_blinking = LedState::Blink(0,0,255, 5);
        let black = LedState::Off;

        println!("State is = {:#?}", LedState::getter(&green_light));
        println!("State is = {:#?}", LedState::getter(&red_breathing));
        println!("State is = {:#?}", LedState::getter(&blue_blinking));
        println!("State is = {:#?}", LedState::getter(&black));

        let null:Option<u8> = Option::None;
        let two = Option::Some(2);

        println!("Value is = {:#?}", null);
        println!("Value is = {:#?}", two);

        // this is not going to work
        // until we convert it
        //println!("Total is = {}", 2+two);


    }
