use std::fs;
use std::time::Instant;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    parent: Option<usize>,
    files: Vec<File>,
    children: Vec<usize>,
}

#[derive(Debug)]
struct Interpreter {
    directories: Vec<Directory>,
    current_directory: usize,
}

#[derive(Debug)]
struct CommandLine {
    command: String,
    args: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    let result = execute(input);
    eprintln!("Elapsed time: {:?}", start.elapsed());
    println!("{}", result);
}

fn execute(input: String) -> usize {
    let interpreter = Interpreter {
        directories: vec![Directory { name: "/".to_string(), parent: None, files: vec![], children: vec![] }],
        current_directory: 0,
    };

    let interpreter = input.split("$ ")
        .filter(|s| s.len() > 0)
        .skip(1)
        .map(|line| parse_command_line(line))
        .fold(interpreter, |interpreter, command_line| {
            match command_line.command.as_str() {
                "cd" => {
                    interpreter.cd(&command_line.args[0])
                }
                "ls" => {
                    interpreter.ls(command_line.output)
                }
                _ => { unimplemented!() }
            }
        });

    let to_free = size_to_free(&interpreter);
    interpreter.directories.iter()
        .map(|directory| interpreter.size_of(directory))
        .filter(|size| *size > to_free)
        .sorted()
        .rev()
        .last().unwrap()
}

impl Interpreter {
    fn cd(&self, name: &String) -> Interpreter {
        if ".." == *name {
            Interpreter {
                directories: self.directories.clone(),
                current_directory: self.current_directory().parent.unwrap(),
            }
        } else {
            Interpreter {
                directories: self.directories.clone(),
                current_directory: self.find_children(name).unwrap(),
            }
        }
    }

    fn ls(&self, output: Vec<String>) -> Interpreter {
        let mut directories = self.directories.clone();
        output.iter().for_each(|f| {
            let x = f.split_once(" ").unwrap();
            match x.0 {
                "dir" => self.add_directory(&mut directories, x.1.to_string()),
                _ => self.add_file(&mut directories, x.1.to_string(), x.0.parse().unwrap()),
            }
        });
        Interpreter { directories, current_directory: self.current_directory }
    }

    fn add_file(&self, directories: &mut Vec<Directory>, name: String, size: usize) {
        directories[self.current_directory].files.push(File {
            name,
            size,
        });
    }

    fn add_directory(&self, directories: &mut Vec<Directory>, name: String) {
        let directory_index = directories.len();
        directories.push(Directory {
            name,
            children: vec![],
            files: vec![],
            parent: Some(self.current_directory),
        });
        directories[self.current_directory].children.push(directory_index);
    }

    fn find_children(&self, name: &String) -> Option<usize> {
        self.current_directory().children.iter().cloned()
            .filter(|i| self.directories[*i].name == *name)
            .last()
    }

    fn current_directory(&self) -> &Directory {
        &self.directories[self.current_directory]
    }

    fn size_of(&self, dir: &Directory) -> usize {
        self.files_size(dir) + self.children_size(dir)
    }

    fn children_size(&self, dir: &Directory) -> usize {
        dir.children.iter()
            .map(|index| self.size_of(&self.directories[*index]))
            .sum::<usize>()
    }

    fn files_size(&self, dir: &Directory) -> usize {
        dir.files.iter()
            .map(|file| file.size)
            .sum::<usize>()
    }
}

fn parse_command_line(command_console: &str) -> CommandLine {
    let instruction = command_console.lines().take(1).last().unwrap().trim();
    let command = instruction.split(" ").take(1).last().unwrap().to_string();
    let args: Vec<String> = instruction.split(" ").skip(1).map(&str::to_string).collect();
    let output: Vec<String> = command_console.lines().skip(1).map(&str::to_string).collect();
    CommandLine {
        command,
        args,
        output,
    }
}

fn size_to_free(interpreter: &Interpreter) -> usize {
    let free_space = 70000000 - interpreter.size_of(&interpreter.directories[0]);
    let to_free = 30000000 - free_space;
    to_free
}

#[test]
fn test_data() {
    assert_eq!(execute(r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
".to_string()), 24933642);
}

