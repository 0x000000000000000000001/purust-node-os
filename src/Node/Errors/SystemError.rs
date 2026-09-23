// Node's `SystemError` values. They are real `Effect.Exception` errors (so they
// can be thrown, caught and coerced like Node's Error subclasses), and the Node
// specific fields (code, errno, syscall, ...) travel in a sentinel cause.
use std::rc::Rc;
use std::sync::Arc;

use Purs_Effect_Exception::PurustExceptionError;

/// Native shape of a SystemError handle. The generated code boxes foreign
/// values as `Rc<SystemError>`, so the alias keeps both sides in sync.
pub type SystemError = PurustExceptionError;

const SENTINEL_NAME: &str = "__purust_node_system_error__";

#[derive(Clone, Default)]
pub struct SystemErrorFields {
    pub code: String,
    pub errno: i64,
    pub syscall: String,
    pub message: String,
    pub path: Option<String>,
    pub address: Option<String>,
    pub dest: Option<String>,
    pub port: Option<i64>,
}

fn push_field(out: &mut String, key: &str, value: &str) {
    out.push_str(key);
    out.push('\t');
    out.push_str(&value.replace('\\', "\\\\").replace('\n', "\\n").replace('\t', "\\t"));
    out.push('\n');
}

fn encode(fields: &SystemErrorFields) -> String {
    let mut out = String::new();
    push_field(&mut out, "code", &fields.code);
    push_field(&mut out, "errno", &fields.errno.to_string());
    push_field(&mut out, "syscall", &fields.syscall);
    push_field(&mut out, "message", &fields.message);
    if let Some(path) = &fields.path {
        push_field(&mut out, "path", path);
    }
    if let Some(address) = &fields.address {
        push_field(&mut out, "address", address);
    }
    if let Some(dest) = &fields.dest {
        push_field(&mut out, "dest", dest);
    }
    if let Some(port) = fields.port {
        push_field(&mut out, "port", &port.to_string());
    }
    out
}

fn unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('\\') => out.push('\\'),
                Some(other) => out.push(other),
                None => {}
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn decode(text: &str) -> SystemErrorFields {
    let mut fields = SystemErrorFields::default();
    for line in text.lines() {
        let mut parts = line.splitn(2, '\t');
        let key = parts.next().unwrap_or("");
        let value = unescape(parts.next().unwrap_or(""));
        match key {
            "code" => fields.code = value,
            "errno" => fields.errno = value.parse().unwrap_or(0),
            "syscall" => fields.syscall = value,
            "message" => fields.message = value,
            "path" => fields.path = Some(value),
            "address" => fields.address = Some(value),
            "dest" => fields.dest = Some(value),
            "port" => fields.port = value.parse().ok(),
            _ => {}
        }
    }
    fields
}

fn error_with_fields(fields: &SystemErrorFields) -> PurustExceptionError {
    let cause = PurustExceptionError {
        message: encode(fields),
        name: SENTINEL_NAME.to_owned(),
        cause: None,
    };
    PurustExceptionError {
        message: fields.message.clone(),
        name: "Error".to_owned(),
        cause: Some(Arc::new(cause)),
    }
}

/// A `SystemError` value as the generated code represents it: a class holding
/// the native handle.
pub fn purust_system_error_value(fields: SystemErrorFields) -> crate::UnknownType {
    crate::Value::Class(Rc::new(Rc::new(error_with_fields(&fields))))
}

/// The thrown form used by `Effect.Exception`: the class holds the error
/// payload directly, which is what `try`/`catchException` expect.
pub fn purust_system_error_error_value(fields: SystemErrorFields) -> crate::UnknownType {
    crate::Value::Class(Rc::new(Arc::new(error_with_fields(&fields))))
}

/// Reads the Node fields back out of a value produced by
/// `purust_system_error_value`. Values that are plain errors yield the message.
fn fields_of_error(error: &PurustExceptionError) -> SystemErrorFields {
    match &error.cause {
        Some(cause) if cause.name == SENTINEL_NAME => decode(&cause.message),
        _ => SystemErrorFields {
            code: "ERR_GENERIC".to_owned(),
            errno: 0,
            syscall: String::new(),
            message: error.message.clone(),
            ..Default::default()
        },
    }
}

/// Reads the Node fields from either shape a SystemError can take: the native
/// handle or the error payload produced by `try`.
pub fn purust_system_error_fields(value: &crate::UnknownType) -> SystemErrorFields {
    let resolved = value.resolve().clone();
    if let crate::Value::Class(payload) = &resolved {
        if let Some(handle) = payload.downcast_ref::<Rc<SystemError>>() {
            return fields_of_error(handle);
        }
        if let Some(error) = payload.downcast_ref::<Arc<PurustExceptionError>>() {
            return fields_of_error(error);
        }
    }
    panic!("Node.Errors.SystemError: expected a SystemError value");
}

pub fn purust_system_error_raise(fields: SystemErrorFields) -> ! {
    Purs_Effect_Exception::purust_exception_raise(purust_system_error_error_value(fields))
}

/// Maps a Rust IO error to Node's error fields.
pub fn purust_system_error_from_io(
    error: &std::io::Error,
    syscall: &str,
    path: Option<&str>,
) -> SystemErrorFields {
    let errno = error.raw_os_error().unwrap_or(0);
    let code = purust_errno_name(errno);
    let message = match path {
        Some(path) => format!("{code}: {}, {syscall} '{path}'", error),
        None => format!("{code}: {}, {syscall}", error),
    };
    SystemErrorFields {
        code: code.to_owned(),
        errno: errno as i64,
        syscall: syscall.to_owned(),
        message,
        path: path.map(|path| path.to_owned()),
        ..Default::default()
    }
}

/// libuv's errno names, matching Node's platform-dependent mapping.
#[cfg(target_os = "macos")]
pub fn purust_errno_name(errno: i32) -> &'static str {
    match errno {
        0 => "UV_UNKNOWN",
        1 => "EPERM",
        2 => "ENOENT",
        3 => "ESRCH",
        4 => "EINTR",
        5 => "EIO",
        6 => "ENXIO",
        7 => "E2BIG",
        8 => "ENOEXEC",
        9 => "EBADF",
        10 => "ECHILD",
        11 => "EDEADLK",
        12 => "ENOMEM",
        13 => "EACCES",
        14 => "EFAULT",
        15 => "ENOTBLK",
        16 => "EBUSY",
        17 => "EEXIST",
        18 => "EXDEV",
        19 => "ENODEV",
        20 => "ENOTDIR",
        21 => "EISDIR",
        22 => "EINVAL",
        23 => "ENFILE",
        24 => "EMFILE",
        25 => "ENOTTY",
        26 => "ETXTBSY",
        27 => "EFBIG",
        28 => "ENOSPC",
        29 => "ESPIPE",
        30 => "EROFS",
        31 => "EMLINK",
        32 => "EPIPE",
        33 => "EDOM",
        34 => "ERANGE",
        35 => "EAGAIN",
        36 => "EINPROGRESS",
        37 => "EALREADY",
        38 => "ENOTSOCK",
        39 => "EDESTADDRREQ",
        40 => "EMSGSIZE",
        41 => "EPROTOTYPE",
        42 => "ENOPROTOOPT",
        43 => "EPROTONOSUPPORT",
        44 => "ESOCKTNOSUPPORT",
        45 => "ENOTSUP",
        46 => "EPFNOSUPPORT",
        47 => "EAFNOSUPPORT",
        48 => "EADDRINUSE",
        49 => "EADDRNOTAVAIL",
        50 => "ENETDOWN",
        51 => "ENETUNREACH",
        52 => "ENETRESET",
        53 => "ECONNABORTED",
        54 => "ECONNRESET",
        55 => "ENOBUFS",
        56 => "EISCONN",
        57 => "ENOTCONN",
        58 => "ESHUTDOWN",
        59 => "ETOOMANYREFS",
        60 => "ETIMEDOUT",
        61 => "ECONNREFUSED",
        62 => "ELOOP",
        63 => "ENAMETOOLONG",
        64 => "EHOSTDOWN",
        65 => "EHOSTUNREACH",
        66 => "ENOTEMPTY",
        67 => "EPROCLIM",
        68 => "EUSERS",
        69 => "EDQUOT",
        70 => "ESTALE",
        71 => "EREMOTE",
        72 => "EBADRPC",
        73 => "ERPCMISMATCH",
        74 => "EPROGUNAVAIL",
        75 => "EPROGMISMATCH",
        76 => "EPROCUNAVAIL",
        77 => "ENOLCK",
        78 => "ENOSYS",
        79 => "EFTYPE",
        80 => "EAUTH",
        81 => "ENEEDAUTH",
        82 => "EPWROFF",
        83 => "EDEVERR",
        84 => "EOVERFLOW",
        85 => "EBADEXEC",
        86 => "EBADARCH",
        87 => "ESHLIBVERS",
        88 => "EBADMACHO",
        89 => "ECANCELED",
        90 => "EIDRM",
        91 => "ENOMSG",
        92 => "EILSEQ",
        93 => "ENOATTR",
        94 => "EBADMSG",
        95 => "EMULTIHOP",
        96 => "ENODATA",
        97 => "ENOLINK",
        98 => "ENOSR",
        99 => "ENOSTR",
        100 => "EPROTO",
        101 => "ETIME",
        102 => "EOPNOTSUPP",
        _ => "UV_UNKNOWN",
    }
}

#[cfg(target_os = "linux")]
pub fn purust_errno_name(errno: i32) -> &'static str {
    match errno {
        1 => "EPERM",
        2 => "ENOENT",
        3 => "ESRCH",
        4 => "EINTR",
        5 => "EIO",
        6 => "ENXIO",
        7 => "E2BIG",
        8 => "ENOEXEC",
        9 => "EBADF",
        10 => "ECHILD",
        11 => "EAGAIN",
        12 => "ENOMEM",
        13 => "EACCES",
        14 => "EFAULT",
        15 => "ENOTBLK",
        16 => "EBUSY",
        17 => "EEXIST",
        18 => "EXDEV",
        19 => "ENODEV",
        20 => "ENOTDIR",
        21 => "EISDIR",
        22 => "EINVAL",
        23 => "ENFILE",
        24 => "EMFILE",
        25 => "ENOTTY",
        26 => "ETXTBSY",
        27 => "EFBIG",
        28 => "ENOSPC",
        29 => "ESPIPE",
        30 => "EROFS",
        31 => "EMLINK",
        32 => "EPIPE",
        33 => "EDOM",
        34 => "ERANGE",
        35 => "EDEADLK",
        36 => "ENAMETOOLONG",
        37 => "ENOLCK",
        38 => "ENOSYS",
        39 => "ENOTEMPTY",
        40 => "ELOOP",
        42 => "ENOMSG",
        43 => "EIDRM",
        60 => "ENOSTR",
        61 => "ENODATA",
        62 => "ETIME",
        63 => "ENOSR",
        71 => "EPROTO",
        74 => "EBADMSG",
        75 => "EOVERFLOW",
        88 => "ENOTSOCK",
        89 => "EDESTADDRREQ",
        90 => "EMSGSIZE",
        91 => "EPROTOTYPE",
        92 => "ENOPROTOOPT",
        93 => "EPROTONOSUPPORT",
        94 => "ESOCKTNOSUPPORT",
        95 => "EOPNOTSUPP",
        96 => "EPFNOSUPPORT",
        97 => "EAFNOSUPPORT",
        98 => "EADDRINUSE",
        99 => "EADDRNOTAVAIL",
        100 => "ENETDOWN",
        101 => "ENETUNREACH",
        102 => "ENETRESET",
        103 => "ECONNABORTED",
        104 => "ECONNRESET",
        105 => "ENOBUFS",
        106 => "EISCONN",
        107 => "ENOTCONN",
        108 => "ESHUTDOWN",
        109 => "ETOOMANYREFS",
        110 => "ETIMEDOUT",
        111 => "ECONNREFUSED",
        112 => "EHOSTDOWN",
        113 => "EHOSTUNREACH",
        114 => "EALREADY",
        115 => "EINPROGRESS",
        116 => "ESTALE",
        122 => "EDQUOT",
        125 => "ECANCELED",
        _ => "UV_UNKNOWN",
    }
}

fn unbox_error(value: &crate::UnknownType) -> SystemErrorFields {
    purust_system_error_fields(value)
}

pub fn Node_Errors_SystemError_getField() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|field, error| {
        let field = purust_core::purust_string_to_utf8_lossy(&field.unwrap_string());
        let fields = unbox_error(&error);
        match field.as_str() {
            "code" => crate::Value::String(fields.code),
            "errno" => crate::mk_int(fields.errno),
            "syscall" => crate::Value::String(fields.syscall),
            "message" => crate::Value::String(fields.message),
            "info" => crate::mk_unit(()),
            other => panic!("Node.Errors.SystemError: unknown field '{other}'"),
        }
    })))
}

pub fn Node_Errors_SystemError_getNullableField() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|field, error| {
        let field = purust_core::purust_string_to_utf8_lossy(&field.unwrap_string());
        let fields = unbox_error(&error);
        let nullable = match field.as_str() {
            "address" => fields
                .address
                .map(|value| Purs_Data_Nullable::Data_Nullable_notNull(crate::Value::String(value)))
                .unwrap_or_else(Purs_Data_Nullable::Data_Nullable_null),
            "dest" => fields
                .dest
                .map(|value| Purs_Data_Nullable::Data_Nullable_notNull(crate::Value::String(value)))
                .unwrap_or_else(Purs_Data_Nullable::Data_Nullable_null),
            "path" => fields
                .path
                .map(|value| Purs_Data_Nullable::Data_Nullable_notNull(crate::Value::String(value)))
                .unwrap_or_else(Purs_Data_Nullable::Data_Nullable_null),
            "port" => fields
                .port
                .map(|value| Purs_Data_Nullable::Data_Nullable_notNull(crate::mk_int(value)))
                .unwrap_or_else(Purs_Data_Nullable::Data_Nullable_null),
            other => panic!("Node.Errors.SystemError: unknown nullable field '{other}'"),
        };
        crate::Value::Class(Rc::new(nullable))
    })))
}

pub fn Node_Errors_SystemError_getSystemErrorName(errno: i64) -> String {
    purust_errno_name(errno as i32).to_owned()
}
