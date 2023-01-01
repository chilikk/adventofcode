#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::str::FromStr;
use std::ops::{AddAssign,SubAssign,Add,Sub,MulAssign,Mul};
use std::cmp::{Ordering,PartialOrd};

struct Task19 {
    blueprints: Vec<Blueprint>,
    acc1: u16,
    acc2: u64,
}

#[derive(Copy,Clone,Debug,PartialEq)]
struct Resources([i16;4]); // Ore, Clay, Obsidian, Geode

impl Resources {
    fn new() -> Self {
        Resources([0,0,0,0])
    }

    fn from_str_one(&mut self, s: &str) {
        let len = s.len();
        if s.ends_with(" ore") {
            self.0[0] = s[..len-4].parse().unwrap();
        } else if s.ends_with(" clay") {
            self.0[1] = s[..len-5].parse().unwrap();
        } else if s.ends_with(" obsidian") {
            self.0[2] = s[..len-9].parse().unwrap();
        }
        // geode is never used to make robots
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other:&Resources) -> Option<Ordering> {
        let mut only_positive = true;
        let mut only_negative = true;
        let mut only_zeros = true;
        for e in (*self - *other).0 {
            if e < 0 {
                only_positive = false;
                only_zeros = false;
            } else if e > 0 {
                only_negative = false;
                only_zeros = false;
            }
        }
        if only_zeros {
            Some(Ordering::Equal)
        } else if only_positive {
            Some(Ordering::Greater)
        } else if only_negative {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, other: Self) {
        for (v1, v2) in self.0.iter_mut().zip(other.0) {
            *v1 += v2
        }
    }
}

impl Add for Resources {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut res = self.clone();
        res += other;
        res
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, other: Self) {
        for (v1, v2) in self.0.iter_mut().zip(other.0) {
            *v1 -= v2
        }
    }
}


impl Sub for Resources {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut res = self.clone();
        res -= other;
        res
    }
}

impl MulAssign<usize> for Resources {
    fn mul_assign(&mut self, other: usize) {
        for v1 in self.0.iter_mut() {
            *v1 *= other as i16
        }
    }
}

impl Mul<usize> for Resources {
    type Output = Self;
    fn mul(self, other: usize) -> Self {
        let mut res = self.clone();
        res *= other;
        res
    }
}

impl FromStr for Resources {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cost = Resources::new();
        match s.split_once(" and ") {
            None => cost.from_str_one(s),
            Some((s1,s2)) => {
                cost.from_str_one(s1);
                cost.from_str_one(s2);
            },
        }
        Ok(cost)
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u16,
    cost: [Resources;4],
}

impl FromStr for Blueprint {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(&s[0..10], "Blueprint ");
        let (idstr, spec) = s.split_once(":").unwrap();
        let id: u16 = idstr[10..].parse().unwrap();
        let costs: Vec<&str> = spec.split(".").collect();
        assert_eq!(&costs[0][0..22], " Each ore robot costs ");
        let ore_cost: Resources = costs[0][22..].parse().unwrap();
        assert_eq!(&costs[1][0..23], " Each clay robot costs ");
        let clay_cost: Resources = costs[1][23..].parse().unwrap();
        assert_eq!(&costs[2][0..27], " Each obsidian robot costs ");
        let obsidian_cost: Resources = costs[2][27..].parse().unwrap();
        assert_eq!(&costs[3][0..24], " Each geode robot costs ");
        let geode_cost: Resources = costs[3][24..].parse().unwrap();
        Ok(Blueprint{
            id: id,
            cost: [ore_cost, clay_cost, obsidian_cost, geode_cost]
        })
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
enum Decision {
    DoNothing,
    Build(usize)
}

impl Blueprint {

    fn max_geodes(&self, t: usize) -> u16 {
        let resources = Resources::new();
        let mut robots = Resources::new();
        robots.0[0] = 1;
        let init_decision = Decision::DoNothing;
        self.max_geodes_step(t, 0, resources, robots, init_decision, Vec::new())
    }

    fn max_geodes_step(
        &self, t: usize, cur_max: u16, mut resources: Resources,
        mut robots: Resources, exec: Decision, last_rejected: Vec<Decision>
    ) -> u16
    {
        match exec {
            Decision::DoNothing => (),
            Decision::Build(i) => {
                resources -= self.cost[i].clone();
                robots.0[i] += 1;
            },
        }
        if ! self.validate_path(t+1, cur_max, &robots) {
            return 0
        }
        let mut possible_decisions: Vec<Decision> = vec![];
        for (i, robot_cost) in self.cost.iter().enumerate() {
            if *robot_cost <= resources {
                let decision = Decision::Build(i);
                if ! last_rejected.contains(&decision) {
                    possible_decisions.push(decision)
                }
            }
        }
        if possible_decisions.len() == 0 || last_rejected.len() == 0 {
            possible_decisions.push(Decision::DoNothing)
        }
        resources += robots.clone();
        if t == 1 {
            cur_max.max(resources.0[3] as u16)
        } else {
            let mut max = 0;
            for decision in possible_decisions.iter() {
                let new_last_rejected;
                if let Decision::DoNothing = decision {
                    if possible_decisions.len() == 1 {
                        new_last_rejected = last_rejected.clone();
                    } else {
                        new_last_rejected = possible_decisions.clone();
                    }
                } else {
                    new_last_rejected = Vec::new();
                }
                max = max.max(self.max_geodes_step(
                        t-1, max, resources.clone(), robots.clone(),
                        (*decision).clone(), new_last_rejected))
            }
            max
        }
    }

    fn validate_path(&self, t: usize, cur_max: u16, robots: &Resources) -> bool {
        let t_require_geode = 1 + Blueprint::min_time_to_mine(cur_max as i16+1);
        let t_require_obsidian = t_require_geode + 1 + Blueprint::min_time_to_mine(self.cost[3].0[2]);
        let t_require_clay = t_require_obsidian + 1 + Blueprint::min_time_to_mine(self.cost[2].0[1]);
        match *robots {
            Resources([_, _, _, 0]) if t == t_require_geode    => false,
            Resources([_, _, 0, _]) if t == t_require_obsidian => false,
            Resources([_, 0, _, _]) if t == t_require_clay     => false,
            _                                                  => true,
        }
    }

    fn min_time_to_mine(n: i16) -> usize {
        let mut t = n as usize;
        let mut sum_to = 1;
        for nrobots in 2..n {
            t = t.min(nrobots as usize - 1 + (n-sum_to) as usize/nrobots as usize);
            sum_to += nrobots;
        }
        t
    }

}

impl Task19 {
    fn new() -> Task19 {
        Task19 {
            blueprints: Vec::new(),
            acc1: 0,
            acc2: 1,
        }
    }
}

impl aoc::AdventurerOfCode for Task19 {
    fn handle_line(&mut self, line: String) {
        let blueprint: Blueprint = line.parse().unwrap();
        self.blueprints.push(blueprint);
        let blueprint = &self.blueprints[self.blueprints.len()-1];
        let max_geodes24 = blueprint.max_geodes(24) as u16;
        println!("{:?} : max 24 {}", blueprint, max_geodes24);
        self.acc1 += blueprint.id * max_geodes24;
        //if blueprint.id <= 3 {
        //    let max_geodes32 = blueprint.max_geodes(32) as u16;
        //    println!("{:?} : max 32 {}", blueprint, max_geodes32);
        //    self.acc2 *= max_geodes32 as u64;
        //}

    }

    fn after(&mut self) {
    }
}

aocfmt!{Task19, self, self.acc1, self.acc2}
aocmain!{Task19}
