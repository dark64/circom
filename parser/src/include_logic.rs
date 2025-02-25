use super::errors::FileOsError;
use program_structure::error_definition::Report;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub struct FileStack {
    current_location: PathBuf,
    black_paths: HashSet<PathBuf>,
    stack: Vec<PathBuf>,
}

impl FileStack {
    pub fn new(src: PathBuf) -> FileStack {
        let mut location = src.clone();
        location.pop();
        FileStack { current_location: location, black_paths: HashSet::new(), stack: vec![src] }
    }

    pub fn add_include(f_stack: &mut FileStack, path: String) -> Result<(), Report> {
        let mut crr = f_stack.current_location.clone();
        crr.push(path.clone());
        let path = std::fs::canonicalize(crr)
            .map_err(|_| FileOsError { path: path.clone() })
            .map_err(|e| FileOsError::produce_report(e))?;
        if !f_stack.black_paths.contains(&path) {
            f_stack.stack.push(path);
        }
        Ok(())
    }

    pub fn take_next(f_stack: &mut FileStack) -> Option<PathBuf> {
        loop {
            match f_stack.stack.pop() {
                None => {
                    break None;
                }
                Some(file) if !f_stack.black_paths.contains(&file) => {
                    f_stack.current_location = file.clone();
                    f_stack.current_location.pop();
                    f_stack.black_paths.insert(file.clone());
                    break Some(file);
                }
                _ => {}
            }
        }
    }
}

pub struct IncludesNode {
    pub path: PathBuf,
    pub custom_gates_pragma: bool,
}

#[derive(Default)]
pub struct IncludesGraph {
    nodes: Vec<IncludesNode>,
    adjacency: HashMap<PathBuf, Vec<usize>>,
    custom_gates_nodes: Vec<usize>,
}

impl IncludesGraph {
    pub fn new() -> IncludesGraph {
        IncludesGraph::default()
    }

    pub fn add_node(
        &mut self,
        path: PathBuf,
        custom_gates_pragma: bool,
        custom_gates_usage: bool
    ) {
        self.nodes.push(IncludesNode { path, custom_gates_pragma });
        if custom_gates_usage {
            self.custom_gates_nodes.push(self.nodes.len() - 1);
        }
    }

    pub fn add_edge(&mut self, path: String) -> Result<(), Report> {
        let mut crr = self.nodes.last().unwrap().path.clone();
        crr.pop();
        crr.push(path.clone());
        let path = std::fs::canonicalize(crr)
            .map_err(|_| FileOsError { path: path })
            .map_err(|e| FileOsError::produce_report(e))?;
        let edges = self.adjacency.entry(path).or_insert(vec![]);
        edges.push(self.nodes.len() - 1);
        Ok(())
    }

    pub fn get_problematic_paths(&self) -> Vec<Vec<PathBuf>> {
        let mut problematic_paths = Vec::new();
        for from in &self.custom_gates_nodes {
            problematic_paths.append(
                &mut self.traverse(*from, Vec::new(), HashSet::new())
            );
        }
        problematic_paths
    }

    fn traverse(
        &self,
        from: usize,
        path: Vec<PathBuf>,
        traversed_edges: HashSet<(usize, usize)>
    ) -> Vec<Vec<PathBuf>> {
        let mut problematic_paths = Vec::new();
        let (from_path, using_pragma) = {
            let node = &self.nodes[from];
            (&node.path, node.custom_gates_pragma)
        };
        let new_path = {
            let mut new_path = path.clone();
            new_path.push(from_path.clone());
            new_path
        };
        if !using_pragma {
            problematic_paths.push(new_path.clone());
        }
        if let Some(edges) = self.adjacency.get(from_path) {
            for to in edges {
                let edge = (from, *to);
                if !traversed_edges.contains(&edge) {
                    let new_traversed_edges = {
                        let mut new_traversed_edges = traversed_edges.clone();
                        new_traversed_edges.insert(edge);
                        new_traversed_edges
                    };
                    problematic_paths.append(
                        &mut self.traverse(*to, new_path.clone(), new_traversed_edges)
                    );
                }
            }
        }
        problematic_paths
    }

    pub fn display_path(path: &Vec<PathBuf>) -> String {
        let path = path.iter().map(|file| -> String {
            let file = format!("{}", file.display());
            let (_, file) = file.rsplit_once("/").unwrap();
            file.clone().to_string()
        }).collect::<Vec<String>>();
        let mut path_covered = format!("{}", path[0]);
        for file in &path[1..] {
            path_covered = format!("{} -> {}", path_covered, file);
        }
        path_covered
    }
}
