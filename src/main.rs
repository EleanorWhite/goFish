extern crate termion;
extern crate rand;

//use termion::event::Key;
//use termion::input::TermRead;
//use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use rand::Rng;
//use termion::async_stdin;

//enum Cards {
//	ACE,
//	TWO,
//	THREE,
//	FOUR,
//	FIVE,
//	SIX,
//	SEVEN,
//	EIGHT,
//	NINE,
//	TEN,
//	JACK,
//	QUEEN,
//	KING
//}

const NUM_RANKS: i32 = 14;
const ranks: [&'static str; NUM_RANKS as usize] = ["ace", "one", "two", "three", "four", "five", "six",
	"seven", "eight", "nine", "ten", "jack", "queen", "king"];
const NUM_SUITES: i32 = 4;
const NUM_HAND: i32 = 5; // number of cards in a hand
const NUM_CARDS: i32 = NUM_SUITES*NUM_RANKS;

	
fn shuffle(mut cards: &mut Vec<String>) {
	let mut rng = rand::thread_rng();
	let mut tmp_deck = Vec::<String>::new();
	
	for i in 0..NUM_CARDS {
		let card = cards.remove(rng.gen_range(0, NUM_CARDS-i)as usize);
		tmp_deck.push(card);
	}
	cards.extend_from_slice(&tmp_deck);
	assert!(cards.len() == NUM_CARDS as usize);
}

fn init_deck() -> Vec<String> {
	let mut deck = Vec::<String>::new();
	for card in 0..ranks.len() {
		for suite in 0..NUM_SUITES {
			deck.push(ranks[card].to_string());
		}
	}
	return deck;
}

fn card_index(card: &String) -> Option<usize> {
	for i in 0..ranks.len() {
		if (ranks[i] == card) {
			return Some(i);
		}
	}
	return None;
}

fn sort_hand(hand: &mut Vec<String>) {
	for i in 0..hand.len() {
		let mut min = i;
		let mut min_val; // = card_index(hand[min])
		match card_index(&hand[min]) {
			None    => { println!("Found unknown card"); return; },
			Some(x) => min_val = x
		}
		for j in i..hand.len() {
			let new;
			match card_index(&hand[j]) {
				None    => { println!("Found unknown card"); return; },
				Some(x) => new = x
			}
			if (new < min_val) {
				min = j;
				//let min_val; // = card_index(hand[min])
				match card_index(&hand[min]) {
					None    => { println!("Found unknown card"); return; },
					Some(x) => min_val = x
				}
			}
		}
		hand.swap(i, min);
	}
}


fn deal_hand(deck: &mut Vec<String>, mut hand: &mut Vec<String>) {
	for i in 0..NUM_HAND {
		let card = deck.pop();
		match card {
			None => println!("ERROR: ran out of cards to deal"),
			Some(x) => hand.push(x)
		}
	}
	sort_hand(&mut hand);
}


fn deal_cards() -> (Vec<String>, Vec<String>, Vec<String>) {
	let mut deck = init_deck();
	shuffle(&mut deck);
	println!("{:?}", deck);
	let mut player_hand = Vec::<String>::new();
	deal_hand(&mut deck, &mut player_hand);  
	
	let mut comp_hand = Vec::<String>::new();
	deal_hand(&mut deck, &mut comp_hand); 
	
	return (deck, player_hand, comp_hand);
	
}

fn player_turn(mut playing_hand: &mut Vec<String>, mut playing_finished: &mut Vec<String>, 
	mut other_hand: &mut Vec<String>, mut deck: &mut Vec<String>) {
	println!("Your Turn!");
	if (playing_hand.is_empty()) {
		if (deck.is_empty()) { // game is over
			return;
		}
		println!("Playing hand is empty! Drawing a card from deck!");
		draw_card(&mut playing_hand, &mut playing_finished, &mut deck, true);
		return;
	}
		
	println!("make a guess!");
	let mut buffer = &mut String::new();
	std::io::stdin().read_line(buffer);
	let mut guess = buffer.trim_right().to_string();
		
	while (!card_in(playing_hand, &guess)) { 
		println!("You guessed {} which you don't have! That's Cheating!", guess);
		println!("make another guess!");
		buffer.clear();
		guess.clear();
    	std::io::stdin().read_line(buffer);
		guess = buffer.trim_right().to_string();
	} 
	let correct = turn(&mut playing_hand, &mut playing_finished, &mut other_hand, 
		 &mut deck, guess.trim_right().to_string(), true);
	// guessing correctly means you go again
	if (correct) {
		println!("Your hand: {:?}, completed: {:?}", playing_hand, playing_finished);
		player_turn(&mut playing_hand, &mut playing_finished, &mut other_hand, &mut deck);
	}
}

fn comp_turn(mut player_hand: &mut Vec<String>, mut comp_hand: &mut Vec<String>, 
	mut comp_finished: &mut Vec<String>, mut deck: &mut Vec<String> ) {
		
	if (comp_hand.is_empty()) {
		if (deck.is_empty()) { // game is over
			return;
		}
		println!("Computer has no cards! Drawing a card from the deck!");
		draw_card(&mut comp_hand, &mut comp_finished, &mut deck, false);
		return;
	}
	
	let mut rng = rand::thread_rng();
	let guess_opt = comp_hand.get(rng.gen_range(0, comp_hand.len()));
	let guess: String;
	
	match guess_opt {
		None => {
			println!("ERROR: Failed to get guess from Computer.");
			return;
		}
		Some(x) => { 
			guess = x.clone()
		}
	}
	println!("Computer's turn! Computer guessed: {}", guess);
	let correct = turn(&mut comp_hand, &mut comp_finished, &mut player_hand, 
		 &mut deck, guess.trim_right().to_string(), false);	
	// guessing correctly means you go again
	if (correct) {
		comp_turn(&mut player_hand, &mut comp_hand, &mut comp_finished, &mut deck);
	}
	
}

fn card_in(hand: &Vec<String>, card: &String)-> bool {
	for c in hand {
		if (c == card) {
			return true;
		}
	}
	return false;
}

fn turn(mut playing_hand: &mut Vec<String>, mut playing_finished: &mut Vec<String>, 
	mut other_hand: &mut Vec<String>, mut deck: &mut Vec<String>, guess: String, is_player: bool) -> bool {
	let mut correct = false;
		
	if (card_in(other_hand, &guess)) {
		println!("Correct!");
		correct = true;
		
		// move card into other hand
		for c in other_hand.iter() {
			if (c == &guess) {
				playing_hand.push(c.to_string());
			}
		}
		other_hand.retain(|x| x != &guess);
		
		has_finished(&mut playing_hand, &mut playing_finished, &guess);
	} else {
		println!("Go Fish!");
		draw_card(&mut playing_hand, &mut playing_finished, &mut deck, is_player)
	}
	sort_hand(&mut playing_hand);
	sort_hand(&mut playing_finished);
	return correct;
}

fn draw_card(mut hand: &mut Vec<String>, mut finished: &mut Vec<String>, deck: &mut Vec<String>, is_player: bool) {
	let draw = deck.pop();
	match draw {
		None => println!("No More Cards Left!!"),
		Some(x) => { hand.push(x.clone());
					 has_finished(&mut hand, &mut finished, &x);
					 if (is_player) { println!("Drew {}", &x); }
				}
	}
}

fn has_finished(hand: &mut Vec<String>, finished: &mut Vec<String>, guess: &String) {
	// take out any full ranks
		let mut count = 0;
		for c in hand.iter() {
			if (c == guess) {
				count = count + 1;
			}
		}
		if (count == 4) {
			hand.retain(|x| x != guess);
			finished.push(guess.to_string());
		}
}

fn game_over(player_finished: &Vec<String>, comp_finished: &Vec<String>) {
	if (comp_finished.len() > player_finished.len()) {
		println!("Computer Wins!");
	} else { // player wins ties
		println!("You win!");
	}
	println!("Your stacks: {:?}, Computer's stacks: {:?}", player_finished, comp_finished);
}

fn main() {
	let (mut deck, mut player_hand, mut comp_hand) = deal_cards();
	let mut player_finished = Vec::<String>::new();
	let mut comp_finished   = Vec::<String>::new();
	
	// check if either side has a finished stack
	for c in player_hand.clone() {
		has_finished(&mut player_hand, &mut player_finished, &c);
	}
	for c in comp_hand.clone() {
		has_finished(&mut comp_hand, &mut comp_finished, &c);
	}

	while true {
		if (deck.is_empty() && player_hand.is_empty()) {
			game_over(&player_finished, &comp_finished);
			return;
		}
		println!("Your hand: {:?}, completed: {:?}", player_hand, player_finished);
		println!("Computer's completed: {:?}", comp_finished);
		player_turn(&mut player_hand, &mut player_finished, &mut comp_hand, &mut deck);
		comp_turn(&mut player_hand, &mut comp_hand, &mut comp_finished, &mut deck);
	}
}


