use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
  // hands 类型: BinaryHeap<(PokerHand, &str)>
  let mut hands: BinaryHeap<_> = hands.iter().map(|&s| (PokerHand::parse(s), s)).collect();
  let (winning, s) = hands.pop().unwrap();
  let mut result = vec![s];
  while let Some((value, s)) = hands.pop() {
    if value < winning {
      break;
    }
    result.push(s);
  }
  result
}

// 在 PokerHand 比较大小时: 先比较牌型(counts) ; 再比较大小(values);
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PokerHand {
  counts: Vec<usize>,
  values: Vec<u8>,
}

fn parse_card(s: &str) -> (u8, u8) {
  // 6D -> (6, D) ; JD -> (J, D) ; QD -> (Q, D);
  let (value, suit) = s.split_at(s.len() - 1);
  (
    match value.parse::<u8>() {
      Ok(v) => v,
      Err(_) => "JQKA".find(value).unwrap() as u8 + 11, // 注意 `A`
    },
    suit.as_bytes()[0], // get ASCII value, like "J".as_bytes() is [74].
  )
}

impl PokerHand {
  fn parse(s: &str) -> Self {
    // `值` 和 `花色` 分别放到 2 个 Vecs 中.
    let (values, suits): (Vec<u8>, Vec<u8>) = s.split_whitespace().map(parse_card).unzip();
    let mut groups = HashMap::<u8, usize>::new();
    for &v in values.iter() {
      *groups.entry(v).or_default() += 1;
    }
    let mut groups: Vec<_> = groups.into_iter().map(|(v, c)| (c, v)).collect();
    groups.sort_unstable_by_key(|&x| Reverse(x));

    let (mut counts, mut values): (Vec<_>, Vec<_>) = groups.iter().copied().unzip();
    if counts.len() == 5 { // 特殊情况处理：A, 5, 4, 3, 2 是合法的顺子，但 A 在这里应当被视为 1
      if values == [14, 5, 4, 3, 2] {
        values = vec![5, 4, 3, 2, 1];
      }

      // 检查是否是顺子 : 如果最大和最小的牌值之差是 4，则是顺子。
      let is_straight = values[0] - values[4] == 4;
      // 检查是否是同花。如果所有的花色都与第一张牌的花色相同，则是同花
      let is_flush = suits[1..].iter().all(|&x| x == suits[0]);

      // 根据牌型更新 counts 向量：
      match (is_straight, is_flush) {
        (true, true)  => counts = vec![5],  // 如果是同花顺，将 counts 设置为 [5] (该牌型最大)
        (false, true) => counts = vec![3, 1, 3], // 是同花，将 counts 设置为 [3, 1, 3]
        (true, false) => counts = vec![3, 1, 2], // 是顺子，将 counts 设置为 [3, 1, 2]
        _ => {}  // 否则，不作更改
      }
      // 其他牌型可以根据值的计数来获得, 比如 
      // - 4 条 ("3S 3H 2S 3D 3C") -> PokerHand { counts: [4, 1], values: [3, 2] }
      // - 葫芦 ("4S 5H 4D 5D 4H") -> PokerHand { counts: [3, 2], values: [4, 5] }
    }
    println!("s: {:?}", s);
    println!("Self: {:?}", Self { counts: counts.clone(), values: values.clone() });
    println!("------------",);
    Self { counts, values }
  }
}



// use std::{
//   cmp::Reverse,
//   collections::{BinaryHeap, HashMap},
//   error::Error,
//   str::FromStr,
// };

// /// Given a list of poker hands, return a list of those hands which win.
// pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
//   if hands.is_empty() {
//     return vec![];
//   }
//   let mut hands: BinaryHeap<(HandValue, &str)> = hands
//     .iter()
//     .map(|&s| (HandValue::new(s.parse().unwrap()), s))
//     .collect();
//   let (winning, s) = hands.pop().unwrap();
//   let mut result = vec![s];
//   while let Some((value, s)) = hands.pop() {
//     if value < winning {
//       break;
//     }
//     result.push(s);
//   }
//   result
// }

// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct Value(u8);
// impl FromStr for Value {
//   type Err = Box<dyn Error>;
//   fn from_str(s: &str) -> Result<Self, Self::Err> {
//     let value = {
//       match s.parse::<u8>() {
//         Ok(v) => v,
//         Err(_) => match s {
//           "J" => 11,
//           "Q" => 12,
//           "K" => 13,
//           "A" => 14,
//           _ => return Err("invalid card value".into()),
//         },
//       }
//     };
//     Ok(Value(value))
//   }
// }

// #[derive(Debug)]
// struct PokerHand {
//   values: [Value; 5],
//   suits: [u8; 5],
//   group_lengths: Vec<u8>,
//   group_values: Vec<Value>,
// }
// impl PokerHand {
//   fn new(values: [Value; 5], suits: [u8; 5]) -> Self {
//     let mut groups = HashMap::<Value, u8>::new();
//     for &v in values.iter() {
//       *groups.entry(v).or_default() += 1;
//     }
//     let mut groups: Vec<_> = groups.into_iter().collect();
//     groups.sort_by_key(|&(v, len)| Reverse((len, v)));
//     let (group_values, group_lengths) = groups.into_iter().unzip();
//     Self {
//       values,
//       suits,
//       group_lengths,
//       group_values,
//     }
//   }
//   fn is_straight(&self) -> Option<Value> {
//     if self.group_lengths.len() == 5 {
//       if self.values[0].0 == self.values[4].0 + 4 {
//         Some(self.values[0])
//       } else if self.values[0].0 == 14 && self.values[1].0 == 5 {
//         Some(self.values[1])
//       } else {
//         None
//       }
//     } else {
//       None
//     }
//   }
//   fn is_flush(&self) -> Option<[Value; 5]> {
//     self.suits[1..5]
//       .iter()
//       .all(|x| x == &self.suits[0])
//       .then(|| self.values)
//   }
// }
// impl FromStr for PokerHand {
//   type Err = Box<dyn Error>;
//   fn from_str(s: &str) -> Result<Self, Self::Err> {
//     let mut hand: Vec<(Value, u8)> = s
//       .split_whitespace()
//       .map(|card| {
//         let i = card.len() - 1;
//         (card[0..i].parse().unwrap(), card.as_bytes()[i])
//       })
//       .collect();
//     assert_eq!(hand.len(), 5);
//     hand.sort();
//     let (values, suits): (Vec<_>, Vec<_>) = hand.into_iter().rev().unzip();
//     Ok(PokerHand::new(
//       values.try_into().unwrap(),
//       suits.try_into().unwrap(),
//     ))
//   }
// }
// #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
// enum HandValue {
//   HighCard([Value; 5]),
//   OnePair([Value; 4]),
//   TwoPair([Value; 3]),
//   ThreeOfAKind([Value; 3]),
//   Straight(Value),
//   Flush([Value; 5]),
//   FullHouse([Value; 2]),
//   FourOfAKind([Value; 2]),
//   StraightFlush(Value),
// }
// impl HandValue {
//   fn new(hand: PokerHand) -> Self {
//     match (hand.is_straight(), hand.is_flush()) {
//       (Some(v), Some(_)) => Self::StraightFlush(v),
//       (Some(v), None) => Self::Straight(v),
//       (None, Some(v)) => Self::Flush(v),
//       (None, None) => {
//         let v = hand.group_values;
//         match *hand.group_lengths {
//           [4, 1] => Self::FourOfAKind(v.try_into().unwrap()),
//           [3, 2] => Self::FullHouse(v.try_into().unwrap()),
//           [3, 1, 1] => Self::ThreeOfAKind(v.try_into().unwrap()),
//           [2, 2, 1] => Self::TwoPair(v.try_into().unwrap()),
//           [2, 1, 1, 1] => Self::OnePair(v.try_into().unwrap()),
//           _ => Self::HighCard(hand.values),
//         }
//       }
//     }
//   }
// }