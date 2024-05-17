fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // Destructure the cat tuple into name and age

    println!("{} is {} years old.", name, age);
}
