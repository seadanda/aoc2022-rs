use std::path::PathBuf;
use std::{error::Error, fs::read_to_string};

type Files = Vec<(usize, String)>;
type Tree = Vec<(PathBuf, Files)>;
type E = Box<dyn Error>;

pub fn parse_input(filename: &str) -> Result<Tree, E> {
    let repl = read_to_string(filename)?;
    let commands = repl.split("$ ");
    let mut wd = PathBuf::new();
    let mut files: Files;

    let mut tree: Tree = Vec::new();

    for command in commands {
        if command.starts_with("cd ..") {
            wd.pop();
        } else if command.starts_with("cd ") {
            let dir = command.split(' ').last().unwrap().trim();
            wd.push(dir);
        } else if command.starts_with("ls") {
            files = command
                .strip_prefix("ls\n")
                .unwrap()
                .lines()
                .map(|l| {
                    if l.starts_with("dir") {
                        (0, l.strip_prefix("dir ").unwrap().to_string())
                    } else {
                        let mut s = l.split_ascii_whitespace();
                        (
                            s.next().unwrap().parse().unwrap(),
                            s.next().unwrap().to_string(),
                        )
                    }
                })
                .collect();

            tree.push((wd.clone(), files.clone()));
        } else {
            continue;
        }
    }

    Ok(tree)
}

pub fn get_directory_sizes(filename: &str) -> Result<Vec<(usize, String)>, E> {
    let mut dirs: Files = Vec::new();
    let mut tree: Tree = parse_input(filename)?;

    // sort tree to reverse hierarchy
    tree.sort_by(|(p1, _), (p2, _)| {
        p2.ancestors()
            .count()
            .partial_cmp(&p1.ancestors().count())
            .unwrap()
    });

    for (dir, contents) in tree {
        let mut sum = 0;
        for (filesize, name) in contents {
            if filesize == 0 {
                // subdirectory - grab total from dirs
                let mut fullname = PathBuf::from(&dir);
                fullname.push(name);
                let (dirsize, _) = dirs
                    .iter()
                    .find(|(_, d)| *d == fullname.to_str().unwrap())
                    .unwrap();
                sum += dirsize;
            } else {
                sum += filesize;
            }
        }
        dirs.push((sum, dir.to_str().unwrap().to_string()))
    }

    Ok(dirs)
}

pub fn get_directory_sum_under(filename: &str, limit: usize) -> Result<usize, E> {
    let dirs = get_directory_sizes(filename)?;

    Ok(dirs
        .iter()
        .map(|(s, _)| if *s <= limit { s } else { &0 })
        .sum())
}

pub fn choose_directory_with_size(filename: &str, limit: usize) -> Result<usize, E> {
    let mut dirs = get_directory_sizes(filename)?;
    // sort dirs
    dirs.sort_by(|(s1, _), (s2, _)| s1.cmp(s2));

    // find first gte limit
    let (dirsize, _)= *dirs.iter().find(|(s, _)| *s >= limit).unwrap();

    Ok(dirsize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chooses_directory_with_size() {
        assert_eq!(
            choose_directory_with_size("input/day7.test", 8381165).unwrap(),
            24933642
        );
    }

    #[test]
    fn gets_directory_sum_limit_100000() {
        assert_eq!(
            get_directory_sum_under("input/day7.test", 100000).unwrap(),
            95437
        );
    }

    #[test]
    fn gets_directory_sizes() {
        assert_eq!(
            get_directory_sizes("input/day7.txt").unwrap(),
            vec![
                (584, String::from("/a/e")),
                (94853, String::from("/a")),
                (24933642, String::from("/d")),
                (48381165, String::from("/"))
            ]
        )
    }

    #[test]
    fn reads_file() {
        assert_eq!(
            read_to_string("input/day7.test").unwrap().lines().count(),
            23
        );
    }
}
