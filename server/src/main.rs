mod db;
pub mod models;
pub mod schema;

pub fn main() {
    println!("Hello");

    let con = &mut db::establish_connection();

    let jobs = db::get_jobs(con);
    for job in jobs {
        println!("{}", job.id);
    }
}
