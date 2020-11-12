# todo
A todo-list/calendar

# Currently in the very early stages
This is a utility which you can use to create customizably repeating todo lists over a calendar.

# Docker
* Docker must be installed, and you need an `.env` file with `DATABASE_URL`, `POSTGRES_DB`, `POSTGRES_USER`, `POSTGRES_PASSWORD` set. (`DATABASE_URL` is a composition of all of the others, should look something like `postgres://POSTGRES_USER:POSTGRES_PASSWORD@db:5432/POSTGRES_DB)`.
* Run `docker build -t todo_prebuild .` to create a faster booting image. (Working on making this better, to skip the current long build time)
* Run `docker-compose up` to start the services. The database will run on `0.0.0.0:5433` should you need direct access, and  the API service will run on `localhost:8080`
* ## NOTE: SET UP dotenv so the service expects the right ports

# Running without Docker
* install [rust and cargo](https://www.rust-lang.org/tools/install)
* an `env.sh` file, with the same variables set as in the docker section, with `export`.
* install postgresql. On a mac, I recommend using Brew `brew install postgresql; brew services start postgresql`.
* install the diesel CLI: `cargo install diesel_cli --no-default-features --features postgres
* Set your environment variables. The `env.sh` file contains the development defaults. Apply them with `source env.sh`.
* run `diesel setup` to create the database and apply the migrations.

* Run the app or the service `cargo run --bin todo_cli`
* Run a command:
```
list
make 01-01-2021 y `New Years` `Happy New Year! Welcome 2021!`
list
```

# Todo
* the internal command `make` is currently busted. Not sure what's up with that yet.
* cal.rs should be changed so that it uses Result<_, TDError>
* Update to the latest version of rust async/await. It's been several months since I've had time to update it and I want to make sure it's as stable as possible. At the time of development (Sept-Nov of `19) async/await was unstable and requires the nightly compiler in order to run.
* Create a client version which can run against the server. Currently the only real way to test is by hitting specific endpoints with a `CURL` or appropriate wrapper (like Postman).
* Rename task_item.lalrpop to parser (or something more semantically clear)

# Planning
With the pivot to using a database, it seems time to come up with a real structure for the code.
Currently:
cli.rs is the head of the program. It parses commands and calls them in async-direct-cmd
async-direct-cmd has the functionality to read/write the database.
task.rs has the TaskItem, Modification and RawTaskItem structs + impl.
* I want defined and planned paths from command -> parse -> action.
what is the parse step?
what is the action step?

with "make 11-16 `New Title` `These are the notes`" what happens?
```
make raw task item
pass to async-direct-cmd::make {
  parse date => { cal::start() }
   * this part is strange. Because the raw date is a vec<u32>, we can't directly get a guaranteed date from it.
  write the database with the values
}
```
So then how do we get the date? I can pass it off separately to cal and then extract everything else separately. It would be nice to get TaskItem::from<RawTaskItem> implemented, but I'm not sure it's necessary here. Another solution would be to make a toSqlString::Result<String, TDError> for the RawTaskItem, so we can determine the date there safely.