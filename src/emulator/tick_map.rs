use crate::emulator::console::Console;
use std::collections::{HashMap, VecDeque};

pub(crate) struct TickMap {
    map: HashMap<u64, VecDeque<fn(&mut Console)>>,
}
