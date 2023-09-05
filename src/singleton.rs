extern "system" {
	fn CreateMutexA(
		opts: *const core::ffi::c_void,
		takeover: u32,
		name: *const i8,
	) -> *const core::ffi::c_void;
	fn ReleaseMutex(m: *const core::ffi::c_void);
	fn CloseHandle(m: *const core::ffi::c_void);
}

pub struct Singleton {
	mutex: *const core::ffi::c_void,
}

impl Singleton {
	#[inline]
	pub fn new() -> core::option::Option<Self> {
		let mutex = unsafe {
			CreateMutexA(
				core::ptr::null(),
				1,
				"ROBLOX_singletonMutex\0".as_ptr() as _,
			)
		};
		if mutex.is_null() {
			None
		} else {
			Some(Self { mutex })
		}
	}
}

impl Drop for Singleton {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			ReleaseMutex(self.mutex);
			CloseHandle(self.mutex);
		}
	}
}
