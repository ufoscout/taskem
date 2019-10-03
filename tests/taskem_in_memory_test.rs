use lazy_static::lazy_static;
use maybe_single::MaybeSingle;

use taskem::Schedulem;
use taskem::repository::in_memory::InMemorySchedulerRepository;
use std::sync::Arc;

mod tests;

lazy_static! {
    pub static ref SINGLETON: MaybeSingle<(Schedulem, &'static str)> = MaybeSingle::new(|| init());
}

fn init() -> (
    Schedulem,
    &'static str,
) {

    let repo = Arc::new(InMemorySchedulerRepository::new());
    let schedulem = Schedulem::new(repo);

    (schedulem, "")
}

pub fn test(callback: fn(&Schedulem) -> Result<(), Box<dyn std::error::Error>>) {
    SINGLETON.get(|(schedulem, _)| {
        callback(&schedulem).unwrap();
    });
}
