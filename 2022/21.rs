#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::collections::BTreeMap;

#[derive(Copy,Clone)]
enum Expr {
    Add,
    Sub,
    Mult,
    Div,
    Equals,
}

struct Task21 {
    evaluated: BTreeMap<String,i64>,
    exprs: BTreeMap<String,(String,String,Expr)>,
    backrefs: BTreeMap<String,Vec<String>>,
    mode: Mode,
}

impl Task21 {
    fn new(mode: Mode) -> Task21 {
        Task21 {
            evaluated: BTreeMap::new(),
            exprs: BTreeMap::new(),
            backrefs: BTreeMap::new(),
            mode: mode,
        }
    }
}

impl Task21 {
    fn add_backref(&mut self, key: String, backref: String) {
        if let Some(v) = self.backrefs.get_mut(&key) {
            v.push(backref);
        } else {
            self.backrefs.insert(key, vec![backref]);
        };
    }

    fn solve(&self) -> i64 {
        if let Mode::Subtask1 = self.mode {
            if let Some(n) = self.evaluated.get("root") {
                *n
            } else {
                panic!("no answer")
            }
        } else {
            if let Some((akey,bkey,Expr::Equals)) = self.exprs.get("root") {
                match (self.evaluated.get(akey), self.evaluated.get(bkey)) {
                    (Some(a), None) => self.solve_eq(bkey, *a),
                    (None, Some(b)) => self.solve_eq(akey, *b),
                    _               => panic!("cannot solve"),
                }
            } else {
                panic!("cannot solve")
            }
        }
    }

    fn try_eval(&mut self, expr_key: String) -> bool {
        if let Some((akey,bkey,expr)) = self.exprs.get(&expr_key) {
            let value = match (self.evaluated.get(akey), self.evaluated.get(bkey)) {
                (Some(a), Some(b)) => Some(Task21::eval_expr(*a, *b, *expr)),
                _                  => None,
            };
            match value {
                Some(v) => {
                    self.exprs.remove(&expr_key);
                    self.evaluated.insert(expr_key, v);
                    true
                },
                None => false,
            }
        } else {
            false
        }
    }

    fn eval_expr(a: i64, b:i64, expr: Expr) -> i64 {
        match expr {
            Expr::Add => a+b,
            Expr::Sub => a-b,
            Expr::Mult => a*b,
            Expr::Div => a/b,
            _ => panic!("cannot solve")
        }
    }

    fn solve_eq(&self, exprkey: &String, res: i64) -> i64 {
        if exprkey == "humn" {
            res
        } else {
            if let Some((akey, bkey, expr)) = self.exprs.get(exprkey) {
                match (self.evaluated.get(akey), self.evaluated.get(bkey), expr) {
                    (Some(a), None, Expr::Add) => self.solve_eq(bkey, res-*a),
                    (Some(a), None, Expr::Sub) => self.solve_eq(bkey, *a-res),
                    (Some(a), None, Expr::Mult) => self.solve_eq(bkey, res/ *a),
                    (Some(a), None, Expr::Div) => self.solve_eq(bkey, *a/res),
                    (None, Some(b), Expr::Add) => self.solve_eq(akey, res-*b),
                    (None, Some(b), Expr::Sub) => self.solve_eq(akey, *b+res),
                    (None, Some(b), Expr::Mult) => self.solve_eq(akey, res/ *b),
                    (None, Some(b), Expr::Div) => self.solve_eq(akey, *b*res),
                    _                         => panic!("cannot solve"),
                }
            } else {
                panic!("cannot solve")
            }
        }
    }
}

impl aoc::AdventurerOfCode for Task21 {
    fn handle_line(&mut self, line: String) {
        let (key, value) = line.split_once(": ").unwrap();
        if let Some((a,b)) = value.split_once(" + ") {
            self.add_backref(a.to_string(), key.to_string());
            self.add_backref(b.to_string(), key.to_string());
            self.exprs.insert(key.to_string(),
                (a.to_string(),b.to_string(),Expr::Add));
        } else if let Some((a,b)) = value.split_once(" - ") {
            self.add_backref(a.to_string(), key.to_string());
            self.add_backref(b.to_string(), key.to_string());
            self.exprs.insert(key.to_string(),
                (a.to_string(),b.to_string(),Expr::Sub));
        } else if let Some((a,b)) = value.split_once(" * ") {
            self.add_backref(a.to_string(), key.to_string());
            self.add_backref(b.to_string(), key.to_string());
            self.exprs.insert(key.to_string(),
                (a.to_string(),b.to_string(),Expr::Mult));
        } else if let Some((a,b)) = value.split_once(" / ") {
            self.add_backref(a.to_string(), key.to_string());
            self.add_backref(b.to_string(), key.to_string());
            self.exprs.insert(key.to_string(),
                (a.to_string(),b.to_string(),Expr::Div));
        } else {
            self.evaluated.insert(key.to_string(), value.parse().unwrap());
        }
        if let Mode::Subtask2 = self.mode {
            if key == "root" {
                if let Some((a,b,_)) = self.exprs.get(key) {
                    let v = (a.to_string(), b.to_string(), Expr::Equals);
                    self.exprs.insert(key.to_string(), v);
                }
            } else if key == "humn" {
                self.exprs.remove(key);
                self.evaluated.remove(key);
            }
        }
    }

    fn after(&mut self) {
        loop {
            let mut solved = 0;
            let keys_copy: Vec<String> = self.evaluated.keys().cloned().collect();
            for key in keys_copy.into_iter() {
                match self.backrefs.remove(&key) {
                    None => (),
                    Some(expr_keys) => {
                        for key in expr_keys.into_iter() {
                            if self.try_eval(key) {
                                solved += 1
                            }
                        }
                    }
                }
            }
            if solved == 0 {
                break
            }
        }
    }
}

aocfmt!{Task21, self, self.solve()}
aocsubtasks!{Task21}
