use std::collections::HashSet;

pub fn run() {
    let content = std::fs::read_to_string("input/day23").expect("Unable to read input");
    let spaces = parse(&content);

    println!("Day 23");
    println!("  Problem 1: {}", problem1(spaces.clone()));
    println!("  Problem 2: {}", problem2(spaces));
}

fn parse(raw: &str) -> [Space; 11] {
    let mut spaces = [
        Space::Empty,
        Space::Empty,
        Space::empty_room(AmphipodType::Amber),
        Space::Empty,
        Space::empty_room(AmphipodType::Bronze),
        Space::Empty,
        Space::empty_room(AmphipodType::Copper),
        Space::Empty,
        Space::empty_room(AmphipodType::Desert),
        Space::Empty,
        Space::Empty,
    ];


    for (i, line) in raw.lines().skip(2).take(2).enumerate() {
        for (c, space) in line.chars().skip(1).zip(spaces.iter_mut()) {
            if let &mut Space::Room { ref mut occupants, ..} = space {
                occupants[i] = match c {
                    'A' => Some(Amphipod::new(AmphipodType::Amber)),
                    'B' => Some(Amphipod::new(AmphipodType::Bronze)),
                    'C' => Some(Amphipod::new(AmphipodType::Copper)),
                    'D' => Some(Amphipod::new(AmphipodType::Desert)),
                    _ => panic!(),
                };
            }
        }
    }

    spaces
}

fn problem1(spaces: [Space; 11]) -> usize {
    let mut tried = HashSet::new();
    foo(&spaces, 0, &mut tried).unwrap()
}

fn problem2(mut spaces: [Space; 11]) -> u32 {
    todo!()
}

fn foo(spaces: &[Space; 11], depth: usize, tried: &mut HashSet<[Space; 11]>) -> Option<usize> {
    tried.insert(spaces.clone());

    //println!("{}   {}", depth, tried.len());
    let mut lowest_cost = None;
    for (i, space) in spaces.iter().enumerate() {
        let amphipod = match space.occupant() {
            Some(a) => a,
            None => continue,
        };

        for (move_index, move_cost) in amphipod.move_costs(i, spaces).iter() {
            let mut new_spaces = spaces.clone();
            let mut occupant = new_spaces[i].vacate();
            occupant.movements_remaining -= 1;
            new_spaces[*move_index].move_in(occupant);
            if tried.contains(&new_spaces) {
                continue;
            }

            let mut cost = *move_cost;
            if !is_solved(&new_spaces) {
                if let Some(rc) = foo(&new_spaces, depth + 1, tried) {
                    cost += rc;
                } else {
                    continue;
                }
            }

            if let Some(lc) = lowest_cost {
                if cost < lc {
                    lowest_cost = Some(cost);
                }
            } else {
                lowest_cost = Some(cost);
            }
        }
    }

    lowest_cost
}

fn is_solved(spaces: &[Space]) -> bool {
    for space in spaces.iter() {
        match space {
            Space::Empty => continue,
            Space::Occupied(_) => return false,
            Space::Room { r#type, occupants } => {
                if let Some(o) = occupants[0] {
                    if &o.r#type != r#type {
                        return false;
                    }
                } else {
                    return false;
                }
                if let Some(o) = occupants[1] {
                    if &o.r#type != r#type {
                        return false;
                    }
                } else {
                    return false;
                }
            },
        }
    }
    true
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Space {
    Empty,
    Occupied(Amphipod),
    Room {
        r#type: AmphipodType,
        occupants: [Option<Amphipod>; 2],
    }
}

impl Space {
    fn empty_room(r#type: AmphipodType) -> Self {
        Self::Room {
            r#type,
            occupants: [None; 2],
        }
    }

    fn occupant(&self) -> Option<&Amphipod> {
        match self {
            Space::Empty => None,
            Space::Occupied(a) => Some(a),
            Space::Room { occupants, ..  } => occupants[0].as_ref().or(occupants[1].as_ref()),
        }
    }

    fn vacate(&mut self) -> Amphipod {
        match self {
            Space::Empty => panic!(),
            Space::Occupied(a) => {let occupant = *a;*self = Space::Empty;occupant},
            Space::Room { ref mut occupants, .. } => {
                let mut a = occupants[0].take();
                if a.is_none() {
                    a = occupants[1].take();
                }
                a.unwrap()
            }
        }
    }

    fn move_in(&mut self, amphipod: Amphipod) {
        match self {
            Space::Empty => {*self = Space::Occupied(amphipod)}
            Space::Occupied(_) => panic!(),
            Space::Room { ref mut occupants, .. } => {
                if occupants[1].is_none() {
                    occupants[1] = Some(amphipod);
                } else if occupants[0].is_none() {
                    occupants[0] = Some(amphipod);
                } else {
                    panic!()
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Amphipod {
    movements_remaining: u8,
    r#type: AmphipodType,
}

impl Amphipod {
    fn new(r#type: AmphipodType) -> Self {
        Self { movements_remaining: 2, r#type }
    }

    fn move_costs(&self, position: usize, spaces: &[Space]) -> Vec<(usize, usize)> {
        if self.movements_remaining == 0 {
            return vec![];
        }

        let mut costs = Vec::new();
        for (i, space) in spaces.iter().enumerate().take(position).rev(){
            let mut steps = 0;
            match space {
                Space::Empty => steps = position - i,
                Space::Occupied(_) => break,
                Space::Room { r#type, occupants } => {
                    if *r#type != self.r#type {
                        continue;
                    }
                    if occupants[0].is_some() {
                        continue;
                    }
                    steps = 2;
                    if let Some(o) = occupants[1] {
                        if o.r#type != *r#type {
                            continue;
                        }
                        steps += 1;
                    }
                },
            }
            costs.push((i, self.r#type.energy(steps)));
        }

        for (i, space) in spaces.iter().enumerate().skip(position + 1) {
            let mut steps = 0;
            match space {
                Space::Empty => steps = i - position,
                Space::Occupied(_) => break,
                Space::Room { r#type, occupants } => {
                    if *r#type != self.r#type {
                        continue;
                    }
                    if occupants[0].is_some() {
                        continue;
                    }
                    steps = 2;
                    if let Some(o) = occupants[1] {
                        if o.r#type != *r#type {
                            continue;
                        }
                        steps += 1;
                    }
                },
            }
            costs.push((i, self.r#type.energy(steps)));
        }

        costs.sort_unstable_by_key(|c| c.1);
        costs
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodType {
    fn energy(&self, steps: usize) -> usize {
        match self {
            AmphipodType::Amber  => steps * 1,
            AmphipodType::Bronze => steps * 10,
            AmphipodType::Copper => steps * 100,
            AmphipodType::Desert => steps * 1000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn example1() {
        let spaces = parse(SAMPLE);
        assert_eq!(problem1(spaces), 12521);
    }
}
