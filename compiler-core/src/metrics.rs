use std::time::Instant;

/// Metrics collected during compilation
#[derive(Debug, Clone)]
pub struct CompilerMetrics {
    /// Starting time
    pub start_time: Instant,
    /// Time spent during the packing phase
    pub pack_time_ms: Option<u64>,
    /// Time spent during the compiling phase
    pub comp_time_ms: Option<u64>,
    /// Time spent during the transforming phase (plugin execution)
    pub plug_time_ms: Option<u64>,
    /// Time spent during the execution phase
    pub exec_time_ms: Option<u64>,
}

impl CompilerMetrics {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            pack_time_ms: None,
            comp_time_ms: None,
            plug_time_ms: None,
            exec_time_ms: None,
        }
    }

    pub fn pack_done(&mut self) {
        self.pack_time_ms = Some(self.start_time.elapsed().as_millis() as u64);
        self.start_time = Instant::now();
    }
    pub fn comp_done(&mut self) {
        self.comp_time_ms = Some(self.start_time.elapsed().as_millis() as u64);
        self.start_time = Instant::now();
    }
    pub fn plug_done(&mut self) {
        self.plug_time_ms = Some(self.start_time.elapsed().as_millis() as u64);
        self.start_time = Instant::now();
    }
    pub fn exec_done(&mut self) {
        self.exec_time_ms = Some(self.start_time.elapsed().as_millis() as u64);
        self.start_time = Instant::now();
    }
}
