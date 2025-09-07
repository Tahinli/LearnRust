#[derive(Debug)]
enum Fruits {
    Apple,
    Banana,
    Strawberry,
}
enum Cake {
    Chocolate,
    Caramel,
    Fruit(Fruits),
    Tahini,
    Butter,
    Vanillin,
}
fn main() {
    println!("Hello, world!");
    let cake1 = Cake::Chocolate;
    let cake2 = Cake::Caramel;
    let cake3 = Cake::Fruit(Fruits::Apple);
    let cake4 = Cake::Fruit(Fruits::Banana);
    let cake5 = Cake::Fruit(Fruits::Strawberry);
    let cake6 = Cake::Tahini;
    let cake7 = Cake::Butter;
    let cake8 = Cake::Vanillin;

    let cakes = [cake1, cake2, cake3, cake4, cake5, cake6, cake7, cake8];
    for i in cakes {
        // our dear match always warn us to cover all possibilities
        // they are like switch case but better I think
        // they can be useful with enums
        match i {
            Cake::Chocolate => println!("Chocolate"),
            Cake::Caramel => println!("Caramel"),
            Cake::Fruit(name) => println!("Fruit {:#?}", name),
            Cake::Tahini => println!("Tahini"),
            // this one is wildcard
            // but when we don't want to use variable
            // we can use it to cover all possibilities
            // that's why it's the last one
            _ => println!("Others!"),
        }
    }
}
