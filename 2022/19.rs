#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::str::FromStr;
use std::ops::{AddAssign,SubAssign,Add,Sub,MulAssign,Mul};
use std::cmp::{Ordering,PartialOrd};

struct Task19 {
    blueprints: Vec<Blueprint>,
    acc1: u16,
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

    fn which_max(&self) -> (usize, i16) {
        let mut max = 0;
        let mut max_i = 0;
        for (i, d) in self.0.iter().enumerate() {
            if *d > max {
                max = *d;
                max_i = i;
            }
        }
        (max_i, max)
    }

    fn which_min(&self) -> (usize, i16) {
        let mut min = 0;
        let mut min_i = 0;
        for (i, d) in self.0.iter().enumerate() {
            if *d < min {
                min = *d;
                min_i = i;
            }
        }
        (min_i, min)
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

impl Blueprint {

    fn get_plan(&self, orereq: i16, resourcetype: usize, n: i16, time: usize) -> Option<(Resources,Resources)> {
        let mut table: Vec<Resources> = vec![Resources::new();time];
        let build1robottime = 24-n;
        if build1robottime > 0 {
            table[build1robottime-1] = self.cost[resourcetype].clone();
        }
    }

    fn can_build(&self, resourcetype: usize, robot_times: Vec<usize>) -> bool {
        let cost = self.cost[resourcetype];
        match self.get_plan(cost.0[0], resourcetype-1, cost.0[resourcetype-1], robot_times[0]-1) {
            None => false,
            Some((resources, robots)) => {
                let mut can = true;
                let mut i = 1;
                while i < robot_times.len() {
                    resources += robots*(robot_times[i]-robot_times[i-1]-1);
                    if resources < cost {
                        can = false;
                        break
                    }
                    i += 1;
                }
                can
            }
        }

    }

    //fn max_geodes(&self) -> i16 {
    //    let mut max_geodes = 0;
    //    let mut rounds = 10;
    //    let mut plan = Resources([0,1,1,1]);
    //    while rounds > 0 {
    //        let robots: Resources = Resources([1,0,0,0]);
    //        let res = self.exec_plan(24, Resources::new(), robots, plan.clone(), Resources::new());
    //        println!("{res:?}");
    //        max_geodes = max_geodes.max(res.0.0[3]);
    //        if res.0.can_build(&self.cost[3]) {
    //            plan.0[3] += 1;
    //        } else {
    //            plan.0[res.1.which_max().0] += 1;
    //        }
    //        rounds -= 1;
    //        println!("{plan:?}: {max_geodes}");
    //    }
    //    max_geodes
    //}

    //fn exec_plan(&self, t: usize, mut resources: Resources,
    //    mut robots: Resources, mut plan: Resources, mut longest_deficit: Resources
    //    ) -> (Resources, Resources) {
    //    //println!("t {t} resources {resources:?} robots {robots:?} plan {plan:?} deficit {deficit:?}");
    //    if t == 0 {
    //        return (resources, longest_deficit)
    //    }
    //    let mut i = 0;
    //    let under_construction: Resources = loop {
    //        if plan.0[i] > 0 {
    //            let mut test_resources = resources.clone();
    //            test_resources -= self.cost[i].clone();
    //            let mut do_build = true;
    //            for i in test_resources.0.iter() {
    //                if *i < 0 {
    //                    do_build = false;
    //                    break
    //                }
    //            }
    //            if do_build {
    //                resources = test_resources;
    //                plan.0[i] -= 1;
    //                let mut ret = [0;4];
    //                ret[i] = 1;
    //                break Resources(ret)
    //            } else {
    //                let max_deficit = test_resources.which_min();
    //                if max_deficit.1 < 0 {
    //                    longest_deficit.0[max_deficit.0] += 1;
    //                }
    //            }
    //        }
    //        i += 1;
    //        if i > 3 {
    //            break Resources([0;4])
    //        }
    //    };
    //    resources += robots.clone();
    //    robots += under_construction;
    //    self.exec_plan(t-1, resources, robots, plan, longest_deficit)
    //}

    //fn max_geodes_step(
    //    &self, t: usize, mut resources: Resources,
    //    mut robots: Resources, exec: Decision, last_rejected: Vec<Decision>
    //) -> u16
    //{
    //    match exec {
    //        Decision::DoNothing => (),
    //        Decision::Build(i) => {
    //            resources -= self.cost[i].clone();
    //        },
    //    }
    //    let mut possible_decisions: Vec<Decision> = vec![];
    //    for (i, robot_cost) in self.cost.iter().enumerate() {
    //        if *robot_cost <= resources {
    //            let decision = Decision::Build(i);
    //            if ! last_rejected.contains(&decision) {
    //                match robots.0 {
    //                    [_, _, _, Geode(0)] if t == 3 => (),
    //                    [_, _, Obsidian(0), _] if t == 5 => (),
    //                    [_, Clay(0), _, _] if t == 7 => (),
    //                    _ => possible_decisions.push(decision),
    //                }
    //            }
    //        }
    //    }
    //    if possible_decisions.len() == 0 || last_rejected.len() == 0 {
    //        possible_decisions.push(Decision::DoNothing);
    //    }
    //    resources += robots.clone();
    //    if t == 1 {
    //        if let Geode(n) = resources.0[3] {
    //            n as u16
    //        } else {
    //            0
    //        }
    //    } else {
    //        match exec {
    //            Decision::DoNothing => (),
    //            Decision::Build(i) => {
    //                robots.0[i] += i;
    //            },
    //        }
    //        let mut max = 0;
    //        for decision in possible_decisions.iter() {
    //            let new_last_rejected;
    //            if let Decision::DoNothing = decision {
    //                if possible_decisions.len() == 1 {
    //                    new_last_rejected = last_rejected.clone();
    //                } else {
    //                    new_last_rejected = possible_decisions.clone();
    //                }
    //            } else {
    //                new_last_rejected = Vec::new();
    //            }
    //            max = max.max(self.max_geodes_step(
    //                    t-1, resources.clone(), robots.clone(),
    //                    (*decision).clone(), new_last_rejected))
    //        }
    //        max
    //    }
    //}
}

impl Task19 {
    fn new() -> Task19 {
        Task19 {
            blueprints: Vec::new(),
            acc1: 0
        }
    }
}

impl aoc::AdventurerOfCode for Task19 {
    fn handle_line(&mut self, line: String) {
        let blueprint: Blueprint = line.parse().unwrap();
        self.blueprints.push(blueprint);
        let blueprint = &self.blueprints[self.blueprints.len()-1];
        blueprint.mk_resource_req_table();
        //self.acc1 += blueprint.id * blueprint.max_geodes() as u16
    }

    fn after(&mut self) {
    }
}

aocfmt!{Task19, self, self.acc1}
aocmain!{Task19}
