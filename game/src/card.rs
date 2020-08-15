use serde;
#[macro_use]
use serde_derive;
use serde::{Deserialize, Serialize};
// use rmp_serde::{Deserializer, Serializer};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Color {
	Green,
	Black,
	Blue,
	Brown,
	Emerald,
	Grey,
	Orange,
	Pink,
	Purple,
	Red,
	Teal,
	Yellow,
	Ambush,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Showdown {
	Share,
	Steal,
	GrabOne,
}

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize)]
pub struct Card<T>(pub T);

impl<T> Card<T> {
	pub fn new(thing: T) -> Self {
		Self(thing)
	}
}

impl Copy for Card<Color> {
}

impl Copy for Card<u8> {
}

impl Copy for Card<Showdown> {
}

pub fn generate_all<T: IntoEnumIterator>() -> Vec<Card<T>> {
	let mut cards = Vec::new();
	for t in T::iter() {
		cards.push(Card::new(t));
	}
	cards
}

pub type VotingCard = Card<Color>;
pub type GoldBarCard = Card<u8>;
pub type FinalShowdownCard = Card<Showdown>;