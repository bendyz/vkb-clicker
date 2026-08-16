use clap::Parser;
use input_linux::sys::BUS_VIRTUAL;
use input_linux::{
    EventKind, EventTime, InputId, Key, KeyEvent, KeyState, SynchronizeEvent, UInputHandle,
};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Autoclicker dla Waylanda - wirtualne urzadzenie myszy przez /dev/uinput.
///
/// Uruchomienie bez --kill rozpoczyna klikanie w biezacej pozycji kursora.
/// Podepnij te komende oraz `vkb-clicker --kill` pod dwie osobne kombinacje
/// klawiszy w ustawieniach skrotow swojego srodowiska (np. KDE System Settings
/// -> Shortcuts -> Custom Shortcuts).
#[derive(Parser, Debug)]
#[command(name = "vkb-clicker", version, about)]
struct Args {
    /// Czas przytrzymania przycisku myszy w milisekundach
    #[arg(long, default_value_t = 20)]
    click_ms: u64,

    /// Czas przerwy pomiedzy kliknieciami w milisekundach
    #[arg(long, default_value_t = 10)]
    pause_ms: u64,

    /// Przycisk myszy do klikania: left, right lub middle
    #[arg(long, default_value = "left")]
    button: String,

    /// Zatrzymaj dzialajaca instancje vkb-clicker i zakoncz
    #[arg(long)]
    kill: bool,
}

fn pid_file() -> PathBuf {
    let dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(dir).join("vkb-clicker.pid")
}

fn read_running_pid() -> Option<i32> {
    let content = std::fs::read_to_string(pid_file()).ok()?;
    let pid: i32 = content.trim().parse().ok()?;
    if Path::new(&format!("/proc/{pid}")).exists() {
        Some(pid)
    } else {
        None
    }
}

fn kill_running() {
    match read_running_pid() {
        Some(pid) => {
            unsafe {
                libc::kill(pid, libc::SIGTERM);
            }
            println!("Wyslano sygnal stop do procesu {pid}.");
            std::thread::sleep(Duration::from_millis(100));
        }
        None => {
            println!("Brak dzialajacej instancji vkb-clicker.");
        }
    }
    let _ = std::fs::remove_file(pid_file());
}

fn resolve_button(name: &str) -> Key {
    match name {
        "left" => Key::ButtonLeft,
        "right" => Key::ButtonRight,
        "middle" => Key::ButtonMiddle,
        other => {
            eprintln!("Nieznany przycisk '{other}' (dozwolone: left, right, middle)");
            std::process::exit(1);
        }
    }
}

fn send_key(handle: &UInputHandle<File>, key: Key, state: KeyState) {
    let time = EventTime::default();
    let events = [
        KeyEvent::new(time, key, state).into_event().into_raw(),
        SynchronizeEvent::report(time).into_event().into_raw(),
    ];
    handle
        .write(&events)
        .expect("blad zapisu zdarzenia do /dev/uinput");
}

fn main() {
    let args = Args::parse();

    if args.kill {
        kill_running();
        return;
    }

    if let Some(pid) = read_running_pid() {
        eprintln!("vkb-clicker juz dziala (PID {pid}). Zatrzymaj go przez `vkb-clicker --kill`.");
        std::process::exit(1);
    }

    let key = resolve_button(&args.button);

    std::fs::write(pid_file(), std::process::id().to_string())
        .expect("nie mozna zapisac pliku pid");

    let running = Arc::new(AtomicBool::new(true));
    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
        })
        .expect("nie mozna zarejestrowac handlera sygnalu");
    }

    let uinput_file = OpenOptions::new()
        .write(true)
        .open("/dev/uinput")
        .expect("nie mozna otworzyc /dev/uinput (sprawdz uprawnienia)");
    let handle = UInputHandle::new(uinput_file);

    handle.set_evbit(EventKind::Key).expect("set_evbit");
    handle.set_keybit(Key::ButtonLeft).expect("set_keybit left");
    handle.set_keybit(Key::ButtonRight).expect("set_keybit right");
    handle
        .set_keybit(Key::ButtonMiddle)
        .expect("set_keybit middle");

    let id = InputId {
        bustype: BUS_VIRTUAL,
        vendor: 0x1234,
        product: 0x5678,
        version: 1,
    };
    handle
        .create(&id, b"vkb-clicker virtual mouse", 0, &[])
        .expect("nie mozna utworzyc urzadzenia uinput");

    println!(
        "vkb-clicker: klik {}ms / przerwa {}ms, przycisk: {} (PID {}).",
        args.click_ms,
        args.pause_ms,
        args.button,
        std::process::id()
    );
    println!("Zatrzymaj przez: vkb-clicker --kill");

    let click_dur = Duration::from_millis(args.click_ms);
    let pause_dur = Duration::from_millis(args.pause_ms);

    let mut pressed = false;
    while running.load(Ordering::SeqCst) {
        send_key(&handle, key, KeyState::PRESSED);
        pressed = true;
        std::thread::sleep(click_dur);
        if !running.load(Ordering::SeqCst) {
            break;
        }

        send_key(&handle, key, KeyState::RELEASED);
        pressed = false;
        std::thread::sleep(pause_dur);
    }

    if pressed {
        send_key(&handle, key, KeyState::RELEASED);
    }
    let _ = handle.dev_destroy();
    let _ = std::fs::remove_file(pid_file());
}
