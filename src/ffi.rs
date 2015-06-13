#![allow(non_camel_case_types)]

use libc::{c_char, c_long, c_ulong, c_void, size_t, timespec};

// dispatch_block_t
pub type dispatch_function_t = extern fn(*mut c_void);
// dispatch_semaphore_t
pub type dispatch_group_t = *mut ();
pub type dispatch_object_t = *mut ();
pub type dispatch_once_t = c_long;
pub type dispatch_queue_t = *mut ();
pub type dispatch_time_t = u64;
// dispatch_source_type_t
// dispatch_fd_t
// dispatch_data_t
// dispatch_data_applier_t
// dispatch_io_t
// dispatch_io_handler_t
// dispatch_io_type_t
// dispatch_io_close_flags_t
// dispatch_io_interval_flags_t
pub type dispatch_queue_attr_t = *const ();

#[link(name = "System", kind = "dylib")]
extern {
    static _dispatch_main_q: ();
    static _dispatch_queue_attr_concurrent: ();

    pub fn dispatch_get_global_queue(identifier: c_long, flags: c_ulong) -> dispatch_queue_t;
    pub fn dispatch_queue_create(label: *const c_char, attr: dispatch_queue_attr_t) -> dispatch_queue_t;
    // dispatch_queue_attr_t dispatch_queue_attr_make_with_qos_class ( dispatch_queue_attr_t attr, dispatch_qos_class_t qos_class, int relative_priority );
    pub fn dispatch_queue_get_label(queue: dispatch_queue_t) -> *const c_char;
    pub fn dispatch_set_target_queue(object: dispatch_object_t, queue: dispatch_queue_t);
    pub fn dispatch_main();

    // void dispatch_async ( dispatch_queue_t queue, dispatch_block_t block );
    pub fn dispatch_async_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    // void dispatch_sync ( dispatch_queue_t queue, dispatch_block_t block );
    pub fn dispatch_sync_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    // void dispatch_after ( dispatch_time_t when, dispatch_queue_t queue, dispatch_block_t block );
    pub fn dispatch_after_f(when: dispatch_time_t, queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    // void dispatch_apply ( size_t iterations, dispatch_queue_t queue, void (^block)(size_t) );
    pub fn dispatch_apply_f(iterations: size_t, queue: dispatch_queue_t, context: *mut c_void, work: extern fn(*mut c_void, size_t));
    // void dispatch_once ( dispatch_once_t *predicate, dispatch_block_t block );
    pub fn dispatch_once_f(predicate: *mut dispatch_once_t, context: *mut c_void, function: dispatch_function_t);

    // void dispatch_group_async ( dispatch_group_t group, dispatch_queue_t queue, dispatch_block_t block );
    pub fn dispatch_group_async_f(group: dispatch_group_t, queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_group_create() -> dispatch_group_t;
    pub fn dispatch_group_enter(group: dispatch_group_t);
    pub fn dispatch_group_leave(group: dispatch_group_t);
    // void dispatch_group_notify ( dispatch_group_t group, dispatch_queue_t queue, dispatch_block_t block );
    pub fn dispatch_group_notify_f(group: dispatch_group_t, queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_group_wait(group: dispatch_group_t, timeout: dispatch_time_t) -> c_long;

    pub fn dispatch_get_context(object: dispatch_object_t) -> *mut c_void;
    pub fn dispatch_release(object: dispatch_object_t);
    pub fn dispatch_resume(object: dispatch_object_t);
    pub fn dispatch_retain(object: dispatch_object_t);
    pub fn dispatch_set_context(object: dispatch_object_t, context: *mut c_void);
    pub fn dispatch_set_finalizer_f(object: dispatch_object_t, finalizer: dispatch_function_t);
    pub fn dispatch_suspend(object: dispatch_object_t);

    // dispatch_semaphore_t dispatch_semaphore_create ( long value );
    // long dispatch_semaphore_signal ( dispatch_semaphore_t dsema );
    // long dispatch_semaphore_wait ( dispatch_semaphore_t dsema, dispatch_time_t timeout );

    // void dispatch_barrier_async ( dispatch_queue_t queue, dispatch_block_t block );
    pub fn dispatch_barrier_async_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    // void dispatch_barrier_sync ( dispatch_queue_t queue, dispatch_block_t block );
    pub fn dispatch_barrier_sync_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);

    // void dispatch_source_cancel ( dispatch_source_t source );
    // dispatch_source_t dispatch_source_create ( dispatch_source_type_t type, uintptr_t handle, unsigned long mask, dispatch_queue_t queue );
    // unsigned long dispatch_source_get_data ( dispatch_source_t source );
    // uintptr_t dispatch_source_get_handle ( dispatch_source_t source );
    // unsigned long dispatch_source_get_mask ( dispatch_source_t source );
    // void dispatch_source_merge_data ( dispatch_source_t source, unsigned long value );
    // void dispatch_source_set_registration_handler ( dispatch_source_t source, dispatch_block_t handler );
    // void dispatch_source_set_registration_handler_f ( dispatch_source_t source, dispatch_function_t handler );
    // void dispatch_source_set_cancel_handler ( dispatch_source_t source, dispatch_block_t handler );
    // void dispatch_source_set_cancel_handler_f ( dispatch_source_t source, dispatch_function_t handler );
    // void dispatch_source_set_event_handler ( dispatch_source_t source, dispatch_block_t handler );
    // void dispatch_source_set_event_handler_f ( dispatch_source_t source, dispatch_function_t handler );
    // void dispatch_source_set_timer ( dispatch_source_t source, dispatch_time_t start, uint64_t interval, uint64_t leeway );
    // long dispatch_source_testcancel ( dispatch_source_t source );

    // void dispatch_read ( dispatch_fd_t fd, size_t length, dispatch_queue_t queue, void (^handler)(dispatch_data_t data, int error) );
    // void dispatch_write ( dispatch_fd_t fd, dispatch_data_t data, dispatch_queue_t queue, void (^handler)(dispatch_data_t data, int error) );

    // dispatch_io_t dispatch_io_create ( dispatch_io_type_t type, dispatch_fd_t fd, dispatch_queue_t queue, void (^cleanup_handler)(int error) );
    // dispatch_io_t dispatch_io_create_with_path ( dispatch_io_type_t type, const char *path, int oflag, mode_t mode, dispatch_queue_t queue, void (^cleanup_handler)(int error) );
    // dispatch_io_t dispatch_io_create_with_io ( dispatch_io_type_t type, dispatch_io_t io, dispatch_queue_t queue, void (^cleanup_handler)(int error) );
    // void dispatch_io_read ( dispatch_io_t channel, off_t offset, size_t length, dispatch_queue_t queue, dispatch_io_handler_t io_handler );
    // void dispatch_io_write ( dispatch_io_t channel, off_t offset, dispatch_data_t data, dispatch_queue_t queue, dispatch_io_handler_t io_handler );
    // void dispatch_io_close ( dispatch_io_t channel, dispatch_io_close_flags_t flags );
    // void dispatch_io_barrier ( dispatch_io_t channel, dispatch_block_t barrier );
    // void dispatch_io_set_high_water ( dispatch_io_t channel, size_t high_water );
    // void dispatch_io_set_low_water ( dispatch_io_t channel, size_t low_water );
    // void dispatch_io_set_interval ( dispatch_io_t channel, uint64_t interval, dispatch_io_interval_flags_t flags );
    // dispatch_fd_t dispatch_io_get_descriptor ( dispatch_io_t channel );

    // dispatch_data_t dispatch_data_create ( const void *buffer, size_t size, dispatch_queue_t queue, dispatch_block_t destructor );
    // size_t dispatch_data_get_size ( dispatch_data_t data );
    // dispatch_data_t dispatch_data_create_map ( dispatch_data_t data, const void **buffer_ptr, size_t *size_ptr );
    // dispatch_data_t dispatch_data_create_concat ( dispatch_data_t data1, dispatch_data_t data2 );
    // dispatch_data_t dispatch_data_create_subrange ( dispatch_data_t data, size_t offset, size_t length );
    // bool dispatch_data_apply ( dispatch_data_t data, dispatch_data_applier_t applier );
    // dispatch_data_t dispatch_data_copy_region ( dispatch_data_t data, size_t location, size_t *offset_ptr );

    pub fn dispatch_time(when: dispatch_time_t, delta: i64) -> dispatch_time_t;
    pub fn dispatch_walltime(when: *const timespec, delta: i64) -> dispatch_time_t;

    // void dispatch_queue_set_specific ( dispatch_queue_t queue, const void *key, void *context, dispatch_function_t destructor );
    // void * dispatch_queue_get_specific ( dispatch_queue_t queue, const void *key );
    // void * dispatch_get_specific ( const void *key );

    // dispatch_block_t dispatch_block_create(dispatch_block_flags_t flags, dispatch_block_t block);
    // dispatch_block_t dispatch_block_create_with_qos_class(dispatch_block_flags_t flags, dispatch_qos_class_t qos_class, int relative_priority, dispatch_block_t block);
    // void dispatch_block_perform(dispatch_block_flags_t flags, dispatch_block_t block);
    // long dispatch_block_wait(dispatch_block_t block, dispatch_time_t timeout);
    // dispatch_block_notify(dispatch_block_t block, dispatch_queue_t queue, dispatch_block_t notification_block);
    // void dispatch_block_cancel(dispatch_block_t block);
    // long dispatch_block_testcancel(dispatch_block_t block);
}

pub fn dispatch_get_main_queue() -> dispatch_queue_t {
    &_dispatch_main_q as *const _ as dispatch_queue_t
}

pub const DISPATCH_QUEUE_SERIAL: dispatch_queue_attr_t = 0 as dispatch_queue_attr_t;
pub static DISPATCH_QUEUE_CONCURRENT: dispatch_queue_attr_t = &_dispatch_queue_attr_concurrent;

pub const DISPATCH_QUEUE_PRIORITY_HIGH: c_long       = 2;
pub const DISPATCH_QUEUE_PRIORITY_DEFAULT: c_long    = 0;
pub const DISPATCH_QUEUE_PRIORITY_LOW: c_long        = -2;
pub const DISPATCH_QUEUE_PRIORITY_BACKGROUND: c_long = -1 << 15;

pub const DISPATCH_TIME_NOW: dispatch_time_t     = 0;
pub const DISPATCH_TIME_FOREVER: dispatch_time_t = !0;

#[cfg(test)]
mod tests {
    use std::ptr;
    use libc::c_void;
    use super::*;

    #[test]
    fn test_serial_queue() {
        extern fn serial_queue_test_add(num: *mut c_void) {
            unsafe {
                *(num as *mut u32) = 1;
            }
        }

        let mut num: u32 = 0;
        let num_ptr: *mut u32 = &mut num;
        unsafe {
            let q = dispatch_queue_create(ptr::null(), DISPATCH_QUEUE_SERIAL);
            dispatch_sync_f(q, num_ptr as *mut c_void, serial_queue_test_add);
            dispatch_release(q);
        }
        assert!(num == 1);
    }
}
