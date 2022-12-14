use std::{fs};

use petgraph::{Graph, stable_graph::NodeIndex, visit::{IntoNodeReferences}, Direction::{Incoming, Outgoing}};


#[cfg(test)]
mod tests {

    use petgraph::visit::IntoNodeReferences;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cwd_change() {
        let mut fs = FileSystem::new();    
        let mut cwd = fs.root;
        assert_eq!(fs.fsgraph[cwd].name, "/");
        fs.add_child(cwd, Inode::new_dir("a".to_owned()));
        fs.add_child(cwd, Inode::new_file("b.txt".to_owned(), 14848514));
        fs.add_child(cwd, Inode::new_file("c.dat".to_owned(), 8504156));
        
        cwd = cd_or_create_dir(&mut fs, cwd, "a");
        assert_eq!(fs.fsgraph[cwd].name, "a");
        fs.add_child(cwd, Inode::new_dir("e".to_owned()));
        fs.add_child(cwd, Inode::new_file("f".to_owned(), 29116));
        fs.add_child(cwd, Inode::new_file("g".to_owned(), 2557));
        fs.add_child(cwd, Inode::new_file("h.lst".to_owned(), 62596));
        
        cwd = cd_or_create_dir(&mut fs, cwd, "e");
        assert_eq!(fs.fsgraph[cwd].name, "e");
        fs.add_child(cwd, Inode::new_file("i".to_owned(), 584));

        cwd = cd_or_create_dir(&mut fs, cwd, "..");
        assert_eq!(fs.fsgraph[cwd].name, "a");
        cwd = cd_or_create_dir(&mut fs, cwd, "..");
        assert_eq!(fs.fsgraph[cwd].name, "/");

        cwd = cd_or_create_dir(&mut fs, cwd, "d");
        assert_eq!(fs.fsgraph[cwd].name, "d");

        fs.add_child(cwd, Inode::new_file("j".to_owned(), 4060174));
        fs.add_child(cwd, Inode::new_file("d.log".to_owned(), 8033020));
        fs.add_child(cwd, Inode::new_file("d.ext".to_owned(), 5626152));
        fs.add_child(cwd, Inode::new_file("d.ext".to_owned(), 7214296));


        for (index, inode) in fs.fsgraph.node_references() {
            if inode.filetype == FType::DIR {
                let size = fs.get_children_size(index);
                let expected = match inode.name.as_str() {
                    "/" => 48381165,
                    "e" => 584,
                    "a" => 94853,
                    "d" => 24933642,
                    _ => panic!("unreachable"),
                };
                assert_eq!(size, expected);
                //println!("{}: {}", inode.name, size);
            }
        }
        

    }


    #[test]
    fn test_part2_solver() {
        let mut fs = FileSystem::new();
        let cwd = fs.root;        
        cd_or_create_dir(&mut fs, cwd, "foo");
        fs.cd(cwd, "foo").unwrap();
    }
}

fn parse_input(filepath: &str) -> FileSystem {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data:Vec<_> = data.trim().split("\n").collect();    
    let mut fs:FileSystem = FileSystem::new();
    let mut cwd = fs.root;
    for line in data {
        if line.starts_with("$") {
            let cmd = &line[2..4];
            if cmd == "cd" {
                let dir = &line[5..];
                cwd = cd_or_create_dir(&mut fs, cwd, dir);
            }
        } else {
            // If line does not start with '$', we have directory listing
            let ls_entry = line.split(" ").collect::<Vec<&str>>();

            // If entry starts with "dir" create new dir under cwd
            if ls_entry[0].trim() == "dir" {
                fs.add_child(cwd, Inode::new_dir(ls_entry[1].to_owned()));
                continue;
            } else {
                // If entry is file, create a new file under cwd
                let size = ls_entry[0].parse::<u32>().unwrap();
                let filename = ls_entry[1];
                fs.add_child(cwd, Inode::new_file(filename.to_owned(), size));
            }
        }
    }
    fs
}

fn _change_cwd(mut cwd: String, cdval: &str) -> String {
    if cdval.starts_with("/") {
        cwd = String::from(cdval);
    } else if cdval == ".." {
        let foo = cwd.split("/").collect::<Vec<&str>>();
        let foo = &foo[..foo.len()-1];
        cwd = foo.join("/");
        if cwd == "" {
            cwd = "/".to_owned();
        }
    } else {
        if cwd.len() == 1 && cwd.chars().nth(0).unwrap() == '/' {
            cwd = format!("/{}", cdval);
        } else {
            cwd = format!("{}/{}", cwd,cdval);
        }
    }
    cwd
}
#[derive(PartialEq)]
#[derive(Debug)]
enum FType {
    DIR,
    FILE,
}
#[derive(Debug)]
struct Inode {
    name: String,
    size: u32,
    filetype: FType, 
}

impl Inode {
    pub fn new(name: String, size: u32, filetype: FType) -> Self {
        Inode { name, size, filetype }
    }

    pub fn new_file(name: String, size: u32) -> Self {
        Inode::new(name, size, FType::FILE)
    }

    pub fn new_dir(name: String) -> Self {
        Inode::new(name, 0, FType::DIR)
    }
}

struct FileSystem {
    fsgraph: Graph<Inode, Child>,
    root: NodeIndex,    
}

impl FileSystem {
    pub fn new() -> Self {
        let root_inode = Inode {
            name: "/".to_string(),
            size: 0,
            filetype: FType::DIR
        };

        let mut fsgraph:Graph<Inode, Child> = Graph::new();
        let root = fsgraph.add_node(root_inode);
        FileSystem { fsgraph, root }
    }

    // Only supports relative path in one level
    pub fn cd(&self, cwd: NodeIndex, path: &str) -> Option<NodeIndex> {
        if path == ".." {
          return self.fsgraph.neighbors_directed(cwd, Incoming).nth(0);
        } 
        // Iterate over all children
        // Get the child inode which has same name as path
        let cwd = self.fsgraph.neighbors_directed(cwd, Outgoing)
                    .filter(|x| self.fsgraph[*x].name == path && self.fsgraph[*x].filetype == FType::DIR)
                    .nth(0);

        // If inode.name == path then return that NodeIndex
        cwd
    }

    pub fn add_child(&mut self, parent: NodeIndex, inode: Inode) -> NodeIndex{
        if self.fsgraph[parent].filetype == FType::FILE {
            panic!("Cannot add child to file inode");
        }

        let graph = &mut self.fsgraph;
        let child = graph.add_node(inode);
        self.fsgraph.add_edge(parent, child, Child);
        child
    }

    pub fn get_children_size(&self, dir: NodeIndex) -> u32 {
        let mut size: u32 = 0;
        let current_node = &self.fsgraph[dir];
        if current_node.filetype == FType::FILE {
            return current_node.size;
        }

        for node in self.fsgraph.neighbors_directed(dir, Outgoing) {
            size += self.get_children_size(node);
        }
        size
    }

}

struct Child;

fn cd_or_create_dir(fs: &mut FileSystem, cwd: NodeIndex, path: &str) -> NodeIndex {
    match fs.cd(cwd, path) {
        Some(i) => {
            return i;
        },
        None => {
            if path == ".." {
                panic!("dir should exist when using ..")
            }
            // Create a dir
            fs.add_child(cwd, Inode::new_dir(path.to_string()));
            fs.cd(cwd, path).unwrap()
        }
    }
}

pub fn part1() {
    let fs = parse_input("./src/day07/input.txt");

    // Parsing is done    
    let mut result = 0;
    for (index, inode) in fs.fsgraph.node_references() {
        if inode.filetype == FType::DIR {
            let size = fs.get_children_size(index);
            if size < 100000 {
                result += size;
            }
        }
    }
    //ANS: 1908462
    println!("ANS: {:?}", result);
}

pub fn part2() {
    let fs = parse_input("./src/day07/input.txt");
    let mut result = 0xffffffff;

    // Parsing is done    
    let occupied = fs.get_children_size(fs.root);
    let total = 70000000;
    let free = total - occupied;
    if free > 30000000 {
        panic!("Unreachable!");
    }
    let need_space = 30000000 - free;
    
    for (index, inode) in fs.fsgraph.node_references() {
        if inode.filetype == FType::DIR {
            let size = fs.get_children_size(index);
            if size > need_space && size < result{
                result = size;
            }            
        }
    }
    
    //ANS: 1908462
    println!("ANS: {:?}", result);
}