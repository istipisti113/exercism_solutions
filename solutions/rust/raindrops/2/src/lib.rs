pub fn raindrops(n: u32) -> String {
    //todo!("what sound does Raindrop #{n} make?")
    let mut sound = String::new();
    if n % 3 == 0 {
        sound += "Pling";
    }
    if n % 5 == 0 {
        sound += "Plang"
    }
    if n % 7 == 0 {
        sound += "Plong"
    }
    if sound == String::new() {
        sound = n.to_string();
    }
    sound
}
