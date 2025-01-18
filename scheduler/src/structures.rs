use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::arch::x86_64::_rdtsc;

//questions
//1. use of PCB to store information of previous task before is was prempted

/// Priority enum - higher numeric values mean higher priority level of a task
enum Priority{
    low = 1, 
    medium = 2, 
    high = 3, 
}

enum ThreadState {
    New,
    Ready,
    Running,
    Waiting,
    Terminated
}

/// @Todo - define budget/system time
struct Capability {
    budget : Arc<Mutex<u32>>,
    init_priority : Priority,
    modified_priority : Priority,
    timer : Timer, 
}

//NEED FOR CONTEXT SWITCHING when preemption and going back happens
struct ProcessControlBlock {

}

struct Cycle {

}

pub fn read_tsc() -> u64 {
        unsafe { _rdtsc() }
    }


// A Scheduling Context (SC) is a data structure 
//that holds the metadata required to manage a thread's execution on the CPU.

struct SchedulingContext{
    budget : Arc<Mutex<u64>>,
    period : u64,
    attached_thread_id : Option<u8>, //is there id for threads?

    init_priority : Priority, //initial priority for the first work
    modified_priority : Priority, //changed priority when pre-empted

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