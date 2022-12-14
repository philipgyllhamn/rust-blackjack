use rand::prelude::*;
use std::io::stdin;

#[derive(Debug)]
#[derive(PartialEq)]
enum Value{
    Ace,
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
}

impl Value{
    fn get_value(num: i32) -> Value{
        match num{
            1 => Value::Ace,
            2 => Value::Two,
            3 => Value::Three,
            4 => Value::Four,
            5 => Value::Five,
            6 => Value::Six,
            7 => Value::Seven,
            8 => Value::Eight,
            9 => Value::Nine,
            10 => Value::Ten,
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            _ => panic!("Invalid Value"),
        }
    }

    fn get_worth(&self) -> i32{
        match self{
            Value::Ace => 11,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
        }
    }
}

#[derive(Debug)]
enum Color{
    Spade,
    Heart,
    Diamond,
    Club
}

impl Color{
    fn get_color(num: i32) -> Color{
        match num{
            1 => Color::Spade,
            2 => Color::Heart,
            3 => Color::Diamond,
            4 => Color::Club,
            _ => panic!("Invalid color"),
        }
    }
}

#[derive(Debug)]
struct Card{
    value: Value,
    color: Color
}

impl Card{
    fn new(value: Value, color: Color) -> Card{
        Card{
            value,
            color
        }
    }
}

#[derive(Debug)]
struct Deck{
    cards: Vec<Card>
}

impl Deck{
    fn new() -> Deck{
        Deck{
            cards: Vec::with_capacity(52)
        }
    }

    fn shuffle_deck(&mut self){
        let mut rng = rand::thread_rng();
        let mut index = self.cards.len();
        while index != 0{
            let rand_index = rng.gen_range(0..index);
            index -= 1;
            self.cards.swap(index, rand_index);
        }
    }

}

struct Player{
    hand: Vec<Card>,
    value: i32,
    money: i32,
    bet: i32,
}

impl Player{

    fn bet_money(&mut self, amount: i32){

        if amount > self.money{
            println!("You don't have enough money to bet that much!");

            return;
        }

        self.money -= amount;
        self.bet = amount;
    }

    fn give_new_hand(&mut self, deck: &mut Deck){
        self.hand.push(pop_at_random(deck));
    }

    fn display_hand(&mut self){
        print!("Your hand: ");
        let mut sum_value = 0;


        let mut counter:usize = 0;
        for card in &self.hand{
            print!("{:?} of {:?}s", card.value, card.color);
            
            if self.hand.len() > 0 && counter != self.hand.len() - 1 {
                print!(" and ");
            }

            if card.value == Value::Ace{
                if sum_value + card.value.get_worth() > 21 {
                    sum_value += 1;
                } else {
                    sum_value += 11;
                }
            }else{
                sum_value += card.value.get_worth();
            }

            counter += 1;
        }

        self.value = sum_value;

        println!("\nYOUR VALUE: {:?}", self.value);
    }
}


struct Dealer{
    hand: Vec<Card>,
    value: i32
}

impl Dealer{

    fn give_new_hand(&mut self, deck: &mut Deck){
        self.hand.push(pop_at_random(deck));
    }

    fn display_hand(&mut self){
        print!("Dealer's hand: ");
        let mut sum_value = 0;

        let mut counter:usize = 0;
        for card in &self.hand{
            print!("{:?} of {:?}s", card.value, card.color);
            
            if self.hand.len() > 0 && counter != self.hand.len() - 1 {
                print!(" and ");
            }

            // TODO: Fix ace value
            if card.value == Value::Ace{
                if sum_value + card.value.get_worth() > 21 {
                    sum_value += 1;
                } else {
                    sum_value += 11;
                }
            }else{
                sum_value += card.value.get_worth();
            }

            counter += 1;
        }

        self.value = sum_value;

        println!("\nDEALERS VALUE: {:?}", self.value);
    }
}

// HELPERS //

// removes random card from deck and returns it
fn pop_at_random(deck: &mut Deck) -> Card{

    let random_index = rand::thread_rng().gen_range(0..=deck.cards.len() - 1);

    let card = deck.cards.remove(random_index);
    
    card
}

// builds a deck of 52 cards and shuffles it
fn build_deck() -> Deck{
    let mut deck = Deck::new();
    for i in 1..=4{
        for j in 1..=13{
            let card = Card::new(Value::get_value(j), Color::get_color(i));
            deck.cards.push(card);
        }
    }

    deck.shuffle_deck();

    deck
}

//////////
// MAIN //
//////////
fn main() {
    let mut player:Player = Player{hand: Vec::new(), value: 0, money: 100, bet: 0};
    let mut dealer:Dealer = Dealer{hand: Vec::new(), value: 0};
    
    
    while player.money > 0 {

        player.bet = 0;

        while player.bet == 0{
            bet_menu(&mut player);
        }


        blackjack_loop(&mut player, &mut dealer)
    }

    println!("You have no more money. Game over.");
}

fn bet_menu(player: &mut Player){
    println!("\nYou have ${:?}", player.money);
    println!("How much would you like to bet?");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let bet = input.trim().parse::<i32>().expect("Please type a number");
    player.bet_money(bet);
}

// GAME LOOP //
fn blackjack_loop(player: &mut Player, dealer: &mut Dealer){
    let mut deck:Deck = build_deck();

    player.hand.clear();
    dealer.hand.clear();

    player.give_new_hand(&mut deck);
    player.give_new_hand(&mut deck);
    dealer.give_new_hand(&mut deck);
    dealer.give_new_hand(&mut deck);

    dealer.display_hand();
    player.display_hand();

    let bust_or_bj: bool = check_bust_bj(player, dealer);

    if bust_or_bj{
        return;
    }

    while player.value < 21 && deck.cards.len() > 0 && dealer.value < 21{
        println!("Your bet: ${:?}", player.bet);
        println!("\nHit or Stand?");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "hit"{
            player.give_new_hand(&mut deck);
            player.display_hand();
        }
        else if input == "stand"{
            break;
        }
        else{
            println!("Invalid input");
        }
    }

    if player.value < 22 {
        dealer_ai(&mut deck, dealer, player);
    }

    display_results(player, dealer);

}

// PLAYS DEALER AI AT THE END //
fn dealer_ai(deck: &mut Deck, dealer: &mut Dealer, player: &mut Player){

    println!("\nDealer's turn");
    println!("-----------------------------------------------------");


    while dealer.value <= 21 {

        let rng = rand::thread_rng().gen_range(1..=5);

        if dealer.value >= player.value {
            break;
        }else if player.value > dealer.value{
            dealer.give_new_hand(deck);
            dealer.display_hand();
        }

        if dealer.value > 19 && rng == 1{
            dealer.give_new_hand(deck);
            dealer.display_hand();
        }else if (dealer.value < 19 && dealer.value > 10) && (rng == 2 || rng == 3 || rng == 4){
            dealer.give_new_hand(deck);
            dealer.display_hand();
        }else if dealer.value < 12 && dealer.value > 1 {
            dealer.give_new_hand(deck);
            dealer.display_hand();
        }else if rng == 5{
            break;
        }
        
    }

}

// CHECKS IF PLAYER OR DEALER BUSTED OR GOT BLACKJAC//
fn check_bust_bj(player: &mut Player, dealer: &mut Dealer) -> bool{
    if player.value > 21 {
        println!("You busted!");
        return true;
    }
    else if dealer.value > 21 {
        println!("Dealer busted!");
        player.money += player.bet * 2;
        return true;
    }
    else if dealer.value == 21 && player.value != 21 {
        println!("Dealer got Blackjack!!");
        return true;
    }
    else if player.value == 21 && dealer.value != 21 {
        println!("You got Blackjack!!");
        player.money += player.bet * 3;
        return true;
    }

    false
}

// DISPLAYS FINAL RESULTS //
fn display_results(player: &mut Player, dealer: &mut Dealer){

    println!("-----------------------------------------------------");

    if player.value == 21 && dealer.value != 21{
        println!("You win!");
        player.money += player.bet * 2;

    }else if player.value > 21{
        println!("You busted! Dealer wins!");
    }
    else if player.value != 21 && dealer.value == 21{
        println!("Dealer wins!");
    }
    else if player.value > 21 && dealer.value > 21{
        println!("You both bust!");
    }
    else if player.value > 21 && dealer.value < 21{
        println!("Dealer wins!");
    }
    else if player.value < 21 && dealer.value > 21{
        println!("You win!");
        player.money += player.bet * 2;
    }
    else if player.value == dealer.value {
        println!("It's a tie!");
        player.money += player.bet;
    }
    else if player.value > dealer.value {
        println!("You win!");
        player.money += player.bet * 2;
    }
    else if player.value < dealer.value{
        println!("Dealer wins!");
    }
}
