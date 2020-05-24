# Git-mob

> A command-line tool for social coding

---

This is a CLI tool for including your co-authors in your commits.

It is essentially a Rust clone of the [git-mob NPM package](https://www.npmjs.com/package/git-mob).

## Install

Just run `cargo install git_mob` and you should be all set.


## Examples

* Add Alice and Bob as a possible co-authors:

      git add-coauthor a "Alice" alice@example.com
      git add-coauthor b "bob" Bob@example.com

* Set Alice as co-author, making your mob consist of you and Alice:

      git mob a

* Set both Alice and Bob as co-authors, making your mob consist of the three of you:

      git mob a b

* Edit Bob's name, since you accidentally capitalized his email instead of his name:

      git edit-coauthor b --name "Bob" --email bob@example.com

* Remove Bob as a possible co-author:

      git delete-coauthor b

* List all available co-authors:

      git mob -l

* Go back to coding by yourself again:

      git solo


## Working features

* `git mob <co-author-initials>`
* `git add-coauthor <initials> "Co-author Name" <co-author-email-address>`
* `git edit-coauthor [--name "Co-author Name"] [--email <co-author-email-address>]`
* `git delete-coauthor <initials>`
* `git mob -l`
* `git solo`


## TODO

There are some missing features from the original NPM package, that are yet to
be implemented, and then there is also a severe lack of tests and documentation.


### Missing features

* `git mob-print`
* `git suggest-coauthors`
* `-o` for overwriting the main author
* `--installTemplate` and `--uninstallTemplate` for prepare-commit-msg


## Why clone an existing, working CLI tool?

Basically, I was looking for some decent size project to write in Rust, and I
didn't have any other ideas.

The NPM package works just fine. There's just one thing that annoys me about it,
and that is that I have to install it in once for every node version that is
used in any repo I work in. With this approach, I only need to install it once.