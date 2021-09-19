pub mod item;
pub mod user;

/*
  Anything that touches the DB is called through these files. At some point the actual database touching
  should be abstracted into the models

  These are also the entry points that the todo_cli uses to get to the database. Once these models are finished
  they should be handled directly by the todo_cli or route handler, and the actions folder should go away.
*/
