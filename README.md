Rust prison CLI

A simple CLI app written in Rust.

Core features:
1.Cli promopts to enter: ID, First name, Last name, Height & weight, Criminal record
2.Store as struct
3.Save as JSON, load back and display
4.Simple menu & clean code structure
5.Add search: search by name or id
6.Deleting a prisoner by id
7.Add terminal colors using colored
8.Add filter: show only prisoner over X height/Weight or with crimes including a keyword
9.CLI argument support(using clap for argument parsing)
10.Serialize into multiple formats: json, csv, toml
11.Encrypted JSON file with aes crate
12.Add log with long and env_logger
13.Unit test for serialize/deserialize
14.Optional SQLlite database(via rusqlite, transition from file based to database)
15.Version keep history of edits(like git)
