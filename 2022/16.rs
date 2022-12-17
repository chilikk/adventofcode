#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::collections::BTreeMap as Map;

#[derive(Clone)]
struct State {
    pos: Vec<usize>,
    flow: u32,
    total: u32,
    open_valves: Vec<bool>,
}

#[derive(Clone)]
enum Action {
    Open,
    Go(usize),
}

struct SearchState {
    time: u8,
    optimal_states: Vec<State>,
}

struct Task16 {
    valves: Map<String, usize>,
    rev_valves: Vec<String>,
    flow_rates: Vec<u32>,
    graph: Vec<Vec<usize>>,
    graph_tmp: Map<String, Vec<String>>,
    acc1: u32,
    acc2: u32,
}

impl Task16 {
    fn new() -> Task16 {
        Task16 {
            valves: Map::new(),
            rev_valves: Vec::new(),
            flow_rates: Vec::new(),
            graph: Vec::new(),
            graph_tmp: Map::new(),
            acc1: 0,
            acc2: 0,
        }
    }

    fn exec_action(&self, i: usize, a: &Action, pos: usize, s: &mut State) {
        match a {
            Action::Open => {
                if !s.open_valves[pos] {
                    s.flow += self.flow_rates[pos];
                    s.open_valves[pos] = true;
                }
            },
            Action::Go(next) => {
                s.pos[i] = *next;
            },
        }
    }

    fn insert_if_optimal(state: State, optimal_states: &mut Vec<State>) {
        for st in optimal_states.iter_mut().filter(|st|(*st).pos == state.pos) {
            let mut same_valve_state_at_pos = true;
            for &pos in state.pos.iter() {
                same_valve_state_at_pos &= st.open_valves[pos] == state.open_valves[pos]
            }
            if same_valve_state_at_pos {
                if st.flow < state.flow && st.total < state.total {
                    *st = state;
                    return
                } else if st.flow >= state.flow && st.total >= state.total {
                    return
                }
            }
        };
        optimal_states.push(state)
    }

    fn search(&self, s: SearchState) -> u32 {
        //println!("{}: {}", s.time, s.optimal_states.len()); // progress
        let mut new_optimal_states = Vec::new();
        for state in s.optimal_states.iter() {
            let mut avail_actions: Vec<Vec<Action>> = vec![Vec::new(); state.pos.len()];
            for (i, &pos) in state.pos.iter().enumerate() {
                if self.flow_rates[pos] > 0 && ! state.open_valves[pos] {
                    avail_actions[i].push(Action::Open);
                }
                for &next in self.graph[pos].iter() {
                    avail_actions[i].push(Action::Go(next));
                }
            }
            let mut next_states: Vec<State> = Vec::new();
            if state.pos.len() == 1 {
                for action in avail_actions[0].iter() {
                    let mut newst = state.clone();
                    self.exec_action(0, action, state.pos[0], &mut newst);
                    next_states.push(newst);
                }
            } else if state.pos.len() == 2 {
                for action1 in avail_actions[0].iter() {
                    for action2 in avail_actions[1].iter() {
                        let mut newst = state.clone();
                        self.exec_action(0, action1, state.pos[0], &mut newst);
                        self.exec_action(1, action2, state.pos[1], &mut newst);
                        next_states.push(newst);
                    }
                }
            } else {
                panic!("not implemented");
            }
            for newst in next_states.into_iter() {
                Task16::insert_if_optimal(newst, &mut new_optimal_states)
            }
        }
        for st in new_optimal_states.iter_mut() {
            st.total += st.flow
        }
        if s.time == 2 {
            let mut totals: Vec<u32> = new_optimal_states.iter()
                .map(|st|st.total).collect();
            totals.sort_by(|a,b|b.cmp(a));
            totals[0]
        } else {
            self.search(SearchState{
                time: s.time - 1,
                optimal_states: new_optimal_states,
            })
        }
    }
}

impl aoc::AdventurerOfCode for Task16 {
    fn handle_line(&mut self, line: String) {
        let (l1, l2) = line.split_once(";").unwrap();
        assert_eq!("Valve ", &l1[..6]);
        assert_eq!(" has flow rate=", &l1[8..23]);
        let name = &l1[6..8];
        let flow_rate = l1[23..].parse().unwrap();
        let idx = self.flow_rates.len();
        self.flow_rates.push(flow_rate);
        self.valves.insert(name.to_owned(), idx);
        let mut vs: Vec<String> = Vec::new();
        if &l2[..24] == " tunnels lead to valves " {
            for v in l2[24..].split(", ") {
                vs.push(v.to_owned())
            }
        } else if &l2[..23] == " tunnel leads to valve " {
            vs.push(l2[23..].to_owned())
        }
        self.graph_tmp.insert(name.to_owned(), vs);
    }

    fn after(&mut self) {
        self.graph.resize(self.flow_rates.len(), Vec::new());
        for (k,vs) in self.graph_tmp.iter() {
            let tunnels = &mut self.graph[self.valves[k]];
            for v in vs.iter() {
                tunnels.push(self.valves[v])
            }
        }

        self.rev_valves.resize(self.flow_rates.len(), String::new());
        for (name, i) in self.valves.iter() {
            self.rev_valves[*i] = name.clone()
        }

        self.acc1 = self.search(SearchState{
            time: 30,
            optimal_states: vec![State{
                pos: vec![self.valves["AA"]],
                flow: 0,
                total: 0,
                open_valves: vec![false;self.flow_rates.len()]
            }],
        });
        self.acc2 = self.search(SearchState{
            time: 26,
            optimal_states: vec![State{
                pos: vec![self.valves["AA"], self.valves["AA"]],
                flow: 0,
                total: 0,
                open_valves: vec![false;self.flow_rates.len()]
            }],
        });
    }
}

aocfmt!{Task16, self, self.acc1, self.acc2}
aocmain!{Task16}
