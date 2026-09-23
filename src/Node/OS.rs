// Native `os` module. Values are read from libc directly; every function maps
// to the same source Node uses (uname, sysctl, /proc, getifaddrs, getpwuid).
use std::rc::Rc;

use Purs_Node_Errors_SystemError::{
    purust_system_error_from_io, purust_system_error_raise, purust_system_error_value,
};

fn string_value(text: &str) -> crate::UnknownType {
    crate::Value::String(purust_core::purust_string_from_utf8(text))
}

fn effect_value(build: impl Fn() -> crate::UnknownType + 'static) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |_| build())))
}

fn effect_string(text: String) -> crate::UnknownType {
    effect_value(move || crate::Value::String(text.clone()))
}

fn effect_int(value: i64) -> crate::UnknownType {
    effect_value(move || crate::mk_int(value))
}

fn effect_number(value: f64) -> crate::UnknownType {
    effect_value(move || crate::mk_number(value))
}

fn nullable(value: Option<crate::UnknownType>) -> crate::UnknownType {
    let nullable = match value {
        Some(value) => Purs_Data_Nullable::Data_Nullable_notNull(value),
        None => Purs_Data_Nullable::Data_Nullable_null(),
    };
    crate::Value::Class(Rc::new(nullable))
}

fn record(fields: Vec<(&str, crate::UnknownType)>) -> crate::UnknownType {
    let mut record = purust_core::RecordFields::new();
    for (key, value) in fields {
        record.insert(key.to_owned(), value);
    }
    crate::Value::DynamicRecord(perceus_ptr::PerceusPtr::new(record))
}

fn object(entries: Vec<(String, crate::UnknownType)>) -> Rc<Purs_Foreign_Object::Object> {
    Rc::new(Purs_Foreign_Object::Object::from_entries(entries))
}

fn option_string(value: Option<&[u8]>) -> Option<crate::UnknownType> {
    value.map(|value| crate::Value::String(purust_core::purust_string_from_utf8(
        &String::from_utf8_lossy(value),
    )))
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub fn Node_OS_eol() -> String {
    "\n".to_owned()
}

pub fn Node_OS_devNull() -> String {
    "/dev/null".to_owned()
}

pub fn Node_OS_constants() -> Rc<Purs_Foreign_Object::Object> {
    let signals = object(vec![
        ("SIGHUP".to_owned(), crate::mk_int(libc::SIGHUP as i64)),
        ("SIGINT".to_owned(), crate::mk_int(libc::SIGINT as i64)),
        ("SIGQUIT".to_owned(), crate::mk_int(libc::SIGQUIT as i64)),
        ("SIGILL".to_owned(), crate::mk_int(libc::SIGILL as i64)),
        ("SIGTRAP".to_owned(), crate::mk_int(libc::SIGTRAP as i64)),
        ("SIGABRT".to_owned(), crate::mk_int(libc::SIGABRT as i64)),
        ("SIGBUS".to_owned(), crate::mk_int(libc::SIGBUS as i64)),
        ("SIGFPE".to_owned(), crate::mk_int(libc::SIGFPE as i64)),
        ("SIGKILL".to_owned(), crate::mk_int(libc::SIGKILL as i64)),
        ("SIGUSR1".to_owned(), crate::mk_int(libc::SIGUSR1 as i64)),
        ("SIGSEGV".to_owned(), crate::mk_int(libc::SIGSEGV as i64)),
        ("SIGUSR2".to_owned(), crate::mk_int(libc::SIGUSR2 as i64)),
        ("SIGPIPE".to_owned(), crate::mk_int(libc::SIGPIPE as i64)),
        ("SIGALRM".to_owned(), crate::mk_int(libc::SIGALRM as i64)),
        ("SIGTERM".to_owned(), crate::mk_int(libc::SIGTERM as i64)),
        ("SIGCHLD".to_owned(), crate::mk_int(libc::SIGCHLD as i64)),
        ("SIGCONT".to_owned(), crate::mk_int(libc::SIGCONT as i64)),
        ("SIGSTOP".to_owned(), crate::mk_int(libc::SIGSTOP as i64)),
        ("SIGTSTP".to_owned(), crate::mk_int(libc::SIGTSTP as i64)),
        ("SIGTTIN".to_owned(), crate::mk_int(libc::SIGTTIN as i64)),
        ("SIGTTOU".to_owned(), crate::mk_int(libc::SIGTTOU as i64)),
        ("SIGURG".to_owned(), crate::mk_int(libc::SIGURG as i64)),
        ("SIGXCPU".to_owned(), crate::mk_int(libc::SIGXCPU as i64)),
        ("SIGXFSZ".to_owned(), crate::mk_int(libc::SIGXFSZ as i64)),
        ("SIGVTALRM".to_owned(), crate::mk_int(libc::SIGVTALRM as i64)),
        ("SIGPROF".to_owned(), crate::mk_int(libc::SIGPROF as i64)),
        ("SIGWINCH".to_owned(), crate::mk_int(libc::SIGWINCH as i64)),
        ("SIGIO".to_owned(), crate::mk_int(libc::SIGIO as i64)),
        ("SIGSYS".to_owned(), crate::mk_int(libc::SIGSYS as i64)),
    ]);
    let errno = object(vec![
        ("EPERM".to_owned(), crate::mk_int(libc::EPERM as i64)),
        ("ENOENT".to_owned(), crate::mk_int(libc::ENOENT as i64)),
        ("EINTR".to_owned(), crate::mk_int(libc::EINTR as i64)),
        ("EIO".to_owned(), crate::mk_int(libc::EIO as i64)),
        ("EBADF".to_owned(), crate::mk_int(libc::EBADF as i64)),
        ("EAGAIN".to_owned(), crate::mk_int(libc::EAGAIN as i64)),
        ("ENOMEM".to_owned(), crate::mk_int(libc::ENOMEM as i64)),
        ("EACCES".to_owned(), crate::mk_int(libc::EACCES as i64)),
        ("EFAULT".to_owned(), crate::mk_int(libc::EFAULT as i64)),
        ("EBUSY".to_owned(), crate::mk_int(libc::EBUSY as i64)),
        ("EEXIST".to_owned(), crate::mk_int(libc::EEXIST as i64)),
        ("ENOTDIR".to_owned(), crate::mk_int(libc::ENOTDIR as i64)),
        ("EISDIR".to_owned(), crate::mk_int(libc::EISDIR as i64)),
        ("EINVAL".to_owned(), crate::mk_int(libc::EINVAL as i64)),
        ("ENFILE".to_owned(), crate::mk_int(libc::ENFILE as i64)),
        ("EMFILE".to_owned(), crate::mk_int(libc::EMFILE as i64)),
        ("ENOSPC".to_owned(), crate::mk_int(libc::ENOSPC as i64)),
        ("EPIPE".to_owned(), crate::mk_int(libc::EPIPE as i64)),
        ("ENOSYS".to_owned(), crate::mk_int(libc::ENOSYS as i64)),
        ("ENOTEMPTY".to_owned(), crate::mk_int(libc::ENOTEMPTY as i64)),
        ("ECONNRESET".to_owned(), crate::mk_int(libc::ECONNRESET as i64)),
        ("ECONNREFUSED".to_owned(), crate::mk_int(libc::ECONNREFUSED as i64)),
        ("ETIMEDOUT".to_owned(), crate::mk_int(libc::ETIMEDOUT as i64)),
        ("EADDRINUSE".to_owned(), crate::mk_int(libc::EADDRINUSE as i64)),
        ("EADDRNOTAVAIL".to_owned(), crate::mk_int(libc::EADDRNOTAVAIL as i64)),
    ]);
    let dlopen = object(vec![
        ("RTLD_LAZY".to_owned(), crate::mk_int(0x1)),
        ("RTLD_NOW".to_owned(), crate::mk_int(0x2)),
        #[cfg(target_os = "macos")]
        ("RTLD_LOCAL".to_owned(), crate::mk_int(0x4)),
        #[cfg(target_os = "macos")]
        ("RTLD_GLOBAL".to_owned(), crate::mk_int(0x8)),
        #[cfg(target_os = "linux")]
        ("RTLD_GLOBAL".to_owned(), crate::mk_int(0x100)),
        #[cfg(target_os = "linux")]
        ("RTLD_LOCAL".to_owned(), crate::mk_int(0)),
    ]);
    let priority = object(vec![
        ("PRIORITY_LOW".to_owned(), crate::mk_int(19)),
        ("PRIORITY_BELOW_NORMAL".to_owned(), crate::mk_int(10)),
        ("PRIORITY_NORMAL".to_owned(), crate::mk_int(0)),
        ("PRIORITY_ABOVE_NORMAL".to_owned(), crate::mk_int(-7)),
        ("PRIORITY_HIGH".to_owned(), crate::mk_int(-14)),
        ("PRIORITY_HIGHEST".to_owned(), crate::mk_int(-20)),
    ]);
    object(vec![
        ("signals".to_owned(), crate::Value::Class(Rc::new(signals))),
        ("errno".to_owned(), crate::Value::Class(Rc::new(errno))),
        ("dlopen".to_owned(), crate::Value::Class(Rc::new(dlopen))),
        ("priority".to_owned(), crate::Value::Class(Rc::new(priority))),
        ("UV_UDP_REUSEADDR".to_owned(), crate::mk_int(4)),
    ])
}

// ---------------------------------------------------------------------------
// Machine information
// ---------------------------------------------------------------------------

pub fn Node_OS_archImpl() -> crate::UnknownType {
    let arch = match std::env::consts::ARCH {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        "x86" => "ia32",
        "arm" => "arm",
        "powerpc" => "ppc",
        "powerpc64" => "ppc64",
        "s390x" => "s390x",
        "mips" => "mips",
        "mips64" => "mips64",
        "riscv64" => "riscv64",
        other => other,
    };
    effect_string(arch.to_owned())
}

pub fn Node_OS_endiannessImpl() -> crate::UnknownType {
    effect_string(if cfg!(target_endian = "little") { "LE" } else { "BE" }.to_owned())
}

fn uname() -> libc::utsname {
    let mut info: libc::utsname = unsafe { std::mem::zeroed() };
    unsafe {
        libc::uname(&mut info);
    }
    info
}

fn c_field(field: &[libc::c_char]) -> String {
    let bytes: Vec<u8> = field
        .iter()
        .take_while(|c| **c != 0)
        .map(|c| *c as u8)
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

pub fn Node_OS_hostname() -> crate::UnknownType {
    let mut buffer = vec![0u8; 256];
    let result = unsafe {
        libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, buffer.len())
    };
    if result != 0 {
        // Fall back to the uname node name.
        return effect_string(c_field(&uname().nodename));
    }
    let length = buffer.iter().position(|c| *c == 0).unwrap_or(buffer.len());
    effect_string(String::from_utf8_lossy(&buffer[..length]).into_owned())
}

pub fn Node_OS_machine() -> crate::UnknownType {
    let info = uname();
    effect_string(c_field(&info.machine))
}

pub fn Node_OS_release() -> crate::UnknownType {
    let info = uname();
    effect_string(c_field(&info.release))
}

pub fn Node_OS_type_() -> crate::UnknownType {
    let info = uname();
    effect_string(c_field(&info.sysname))
}

pub fn Node_OS_version() -> crate::UnknownType {
    let info = uname();
    effect_string(c_field(&info.version))
}

pub fn Node_OS_homedir() -> crate::UnknownType {
    match std::env::var_os("HOME") {
        Some(home) => effect_string(home.to_string_lossy().into_owned()),
        None => effect_string(String::new()),
    }
}

pub fn Node_OS_tmpdir() -> crate::UnknownType {
    let mut path = std::env::temp_dir().to_string_lossy().into_owned();
    while path.len() > 1 && path.ends_with('/') {
        path.pop();
    }
    effect_string(path)
}

pub fn Node_OS_uptime() -> crate::UnknownType {
    #[cfg(target_os = "macos")]
    {
        let mut boot = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut size = std::mem::size_of::<libc::timeval>();
        let name = b"kern.boottime\0";
        let result = unsafe {
            libc::sysctl(
                name.as_ptr() as *mut libc::c_int,
                0,
                std::ptr::null_mut(),
                &mut size,
                &mut boot as *mut libc::timeval as *mut libc::c_void,
                std::mem::size_of::<libc::timeval>(),
            )
        };
        // `sysctlbyname` is the portable spelling.
        let result = if result == 0 { result } else {
            unsafe {
                libc::sysctlbyname(
                    name.as_ptr() as *const libc::c_char,
                    &mut boot as *mut libc::timeval as *mut libc::c_void,
                    &mut size,
                    std::ptr::null_mut(),
                    0,
                )
            }
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|elapsed| elapsed.as_secs_f64())
            .unwrap_or(0.0);
        if result == 0 {
            let uptime = now - boot.tv_sec as f64 - boot.tv_usec as f64 / 1_000_000.0;
            return effect_number(uptime.max(0.0));
        }
        return effect_number(0.0);
    }
    #[cfg(target_os = "linux")]
    {
        let mut info: libc::sysinfo = unsafe { std::mem::zeroed() };
        if unsafe { libc::sysinfo(&mut info) } == 0 {
            return effect_number(info.uptime as f64);
        }
        effect_number(0.0)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        effect_number(0.0)
    }
}

pub fn Node_OS_loadavgImpl() -> crate::UnknownType {
    effect_value(|| {
        let mut averages = [0.0f64; 3];
        let count = unsafe { libc::getloadavg(averages.as_mut_ptr(), 3) };
        if count < 0 {
            return crate::mk_array(vec![crate::mk_number(0.0); 3]);
        }
        crate::mk_array(vec![
            crate::mk_number(averages[0]),
            crate::mk_number(averages[1]),
            crate::mk_number(averages[2]),
        ])
    })
}

// ---------------------------------------------------------------------------
// Memory
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
fn sysctl_u64(name: &[u8]) -> Option<u64> {
    let mut value: u64 = 0;
    let mut size = std::mem::size_of::<u64>();
    let name = name.to_vec();
    let result = unsafe {
        libc::sysctlbyname(
            name.as_ptr() as *const libc::c_char,
            &mut value as *mut u64 as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if result == 0 {
        Some(value)
    } else {
        None
    }
}

#[cfg(target_os = "macos")]
fn page_size() -> u64 {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as u64 }
}

#[cfg(target_os = "macos")]
fn free_memory_bytes() -> u64 {
    let mut stats: libc::vm_statistics64 = unsafe { std::mem::zeroed() };
    let mut count = libc::HOST_VM_INFO64_COUNT;
    let result = unsafe {
        libc::host_statistics64(
            libc::mach_host_self(),
            libc::HOST_VM_INFO64,
            &mut stats as *mut libc::vm_statistics64 as libc::host_info64_t,
            &mut count,
        )
    };
    if result == 0 {
        let pagesize = page_size();
        (stats.free_count as u64 + stats.speculative_count as u64) * pagesize
    } else {
        0
    }
}

pub fn Node_OS_totalmem() -> crate::UnknownType {
    #[cfg(target_os = "macos")]
    {
        effect_int(sysctl_u64(b"hw.memsize\0").unwrap_or(0) as i64)
    }
    #[cfg(target_os = "linux")]
    {
        let mut info: libc::sysinfo = unsafe { std::mem::zeroed() };
        let total = if unsafe { libc::sysinfo(&mut info) } == 0 {
            info.totalram as u64 * info.mem_unit as u64
        } else {
            0
        };
        effect_int(total as i64)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        effect_int(0)
    }
}

pub fn Node_OS_freemem() -> crate::UnknownType {
    #[cfg(target_os = "macos")]
    {
        effect_int(free_memory_bytes() as i64)
    }
    #[cfg(target_os = "linux")]
    {
        let mut info: libc::sysinfo = unsafe { std::mem::zeroed() };
        let free = if unsafe { libc::sysinfo(&mut info) } == 0 {
            info.freeram as u64 * info.mem_unit as u64
        } else {
            0
        };
        effect_int(free as i64)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        effect_int(0)
    }
}

// ---------------------------------------------------------------------------
// Priorities
// ---------------------------------------------------------------------------

fn float_argument(value: &crate::UnknownType) -> Option<i64> {
    match value.resolve() {
        crate::Value::Unit => None,
        crate::Value::Int(number) => Some(*number),
        crate::Value::Number(number) => Some(*number as i64),
        crate::Value::Class(payload) => payload.downcast_ref::<i64>().copied(),
        _ => None,
    }
}

pub fn Node_OS_getPriorityImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        let pid = float_argument(&value).unwrap_or(0);
        let priority = unsafe { libc::getpriority(libc::PRIO_PROCESS, pid as libc::id_t) };
        crate::mk_int(priority as i64)
    })))
}

pub fn Node_OS_setPriorityImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|value| {
        if let Some(priority) = float_argument(&value) {
            // Failing is not fatal here: lowering a priority always succeeds on
            // unix, and raising it needs privileges Node also cannot grant.
            unsafe {
                libc::setpriority(libc::PRIO_PROCESS, 0, priority as libc::c_int);
            }
        }
        crate::Value::Unit
    })))
}

// ---------------------------------------------------------------------------
// CPUs
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
fn cpu_times() -> Vec<(f64, f64, f64, f64, f64)> {
    // (user, nice, sys, idle, irq) in milliseconds, per logical core.
    let mut count: libc::natural_t = 0;
    let mut info: libc::processor_info_array_t = std::ptr::null_mut();
    let mut info_count: libc::mach_msg_type_number_t = 0;
    let result = unsafe {
        libc::host_processor_info(
            libc::mach_host_self(),
            libc::PROCESSOR_CPU_LOAD_INFO,
            &mut count,
            &mut info,
            &mut info_count,
        )
    };
    if result != 0 || info.is_null() {
        return Vec::new();
    }
    let ticks_per_second = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as f64;
    let scale = if ticks_per_second > 0.0 {
        1000.0 / ticks_per_second
    } else {
        10.0
    };
    let values = unsafe { std::slice::from_raw_parts(info, count as usize * 4) };
    let mut times = Vec::with_capacity(count as usize);
    for index in 0..count as usize {
        let user = values[index * 4] as f64 * scale;
        let system = values[index * 4 + 1] as f64 * scale;
        let idle = values[index * 4 + 2] as f64 * scale;
        let nice = values[index * 4 + 3] as f64 * scale;
        times.push((user, nice, system, idle, 0.0));
    }
    let mut deallocate_count: libc::mach_msg_type_number_t = info_count;
    unsafe {
        libc::vm_deallocate(
            libc::mach_task_self(),
            info as libc::vm_address_t,
            (deallocate_count as usize * std::mem::size_of::<libc::integer_t>()) as libc::vm_size_t,
        );
        deallocate_count = 0;
    }
    let _ = deallocate_count;
    times
}

#[cfg(target_os = "macos")]
fn cpu_model() -> String {
    sysctl_string(b"machdep.cpu.brand_string\0")
        .or_else(|| sysctl_string(b"hw.model\0"))
        .unwrap_or_default()
}

#[cfg(target_os = "macos")]
fn cpu_speed_mhz() -> i64 {
    sysctl_u64(b"hw.cpufrequency\0")
        .map(|hz| (hz / 1_000_000) as i64)
        .unwrap_or(0)
}

#[cfg(target_os = "linux")]
fn cpu_model() -> String {
    std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|text| {
            text.lines()
                .find(|line| line.starts_with("model name"))
                .and_then(|line| line.split(':').nth(1))
                .map(|value| value.trim().to_owned())
        })
        .unwrap_or_default()
}

#[cfg(target_os = "linux")]
fn cpu_speed_mhz() -> i64 {
    std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|text| {
            text.lines()
                .find(|line| line.starts_with("cpu MHz"))
                .and_then(|line| line.split(':').nth(1))
                .and_then(|value| value.trim().parse::<f64>().ok())
                .map(|value| value as i64)
        })
        .unwrap_or(0)
}

#[cfg(target_os = "linux")]
fn cpu_times() -> Vec<(f64, f64, f64, f64, f64)> {
    let ticks_per_second = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as f64;
    let scale = if ticks_per_second > 0.0 {
        1000.0 / ticks_per_second
    } else {
        10.0
    };
    let mut times = Vec::new();
    if let Ok(text) = std::fs::read_to_string("/proc/stat") {
        for line in text.lines() {
            if !line.starts_with("cpu") || line.starts_with("cpu ") {
                continue;
            }
            let fields: Vec<f64> = line
                .split_whitespace()
                .skip(1)
                .filter_map(|value| value.parse().ok())
                .collect();
            if fields.len() >= 5 {
                let user = fields[0] * scale;
                let nice = fields[1] * scale;
                let system = fields[2] * scale;
                let idle = fields[3] * scale;
                let irq = (fields[4] + fields.get(5).copied().unwrap_or(0.0)) * scale;
                times.push((user, nice, system, idle, irq));
            }
        }
    }
    times
}

#[cfg(target_os = "macos")]
fn sysctl_string(name: &[u8]) -> Option<String> {
    let mut size: usize = 0;
    let name = name.to_vec();
    let result = unsafe {
        libc::sysctlbyname(
            name.as_ptr() as *const libc::c_char,
            std::ptr::null_mut(),
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if result != 0 || size == 0 {
        return None;
    }
    let mut buffer = vec![0u8; size];
    let result = unsafe {
        libc::sysctlbyname(
            name.as_ptr() as *const libc::c_char,
            buffer.as_mut_ptr() as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if result != 0 {
        return None;
    }
    let length = buffer.iter().position(|c| *c == 0).unwrap_or(buffer.len());
    Some(String::from_utf8_lossy(&buffer[..length]).into_owned())
}

pub fn Node_OS_cpus() -> crate::UnknownType {
    effect_value(|| {
        let times = cpu_times();
        let model = cpu_model();
        let speed = cpu_speed_mhz();
        let entries: Vec<crate::UnknownType> = times
            .iter()
            .map(|(user, nice, system, idle, irq)| {
                record(vec![
                    ("model", string_value(&model)),
                    ("speed", crate::mk_int(speed)),
                    (
                        "times",
                        record(vec![
                            ("user", crate::mk_number(*user)),
                            ("nice", crate::mk_number(*nice)),
                            ("sys", crate::mk_number(*system)),
                            ("idle", crate::mk_number(*idle)),
                            ("irq", crate::mk_number(*irq)),
                        ]),
                    ),
                ])
            })
            .collect();
        crate::mk_array(entries)
    })
}

// ---------------------------------------------------------------------------
// Network interfaces
// ---------------------------------------------------------------------------

fn sockaddr_family(address: *const libc::sockaddr) -> i32 {
    unsafe { (*address).sa_family as i32 }
}

fn numeric_address(address: *const libc::sockaddr, length: libc::socklen_t) -> Option<String> {
    let mut host = vec![0i8; libc::NI_MAXHOST as usize];
    let result = unsafe {
        libc::getnameinfo(
            address,
            length,
            host.as_mut_ptr(),
            libc::NI_MAXHOST,
            std::ptr::null_mut(),
            0,
            libc::NI_NUMERICHOST,
        )
    };
    if result != 0 {
        return None;
    }
    let length = host.iter().position(|c| *c == 0).unwrap_or(host.len());
    let bytes: Vec<u8> = host[..length].iter().map(|c| *c as u8).collect();
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

fn ipv4_netmask(address: *const libc::sockaddr) -> Option<[u8; 4]> {
    let address = address as *const libc::sockaddr_in;
    unsafe { Some((*address).sin_addr.s_addr.to_ne_bytes()) }
}

fn ipv6_netmask(address: *const libc::sockaddr) -> Option<[u8; 16]> {
    let address = address as *const libc::sockaddr_in6;
    unsafe { Some((*address).sin6_addr.s6_addr) }
}

fn prefix_length(bytes: &[u8]) -> u32 {
    let mut count = 0;
    for byte in bytes {
        if *byte == 0xff {
            count += 8;
        } else {
            count += byte.leading_ones();
            break;
        }
    }
    count
}

/// RFC 5952 style formatting: lowercase hex, longest zero run compressed.
fn format_ipv6(bytes: &[u8]) -> String {
    let mut segments: Vec<u16> = Vec::with_capacity(8);
    for pair in bytes.chunks(2) {
        if pair.len() == 2 {
            segments.push(u16::from_be_bytes([pair[0], pair[1]]));
        }
    }
    let mut best_start = segments.len();
    let mut best_length = 0;
    let mut index = 0;
    while index < segments.len() {
        if segments[index] == 0 {
            let start = index;
            while index < segments.len() && segments[index] == 0 {
                index += 1;
            }
            let length = index - start;
            if length > best_length && length >= 2 {
                best_start = start;
                best_length = length;
            }
        } else {
            index += 1;
        }
    }
    let mut text = String::new();
    let mut position = 0;
    while position < segments.len() {
        if position == best_start {
            text.push_str(if position == 0 { "::" } else { ":" });
            position += best_length;
            continue;
        }
        if !text.is_empty() && !text.ends_with(':') {
            text.push(':');
        }
        text.push_str(&format!("{:x}", segments[position]));
        position += 1;
    }
    if text.is_empty() {
        text.push_str("::");
    }
    text
}

fn format_mac(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join(":")
}

#[cfg(target_os = "macos")]
fn interface_mac(name: &str, entries: &[(*const libc::ifaddrs, String)]) -> Option<String> {
    for (entry, entry_name) in entries {
        if entry_name != name {
            continue;
        }
        let address = unsafe { (**entry).ifa_addr };
        if address.is_null() || sockaddr_family(address) != libc::AF_LINK {
            continue;
        }
        let link = address as *const libc::sockaddr_dl;
        let data = unsafe { (*link).sdl_data.as_ptr() };
        let name_length = unsafe { (*link).sdl_nlen } as usize;
        let address_length = unsafe { (*link).sdl_alen } as usize;
        if address_length == 6 {
            let bytes = unsafe { std::slice::from_raw_parts(data.add(name_length) as *const u8, 6) };
            return Some(format_mac(bytes));
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn interface_mac(name: &str, entries: &[(*const libc::ifaddrs, String)]) -> Option<String> {
    for (entry, entry_name) in entries {
        if entry_name != name {
            continue;
        }
        let address = unsafe { (**entry).ifa_addr };
        if address.is_null() || sockaddr_family(address) != libc::AF_PACKET {
            continue;
        }
        let link = address as *const libc::sockaddr_ll;
        let length = unsafe { (*link).sll_halen } as usize;
        if length == 6 {
            let bytes = unsafe { std::slice::from_raw_parts((*link).sll_addr.as_ptr(), 6) };
            return Some(format_mac(bytes));
        }
    }
    None
}

pub fn Node_OS_networkInterfacesImpl() -> crate::UnknownType {
    effect_value(|| {
        let mut head: *mut libc::ifaddrs = std::ptr::null_mut();
        if unsafe { libc::getifaddrs(&mut head) } != 0 {
            return crate::Value::Class(Rc::new(object(Vec::new())));
        }
        let mut entries: Vec<(*const libc::ifaddrs, String)> = Vec::new();
        let mut current = head;
        while !current.is_null() {
            let name = unsafe { std::ffi::CStr::from_ptr((*current).ifa_name) }
                .to_string_lossy()
                .into_owned();
            entries.push((current as *const libc::ifaddrs, name));
            current = unsafe { (*current).ifa_next };
        }

        let mut by_name: Vec<(String, Vec<crate::UnknownType>)> = Vec::new();
        for (entry, name) in entries.iter() {
            let address = unsafe { (**entry).ifa_addr };
            if address.is_null() {
                continue;
            }
            let family = sockaddr_family(address);
            if family != libc::AF_INET && family != libc::AF_INET6 {
                continue;
            }
            let address_length = if family == libc::AF_INET {
                std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t
            } else {
                std::mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t
            };
            let Some(address_text) = numeric_address(address, address_length) else {
                continue;
            };
            let netmask = unsafe { (**entry).ifa_netmask };
            let (netmask_text, cidr) = match family {
                libc::AF_INET => match ipv4_netmask(netmask) {
                    Some(mask) => {
                        let text = mask
                            .iter()
                            .map(|byte| byte.to_string())
                            .collect::<Vec<_>>()
                            .join(".");
                        (text, Some(format!("{address_text}/{}", prefix_length(&mask))))
                    }
                    None => (String::new(), None),
                },
                _ => match ipv6_netmask(netmask) {
                    Some(mask) => {
                        let text = format_ipv6(&mask);
                        (text, Some(format!("{address_text}/{}", prefix_length(&mask))))
                    }
                    None => (String::new(), None),
                },
            };
            let flags = unsafe { (**entry).ifa_flags } as i32;
            let internal = flags & libc::IFF_LOOPBACK != 0;
            let scope_id = if family == libc::AF_INET6 {
                let address = address as *const libc::sockaddr_in6;
                let scope = unsafe { (*address).sin6_scope_id };
                if scope == 0 {
                    None
                } else {
                    Some(crate::mk_int(scope as i64))
                }
            } else {
                None
            };
            let mac = interface_mac(name, &entries)
                .unwrap_or_else(|| "00:00:00:00:00:00".to_owned());
            let interface = record(vec![
                ("address", string_value(&address_text)),
                ("netmask", string_value(&netmask_text)),
                (
                    "family",
                    string_value(if family == libc::AF_INET { "IPv4" } else { "IPv6" }),
                ),
                ("mac", string_value(&mac)),
                ("internal", crate::mk_bool(internal)),
                ("scopeId", nullable(scope_id)),
                ("cidr", nullable(cidr.map(|text| string_value(&text)))),
            ]);
            match by_name.iter_mut().find(|(existing, _)| existing == name) {
                Some((_, interfaces)) => interfaces.push(interface),
                None => by_name.push((name.clone(), vec![interface])),
            }
        }
        unsafe { libc::freeifaddrs(head) };
        let entries = by_name
            .into_iter()
            .map(|(name, interfaces)| (name, crate::mk_array(interfaces)))
            .collect();
        crate::Value::Class(Rc::new(object(entries)))
    })
}

// ---------------------------------------------------------------------------
// User information
// ---------------------------------------------------------------------------

struct PasswdEntry {
    uid: u32,
    gid: u32,
    username: Vec<u8>,
    homedir: Vec<u8>,
    shell: Option<Vec<u8>>,
}

fn passwd_entry() -> std::io::Result<PasswdEntry> {
    let uid = unsafe { libc::geteuid() };
    let mut pwd: libc::passwd = unsafe { std::mem::zeroed() };
    let mut result: *mut libc::passwd = std::ptr::null_mut();
    let mut buffer = vec![0u8; 4096];
    let status = unsafe {
        libc::getpwuid_r(
            uid,
            &mut pwd,
            buffer.as_mut_ptr() as *mut libc::c_char,
            buffer.len(),
            &mut result,
        )
    };
    if status != 0 {
        return Err(std::io::Error::from_raw_os_error(status));
    }
    if result.is_null() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no passwd entry",
        ));
    }
    let read = |pointer: *const libc::c_char| -> Vec<u8> {
        if pointer.is_null() {
            Vec::new()
        } else {
            unsafe { std::ffi::CStr::from_ptr(pointer) }
                .to_bytes()
                .to_vec()
        }
    };
    let shell = read(pwd.pw_shell);
    Ok(PasswdEntry {
        uid: pwd.pw_uid,
        gid: pwd.pw_gid,
        username: read(pwd.pw_name),
        homedir: read(pwd.pw_dir),
        shell: if shell.is_empty() { None } else { Some(shell) },
    })
}

pub fn Node_OS_userInfoImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|options| {
        let encoding = options
            .__purust_foreign_object()
            .get("encoding")
            .map(|value| value.unwrap_string())
            .unwrap_or_else(|| "utf8".to_owned());
        let entry = passwd_entry().unwrap_or_else(|error| {
            purust_system_error_raise(purust_system_error_from_io(
                &error,
                "uv_os_get_passwd",
                None,
            ))
        });
        let use_buffer = encoding == "buffer";
        let encode = |bytes: &[u8]| -> crate::UnknownType {
            if use_buffer {
                crate::Value::Class(Rc::new(Purs_Node_Buffer_Immutable::purust_buffer_from_bytes(
                    bytes.to_vec(),
                )))
            } else {
                let name = Purs_Node_Encoding::purust_encoding_from_name(&encoding);
                crate::Value::String(Purs_Node_Encoding::purust_encoding_decode(name, bytes))
            }
        };
        let username = encode(&entry.username);
        let homedir = encode(&entry.homedir);
        let shell = match &entry.shell {
            Some(shell) => nullable(Some(encode(shell))),
            None => nullable(None),
        };
        record(vec![
            ("uid", crate::mk_int(entry.uid as i64)),
            ("gid", crate::mk_int(entry.gid as i64)),
            ("username", username),
            ("homedir", homedir),
            ("shell", shell),
        ])
    })))
}

// Keep the SystemError helpers linked for the child-process package, which
// shares the same error representation.
pub fn purust_os_system_error_value(fields: Purs_Node_Errors_SystemError::SystemErrorFields) -> crate::UnknownType {
    purust_system_error_value(fields)
}

pub fn purust_os_option_string(value: Option<&[u8]>) -> Option<crate::UnknownType> {
    option_string(value)
}
