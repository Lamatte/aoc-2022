use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    let result = execute(input);
    eprintln!("Elapsed time: {:?}", start.elapsed());
    println!("{}", result);
}

#[derive(Debug)]
struct CommandLine {
    command: String,
    args: Vec<String>,
    output: Vec<String>,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    children: Vec<Directory>,
    files: Vec<PlainFile>,
    parent: Option<Box<Directory>>,
}

#[derive(Debug, Clone)]
struct PlainFile {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct Context {
    root: Box<Directory>,
    current_directory: Box<Directory>,
}

impl Context {
    fn cd(&self, name: String) -> Context {
        Context {
            root: self.root.clone(),
            current_directory: Box::new(Directory {
                name,
                children: vec![],
                files: vec![],
                parent: Some(self.current_directory.clone()),
            })
        }
    }
}

fn execute(input: String) -> usize {
    let root = Box::new(Directory { name: "/".to_string(), children: vec![], files: vec![], parent: None });
    let context = input.split("$ ")
        .filter(|s| s.len() > 0)
        .skip(1)
        .map(|command_console| {
            let instruction = command_console.lines().take(1).last().unwrap().trim();
            let command = instruction.split(" ").take(1).last().unwrap().to_string();
            let args: Vec<String> = instruction.split(" ").skip(1).map(&str::to_string).collect();
            let output: Vec<String> = command_console.lines().skip(1).map(&str::to_string).collect();
            CommandLine {
                command,
                args,
                output,
            }
        })
        .fold(Context { root: root.clone(), current_directory: root.clone() }, |mut context, command_line| {
            match command_line.command.as_str() {
                "cd" => {
                    //eprintln!("Creating dir {} inside {}", command_line.args[0], context.current_directory.name);
                    context.cd(command_line.args[0].clone())
                }
                "ls" => {
                    command_line.output.iter().for_each(|f|{
                        let x = f.split_once(" ").unwrap();
                        match x.0 {
                            "dir" => {
                                eprintln!("Creating directory {} inside {}", x.1, context.current_directory.name);
                                context.current_directory.children.push(Directory {
                                    name: x.1.to_string(),
                                    children: vec![],
                                    files: vec![],
                                    parent: None,
                                });
                                eprintln!("{:?}", context.current_directory);
                                eprintln!("{:?}", context.root);
                            }
                            _ => {
                                eprintln!("Creating file {} inside {}", x.1, context.current_directory.name);
                                context.current_directory.files.push(PlainFile {
                                    name: x.1.to_string(),
                                    size: x.0.parse().unwrap(),
                                });
                            }
                        }
                    });
                    context
                }
                _ => { unimplemented!() }
            }
        });
    eprintln!("{:?}", context.root);
    0
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
".to_string()), 95437);
}

