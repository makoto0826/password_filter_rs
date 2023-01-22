use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use once_cell::sync::OnceCell;
use windows::Win32::Foundation::{BOOL, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "stdcall" fn InitializeChangeNotify() -> BOOL {
    BOOL::from(true)
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "stdcall" fn PasswordFilter(
    account_name: *mut UNICODE_STRING,
    full_name: *mut UNICODE_STRING,
    password: *mut UNICODE_STRING,
    set_operation: BOOL,
) -> BOOL {
    BOOL::from(true)
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "stdcall" fn PasswordChangeNotify(
    user_name: *mut UNICODE_STRING,
    relative_id: u32,
    new_password: *mut UNICODE_STRING,
) -> NTSTATUS {
    if user_name.is_null() || new_password.is_null() {
        return STATUS_SUCCESS;
    }

    unsafe {
        let Ok(user_name) = (*user_name).Buffer.to_string() else {
            return STATUS_SUCCESS;
        };

        let Ok(new_password) = (*new_password).Buffer.to_string() else {
            return STATUS_SUCCESS;
        };

        let context_lock = Context::get_or_init();
        let context = context_lock.lock().unwrap();

        context.send(Command::ChangedPassword {
            user_name,
            new_password,
        });
    }

    return STATUS_SUCCESS;
}

enum Command {
    ChangedPassword {
        user_name: String,
        new_password: String,
    },
    Terminated,
}

struct Context {
    sender: Sender<Command>,
    handle: Option<JoinHandle<()>>,
}

impl Context {
    fn get_or_init() -> Arc<Mutex<Context>> {
        static CONTEXT: OnceCell<Arc<Mutex<Context>>> = OnceCell::new();

        CONTEXT.get_or_init(|| {
            let (sender, receiver) = mpsc::channel();
            let handle = run_worker(receiver);

            Arc::new(Mutex::new(Context {
                sender,
                handle: Some(handle),
            }))
        });

        CONTEXT.get().unwrap().clone()
    }

    fn send(&self, command: Command) {
        let _ = self.sender.send(command);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if self.handle.is_none() {
            return;
        }

        self.send(Command::Terminated);
        let _ = self.handle.take().unwrap().join();
    }
}

fn run_worker(receiver: Receiver<Command>) -> JoinHandle<()> {
    thread::spawn(move || {
        while let Ok(command) = receiver.recv() {
            match command {
                Command::ChangedPassword {
                    user_name,
                    new_password,
                } => {
                    println!("{} {}", user_name, new_password);
                }
                Command::Terminated => return,
            }
        }
    })
}
