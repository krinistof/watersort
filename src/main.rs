#![allow(dead_code)]

use std::{error::Error, vec};

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    Orange,
    Pink,
    LightGreen,
    DarkGreen,
    Cyan,
    Purple,
    Grey,
    Brown,
    LightBrown,
}

#[derive(Debug, Clone)]
struct Glass {
    content: Vec<Color>,
}

impl Glass {
    fn from(src: Vec<Color>) -> Self {
        let mut new = Self::default();
        new.content.extend(src);
        new
    }

    fn complete(&self) -> bool {
        let content = &self.content;

        if content.is_empty() {
            true
        } else {
            let first = content.first().unwrap();
            self.is_full() && content.iter().skip(1).all(|c| c == first)
        }
    }

    fn is_full(&self) -> bool {
        self.content.len() == 4
    }

    fn is_empty(&self) -> bool {
        self.content.len() == 0
    }
}

impl Default for Glass {
    fn default() -> Self {
        Glass {
            content: Vec::with_capacity(4),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct State {
    glasses: Vec<Glass>,
    steps: Vec<Pour>,
}

impl State {
    fn solved(&self) -> bool {
        self.glasses.iter().all(Glass::complete)
    }

    fn is_legal(&self, pour: &Pour) -> bool {
        self.step(pour).is_ok()
    }

    fn step(&self, step: &Pour) -> Result<Self, Box<dyn Error>> {
        let mut new = self.clone();

        let source = self.glasses.get(step.from).ok_or("Invalid source index")?;
        let destination = self
            .glasses
            .get(step.to)
            .ok_or("Invalid destination index")?;

        if !destination.is_empty() && source.content.last() != destination.content.last() {
            return Err("Not matching colors".into());
        }

        loop {
            let source = new.glasses.get(step.from).ok_or("Invalid source index")?;
            let destination = new
                .glasses
                .get(step.to)
                .ok_or("Invalid destination index")?;

            if source.is_empty() {}
            if destination.is_full() {
                return Err("No space left in destination!".into());
            }

            let moved = {
                let source = new
                    .glasses
                    .get_mut(step.from)
                    .ok_or("Invalid source index")?;
                match source.content.pop() {
                    Some(color) => color,
                    None => {
                        return Err("Source glass is empty".into());
                    }
                }
            };

            {
                let destination = new
                    .glasses
                    .get_mut(step.to)
                    .ok_or("Invalid destination index")?;
                destination.content.push(moved);
            }

            if let Some(source_top) = new.glasses.get(step.from).unwrap().content.last() {
                let destination = new.glasses.get(step.to).unwrap();
                let destination_top = destination.content.last().unwrap();
                if source_top != destination_top || destination.is_full() {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(new)
    }

    fn solution(&self) -> Vec<Pour> {
        if self.solved() {
            return self.steps.clone();
        }

        for pour in self.legal_moves() {
            let mut attempt = self.step(&pour).unwrap();
            attempt.steps.push(pour);
            let result = attempt.solution();
            if !result.is_empty() {
                return result;
            }
        }

        vec![]
    }

    fn legal_moves(&self) -> Vec<Pour> {
        let n = self.glasses.len();
        let mut result = vec![];
        // TODO room for optimization
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let p = Pour { from: i, to: j };
                if self.is_legal(&p) {
                    result.push(p);
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pour {
    from: usize,
    to: usize,
}

fn main() {
    use Color::*;

    let game = State {
        glasses: vec![
            Glass::from(vec![Purple, Green, Brown, Yellow]),
            Glass::from(vec![LightBrown, Brown, Yellow, DarkGreen]),
            Glass::from(vec![LightBrown, Cyan, LightGreen, Red]),
            Glass::from(vec![LightGreen, LightGreen, Pink, Orange]),
            Glass::from(vec![LightGreen, Green, Grey, Pink]),
            Glass::from(vec![Brown, Grey, Grey, Yellow]),
            Glass::from(vec![Cyan, Purple, Pink, DarkGreen]),
            Glass::from(vec![Orange, LightBrown, Cyan, Orange]),
            Glass::from(vec![Green, Red, Brown, Grey]),
            Glass::from(vec![Green, Red, Cyan, Purple]),
            Glass::from(vec![Blue, LightBrown, Red, Orange]),
            Glass::from(vec![Blue, Blue, DarkGreen, Purple]),
            Glass::from(vec![Yellow, Blue, DarkGreen, Pink]),
            Glass::default(),
            Glass::default(),
        ],
        ..Default::default()
    };

    println!("{:?}", game.solution());
}

#[cfg(test)]
mod scenario {
    use super::*;
    use Color::*;

    #[test]
    fn last_step() {
        let game = State {
            glasses: vec![Glass::from(vec![Blue, Blue]), Glass::from(vec![Blue, Blue])],
            ..Default::default()
        };
        assert!(!game.solved());

        assert_eq!(game.solution(), vec![Pour { from: 0, to: 1 }]);
    }

    #[test]
    fn ordered() {
        let ordered = State {
            glasses: vec![
                Glass::from(vec![Orange; 4]),
                Glass::from(vec![Green; 4]),
                Glass::from(vec![Yellow; 4]),
                Glass::from(vec![Blue; 4]),
                Glass::from(vec![Red; 4]),
                Glass::default(),
            ],
            ..Default::default()
        };
        assert!(ordered.solved());
    }

    #[test]
    fn single_free_tube() {
        let game = State {
            glasses: vec![
                Glass::from(vec![Orange, Blue, Green, Blue]),
                Glass::from(vec![Green, Orange, Red, Blue]),
                Glass::from(vec![Green, Yellow, Green, Red]),
                Glass::from(vec![Red, Orange, Orange, Red]),
                Glass::from(vec![Yellow, Yellow, Yellow, Blue]),
                Glass::default(),
            ],
            ..Default::default()
        };

        assert_eq!(game.solution().len(), 15);
    }
}
