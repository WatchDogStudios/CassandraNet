use superluminal_perf::prelude::*;

/// Public interface for profiling with cassandra-net. This is a wrapper around the superluminal_perf crate. if you want to use a different profiler, you can implement this trait for that profiler.
impl cnprofile {
    pub fn start_event(&self, event_name: &str) {
        superluminal_perf::begin_event(event_name);
    }

    pub fn end_event(&self, event_name: &str) {
        superluminal_perf::end_event();
    }

    pub fn set_thread_name(&self, thread_name: &str) {
        superluminal_perf::set_current_thread_name(thread_name);
    }

    pub fn start_event_with_data(&self, event_name: &str, data: &str, color: u32) {
        superluminal_perf::begin_event_with_data(event_name, data, color);
    }

    pub fn unregister_fiber(in_fiber_id: u64) {
        superluminal_perf::unregister_fiber(in_fiber_id);
    }

    pub fn begin_fiber_switch(in_current_fiber_id: u64, in_new_fiber_id: u64) {
        superluminal_perf::begin_fiber_switch(in_current_fiber_id, in_new_fiber_id);
    }

    pub fn end_fiber_switch(in_fiber_id: u64) {
        superluminal_perf::end_fiber_switch(in_fiber_id);
    }
}
