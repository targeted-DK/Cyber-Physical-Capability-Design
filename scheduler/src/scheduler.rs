//Module: cpos::Scheduler
//File : scheduler.rs
//Author : Donggyu Kim(DK)
//Purpose : Defines object structs needed to implement thread-level scheduling based on Tcap and scCap paper

use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, Instant};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const DEADLINE_SUCCESS : usize = 0;
const DEADLINE_MISS: usize = 1;
const MAX_TASKS : usize = 8; // can be changed
const INIT_BUDGET : usize = 10;  // can be changed
static mut running_task_id : Arc<Mutex<u8>> = Arc::new(Mutex::new(0));


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
//Capability provides the temporal capability to a task
struct Capability {
    budget : u32
    priority : Priority,
    timer : Timer,
}

//Timer struct used to trigger replenishment of budget of a capability or release task, etc.
struct Timer {
    timestamp : SystemTime,
}

//ThreadScheduler struct that uses information of each thread based on given policy
struct ThreadScheduler {
    policy : Policy,
    timer : Timer,
    task_list : Vec<Capability>, 
}

struct Thread {
    capability : Capability,
    scheduler : ThreadScheduler,
}

#[derive(Eq, PartialEq)]
struct Task {
    execution_time: Instant,
    priority : Priority,
    task_id: u8,
}

impl Task {
    pub fn new(execution_time : usize) -> Self{
        Self {
            execution_time : execution_time,
            task_id : running_task_id,
        }
        let mut counter = running_task_id.lock().unwrap();
        *counter += 1;

    }

    // pub fn run(&self) {
    //     let mut sum = 0;
    //     for tick in iter {
    //         sum += 1;
    //     }
    // }
} 

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.execution_time.cmp(&other.execution_time)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Capability {
    pub fn new(budget : u32, priority : Priority, timer : Timer) -> Self {
        Self {
            budget, 
            priority,
            timer
        }
    }
}

impl ThreadScheduler {
    pub fn new(policy : Policy, timer : Timer) -> Self {
        Self {
            policy, 
            timer,
            // task_list : Vec::with_capacity(MAX_TASKS),
            task_list : BinaryHeap<Reverse<T>>,
        }
    }

     //replenish budget of a Capability with a fixed amount
    pub fn replenish_budget(&self){
        capability.budget = INIT_BUDGET; //for now use 10 as a default budget for replenishment
         
    }

    //decision function that decides whether the task will miss or beat deadline
    //this function uses other functions that return boolean values depending on budget and deadline of a Capability
    pub fn decision_function(&self) {

        if (is_budget_exceeded_before_deadline(capability)) {
            borrow_budget(capability, other_capability){
                //what if this becomes nested?
                //for example you can borrow again here if you still need a budget
                //should I implement maximum budget borrowing count?
            }

            //
          
        }

        return is_complete_before_deadline(capability)
    }

   
    pub fn is_budget_exceeded(&self) -> bool {

    }


    pub fn is_complete_before_deadline(&self) ->  bool {

    }

    // pub fn is_budget_left(&self) -> bool {

    // }
    

    pub fn borrow_budget(&self, other_capability: &mut Capability) {
        
        
    }

    pub fn donate_excess_budget(&self, other_capability: &mut Capability) {

    }

    

}



//few reminders :
//1. preemption is scheduler's job
//2. check linux for other designs
//3. scheduler deals with multiple threads


//questions :
//deadline?