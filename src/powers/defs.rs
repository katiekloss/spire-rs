use crate::powers::PowerImpl;

pub static AFTERIMAGE: PowerImpl = PowerImpl {
    id: "Afterimage",
    card_played: Some(|_, encounter| {
        encounter.player.block += 1;
    }),
};