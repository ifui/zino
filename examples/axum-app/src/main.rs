mod controller;
mod router;
mod schedule;

use zino::Application;

fn main() {
    zino::AxumCluster::boot()
        .register(router::init_routes())
        .spawn(schedule::init_jobs())
        .run(schedule::init_async_jobs())
}
