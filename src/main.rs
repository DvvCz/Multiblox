mod singleton;

#[derive(PartialEq, Clone)]
enum Events {
	Close,
}

fn main() {
	// If you're overwhelmed by the amount of code here, don't worry
	// This single variable declaration is all that matters with regards to patching Roblox.
	// The rest of it just makes this use a system tray icon.
	let singleton = singleton::Singleton::new().expect("Failed to acquire mutex");

	let (stop_send, stop_recv) = std::sync::mpsc::channel::<bool>();
	let (event_send, event_recv) = std::sync::mpsc::channel::<Events>();

	let icon = trayicon::TrayIconBuilder::new()
		.sender(event_send)
		.icon_from_buffer(include_bytes!("../assets/icon.ico"))
		.tooltip("Multiblox")
		.menu(trayicon::MenuBuilder::new().item("Close", Events::Close))
		.build()
		.expect("Failed to create trayicon");

	let join = std::thread::spawn(move || {
		event_recv.iter().next();
		let _ = stop_send.send(true); // Couldn't have hung up
	});

	loop {
		use winapi::um::winuser;

		let _ = icon;
		let _ = singleton;

		if let Ok(true) = stop_recv.try_recv() {
			let _ = join.join(); // Can't possibly panic
			break;
		}

		let mut msg = core::mem::MaybeUninit::uninit();
		let bret = unsafe { winuser::GetMessageA(msg.as_mut_ptr(), 0 as _, 0, 0) };
		if bret > 0 {
			unsafe {
				winuser::TranslateMessage(msg.as_ptr());
				winuser::DispatchMessageA(msg.as_ptr());
			}
		} else {
			break;
		}
	}
}
