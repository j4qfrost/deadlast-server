use crate::card::*;
use rand;
use rand::Rng;

pub trait Slot<T> {
	fn new() -> Self {
		Self(None)
	}
	fn place_card(&mut self, card: Option<T>) -> Option<T>;
}

struct VotingSlot(Option<VotingCard>);

impl Slot<VotingCard> for VotingSlot {
	fn place_card(&mut self, card: Option<VotingCard>) -> Option<VotingCard> {
		if let Some(old_card) = self.0 {
			self.0 = card;
			Some(old_card)
		} else {
			self.0 = card;
			None
		}
	}
}

struct ShowdownSlot(Option<FinalShowdownCard>);

impl Slot<FinalShowdownCard> for ShowdownSlot {
	fn place_card(&mut self, card: Option<FinalShowdownCard>) -> Option<FinalShowdownCard> {
		if let Some(old_card) = self.0 {
			self.0 = card;
			Some(old_card)
		} else {
			self.0 = card;
			None
		}
	}
}

struct Hand<T> {
	cards: Vec<T>,
}

fn play<T: PartialEq>(hand: &mut Hand<T>, thing: T, slot: &mut Slot<T>) {
	let card = hand.cards.remove_item(&thing);
	if let Some(old_card) = slot.place_card(card) {
		hand.cards.push(old_card);
	}
}

impl Hand<VotingCard> {
	pub fn new(color: Color) -> Self {
		let mut cards = generate_all::<Color>();
		cards.remove_item(&Card::new(color));
		Self {
			cards,
		}
	}

	pub fn play(&mut self, card: VotingCard, slot: &mut Slot<VotingCard>) {
		play::<VotingCard>(self, card, slot);
	}
}

impl Hand<FinalShowdownCard> {
	pub fn new() -> Self {
		let cards = generate_all::<Showdown>();
		Self {
			cards,
		}
	}

	pub fn play(&mut self, card: FinalShowdownCard, slot: &mut Slot<FinalShowdownCard>) {
		play::<FinalShowdownCard>(self, card, slot);
	}
}

impl Hand<GoldBarCard> {
	pub fn new() -> Self {
		Self {
			cards: Vec::new(),
		}
	}

	pub fn calculate_score(&self) -> u8 {
		self.cards.iter().fold(0, |sum, card| sum + card.0 )
	}
}

struct GoldDeck {
	threes: u8,
	fours: u8,
	fives: u8,
}

impl GoldDeck {
	pub fn new() -> Self {
		Self {
			threes: 26,
			fours: 43,
			fives: 11,
		}
	}
	pub fn draw(&mut self) -> GoldBarCard {
		let total_cards = self.threes + self.fours + self.fives;

		let mut rng = rand::thread_rng();
    	let n: u8 = rng.gen_range(0, total_cards);

		if n < self.threes {
			self.threes -= 1;
    		GoldBarCard::new(3)
		} else if n < total_cards - self.fives {
			self.fours -= 1;
			GoldBarCard::new(4)
		} else {
			self.fives -= 1;
			GoldBarCard::new(5)
		}
	}
}