// this is kind of null implementation
// but better
// because we have to specify type
// and compiler will make sure we can not be able to treat as a non null
// <T> is generic type parameter we will learn later
// for now it can accept any type of data
// "T" can be everything
#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

// this feels like event-driven programming
// in this code, we basically have led states
// that they have different type of associations
// On state just keeps rgb colour
// Blink and Breath keeps rgb and time value
// Off keeps nothing
#[derive(Debug)]
enum LedState {
    On(u8, u8, u8),
    Blink(u8, u8, u8, u16),
    Breath(u8, u8, u8, u16),
    Off,
}
impl LedState {
    // this is also method, like we did in structs
    fn getter(&self) -> &Self {
        self
    }

    fn export_values(&self) -> Vec<u16> {
        match self {
            LedState::On(r, g, b) => vec![*r as u16, *g as u16, *b as u16],
            LedState::Blink(r, g, b, effect_time) => {
                vec![*r as u16, *g as u16, *b as u16, *effect_time]
            }
            LedState::Breath(r, g, b, effect_time) => {
                vec![*r as u16, *g as u16, *b as u16, *effect_time]
            }
            LedState::Off => vec![],
        }
    }
}

fn main() {
    println!("Hello, world!");

    let green_light = LedState::On(0, 255, 0);
    let red_breathing = LedState::Breath(255, 0, 0, 10);
    let blue_blinking = LedState::Blink(0, 0, 255, 5);
    let black = LedState::Off;

    println!("State is = {:#?}", LedState::getter(&green_light));
    println!("State is = {:#?}", LedState::getter(&red_breathing));
    println!("State is = {:#?}", LedState::getter(&blue_blinking));
    println!("State is = {:#?}", LedState::getter(&black));

    println!("Values Are = {:#?}", green_light.export_values());
    println!("Values Are = {:#?}", red_breathing.export_values());
    println!("Values Are = {:#?}", blue_blinking.export_values());
    println!("Values Are = {:#?}", black.export_values());

    let null: Option<u8> = Option::None;
    let two = Option::Some(2);

    println!("Value is = {:#?}", null);
    println!("Value is = {:#?}", two);

    // this is not going to work
    // until we convert it
    // println!("Total is = {}", 2+two);
}
