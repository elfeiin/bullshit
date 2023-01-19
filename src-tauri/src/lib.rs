pub struct Action {

}

pub enum Rank {
    Ace,
    Pa,
    Re,
    Ci,
    Vo,
    Mu,
    Xa,
    Ze,
    Bi,
    So,
    Dau,
    Jack,
    Queen,
    King,
}

pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

pub struct Card {
    rank: Rank,
    suit: Suit,
}

pub struct Player {
    id: u8,
    name: String,
    hand: Vec<Card>,
}

pub struct Game {
    stack: Vec<Card>,
    turn: u8,
    previous_bluff: Vec<Card>,
    players: [Option<Player>; 10],
}
