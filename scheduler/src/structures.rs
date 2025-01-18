use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};


/// Priority enum - higher numeric values mean higher priority level of a task
enum Priority{
    low = 1, 
    medium = 2, 
    high = 3, 
}

enum ThreadState {
    Inactive, 
    Running,
    Idle,
    Expired,
}

/// @Todo - define budget/system time
struct Capability {
    budget : Arc<Mutex<u32>>,
    init_priority : Priority,
    modified_priority : Priority,
    timer : Timer,
}



struct Task {
    budget_needed : Arc<Mutex<u32>>,
    isAssignedThread : bool,
    isBudgetLeft : bool,
    deadline : u32,
}

///Timer Struct to trigger events or track timels
///
///# Arguments
///
/// * 'timestamp' - stores a future time value that triggers certain event such as deadline expiration and preemption
///              - or tracks time based on CPU cylces, such as rdtsc that reads timestampe 
///
struct Timer {

    isTriggered : bool,
    //use system time or rdtsc?
    timestamp : SystemTime,


}


struct ThreadScheduler {

}

///SchedulingContextObject(scCap) - represents the maximum CPU bandwidth the thread can consume
///
///# Parameters
///- 'period'
///- 'cap'
///
///# Returns
///
struct SchedulingContextObject{
    period : Time,
    cap : Capability, //capability possess budget field
    task : Task,

}

struct Task {
    isDone : bool,
    priority : Priority,
    
    
}


impl SchedulingContext {
    fn new() -> Self {
        Self {
            // name: Capability::new();
            // 
        }
    }
   
    
}