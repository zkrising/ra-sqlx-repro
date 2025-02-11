# Rust Analyzer + Multiple Sqlx Bug Repro

This is a reproduction of the bug described in [https://github.com/rust-lang/rust-analyzer/issues/18533](Issue #18533).

This repo is a workspace with two crates, `sqlite_red` and `sqlite_yellow`.

In `sqlite_red`, there exists a table called `red_things`, and in `sqlite_yellow` there exists `yellow_things`. Both of these crates run queries on their respective tables.

`cargo check` outputs no issues, and both binaries run correctly.

However, if you open this repository in vscode with rust-analyzer enabled, you will see
errors in one of the crates' `main.rs` files. Which one gives you an error seems to depend
on what file you view first. You might also have to edit the files and save them to
trigger the bug, either way I've found the bug happens after trying to do things in
both `main.rs` files.

# The bug

The error message itself will state "no such table `red_things`" or vice versa, in the crate
where `red_things` most definitely exists. This is because (as far as I can tell) the sqlx
queries are being ran on **BOTH** database urls. Not one or the other, but **BOTH**.

You can see other suspicious behaviour here, like, putting a syntax error in the query will give you two error messages instead of one.

Note that none of these messages are present in `cargo check`. Just in rust-analyzer output.