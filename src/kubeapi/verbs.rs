use super::*;

#[derive(Debug)]
pub enum Verb {
    Create,
    Get,
    List,
    Watch,
    Delete,
    DeleteCollection,
    Update,
    Patch,
}

impl Verb {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Get => "get",
            Self::List => "list",
            Self::Watch => "watch",
            Self::Delete => "delete",
            Self::DeleteCollection => "delete_collection",
            Self::Update => "update",
            Self::Patch => "patch",
        }
    }
}

impl fmt::Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
