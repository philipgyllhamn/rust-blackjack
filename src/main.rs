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
}

struct Dealer{
    hand: Vec<Card>
}

impl Dealer{
    fn give_new_hand(&mut self, deck: &mut Deck){
        self.hand.push(pop_at_random(deck));
    }
}

fn pop_at_random(deck: &mut Deck) -> Card{

    let random_index = rand::thread_rng().gen_range(0..=deck.cards.len());

    let card = deck.cards.remove(random_index);
    
    card
}

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

    println!("{:?}", player.hand);
    println!("{:?}", dealer.hand);

    println!("{:?}", deck.cards.len());

}
