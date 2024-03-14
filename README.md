# Rust Todo CLI
## Description
Created using Rust 1.76 with CLAP and SQLX. It was a project to start learning the basics of Rust and I don't think that I have done too bad.
## How to Use
In order to use this, create a .env file and make sure to have the <ins>**DATABASE_URL**</ins> set to a PostgreSQL database. Run the migration to create the table and that's it!
## Commands
1. List
2. Add [DESCRIPTION]
3. Done [ID]

#### List
```bash
    todo_cli list
```

#### Add
```bash
    todo_cli add "DESCRIPTION"
```

#### Done
```bash
    todo_cli done "ID"
```
