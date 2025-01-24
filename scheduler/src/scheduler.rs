//Module: cpos::Scheduler
//File : scheduler.rs
//Author : Donggyu Kim(DK)
//Purpose : Defines object structs needed to implement thread-level scheduling based on Tcap and scCap paper

use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

//Priority enum that is assigned to Capability struct,
//which indicates priority(importance) of a Capability's task
//Higher numeric values mean higher priority level
enum Priority{
    low = 1, 
    medium = 2, 
    high = 3, 
}

//List of possible pre-defined scheduling policies encoded to an enum
enum Policy {
    //FCFS, for example
}

//5 different thread state based on exeucution and budget
enum ThreadState {
    Release, 
    Suspension, //when there is no budget
    Initialization, //when there inited with some budget
    Resumption, //when there is some budget
    Complete,
}


//Capability struct that includes 3 descriptors : budget, priority, timer
struct Capability {
    budget : u32
    priority : Priority,
    timer : Timer,
}

//Timer struct used to trigger replenishment of budget of a capability or release task, etc.
struct Timer {
    isTriggered : bool,
    //use system time or rdtsc?
    timestamp : SystemTime,
}

//ThreadScheduler struct that uses information of each thread based on given policy
struct ThreadScheduler {
    policy : Policy,
    timer : Timer,
}

struct Thread {
    capability : Capability,
    scheduler : ThreadScheduler,
}

impl Capability {
    pub fn new(budget : u32, priority : Priority, timer : Timer) -> Self {
        Self {budget, 
            priority,
            timer
        }
    }
}

impl ThreadScheduler {
    pub fn new() -> Self {
        Self {}
    }
}
