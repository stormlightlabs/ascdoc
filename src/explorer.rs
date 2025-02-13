use std::path::PathBuf;
use std::time::SystemTime;
use std::{fs, io};

// This enum represents the different types of entries we might find
#[derive(Debug, Clone)]
pub enum NodeType {
    File,
    Directory,
    Other,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub path: PathBuf,
    pub entry_type: NodeType,
    pub is_dir: bool,
    pub children: Vec<TreeNode>,
    _modified: SystemTime,
}

impl TreeNode {
    fn new(path: &PathBuf, m: fs::Metadata) -> io::Result<Self> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let is_dir = m.is_dir();
        let modified = m.modified()?;

        let entry_type = if is_dir {
            NodeType::Directory
        } else if m.is_file() {
            NodeType::File
        } else {
            NodeType::Other
        };

        Ok(Self {
            path: path.to_owned(),
            name,
            is_dir,
            _modified: modified,
            children: Vec::new(),
            entry_type,
        })
    }

    pub fn from_path(path: &PathBuf) -> std::io::Result<Self> {
        let metadata = fs::metadata(&path)?;

        let mut fs_entry = TreeNode::new(path, metadata)?;

        if fs_entry.is_dir {
            for entry in fs::read_dir(&fs_entry.path)? {
                let entry = entry?;
                fs_entry.children.push(TreeNode::from_path(&entry.path())?);
            }
        }

        Ok(fs_entry)
    }
}
