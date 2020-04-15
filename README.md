# todo
A todo-list/calendar

# Currently in the very early stages
This is a utility which you can use to create customizably repeating todo lists over a calendar.

# Running
* install postgresql. On a mac, I recommend using Brew `brew install postgresql; brew services start postgresql`.
* create a database. The default name for the repo is `caldata`: `createdb caldata`.
* _optional_ If you're using the dumped backup, use `psql caldata < ./dumps/dumpname`.
* install the rust toolchain nightly version `rustc 1.40.0-nightly (91fd6283e 2019-11-02)`: `rustup toolchain install nightly-2019-11-02`.
* Set your environment variables. The `env.sh` file contains the development defaults. Apply them with `source env.sh`.
* Set your cargo to use the nightly build version and run the cli: `rustup toolchain default nightly; cargo run --bin cli`
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