use std::{cell::Cell, sync::{Arc, Barrier, Mutex, MutexGuard}, vec};

use crossbeam::queue::ArrayQueue;
use singlyton::SingletonOption;
use gmod::{lua::LuaReference, lua_string};

thread_local! {
	static PENDING: Cell<usize> = Cell::new(0);
}

pub fn send(lua: gmod::lua::State, arg: Argument) {
	let worker_channel = WORKER_CHANNEL.get().clone();
	
	let worker_channel = lock_channel(&worker_channel);
	match worker_channel {
		Some(channel) => {
			let res = channel.push(arg);

			match res {
				Ok(()) => {
					PENDING.with(|pending| {
						pending.set(pending.get() + 1);
					});
				},
				Err(_) => ()
			}
		}
		None => ()
	}

    if (PENDING.with(|pending| pending.get())) == 1 {
		#[cfg(debug_assertions)]
        println!("Creating timer");
        unsafe {
            lua.get_global(lua_string!("timer"));
            lua.get_field(-1, lua_string!("Create"));
            lua.push_string("compress_worker");
            lua.push_integer(0);
            lua.push_integer(0);
            lua.push_function(think);
            lua.call(4, 0);
            lua.pop();
        }
    }
}

pub struct Argument {
    pub(crate) data: Vec<u8>,
	pub(crate) operarion_is_compress: bool,
	pub(crate) callback: LuaReference,
}

pub enum CallbackResult {
	Success(Argument),
    Failure,
}

static WORKER_THREAD: SingletonOption<std::thread::JoinHandle<()>> = SingletonOption::new();
static WORKER_CHANNEL: SingletonOption<Arc<Mutex<ArrayQueue<Argument>>>> = SingletonOption::new();
static CALLBACK_CHANNEL: SingletonOption<Arc<Mutex<ArrayQueue<CallbackResult>>>> = SingletonOption::new();

fn process(arg: Argument) {
    let queue = CALLBACK_CHANNEL.get();
	let mut data = arg.data.clone();

	let data_slice = data.as_mut_slice();

	let res;

	match arg.operarion_is_compress {
		true => {
			res = gmod_lzma::compress(data_slice, 9).unwrap();
		},
		false => {
			res = gmod_lzma::decompress(data_slice).unwrap();
		}
	}

	let result = Argument {
		data: res,
		operarion_is_compress: arg.operarion_is_compress,
		callback: arg.callback,
	};
	queue.lock().unwrap().force_push(CallbackResult::Success(result));

}

fn lock_channel<T>(channel: &Arc<Mutex<ArrayQueue<T>>>) -> Option<MutexGuard<'_, ArrayQueue<T>>> {
    match channel.lock() {
        Ok(guard) => Some(guard),
        Err(_) => None,
    }
}

fn worker(barrier: Arc<Barrier>) {
    let argument_queue = Arc::new(Mutex::new(ArrayQueue::<Argument>::new(100)));
    let return_queue = Arc::new(Mutex::new(ArrayQueue::<CallbackResult>::new(100)));

    WORKER_CHANNEL.replace(argument_queue.clone());
	CALLBACK_CHANNEL.replace(return_queue.clone());

	barrier.wait();

	let rt = tokio::runtime::Builder::new_current_thread()
		.thread_name("compress-worker-thread")
		.worker_threads(4)
		.build()
		.expect("Failed to start Tokio runtime");

	println!("[COMPRESS] Worker thread started");

	let worker_channel = argument_queue.clone();

	rt.block_on(async move {
		loop {
			let request_option = {
				let worker_channel_guard = worker_channel.lock();
				match worker_channel_guard {
					Ok(mut guard) => guard.pop(),
					Err(_) => None
				}
			};

			if let Some(request) = request_option {
				tokio::task::spawn_blocking(move || {
					process(request);
				});
			}
		}
	});
}

unsafe extern "C-unwind" fn think(lua: gmod::lua::State) -> i32 {
    let callback_channel = CALLBACK_CHANNEL.get().clone();

    while let Some(result) = callback_channel.lock().unwrap().pop() {
        match result {
            CallbackResult::Success(callback) => {
				let data = callback.data.clone();

				lua.from_reference(callback.callback);
				lua.dereference(callback.callback);

				lua.push_binary_string(data.as_ref());
				lua.pcall_ignore(1, 0);
            },
            CallbackResult::Failure => {}
        }

        let pending = PENDING.with(|pending| {
            let n = pending.get().saturating_sub(1);
            pending.set(n);
            n
        });

        if pending == 0 {
            #[cfg(debug_assertions)]
            println!("Removing timer");
            // Remove the worker hook
            lua.get_global(lua_string!("timer"));
            lua.get_field(-1, lua_string!("Remove"));
            lua.push_string("compress_worker");
            lua.call(1, 0);
            lua.pop();

            return 0;
        }
    }

    0
}


pub fn init() {
	let barrier = Arc::new(Barrier::new(2));
	let barrier_ref = barrier.clone();

	WORKER_THREAD.replace(std::thread::spawn(move || worker(barrier_ref)));

	barrier.wait();
}

pub fn shutdown(lua: gmod::lua::State) {
	unsafe {
		// Remove the worker hook
		lua.get_global(lua_string!("timer"));
		lua.get_field(-1, lua_string!("Remove"));
		lua.push_string("compress_worker");
		lua.call(1, 0);
		lua.pop();
	}

	{
		// Drop the channels, allowing us to join with the worker thread
		CALLBACK_CHANNEL.take();
		WORKER_CHANNEL.take();
	}

	if let Some(handle) = WORKER_THREAD.take() {
		handle.join().ok();
	}
}