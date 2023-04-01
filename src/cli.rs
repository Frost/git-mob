use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, name = "git-mob")]
/// Assemble a group of co-authors to help you on your coding quest
pub struct GitMob {
    /// Prints list of available co-authors
    #[clap(short, long)]
    pub list: bool,
    /// Overwrite the main author
    #[clap(short, long)]
    pub overwrite: Option<String>,
    /// A list of co-author initials
    pub coauthors: Vec<String>,
}

#[derive(Parser, Debug)]
#[clap(name = "git-add-coauthor", version)]
/// Add a co-author to your list of available co-authors
pub struct GitAddCoauthor {
    /// Co-author initials
    pub initials: String,
    /// The name of the co-author, in quotes, e.g. "Foo Bar"
    pub name: String,
    /// The email of the co-author
    pub email: String,
}

#[derive(Parser, Debug)]
#[clap(name = "git-edit-coauthor", version)]
/// Edit a co-author in your list of available co-authors
pub struct GitEditCoauthor {
    /// Co-author initials
    pub initials: String,
    /// The name of the co-author, in quotes, e.g. "Foo Bar"
    pub name: String,
    /// The email of the co-author
    pub email: String,
}

#[derive(Parser, Debug)]
#[clap(name = "git-delete-coauthor", version)]
/// Delete a co-author from your list of available co-authors
pub struct GitDeleteCoauthor {
    /// Co-author initials
    pub initials: String,
    /// The name of the co-author, in quotes, e.g. "Foo Bar"
    pub name: String,
    /// The email of the co-author
    pub email: String,
}

#[derive(Parser, Debug)]
#[clap(name = "git-solo", version)]
/// Disband the mob and continue working solo.
pub struct GitSolo {}
