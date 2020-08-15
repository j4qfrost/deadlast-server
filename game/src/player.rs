use crate::card_container::*;
use crate::card::*;
use crate::action::*;

struct Player {
	color: Color,
	voting_cards: Hand<VotingCard>,
	gold_bars: Hand<GoldBarCard>,
	showdown_cards: Hand<FinalShowdownCard>,
	voting_slot: VotingSlot,
	showdown_slot: ShowdownSlot,
}

impl Player {
	pub fn new(color: Color) -> Self {
		Self {
			color,
			voting_cards: Hand<VotingCard>::new(),
			gold_bars: Hand<GoldBarCard>::new(),
			showdown_cards: Hand<FinalShowdownCard>::new(),
			voting_slot: VotingSlot::new(),
			showdown_slot: ShowdownSlot::new(),
		}
	}

	pub fn vote(&mut self, card: VotingCard) -> Action<Color, VotingCard> {
		self.voting_cards.play(card, self.voting_slot);
		Action<Color, VotingCard>::new(self.color, "vote", card)
	}

	pub fn showdown(&mut self, card: FinalShowdownCard) -> Action<Color, FinalShowdownCard> {
		self.showdown_cards.play(card, self.showdown_slot);
		Action<Color, VotingCard>::new(self.color, "showdown", card)
	}

	pub fn reveal<T>() -> Action<Color, Card<T>> {

	}

	pub fn score(&self) -> u8 {
		self.gold_bars.calculate_score()
	}

	pub fn eliminate_vote(&mut self, card: VotingCard) {
		self.voting_cards.remove_item(card);
	}
}