/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
///
///
///
use std::collections::HashSet;

struct Hand<'a>{
    hand_row: &'a str
}

trait PokerHand {
    fn to_vec(&self) -> Vec<&str>;
}

impl PokerHand for Hand<'> {
    fn to_vec(&self) -> Vec<&'a str> {
        let cards: Vec<&str> = self.hand_row.split(' ').collect();
        cards
    }
}

fn is_hand_equal<'a>(hand_a: &'a str, hand_b: &'a str) -> bool {
    hand_a.chars().zip(hand_b.chars()).filter(|(a,b)| a != b).count() == 0 
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {

    let vec_hands =  hands.iter().map( |hand| map_hand_to_vec(hand) );
    


    let s = HashSet::from([1,2]);
    if hands.len() > 0 {
        let all_equal = hands.iter().all(| hand | is_hand_equal(hand, hands[0]) );
        if all_equal {
            return hands.to_vec()
        }
        return vec!(hands[0])
    }
    return [].to_vec() 
}

