fn main() {
    println!("Hello, world!");
    // let input = &["4S 5S 7H 8D JC"];
    // let input = &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"];
    // winning_hands(input);
    // let input = &["2S 5H 6S 8D 7H", "3S 4D 6D 8C 7S"];
    // winning_hands(input);
    // let input = &["4S 6C 7S 8D 5H", "5S 7H 8S 9D 6H"];
    // winning_hands(input);
    let input = &["2H 3H 4H 5H 6H", "4D AD 3D 2D 5D"];
    winning_hands(input);
}

use std::collections::{ HashMap, BTreeSet };
fn is_straight(_rank_order:&BTreeSet<&str>, all_rank:&[&str]) -> bool{
    let all_rank_num = all_rank.iter().enumerate().map(|(_idx,x)| {
        let n = match *x {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" if all_rank.contains(&"2") => 1,
            "A" => 14,
            _ => x.parse::<u32>().unwrap()
        };
        n
    }).collect::<Vec<_>>();
    let min = all_rank_num.iter().min().unwrap();
    let max = all_rank_num.iter().max().unwrap();
    (max - min + 1) as usize == all_rank_num.len() 
        && all_rank_num.iter().collect::<std::collections::HashSet<_>>().len() == all_rank_num.len()
}
fn is_four(rank:&HashMap<&str,u8>) -> bool {
    for (_,v) in rank {
        if *v == 4 {
            return true;
        }
    }
    false
}
fn is_full_house(rank:&HashMap<&str,u8>) -> bool{
    let val = rank.values().collect::<Vec<_>>();
    rank.keys().len() == 2 && (*val[0] == 2|| *val[0] == 3)
}
fn is_three(rank:&HashMap<&str,u8>) -> bool {
    let mut has_three = false;
    for (_,v) in rank {
        if *v == 3 {
            has_three = true;
            break;
        }
    }
    has_three && rank.keys().len() == 3
}
fn is_two_pairs(rank:&HashMap<&str,u8>) -> bool{
    let mut has_two = 0;
    for (_,v) in rank {
        if *v == 2 {
            has_two += 1;
        }
    }
    has_two == 2 && rank.keys().len() == 3
}
fn is_one_pair(rank:&HashMap<&str,u8>) -> bool{
    let mut has_two = 0;
    for (_,v) in rank {
        if *v == 2 {
            has_two += 1;
        }
    }
    has_two == 1 && rank.keys().len() == 4
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRanking {
    HighCard(u32),
    OnePair(u32),
    TwoPairs(u32),
    ThreeOfAKind(u32),
    Straight(u32),
    Flush(u32),
    FullHouse(u32),
    FourOfAKind(u32),
    StraightFlush(u32),
}
struct Hand<'a> {
    hand:&'a str
}
impl<'a> Hand<'a> {
    fn new(hand:&'a str) -> Self{
        Self {
            hand
        }
    }
    fn hand_ranking(&self) -> HandRanking{
        let (rank, rank_order, all_rank) = self.rank();
        let suit = self.suit();
        if rank_order.len() == 5 && is_straight(&rank_order, &all_rank) && suit.keys().count() == 1 {
            return HandRanking::StraightFlush(self.val());
        }else if is_four(&rank) {
            return HandRanking::FourOfAKind(self.val());
        }else if is_full_house(&rank) {
            return HandRanking::FullHouse(self.val());
        }else if suit.keys().count() == 1 {
            return HandRanking::Flush(self.val());
        }else if rank_order.len() == 5 && is_straight(&rank_order, &all_rank) {
            return HandRanking::Straight(self.val());
        }else if is_three(&rank) {
            return HandRanking::ThreeOfAKind(self.val());
        }else if is_two_pairs(&rank) {
            return HandRanking::TwoPairs(self.val());
        }else if is_one_pair(&rank) {
            return HandRanking::OnePair(self.val());
        }
        HandRanking::HighCard(self.val())
    }
    fn rank(&self) -> (HashMap<&str,u8>, BTreeSet<&str>, Vec<&str>){
        let mut rank = HashMap::new();
        let mut rank_order = BTreeSet::new();
        let mut all_rank = vec![];
        self.hand.split(" ").for_each(|ch| {
            rank.entry(&ch[0..&ch.len()-1]).and_modify(|n| *n += 1).or_insert(1);
            rank_order.insert(&ch[0..&ch.len()-1]);
            all_rank.push(&ch[0..&ch.len()-1]);
        });
        (rank, rank_order, all_rank)
    }
    fn suit(&self) -> HashMap<&str,u8>{
        let mut suit = HashMap::new();
        self.hand.split(" ").for_each(|ch| {
            suit.entry(&ch[&ch.len()-1..]).and_modify(|n| *n += 1).or_insert(1);
        });
        suit
    }
    fn val(&self) -> u32 {
        let (rank, _rank_order, _) = self.rank();
        let mut ret = vec![];
        for i in [1u8,2,3,4,5]{
            let mut tmp = vec![];
            rank.iter().for_each(|x| {
                if &i == x.1 {
                    let n = match *x.0 {
                        "J" => 11,
                        "Q" => 12,
                        "K" => 13,
                        "A" if rank.contains_key(&"2") => 1,
                        "A" => 14,
                        _ => x.0.parse::<u32>().unwrap()
                    };
                    tmp.insert(0, n);
                }
            });
            tmp.sort();
            for ele in tmp {
                ret.insert(0, ele);
            }
        }
        ret.reverse();
        ret.iter().enumerate().fold(0, |acc,x| {
            acc + x.1.pow(x.0 as u32 + 1)
        })
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut ret = vec![hands[0]];
    for i in hands.iter().skip(1) {
        if Hand::new(i).hand_ranking() > Hand::new(ret[0]).hand_ranking() {
            ret = vec![i];
        }else if Hand::new(i).hand_ranking() == Hand::new(ret[0]).hand_ranking() {
            ret.push(i);
        }
    }
    ret
}