pub enum Card {
    SilentStrike,
    SilentDefend
}

pub struct CardInstance {
    pub card: Card,
    pub cost: u8,
    // secondary_cost: u8 // regent
}