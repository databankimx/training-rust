use std::cell::Cell;
use std::thread;

// Cell lets query change the count through &self. Cell is Send but not Sync, so DbConnection is Send but not Sync too.
struct DbConnection {
    queries_run: Cell<u32>,
}

impl DbConnection {
    fn new() -> DbConnection {
        DbConnection {
            queries_run: Cell::new(0),
        }
    }

    fn query(&self, sql: &str) -> String {
        self.queries_run.set(self.queries_run.get() + 1);
        format!("result of {sql}")
    }
}

fn main() {
    // The connection moves into the thread, which is allowed, because DbConnection is Send.
    let conn = DbConnection::new();
    let handle = thread::spawn(move || {
        println!("{}", conn.query("SELECT 1"));
        conn.queries_run.get()
    });

    let count = handle.join().unwrap();
    println!("queries run on the connection: {count}");

    // Try: share one connection between two threads with Arc<DbConnection>. What does the compiler say, and why?
}
