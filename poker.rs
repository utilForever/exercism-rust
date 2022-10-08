use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::HashMap,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
enum Category {
    HighCard(Hand),
    OnePair(Rank, Rank, Rank, Rank),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Hand),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(hand: &str) -> Self {
        let mut cards = Vec::new();

        for card in hand.split_whitespace() {
            let mut card = String::from(card);

            if card.starts_with("10") {
                card = card.replace("10", "T");
            }

            let suit = match &card[1..2] {
                "D" => Suit::Diamond,
                "C" => Suit::Club,
                "H" => Suit::Heart,
                "S" => Suit::Spade,
                _ => panic!("Invalid suit"),
            };

            let rank = match &card[0..1] {
                "2" => Rank::Two,
                "3" => Rank::Three,
                "4" => Rank::Four,
                "5" => Rank::Five,
                "6" => Rank::Six,
                "7" => Rank::Seven,
                "8" => Rank::Eight,
                "9" => Rank::Nine,
                "T" => Rank::Ten,
                "J" => Rank::Jack,
                "Q" => Rank::Queen,
                "K" => Rank::King,
                "A" => Rank::Ace,
                _ => panic!("Invalid rank"),
            };

            cards.push(Card { rank, suit });
        }

        cards.sort_by(|a, b| a.rank.cmp(&b.rank));

        Self { cards }
    }

    fn get_category(&self) -> Category {
        let is_suit_all_same = self
            .cards
            .iter()
            .all(|card| card.suit == self.cards[0].suit);
        let is_straight_normal = self
            .cards
            .windows(2)
            .all(|window| window[0].rank as i64 + 1 == window[1].rank as i64);
        let is_straight_baby = self.cards[0].rank == Rank::Two
            && self.cards[1].rank == Rank::Three
            && self.cards[2].rank == Rank::Four
            && self.cards[3].rank == Rank::Five
            && self.cards[4].rank == Rank::Ace;
        let is_straight = is_straight_normal || is_straight_baby;

        let ranks: HashMap<Rank, i64> = self.cards.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card.rank).or_insert(0) += 1;
            acc
        });
        let mut ranks = ranks
            .iter()
            .map(|(rank, count)| (*rank, *count))
            .collect::<Vec<(Rank, i64)>>();
        ranks.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

        // Check straight flush
        if is_suit_all_same && is_straight {
            if is_straight_baby {
                return Category::StraightFlush(Rank::Five);
            } else {
                return Category::StraightFlush(self.cards[4].rank);
            }
        }

        // Check four of a kind
        if ranks[0].1 == 4 {
            return Category::FourOfAKind(ranks[0].0, ranks[1].0);
        }

        // Check full house
        if ranks[0].1 == 3 && ranks[1].1 == 2 {
            return Category::FullHouse(ranks[0].0, ranks[1].0);
        }

        // Check flush
        if is_suit_all_same {
            return Category::Flush(self.clone());
        }

        // Check straight
        if is_straight {
            if is_straight_baby {
                return Category::Straight(ranks[1].0);
            } else {
                return Category::Straight(ranks[0].0);
            }
        }

        // Check three of a kind
        if ranks[0].1 == 3 {
            return Category::ThreeOfAKind(ranks[0].0, ranks[1].0, ranks[2].0);
        }

        // Check two pair
        if ranks[0].1 == 2 && ranks[1].1 == 2 {
            return Category::TwoPair(ranks[0].0, ranks[1].0, ranks[2].0);
        }

        // Check one pair
        if ranks[0].1 == 2 {
            return Category::OnePair(ranks[0].0, ranks[1].0, ranks[2].0, ranks[3].0);
        }

        // Check high card
        Category::HighCard(self.clone())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let convert = |category: Category| -> u8 {
            match category {
                Category::HighCard(_) => 1,
                Category::OnePair(_, _, _, _) => 2,
                Category::TwoPair(_, _, _) => 3,
                Category::ThreeOfAKind(_, _, _) => 4,
                Category::Straight(_) => 5,
                Category::Flush(_) => 6,
                Category::FullHouse(_, _) => 7,
                Category::FourOfAKind(_, _) => 8,
                Category::StraightFlush(_) => 9,
            }
        };

        let category = self.get_category();
        let other_category = other.get_category();
        let ret_compare = convert(category.clone()).cmp(&convert(other_category.clone()));

        match ret_compare {
            Ordering::Less | Ordering::Greater => Some(ret_compare),
            Ordering::Equal => {
                let ret = match (category, other_category) {
                    (Category::HighCard(hand), Category::HighCard(other_hand)) => {
                        Some(hand.cards.cmp(&other_hand.cards))
                    }
                    (
                        Category::OnePair(rank1, rank2, rank3, rank4),
                        Category::OnePair(other_rank1, other_rank2, other_rank3, other_rank4),
                    ) => Some(
                        rank1.cmp(&other_rank1).then(
                            rank2
                                .cmp(&other_rank2)
                                .then(rank3.cmp(&other_rank3).then(rank4.cmp(&other_rank4))),
                        ),
                    ),
                    (
                        Category::TwoPair(rank1, rank2, rank3),
                        Category::TwoPair(other_rank1, other_rank2, other_rank3),
                    ) => Some(
                        rank1
                            .cmp(&other_rank1)
                            .then(rank2.cmp(&other_rank2).then(rank3.cmp(&other_rank3))),
                    ),
                    (
                        Category::ThreeOfAKind(rank1, rank2, rank3),
                        Category::ThreeOfAKind(other_rank1, other_rank2, other_rank3),
                    ) => Some(
                        rank1
                            .cmp(&other_rank1)
                            .then(rank2.cmp(&other_rank2).then(rank3.cmp(&other_rank3))),
                    ),
                    (Category::Straight(rank), Category::Straight(other_rank)) => {
                        Some(rank.cmp(&other_rank))
                    }
                    (Category::Flush(hand), Category::Flush(other_hand)) => {
                        Some(hand.cards.cmp(&other_hand.cards))
                    }
                    (
                        Category::FullHouse(rank1, rank2),
                        Category::FullHouse(other_rank1, other_rank2),
                    ) => Some(rank1.cmp(&other_rank1).then(rank2.cmp(&other_rank2))),
                    (
                        Category::FourOfAKind(rank1, rank2),
                        Category::FourOfAKind(other_rank1, other_rank2),
                    ) => Some(rank1.cmp(&other_rank1).then(rank2.cmp(&other_rank2))),
                    (Category::StraightFlush(rank), Category::StraightFlush(other_rank)) => {
                        Some(rank.cmp(&other_rank))
                    }
                    _ => None,
                };

                ret.filter(|compare| matches!(compare, Ordering::Less | Ordering::Greater))
            }
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut ret = Vec::new();
    let mut win_hand = None;

    for hand_str in hands.iter().copied() {
        let hand = Hand::new(hand_str);

        match &win_hand {
            Some(cur_win_hand) => match hand.partial_cmp(cur_win_hand) {
                Some(order) => match order {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        ret.push(hand_str);
                    }
                    Ordering::Greater => {
                        win_hand = Some(hand);
                        ret = vec![hand_str];
                    }
                },
                None => {
                    ret.push(hand_str);
                }
            },
            None => {
                win_hand = Some(hand);
                ret = vec![hand_str];
            }
        }
    }

    ret
}
