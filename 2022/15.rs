#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::collections::BTreeSet;

#[derive(Clone,PartialEq,PartialOrd,Eq,Ord)]
struct Point{x: i32, y: i32}
struct Sensor{p: Point, coverage: i32}
#[derive(PartialEq,PartialOrd,Eq,Ord)]
struct Range{start: i32, end: i32}

impl std::str::FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Point, ()> {
        let s1 = s.replace("x=","");
        let (x, y) = s1.split_once(", y=").unwrap();
        Ok(Point{x: x.parse().unwrap(), y: y.parse().unwrap()})
    }
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
        (self.x-other.x).abs() + (self.y-other.y).abs()
    }
}

impl Range {
    fn contains(&self, x: i32) -> bool {
        self.start <= x && self.end >= x
    }

    fn len(&self) -> i32 {
        self.end - self.start + 1
    }
}

impl Sensor {
    fn find_point_just_outside_range(&self, x: i32) -> Option<(Point, Point)> {
        let xdist = (self.p.x-x).abs();
        if xdist > self.coverage + 1 {
            None
        } else {
            Some((Point{x: x, y: self.p.y+(self.coverage+1-xdist)},
                  Point{x: x, y: self.p.y-(self.coverage+1-xdist)}))
        }
    }
}

struct Task15 {
    sensors: Vec<Sensor>,
    beacons: BTreeSet<Point>,
}

impl Task15 {
    fn new() -> Task15 {
        Task15 {
            sensors: Vec::new(),
            beacons: BTreeSet::new(),
        }
    }

    fn task1(&self, controlline: i32) -> i32 {
        let mut ranges: Vec<Range> = Vec::new();
        for sensor in self.sensors.iter() {
            if sensor.p.y == controlline {
                ranges.push(Range{start: sensor.p.x, end: sensor.p.x})
            }
            let controldist = (sensor.p.y-controlline).abs();
            if controldist <= sensor.coverage {
                ranges.push(Range{
                    start: sensor.p.x-(sensor.coverage-controldist),
                    end: sensor.p.x+(sensor.coverage-controldist),
                })
            }
        }
        //compact ranges
        ranges.sort();
        let mut i = 0;
        while i < ranges.len()-1 {
            if ranges[i].contains(ranges[i+1].start) {
                ranges[i].end = ranges[i].end.max(ranges[i+1].end);
                ranges.remove(i+1);
            } else {
                i += 1
            }
        }
        let mut positions = 0;
        for range in ranges.iter() {
            positions += range.len();
            for beacon in self.beacons.iter() {
                if beacon.y == controlline && range.contains(beacon.x) {
                    positions -= 1
                }
            }
        }
        positions
    }

    fn task2(&self, min: Point, max: Point) -> i64 {
        // iterate over points just outside sensor range
        for sensor in self.sensors.iter() {
            if sensor.p.dist(&min) > sensor.coverage {
                for x in min.x..max.x+1 {
                    if let Some((pabove, pbelow)) = sensor.find_point_just_outside_range(x) {
                        if pabove.y >= min.y && pabove.y <= max.y {
                            match self.task2_test_point(&pabove) {
                                Some(v) => return v,
                                None => ()
                            }
                        }
                        if pbelow.y <= max.y && pbelow.y >= min.y {
                            match self.task2_test_point(&pbelow) {
                                Some(v) => return v,
                                None => ()
                            }
                        }
                    }
                }
            }
        };
        0
    }

    fn task2_test_point(&self, p: &Point) -> Option<i64> {
        for sensor in self.sensors.iter() {
            if sensor.p.dist(p) <= sensor.coverage {
                return None
            }
        }
        return Some(p.x as i64*4000000 + p.y as i64)
    }
}

aocfmt!{Task15, self, self.task1(2000000), self.task2(Point{x:0,y:0}, Point{x:4000000,y:4000000})}

impl aoc::AdventurerOfCode for Task15 {
    fn handle_line(&mut self, line: String) {
        let line1 = line.replace("Sensor at ", "");
        let (s, b) = line1.split_once(": closest beacon is at ").unwrap();
        let sensor: Point = s.parse().unwrap();
        let beacon: Point = b.parse().unwrap();
        let dist = sensor.dist(&beacon);
        self.sensors.push(Sensor{p: sensor, coverage: dist});
        self.beacons.insert(beacon);
    }

    fn after(&mut self) {
    }
}

aocmain!{Task15}
