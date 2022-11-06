pub mod database;

use database::*;

pub fn main() {
    println!("Hello");

    let con = &mut establish_connection();

    let jobs = get_jobs(con);

    if jobs.len() > 0 {
        for job in jobs {}
    }
}
