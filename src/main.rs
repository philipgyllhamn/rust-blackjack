use rand::prelude::*;

#[derive(Debug)]
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
    hand: Vec<Card>
}

impl Player{
    fn give_new_hand(&mut self, deck: &mut Deck){
        self.hand.push(pop_at_random(deck));
    }

    fn display_hand(&self){
        print!("Your hand: ");
        let mut sum_value = 0;

        // TODO: fix check so if counter is on last element it doesn't print 'and'
        let mut counter:usize = 0;
        for card in &self.hand{
            print!("{:?} of {:?}s", card.value, card.color);
            
            if &self.hand.len() > &0 && &counter != &self.hand.len() {
                print!(" and ");
            }

            sum_value += card.value.get_worth();
            counter += 1;
        }

        println!("\nYOUR VALUE: {:?}", sum_value);
    }
}

struct Dealer{
    hand: Vec<Card>
}

impl Dealer{
    fn give_new_hand(&mut self, deck: &mut Deck){
        self.hand.push(pop_at_random(deck));
    }

    fn display_hand(&self){
        print!("Dealer's hand: ");
        let mut sum_value = 0;

        let mut counter:usize = 0;
        for card in &self.hand{
            print!("{:?} of {:?}s", card.value, card.color);
            
            if &self.hand.len() > &0 && &counter != &self.hand.len() {
                print!(" and ");
            }

            sum_value += card.value.get_worth();
            counter += 1;
        }

        println!("\nDEALERS VALUE: {:?}", sum_value);
    }
}

// HELPERS //

// removes random card from deck and returns it
fn pop_at_random(deck: &mut Deck) -> Card{

    let random_index = rand::thread_rng().gen_range(0..=deck.cards.len());

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


// MAIN //
fn main() {
    let mut deck = build_deck();
    let mut player:Player = Player{hand: Vec::new()};
    let mut dealer:Dealer = Dealer{hand: Vec::new()};

    // println!("{:?}", deck.cards);
    println!("{:?}", deck.cards.len());

    game_loop(&mut deck, &mut player, &mut dealer)

}

fn game_loop(deck: &mut Deck, player: &mut Player, dealer: &mut Dealer){
    println!("{:?}", deck.cards.len());
    player.give_new_hand(deck);
    dealer.give_new_hand(deck);
    player.give_new_hand(deck);
    dealer.give_new_hand(deck);

    dealer.display_hand();
    player.display_hand();

    println!("{:?}", deck.cards.len());

}
