use rocket::Route;

mod events;

pub fn routes() -> Vec<Route> {
    routes![
        events::proxy_job_events,
    ]
}
