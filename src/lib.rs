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
    return STATUS_SUCCESS;
}
