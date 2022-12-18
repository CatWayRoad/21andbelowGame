mod elements;

fn main() {
    use elements::cards::*;
    let deck = CardType::get_deck();
    println!("{:?} \n\n {}", deck, deck.len())
}
