    use candid::{types::{number::Nat,},CandidType,Deserialize,};
    use ic_cdk_macros::*;
    use std::cell::RefCell;

thread_local!{
    static TODOS : RefCell<Vec<Todo>>  = RefCell::new(Default::default());
    static ID_COUNTER : RefCell<Nat> = RefCell::new(Nat::from(0_u32));

}

#[derive(Clone,Debug,Default,CandidType,Deserialize)]
struct Todo {
    id: Nat,
    task : String,
    completed : Status
}

impl Todo{
    fn new(task: String, id :Nat) -> Todo{
        Todo{
            id,
            task ,
            completed : Status::Pending
        }
    }

}


#[update]
fn create_todo(task: String) -> Result<Todo,()>{
    let id = ID_COUNTER.with(|el| el.borrow_mut().0.clone());
   let todo = Todo::new(task,Nat::from(id));
    TODOS.with(|el| el.borrow_mut().push(todo.clone()));
   ID_COUNTER.with(|el| *el.borrow_mut() += 1_u32 );

   Ok(todo)
}

#[derive(Clone,Debug,Default,CandidType,Deserialize)]
enum Status {
Done,
#[default]
Pending,
}
