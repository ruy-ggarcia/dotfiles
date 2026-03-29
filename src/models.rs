#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub shell: Shell,
}
