use std::sync::{Arc, Mutex};
use crate::questions::Question;

#[derive(Clone, Default)]
pub struct AppDatabase{
    pub questions: Arc<Mutex<Vec<Question>>>,
}

// think of Arc<>  as a shared ptr.