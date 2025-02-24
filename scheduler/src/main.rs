//Module: cpos::Scheduler
//File : scheduler.rs
//Author : Donggyu Kim(DK)
//Purpose : Defines object structs needed to implement thread-level scheduling based on Tcap and scCap paper

use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime, Instant};
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const DEADLINE_SUCCESS : usize = 0;
const DEADLINE_MISS: usize = 1;
const MAX_TASKS : usize = 8; // can be changed
const INIT_BUDGET : usize = 10;  // can be changed
static TASK_ID_COUNTER: AtomicU8 = AtomicU8::new(0);



// Arc::new(Mutex::new(Stats));

//Priority enum that is assigned to Capability struct,
//which indicates priority(importance) of a Capability's task
//Higher numeric values mean higher priority level  
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Priority{
    Low = 1, 
    Medium = 2, 
    High = 3, 
}

//List of possible pre-defined scheduling policies encoded to an enum
enum Policy {
    //FCFS, for example
    Monotonic = 1, //using simple monotonic policy for now
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
    budget : u32,
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
    // timer : Timer,
    task_list : Vec<Task>, 
}

struct Thread {
    capability : Capability,
    scheduler : ThreadScheduler,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Task {
    task_id: u8,
    execution_time: u8,
    priority : Priority,
}

impl Task {
    pub fn new(execution_time : u8, priority : Priority) -> Self {
        Self {
            execution_time,
            task_id : TASK_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            priority,
        }
        // let mut counter = RUNNING_TASK_ID.lock().unwrap();

        // *counter += 1;

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

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Priority::High, Priority::High) => std::cmp::Ordering::Equal,
            (Priority::High, _) => std::cmp::Ordering::Greater,
            (Priority::Medium, Priority::High) => std::cmp::Ordering::Less,
            (Priority::Medium, Priority::Medium) => std::cmp::Ordering::Equal,
            (Priority::Medium, Priority::Low) => std::cmp::Ordering::Greater,
            (Priority::Low, Priority::Low) => std::cmp::Ordering::Equal,
            (Priority::Low, _) => std::cmp::Ordering::Less,
        }
    }
}

// Implement PartialOrd for Priority
impl PartialOrd for Priority {
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
    pub fn new(policy: Policy) -> Self {
        Self {
            policy, 
            // timer,
            task_list: Vec::<Task>::new(),
        }
    }


    pub fn add_task(&mut self, task: Task) {
        self.task_list.push(task);
        self.task_list.sort_by(|a, b| {
            b.priority.cmp(&a.priority) 
                .then_with(|| a.execution_time.cmp(&b.execution_time)) 
        });
    }

    pub fn run(&self) {

        for task in &self.task_list {
            println!("Running task number : {}", task.task_id);
            println!("Running time : {}", task.execution_time);
            println!("Priority : {:?}", task.priority);

        }
    }
    

     //replenish budget of a Capability with a fixed amount
    // pub fn replenish_budget(&self){
    //     capability.budget = INIT_BUDGET; //for now use 10 as a default budget for replenishment
         
    // }

    // //decision function that decides whether the task will miss or beat deadline
    // //this function uses other functions that return boolean values depending on budget and deadline of a Capability
    // pub fn decision_function(&self) {

    //     if (is_budget_exceeded_before_deadline(capability)) {
    //         borrow_budget(capability, other_capability){
    //             //what if this becomes nested?
    //             //for example you can borrow again here if you still need a budget
    //             //should I implement maximum budget borrowing count?
    //         }

    //         //
          
    //     }

    //     return is_complete_before_deadline(capability)
    // }

   
    // pub fn is_budget_exceeded(&self) -> bool {

    // }


    // pub fn is_complete_before_deadline(&self) ->  bool {

    // }

    // // pub fn is_budget_left(&self) -> bool {

    // // }
    

    // pub fn borrow_budget(&self, other_capability: &mut Capability) {
        
        
    // }

    // pub fn donate_excess_budget(&self, other_capability: &mut Capability) {

    // }

    

}


fn main() {

    let mut thread_sched : ThreadScheduler = ThreadScheduler::new(Policy::Monotonic);

    thread_sched.add_task(Task::new(6, Priority::Low));  
    thread_sched.add_task(Task::new(10, Priority::Low));
    thread_sched.add_task(Task::new(20, Priority::Medium));   
    thread_sched.add_task(Task::new(4, Priority::Medium));  
    thread_sched.add_task(Task::new(10, Priority::High));   

    thread_sched.run();
}
