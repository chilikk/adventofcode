mod aoc;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

struct File {
    //name: String,
    size: u32,
}

struct Dir {
    name: String,
    parent: Option<Rc<RefCell<Dir>>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

impl Dir {
    fn size(&self) -> u32 {
        let mut size = 0u32;
        for dirref in self.dirs.iter() {
            size += dirref.borrow().size()
        }
        for file in self.files.iter() {
            size += file.size
        }
        size
    }

    fn task1(&self) -> u32 {
        let mut size = 0u32;
        let selfsize = self.size();
        if selfsize <= 100000 {
            size += selfsize
        }
        for dirref in self.dirs.iter() {
            size += dirref.borrow().task1()
        }
        size
    }

    fn task2(&self, req: u32, curmin: u32) -> u32 {
        let mut cur = curmin;
        let selfsize = self.size();
        if selfsize >= req && selfsize < curmin {
            cur = selfsize;
        }
        for dirref in self.dirs.iter() {
            cur = dirref.borrow().task2(req, cur)
        }
        cur
    }
}

enum Cmd {
    Ls,
    None,
}

struct Task7 {
    root: Rc<RefCell<Dir>>,
    cwd: Option<Rc<RefCell<Dir>>>,
    cmd: Cmd,
}

impl Task7 {
    fn new() -> Task7 {
        let root = Rc::new(RefCell::new(Dir {
            name: String::from("/"),
            parent: None,
            dirs: Vec::new(),
            files: Vec::new(),
        }));
        Task7 {
            root: root,
            cwd: None,
            cmd: Cmd::None,
        }
    }

    fn cd(& mut self, arg: &str) {
        if arg == "/" {
            self.cwd = Some(self.root.clone())
        } else if arg == ".." {
            let newcwd = match &self.cwd {
                Some(cwd) => match &cwd.borrow().parent {
                    Some(parent) => Some(parent.clone()),
                    None => Some(self.root.clone()),
                },
                None => None
            };
            self.cwd = newcwd
        } else {
            self.cwd = match &self.cwd {
                Some(cwd) => {
                    let mut newcwd = None;
                    for dir in cwd.borrow().dirs.iter() {
                        if arg == dir.borrow().name {
                            newcwd = Some(dir.clone());
                        }
                    };
                    newcwd
                },
                None => None,
            }
        }
    }

    fn handle_cmd(& mut self, line: String) {
        let (prompt, cmdline) = line.split_once(' ').unwrap();
        assert_eq!(prompt, "$");
        if cmdline == "ls" {
            self.cmd = Cmd::Ls
        } else {
            let (exec, arg) = cmdline.split_once(' ').unwrap();
            if exec == "cd" {
                self.cd(arg)
            }
        }
    }

    fn handle_ls(& mut self, line: String) {
        let (token1, token2) = line.split_once(' ').unwrap();
        if token1 == "$" {
            self.cmd = Cmd::None;
            self.handle_cmd(line)
        } else if token1 == "dir" {
            let cwd = self.cwd.as_ref().unwrap();
            let mut cwd_mut = cwd.borrow_mut();
            cwd_mut.dirs.push(Rc::new(RefCell::new(Dir {
                name: token2.to_string(),
                parent: Some(cwd.clone()),
                dirs: Vec::new(),
                files: Vec::new(),
            })))
        } else {
            let cwd = self.cwd.as_ref().unwrap();
            let mut cwd_mut = cwd.borrow_mut();
            cwd_mut.files.push(File {
                //name: token2.to_string(),
                size: token1.parse().unwrap()
            })
        }
    }

    fn task1(&self) -> u32 {
        self.root.borrow().task1()
    }

    fn task2(&self) -> u32 {
        let root_size = self.root.borrow().size();
        let req = root_size - (70_000_000 - 30_000_000);
        self.root.borrow().task2(req, root_size)
    }
}

impl fmt::Display for Task7 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.task1(), self.task2())
    }
}

impl aoc::AdventurerOfCode for Task7 {
    fn handle_line(&mut self, line: String) {
        match self.cmd {
            Cmd::None => self.handle_cmd(line),
            Cmd::Ls => self.handle_ls(line),
        }
    }

    fn after(&mut self) {
    }
}

fn main() {
    aoc::run(&mut Task7::new())
}
