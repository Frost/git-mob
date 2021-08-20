# Git-mob

> A command-line tool for social coding

---

This is a CLI tool for including your co-authors in your commits.

It is essentially a Rust clone of the [git-mob NPM package](https://www.npmjs.com/package/git-mob).

## Install

### MacOS

If you have [Homebrew](https://brew.sh) installed, you can install `git-mob` using my tap:

    brew install frost/tap/git-mob

Otherwise, see the section on binary packages

### Binary packages for MacOS, Ubuntu, and Windows

Download the [latest release](https://github.com/Frost/git-mob/releases/latest) and extract it somewhere so that the binaries end up in your `$PATH`.

### From source

Just run `cargo install git_mob` and you should be all set.

## A note on commit template and git-mob

`git-mob` _will_ currently override any existing `commit.template` setting
in any project where it is run. It does this in order to ensure that `git
commit` will pick up your current mob.

The future plan is to do something a bit smarter, like first detecting if the
repo already has a `commit.template` setting, and in that case, modify the
existing template by adding `Co-authored-by:` trailers to it, or something
similar.


## Examples

* Add Alice, Bob, and yourself as a possible co-authors:

      git add-coauthor a "Alice" alice@example.com
      git add-coauthor b "bob" Bob@example.com
      git add-coauthor me "me" myself.i@example.com

* Set Alice as co-author, making your mob consist of you and Alice:

      git mob a

* Set both Alice and Bob as co-authors, making your mob consist of the three of you:

      git mob a b

* Set Alice as the main author for any commits you make, since she is the one
  doing most of the thinking anyway, and add yourself as a mob member:

      git mob -o a b me

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
* `git -o <initials>` for overwriting the main author
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
* `--installTemplate` and `--uninstallTemplate` for prepare-commit-msg


## Why clone an existing, working CLI tool?

Basically, I was looking for some decent size project to write in Rust, and I
didn't have any other ideas.

The NPM package works just fine. There's just one thing that annoys me about it,
and that is that I have to install it in once for every node version that is
used in any repo I work in. With this approach, I only need to install it once.
