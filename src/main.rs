use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process;

#[derive(Debug)]
struct Card{
    suit: String,
    n: u32
}

impl Card{
    fn new(suit:String, n:u32) -> Card{
        Card {
            suit: suit,
            n: n
        }
    }
}//end impl Card

// for compare Card struct
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.n.cmp(&other.n)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Eq for Card {}

#[derive(Debug)]
struct Deck{
    cards: Vec<Card>,
}

impl Deck{
    fn new() -> Deck{
        let mut cards = Vec::new();
        let suits = ["spade", "heart", "diamond", "club"];
        for s in suits.iter() {
            for n in 1..11{
                let c = Card::new(s.to_string(), n);
                cards.push(c);
                if n == 10 {
                    for _i in 1..4{
                        let c = Card::new(s.to_string(), n);
                        cards.push(c);
                    }
                }
            }
        }
        Deck{
            cards: cards
        }
    }// end fn new

    fn draw(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(1, self.cards.len());
        self.cards.remove(i)
    }
}// end impl Deck

pub trait CardCollection {
    fn calc(&mut self) -> u32;
}

impl CardCollection for Vec<Card> {
    fn calc(&mut self) -> u32{
        let tmp = self;
        let mut n = 0;
        // first loop, calc other `A`
        for c in tmp.iter() {
            if c.n == 1 {
                continue;
            }
            n = n + c.n;
        }

        // second loop, calc `A`
        for c in tmp.iter(){
            if c.n != 1 {
                continue;
            }

            match n.cmp(&10){
                Ordering::Greater => {
                    n = n + 1;
                },
                _ => {
                    n = n + 11
                }
            }
        }
        n
    }
}


#[derive(Debug)]
struct Player {
    tip: f32,
    hand: Vec<Card>,
}

impl Player {
    fn new(tip:f32) -> Player {
        Player {
            tip: tip,
            hand: Vec::new()
        }
    }

    fn hit(&mut self, deck:&mut Deck){
        let new_card = deck.draw();
        self.hand.push(new_card);
    }

    fn stand(&mut self, opponent:&mut Player, fee:f32){
        let player_n = self.hand.calc();
        let opponent_n = opponent.hand.calc();
        println!("\nYour score: {}, Opponent score: {}", player_n, opponent_n);
        match opponent_n.cmp(&21){
            Ordering::Greater => {
                println!("You Win!, Opponent Over 21");
                self.tip = self.tip + fee + fee*1.0;
            },
            _ => {
                match player_n.cmp(&opponent_n){
                    Ordering::Greater => {
                        println!("You Win!, Over Opponent");
                        if player_n == 21 {
                            self.tip = self.tip + fee + fee*1.5;
                        } else {
                            self.tip = self.tip + fee + fee*1.0;
                        }
                    },
                    Ordering::Equal => {
                        println!("Draw");
                        self.tip = self.tip + fee;
                    },
                    Ordering::Less => {
                        println!("You Lose!, Less than Oppponent");
                    },
                }
            }
        }
    }
}

fn main() {
    let mut d = Deck::new();
    let mut player = Player::new(5.0);
    let mut dealer = Player::new(0.0);

    let mut fee = 0.0;
    println!("Notion!: In this game, `A` deal with 11. But, if your score is greater than 21, `A` deal with 1 \n");

    loop{
        if fee == 0.0 {
            // game 開始処理
            player.hand = Vec::new();
            dealer.hand = Vec::new();
            player.hit(&mut d);
            player.hit(&mut d);
            dealer.hit(&mut d);
            dealer.hit(&mut d);

            // bet 処理
            let mut guess = String::new();
            println!("Start!\nHow tip you paid?(Your tip:{})", player.tip);
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            guess.pop();// 末尾の改行削除
            fee = guess.parse().unwrap();
            player.tip = player.tip - fee;
            if player.tip < 0.0 {
                println!("You don't have that tips!, You are lier, Go out!");
                process::exit(1);
            }
            if fee < 1.0 {
                println!("You are dirty,  Go out!");
                process::exit(1);
            }
        }

        // player action 処理
        let mut guess = String::new();
        let score = player.hand.calc();
        println!("\nPlease input your action(hit or stand)");
        println!("Your Hands: {:?}\nYour Score: {}\nBet: {}\nYour Tip:{}", player.hand, score,fee, player.tip);
        println!("Dealer One Hand: {:?}", dealer.hand[0]);
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess.pop();// 末尾の改行削除
        println!(""); //改行

        match guess.as_str() {
            "hit" => {
                player.hit(&mut d);
                println!("Your new card: {:?}", player.hand.last().unwrap());
                let score = player.hand.calc();
                if score > 21 {
                    println!("Your hand:{:?}", player.hand);
                    println!("Your Score({}) over 21!, You Lose!", score);
                    fee = 0.0;
                }
            },
            "stand" => {
                println!("Dealer Hand:{:?}", dealer.hand);
                while dealer.hand.calc() < 17 {
                    dealer.hit(&mut d);
                    println!("Dealer Hand:{:?}", dealer.hand);
                }
                player.stand(&mut dealer, fee);
                fee = 0.0
            },
            _ => println!("No action"),
        }
        println!(""); //改行
    }
}
