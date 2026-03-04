# SQLite Rust FFI Dependencies
This file maps which functions depend on which other functions.

## `aggregate_context.rs`
Requires implementations from:
  - [`sqlite3_aggregate_context`](sqlite3_aggregate_context.rs) - sqlite3_aggregate_context.rs
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs

## `aggregate_count.rs`
Requires implementations from:
  - [`sqlite3_aggregate_count`](sqlite3_aggregate_count.rs) - sqlite3_aggregate_count.rs
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_expired`](sqlite3_expired.rs) - sqlite3_expired.rs
  - [`sqlite3_global_recover`](sqlite3_global_recover.rs) - sqlite3_global_recover.rs
  - [`sqlite3_memory_alarm`](sqlite3_memory_alarm.rs) - sqlite3_memory_alarm.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_thread_cleanup`](sqlite3_thread_cleanup.rs) - sqlite3_thread_cleanup.rs
  - [`sqlite3_transfer_bindings`](sqlite3_transfer_bindings.rs) - sqlite3_transfer_bindings.rs

## `api_routines.rs`
Requires implementations from:
  - [`sqlite3_api_routines`](sqlite3_api_routines.rs) - sqlite3_api_routines.rs

## `auto_extension.rs`
Requires implementations from:
  - [`sqlite3_api_routines`](sqlite3_api_routines.rs) - sqlite3_api_routines.rs
  - [`sqlite3_auto_extension`](sqlite3_auto_extension.rs) - sqlite3_auto_extension.rs
  - [`sqlite3_cancel_auto_extension`](sqlite3_cancel_auto_extension.rs) - sqlite3_cancel_auto_extension.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_mprintf`](sqlite3_mprintf.rs) - sqlite3_mprintf.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open16`](sqlite3_open16.rs) - sqlite3_open16.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_reset_auto_extension`](sqlite3_reset_auto_extension.rs) - sqlite3_reset_auto_extension.rs

## `autovacuum_pages.rs`
Requires implementations from:
  - [`sqlite3_autovacuum_pages`](sqlite3_autovacuum_pages.rs) - sqlite3_autovacuum_pages.rs

## `backup.rs`
Requires implementations from:
  - [`sqlite3_backup`](sqlite3_backup.rs) - sqlite3_backup.rs
  - [`sqlite3_backup_finish`](sqlite3_backup_finish.rs) - sqlite3_backup_finish.rs
  - [`sqlite3_backup_init`](sqlite3_backup_init.rs) - sqlite3_backup_init.rs

## `backup_finish.rs`
Requires implementations from:
  - [`sqlite3_backup`](sqlite3_backup.rs) - sqlite3_backup.rs
  - [`sqlite3_backup_finish`](sqlite3_backup_finish.rs) - sqlite3_backup_finish.rs
  - [`sqlite3_backup_init`](sqlite3_backup_init.rs) - sqlite3_backup_init.rs
  - [`sqlite3_backup_pagecount`](sqlite3_backup_pagecount.rs) - sqlite3_backup_pagecount.rs
  - [`sqlite3_backup_remaining`](sqlite3_backup_remaining.rs) - sqlite3_backup_remaining.rs
  - [`sqlite3_backup_step`](sqlite3_backup_step.rs) - sqlite3_backup_step.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_errmsg16`](sqlite3_errmsg16.rs) - sqlite3_errmsg16.rs
  - [`sqlite3_rsync`](sqlite3_rsync.rs) - sqlite3_rsync.rs

## `bind_blob.rs`
Requires implementations from:
  - [`sqlite3_bind_`](sqlite3_bind_.rs) - sqlite3_bind_.rs
  - [`sqlite3_bind_blob`](sqlite3_bind_blob.rs) - sqlite3_bind_blob.rs
  - [`sqlite3_bind_double`](sqlite3_bind_double.rs) - sqlite3_bind_double.rs
  - [`sqlite3_bind_int`](sqlite3_bind_int.rs) - sqlite3_bind_int.rs
  - [`sqlite3_bind_null`](sqlite3_bind_null.rs) - sqlite3_bind_null.rs
  - [`sqlite3_bind_parameter_count`](sqlite3_bind_parameter_count.rs) - sqlite3_bind_parameter_count.rs
  - [`sqlite3_bind_parameter_index`](sqlite3_bind_parameter_index.rs) - sqlite3_bind_parameter_index.rs
  - [`sqlite3_bind_parameter_name`](sqlite3_bind_parameter_name.rs) - sqlite3_bind_parameter_name.rs
  - [`sqlite3_bind_pointer`](sqlite3_bind_pointer.rs) - sqlite3_bind_pointer.rs
  - [`sqlite3_bind_text`](sqlite3_bind_text.rs) - sqlite3_bind_text.rs
  - [`sqlite3_bind_value`](sqlite3_bind_value.rs) - sqlite3_bind_value.rs
  - [`sqlite3_bind_zeroblob`](sqlite3_bind_zeroblob.rs) - sqlite3_bind_zeroblob.rs
  - [`sqlite3_limit`](sqlite3_limit.rs) - sqlite3_limit.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs

## `bind_parameter_count.rs`
Requires implementations from:
  - [`sqlite3_bind`](sqlite3_bind.rs) - sqlite3_bind.rs
  - [`sqlite3_bind_parameter_count`](sqlite3_bind_parameter_count.rs) - sqlite3_bind_parameter_count.rs
  - [`sqlite3_bind_parameter_index`](sqlite3_bind_parameter_index.rs) - sqlite3_bind_parameter_index.rs
  - [`sqlite3_bind_parameter_name`](sqlite3_bind_parameter_name.rs) - sqlite3_bind_parameter_name.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `bind_parameter_index.rs`
Requires implementations from:
  - [`sqlite3_bind`](sqlite3_bind.rs) - sqlite3_bind.rs
  - [`sqlite3_bind_parameter_count`](sqlite3_bind_parameter_count.rs) - sqlite3_bind_parameter_count.rs
  - [`sqlite3_bind_parameter_index`](sqlite3_bind_parameter_index.rs) - sqlite3_bind_parameter_index.rs
  - [`sqlite3_bind_parameter_name`](sqlite3_bind_parameter_name.rs) - sqlite3_bind_parameter_name.rs
  - [`sqlite3_prepare16_v2`](sqlite3_prepare16_v2.rs) - sqlite3_prepare16_v2.rs
  - [`sqlite3_prepare16_v3`](sqlite3_prepare16_v3.rs) - sqlite3_prepare16_v3.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `bind_parameter_name.rs`
Requires implementations from:
  - [`sqlite3_bind`](sqlite3_bind.rs) - sqlite3_bind.rs
  - [`sqlite3_bind_parameter_count`](sqlite3_bind_parameter_count.rs) - sqlite3_bind_parameter_count.rs
  - [`sqlite3_bind_parameter_index`](sqlite3_bind_parameter_index.rs) - sqlite3_bind_parameter_index.rs
  - [`sqlite3_bind_parameter_name`](sqlite3_bind_parameter_name.rs) - sqlite3_bind_parameter_name.rs
  - [`sqlite3_prepare16`](sqlite3_prepare16.rs) - sqlite3_prepare16.rs
  - [`sqlite3_prepare16_v2`](sqlite3_prepare16_v2.rs) - sqlite3_prepare16_v2.rs
  - [`sqlite3_prepare16_v3`](sqlite3_prepare16_v3.rs) - sqlite3_prepare16_v3.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `blob.rs`
Requires implementations from:
  - [`sqlite3_blob`](sqlite3_blob.rs) - sqlite3_blob.rs
  - [`sqlite3_blob_bytes`](sqlite3_blob_bytes.rs) - sqlite3_blob_bytes.rs
  - [`sqlite3_blob_close`](sqlite3_blob_close.rs) - sqlite3_blob_close.rs
  - [`sqlite3_blob_open`](sqlite3_blob_open.rs) - sqlite3_blob_open.rs
  - [`sqlite3_blob_read`](sqlite3_blob_read.rs) - sqlite3_blob_read.rs
  - [`sqlite3_blob_reopen`](sqlite3_blob_reopen.rs) - sqlite3_blob_reopen.rs
  - [`sqlite3_blob_write`](sqlite3_blob_write.rs) - sqlite3_blob_write.rs

## `blob_bytes.rs`
Requires implementations from:
  - [`sqlite3_blob`](sqlite3_blob.rs) - sqlite3_blob.rs
  - [`sqlite3_blob_bytes`](sqlite3_blob_bytes.rs) - sqlite3_blob_bytes.rs
  - [`sqlite3_blob_close`](sqlite3_blob_close.rs) - sqlite3_blob_close.rs
  - [`sqlite3_blob_open`](sqlite3_blob_open.rs) - sqlite3_blob_open.rs

## `blob_close.rs`
Requires implementations from:
  - [`sqlite3_blob`](sqlite3_blob.rs) - sqlite3_blob.rs
  - [`sqlite3_blob_close`](sqlite3_blob_close.rs) - sqlite3_blob_close.rs
  - [`sqlite3_blob_open`](sqlite3_blob_open.rs) - sqlite3_blob_open.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs

## `blob_open.rs`
Requires implementations from:
  - [`sqlite3_bind_zeroblob`](sqlite3_bind_zeroblob.rs) - sqlite3_bind_zeroblob.rs
  - [`sqlite3_blob`](sqlite3_blob.rs) - sqlite3_blob.rs
  - [`sqlite3_blob_bytes`](sqlite3_blob_bytes.rs) - sqlite3_blob_bytes.rs
  - [`sqlite3_blob_close`](sqlite3_blob_close.rs) - sqlite3_blob_close.rs
  - [`sqlite3_blob_open`](sqlite3_blob_open.rs) - sqlite3_blob_open.rs
  - [`sqlite3_blob_read`](sqlite3_blob_read.rs) - sqlite3_blob_read.rs
  - [`sqlite3_blob_reopen`](sqlite3_blob_reopen.rs) - sqlite3_blob_reopen.rs
  - [`sqlite3_blob_write`](sqlite3_blob_write.rs) - sqlite3_blob_write.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_result_zeroblob`](sqlite3_result_zeroblob.rs) - sqlite3_result_zeroblob.rs

## `blob_read.rs`
Requires implementations from:
  - [`sqlite3_blob`](sqlite3_blob.rs) - sqlite3_blob.rs
  - [`sqlite3_blob_bytes`](sqlite3_blob_bytes.rs) - sqlite3_blob_bytes.rs
  - [`sqlite3_blob_close`](sqlite3_blob_close.rs) - sqlite3_blob_close.rs
  - [`sqlite3_blob_open`](sqlite3_blob_open.rs) - sqlite3_blob_open.rs
  - [`sqlite3_blob_read`](sqlite3_blob_read.rs) - sqlite3_blob_read.rs
  - [`sqlite3_blob_write`](sqlite3_blob_write.rs) - sqlite3_blob_write.rs

## `blob_reopen.rs`
Requires implementations from:
  - [`sqlite3_blob`](sqlite3_blob.rs) - sqlite3_blob.rs
  - [`sqlite3_blob_bytes`](sqlite3_blob_bytes.rs) - sqlite3_blob_bytes.rs
  - [`sqlite3_blob_open`](sqlite3_blob_open.rs) - sqlite3_blob_open.rs
  - [`sqlite3_blob_read`](sqlite3_blob_read.rs) - sqlite3_blob_read.rs
  - [`sqlite3_blob_reopen`](sqlite3_blob_reopen.rs) - sqlite3_blob_reopen.rs
  - [`sqlite3_blob_write`](sqlite3_blob_write.rs) - sqlite3_blob_write.rs

## `blob_write.rs`
Requires implementations from:
  - [`sqlite3_blob`](sqlite3_blob.rs) - sqlite3_blob.rs
  - [`sqlite3_blob_bytes`](sqlite3_blob_bytes.rs) - sqlite3_blob_bytes.rs
  - [`sqlite3_blob_close`](sqlite3_blob_close.rs) - sqlite3_blob_close.rs
  - [`sqlite3_blob_open`](sqlite3_blob_open.rs) - sqlite3_blob_open.rs
  - [`sqlite3_blob_read`](sqlite3_blob_read.rs) - sqlite3_blob_read.rs
  - [`sqlite3_blob_write`](sqlite3_blob_write.rs) - sqlite3_blob_write.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs

## `busy_handler.rs`
Requires implementations from:
  - [`sqlite3_busy_handler`](sqlite3_busy_handler.rs) - sqlite3_busy_handler.rs
  - [`sqlite3_busy_timeout`](sqlite3_busy_timeout.rs) - sqlite3_busy_timeout.rs

## `busy_timeout.rs`
Requires implementations from:
  - [`sqlite3_busy_handler`](sqlite3_busy_handler.rs) - sqlite3_busy_handler.rs
  - [`sqlite3_busy_timeout`](sqlite3_busy_timeout.rs) - sqlite3_busy_timeout.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs

## `c_abort.rs`
Requires implementations from:
  - [`sqlite3_bind`](sqlite3_bind.rs) - sqlite3_bind.rs
  - [`sqlite3_file_control`](sqlite3_file_control.rs) - sqlite3_file_control.rs
  - [`sqlite3_interrupt`](sqlite3_interrupt.rs) - sqlite3_interrupt.rs
  - [`sqlite3_log`](sqlite3_log.rs) - sqlite3_log.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs

## `c_abort_rollback.rs`
Requires implementations from:
  - [`sqlite3_extended_errcode`](sqlite3_extended_errcode.rs) - sqlite3_extended_errcode.rs
  - [`sqlite3_extended_result_codes`](sqlite3_extended_result_codes.rs) - sqlite3_extended_result_codes.rs

## `c_access_exists.rs`
Requires implementations from:
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs

## `c_alter_table.rs`
Requires implementations from:
  - [`sqlite3_set_authorizer`](sqlite3_set_authorizer.rs) - sqlite3_set_authorizer.rs

## `c_any.rs`
Requires implementations from:
  - [`sqlite3_create_collation`](sqlite3_create_collation.rs) - sqlite3_create_collation.rs

## `c_carray_blob.rs`
Requires implementations from:
  - [`sqlite3_carray_bind`](sqlite3_carray_bind.rs) - sqlite3_carray_bind.rs

## `c_checkpoint_full.rs`
Requires implementations from:
  - [`sqlite3_wal_checkpoint_v2`](sqlite3_wal_checkpoint_v2.rs) - sqlite3_wal_checkpoint_v2.rs

## `c_config_covering_index_scan.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_db_config`](sqlite3_db_config.rs) - sqlite3_db_config.rs
  - [`sqlite3_deserialize`](sqlite3_deserialize.rs) - sqlite3_deserialize.rs
  - [`sqlite3_hard_heap_limit64`](sqlite3_hard_heap_limit64.rs) - sqlite3_hard_heap_limit64.rs
  - [`sqlite3_initialize`](sqlite3_initialize.rs) - sqlite3_initialize.rs
  - [`sqlite3_int64`](sqlite3_int64.rs) - sqlite3_int64.rs
  - [`sqlite3_log`](sqlite3_log.rs) - sqlite3_log.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_mem_methods`](sqlite3_mem_methods.rs) - sqlite3_mem_methods.rs
  - [`sqlite3_memory_highwater`](sqlite3_memory_highwater.rs) - sqlite3_memory_highwater.rs
  - [`sqlite3_memory_used`](sqlite3_memory_used.rs) - sqlite3_memory_used.rs
  - [`sqlite3_mutex_methods`](sqlite3_mutex_methods.rs) - sqlite3_mutex_methods.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open16`](sqlite3_open16.rs) - sqlite3_open16.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_pcache_methods2`](sqlite3_pcache_methods2.rs) - sqlite3_pcache_methods2.rs
  - [`sqlite3_shutdown`](sqlite3_shutdown.rs) - sqlite3_shutdown.rs
  - [`sqlite3_snprintf`](sqlite3_snprintf.rs) - sqlite3_snprintf.rs
  - [`sqlite3_soft_heap_limit64`](sqlite3_soft_heap_limit64.rs) - sqlite3_soft_heap_limit64.rs
  - [`sqlite3_status64`](sqlite3_status64.rs) - sqlite3_status64.rs

## `c_dbconfig_defensive.rs`
Requires implementations from:
  - [`sqlite3_db_config`](sqlite3_db_config.rs) - sqlite3_db_config.rs
  - [`sqlite3_enable_load_extension`](sqlite3_enable_load_extension.rs) - sqlite3_enable_load_extension.rs
  - [`sqlite3_exec`](sqlite3_exec.rs) - sqlite3_exec.rs
  - [`sqlite3_load_extension`](sqlite3_load_extension.rs) - sqlite3_load_extension.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_table_column_metadata`](sqlite3_table_column_metadata.rs) - sqlite3_table_column_metadata.rs

## `c_dbstatus_options.rs`
Requires implementations from:
  - [`sqlite3_db_status`](sqlite3_db_status.rs) - sqlite3_db_status.rs

## `c_deny.rs`
Requires implementations from:
  - [`sqlite3_vtab_on_conflict`](sqlite3_vtab_on_conflict.rs) - sqlite3_vtab_on_conflict.rs

## `c_deserialize_freeonclose.rs`
Requires implementations from:
  - [`sqlite3_deserialize`](sqlite3_deserialize.rs) - sqlite3_deserialize.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc64`](sqlite3_malloc64.rs) - sqlite3_malloc64.rs
  - [`sqlite3_realloc64`](sqlite3_realloc64.rs) - sqlite3_realloc64.rs

## `c_deterministic.rs`
Requires implementations from:
  - [`sqlite3_create_function`](sqlite3_create_function.rs) - sqlite3_create_function.rs
  - [`sqlite3_create_function16`](sqlite3_create_function16.rs) - sqlite3_create_function16.rs
  - [`sqlite3_create_function_v2`](sqlite3_create_function_v2.rs) - sqlite3_create_function_v2.rs
  - [`sqlite3_result_subtype`](sqlite3_result_subtype.rs) - sqlite3_result_subtype.rs
  - [`sqlite3_value_subtype`](sqlite3_value_subtype.rs) - sqlite3_value_subtype.rs

## `c_fail.rs`
Requires implementations from:
  - [`sqlite3_authorizer`](sqlite3_authorizer.rs) - sqlite3_authorizer.rs
  - [`sqlite3_set_authorizer`](sqlite3_set_authorizer.rs) - sqlite3_set_authorizer.rs
  - [`sqlite3_vtab_on_conflict`](sqlite3_vtab_on_conflict.rs) - sqlite3_vtab_on_conflict.rs

## `c_fcntl_begin_atomic_write.rs`
Requires implementations from:
  - [`sqlite3_deserialize`](sqlite3_deserialize.rs) - sqlite3_deserialize.rs
  - [`sqlite3_file`](sqlite3_file.rs) - sqlite3_file.rs
  - [`sqlite3_file_control`](sqlite3_file_control.rs) - sqlite3_file_control.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_int64`](sqlite3_int64.rs) - sqlite3_int64.rs
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_mprintf`](sqlite3_mprintf.rs) - sqlite3_mprintf.rs
  - [`sqlite3_str`](sqlite3_str.rs) - sqlite3_str.rs
  - [`sqlite3_total_changes`](sqlite3_total_changes.rs) - sqlite3_total_changes.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs

## `c_index_constraint_eq.rs`
Requires implementations from:
  - [`sqlite3_index_info`](sqlite3_index_info.rs) - sqlite3_index_info.rs
  - [`sqlite3_vtab_collation`](sqlite3_vtab_collation.rs) - sqlite3_vtab_collation.rs
  - [`sqlite3_vtab_rhs_value`](sqlite3_vtab_rhs_value.rs) - sqlite3_vtab_rhs_value.rs

## `c_index_scan_hex.rs`
Requires implementations from:
  - [`sqlite3_index_info`](sqlite3_index_info.rs) - sqlite3_index_info.rs

## `c_iocap_atomic.rs`
Requires implementations from:
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs

## `c_limit_attached.rs`
Requires implementations from:
  - [`sqlite3_limit`](sqlite3_limit.rs) - sqlite3_limit.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs

## `c_lock_exclusive.rs`
Requires implementations from:
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs

## `c_mutex_fast.rs`
Requires implementations from:
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_mutex_alloc`](sqlite3_mutex_alloc.rs) - sqlite3_mutex_alloc.rs
  - [`sqlite3_randomness`](sqlite3_randomness.rs) - sqlite3_randomness.rs

## `c_open_autoproxy.rs`
Requires implementations from:
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs
  - [`sqlite3_vfs.xOpen`](sqlite3_vfs.xOpen.rs) - sqlite3_vfs.xOpen.rs

## `c_prepare_dont_log.rs`
Requires implementations from:
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_log`](sqlite3_log.rs) - sqlite3_log.rs
  - [`sqlite3_normalized_sql`](sqlite3_normalized_sql.rs) - sqlite3_normalized_sql.rs
  - [`sqlite3_prepare16_v3`](sqlite3_prepare16_v3.rs) - sqlite3_prepare16_v3.rs
  - [`sqlite3_prepare_v3`](sqlite3_prepare_v3.rs) - sqlite3_prepare_v3.rs

## `c_scanstat_est.rs`
Requires implementations from:
  - [`sqlite3_int64`](sqlite3_int64.rs) - sqlite3_int64.rs
  - [`sqlite3_stmt_scanstatus`](sqlite3_stmt_scanstatus.rs) - sqlite3_stmt_scanstatus.rs

## `c_scm_branch.rs`
Requires implementations from:
  - [`sqlite3_libversion`](sqlite3_libversion.rs) - sqlite3_libversion.rs
  - [`sqlite3_libversion_number`](sqlite3_libversion_number.rs) - sqlite3_libversion_number.rs
  - [`sqlite3_sourceid`](sqlite3_sourceid.rs) - sqlite3_sourceid.rs

## `c_serialize_nocopy.rs`
Requires implementations from:
  - [`sqlite3_deserialize`](sqlite3_deserialize.rs) - sqlite3_deserialize.rs
  - [`sqlite3_serialize`](sqlite3_serialize.rs) - sqlite3_serialize.rs

## `c_setlk_block_on_connect.rs`
Requires implementations from:
  - [`sqlite3_setlk_timeout`](sqlite3_setlk_timeout.rs) - sqlite3_setlk_timeout.rs

## `c_shm_exclusive.rs`
Requires implementations from:
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs

## `c_shm_nlock.rs`
Requires implementations from:
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs

## `c_static.rs`
Requires implementations from:
  - [`sqlite3_destructor_type`](sqlite3_destructor_type.rs) - sqlite3_destructor_type.rs
  - [`sqlite3_result_blob`](sqlite3_result_blob.rs) - sqlite3_result_blob.rs

## `c_status_malloc_count.rs`
Requires implementations from:
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_mem_methods`](sqlite3_mem_methods.rs) - sqlite3_mem_methods.rs
  - [`sqlite3_realloc`](sqlite3_realloc.rs) - sqlite3_realloc.rs
  - [`sqlite3_status`](sqlite3_status.rs) - sqlite3_status.rs

## `c_stmtstatus_counter.rs`
Requires implementations from:
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt_status`](sqlite3_stmt_status.rs) - sqlite3_stmt_status.rs

## `c_sync_dataonly.rs`
Requires implementations from:
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs

## `c_testctrl_always.rs`
Requires implementations from:
  - [`sqlite3_test_control`](sqlite3_test_control.rs) - sqlite3_test_control.rs

## `c_trace.rs`
Requires implementations from:
  - [`sqlite3_expanded_sql`](sqlite3_expanded_sql.rs) - sqlite3_expanded_sql.rs
  - [`sqlite3_profile`](sqlite3_profile.rs) - sqlite3_profile.rs
  - [`sqlite3_trace`](sqlite3_trace.rs) - sqlite3_trace.rs
  - [`sqlite3_trace_v2`](sqlite3_trace_v2.rs) - sqlite3_trace_v2.rs

## `c_txn_none.rs`
Requires implementations from:
  - [`sqlite3_txn_state`](sqlite3_txn_state.rs) - sqlite3_txn_state.rs

## `c_vtab_constraint_support.rs`
Requires implementations from:
  - [`sqlite3_vtab_config`](sqlite3_vtab_config.rs) - sqlite3_vtab_config.rs
  - [`sqlite3_vtab_on_conflict`](sqlite3_vtab_on_conflict.rs) - sqlite3_vtab_on_conflict.rs

## `c_win32_data_directory_type.rs`
Requires implementations from:
  - [`sqlite3_win32_set_directory`](sqlite3_win32_set_directory.rs) - sqlite3_win32_set_directory.rs

## `cancel_auto_extension.rs`
Requires implementations from:
  - [`sqlite3_auto_extension`](sqlite3_auto_extension.rs) - sqlite3_auto_extension.rs
  - [`sqlite3_cancel_auto_extension`](sqlite3_cancel_auto_extension.rs) - sqlite3_cancel_auto_extension.rs

## `carray_bind.rs`
Requires implementations from:
  - [`sqlite3_carray_bind`](sqlite3_carray_bind.rs) - sqlite3_carray_bind.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `changes.rs`
Requires implementations from:
  - [`sqlite3_changes`](sqlite3_changes.rs) - sqlite3_changes.rs
  - [`sqlite3_total_changes`](sqlite3_total_changes.rs) - sqlite3_total_changes.rs

## `clear_bindings.rs`
Requires implementations from:
  - [`sqlite3_clear_bindings`](sqlite3_clear_bindings.rs) - sqlite3_clear_bindings.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `close.rs`
Requires implementations from:
  - [`sqlite3_backup`](sqlite3_backup.rs) - sqlite3_backup.rs
  - [`sqlite3_backups`](sqlite3_backups.rs) - sqlite3_backups.rs
  - [`sqlite3_close`](sqlite3_close.rs) - sqlite3_close.rs
  - [`sqlite3_close_v2`](sqlite3_close_v2.rs) - sqlite3_close_v2.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open16`](sqlite3_open16.rs) - sqlite3_open16.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs

## `collation_needed.rs`
Requires implementations from:
  - [`sqlite3_collation_needed`](sqlite3_collation_needed.rs) - sqlite3_collation_needed.rs
  - [`sqlite3_create_collation`](sqlite3_create_collation.rs) - sqlite3_create_collation.rs
  - [`sqlite3_create_collation16`](sqlite3_create_collation16.rs) - sqlite3_create_collation16.rs
  - [`sqlite3_create_collation_v2`](sqlite3_create_collation_v2.rs) - sqlite3_create_collation_v2.rs

## `column_blob.rs`
Requires implementations from:
  - [`sqlite3_bind_value`](sqlite3_bind_value.rs) - sqlite3_bind_value.rs
  - [`sqlite3_column_blob`](sqlite3_column_blob.rs) - sqlite3_column_blob.rs
  - [`sqlite3_column_bytes`](sqlite3_column_bytes.rs) - sqlite3_column_bytes.rs
  - [`sqlite3_column_bytes16`](sqlite3_column_bytes16.rs) - sqlite3_column_bytes16.rs
  - [`sqlite3_column_count`](sqlite3_column_count.rs) - sqlite3_column_count.rs
  - [`sqlite3_column_double`](sqlite3_column_double.rs) - sqlite3_column_double.rs
  - [`sqlite3_column_int`](sqlite3_column_int.rs) - sqlite3_column_int.rs
  - [`sqlite3_column_text`](sqlite3_column_text.rs) - sqlite3_column_text.rs
  - [`sqlite3_column_type`](sqlite3_column_type.rs) - sqlite3_column_type.rs
  - [`sqlite3_column_value`](sqlite3_column_value.rs) - sqlite3_column_value.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_result_value`](sqlite3_result_value.rs) - sqlite3_result_value.rs
  - [`sqlite3_snprintf`](sqlite3_snprintf.rs) - sqlite3_snprintf.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt*`](sqlite3_stmt*.rs) - sqlite3_stmt*.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_value_bytes`](sqlite3_value_bytes.rs) - sqlite3_value_bytes.rs
  - [`sqlite3_value_int`](sqlite3_value_int.rs) - sqlite3_value_int.rs
  - [`sqlite3_value_text`](sqlite3_value_text.rs) - sqlite3_value_text.rs

## `column_count.rs`
Requires implementations from:
  - [`sqlite3_column_count`](sqlite3_column_count.rs) - sqlite3_column_count.rs
  - [`sqlite3_data_count`](sqlite3_data_count.rs) - sqlite3_data_count.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `column_database_name.rs`
Requires implementations from:
  - [`sqlite3_column_database_name`](sqlite3_column_database_name.rs) - sqlite3_column_database_name.rs
  - [`sqlite3_column_origin_name`](sqlite3_column_origin_name.rs) - sqlite3_column_origin_name.rs
  - [`sqlite3_column_table_name`](sqlite3_column_table_name.rs) - sqlite3_column_table_name.rs
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `column_decltype.rs`
Requires implementations from:
  - [`sqlite3_column_decltype`](sqlite3_column_decltype.rs) - sqlite3_column_decltype.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `column_name.rs`
Requires implementations from:
  - [`sqlite3_column_name`](sqlite3_column_name.rs) - sqlite3_column_name.rs
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `commit_hook.rs`
Requires implementations from:
  - [`sqlite3_commit_hook`](sqlite3_commit_hook.rs) - sqlite3_commit_hook.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_rollback_hook`](sqlite3_rollback_hook.rs) - sqlite3_rollback_hook.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_update_hook`](sqlite3_update_hook.rs) - sqlite3_update_hook.rs

## `compileoption_get.rs`
Requires implementations from:
  - [`sqlite3_compileoption_get`](sqlite3_compileoption_get.rs) - sqlite3_compileoption_get.rs
  - [`sqlite3_compileoption_used`](sqlite3_compileoption_used.rs) - sqlite3_compileoption_used.rs

## `complete.rs`
Requires implementations from:
  - [`sqlite3_complete`](sqlite3_complete.rs) - sqlite3_complete.rs
  - [`sqlite3_complete16`](sqlite3_complete16.rs) - sqlite3_complete16.rs
  - [`sqlite3_initialize`](sqlite3_initialize.rs) - sqlite3_initialize.rs

## `config.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_initialize`](sqlite3_initialize.rs) - sqlite3_initialize.rs
  - [`sqlite3_os_init`](sqlite3_os_init.rs) - sqlite3_os_init.rs
  - [`sqlite3_shutdown`](sqlite3_shutdown.rs) - sqlite3_shutdown.rs

## `context.rs`
Requires implementations from:
  - [`sqlite3_aggregate_context`](sqlite3_aggregate_context.rs) - sqlite3_aggregate_context.rs
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_context_db_handle`](sqlite3_context_db_handle.rs) - sqlite3_context_db_handle.rs
  - [`sqlite3_get_auxdata`](sqlite3_get_auxdata.rs) - sqlite3_get_auxdata.rs
  - [`sqlite3_result`](sqlite3_result.rs) - sqlite3_result.rs
  - [`sqlite3_result_blob`](sqlite3_result_blob.rs) - sqlite3_result_blob.rs
  - [`sqlite3_result_blob64`](sqlite3_result_blob64.rs) - sqlite3_result_blob64.rs
  - [`sqlite3_result_double`](sqlite3_result_double.rs) - sqlite3_result_double.rs
  - [`sqlite3_result_error`](sqlite3_result_error.rs) - sqlite3_result_error.rs
  - [`sqlite3_result_error16`](sqlite3_result_error16.rs) - sqlite3_result_error16.rs
  - [`sqlite3_result_error_code`](sqlite3_result_error_code.rs) - sqlite3_result_error_code.rs
  - [`sqlite3_result_error_nomem`](sqlite3_result_error_nomem.rs) - sqlite3_result_error_nomem.rs
  - [`sqlite3_result_error_toobig`](sqlite3_result_error_toobig.rs) - sqlite3_result_error_toobig.rs
  - [`sqlite3_result_int`](sqlite3_result_int.rs) - sqlite3_result_int.rs
  - [`sqlite3_result_int64`](sqlite3_result_int64.rs) - sqlite3_result_int64.rs
  - [`sqlite3_result_null`](sqlite3_result_null.rs) - sqlite3_result_null.rs
  - [`sqlite3_result_pointer`](sqlite3_result_pointer.rs) - sqlite3_result_pointer.rs
  - [`sqlite3_result_subtype`](sqlite3_result_subtype.rs) - sqlite3_result_subtype.rs
  - [`sqlite3_result_text`](sqlite3_result_text.rs) - sqlite3_result_text.rs
  - [`sqlite3_result_text16`](sqlite3_result_text16.rs) - sqlite3_result_text16.rs
  - [`sqlite3_result_text16be`](sqlite3_result_text16be.rs) - sqlite3_result_text16be.rs
  - [`sqlite3_result_text16le`](sqlite3_result_text16le.rs) - sqlite3_result_text16le.rs
  - [`sqlite3_result_text64`](sqlite3_result_text64.rs) - sqlite3_result_text64.rs
  - [`sqlite3_result_value`](sqlite3_result_value.rs) - sqlite3_result_value.rs
  - [`sqlite3_result_zeroblob`](sqlite3_result_zeroblob.rs) - sqlite3_result_zeroblob.rs
  - [`sqlite3_result_zeroblob64`](sqlite3_result_zeroblob64.rs) - sqlite3_result_zeroblob64.rs
  - [`sqlite3_set_auxdata`](sqlite3_set_auxdata.rs) - sqlite3_set_auxdata.rs
  - [`sqlite3_user_data`](sqlite3_user_data.rs) - sqlite3_user_data.rs

## `context_db_handle.rs`
Requires implementations from:
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_context_db_handle`](sqlite3_context_db_handle.rs) - sqlite3_context_db_handle.rs
  - [`sqlite3_create_function`](sqlite3_create_function.rs) - sqlite3_create_function.rs
  - [`sqlite3_create_function16`](sqlite3_create_function16.rs) - sqlite3_create_function16.rs

## `create_collation.rs`
Requires implementations from:
  - [`sqlite3_close`](sqlite3_close.rs) - sqlite3_close.rs
  - [`sqlite3_collation_needed`](sqlite3_collation_needed.rs) - sqlite3_collation_needed.rs
  - [`sqlite3_collation_needed16`](sqlite3_collation_needed16.rs) - sqlite3_collation_needed16.rs
  - [`sqlite3_create_collation`](sqlite3_create_collation.rs) - sqlite3_create_collation.rs
  - [`sqlite3_strnicmp`](sqlite3_strnicmp.rs) - sqlite3_strnicmp.rs

## `create_filename.rs`
Requires implementations from:
  - [`sqlite3_create_filename`](sqlite3_create_filename.rs) - sqlite3_create_filename.rs
  - [`sqlite3_filename`](sqlite3_filename.rs) - sqlite3_filename.rs
  - [`sqlite3_filename_database`](sqlite3_filename_database.rs) - sqlite3_filename_database.rs
  - [`sqlite3_filename_journal`](sqlite3_filename_journal.rs) - sqlite3_filename_journal.rs
  - [`sqlite3_filename_wal`](sqlite3_filename_wal.rs) - sqlite3_filename_wal.rs
  - [`sqlite3_free_filename`](sqlite3_free_filename.rs) - sqlite3_free_filename.rs
  - [`sqlite3_module`](sqlite3_module.rs) - sqlite3_module.rs
  - [`sqlite3_uri_boolean`](sqlite3_uri_boolean.rs) - sqlite3_uri_boolean.rs
  - [`sqlite3_uri_int64`](sqlite3_uri_int64.rs) - sqlite3_uri_int64.rs
  - [`sqlite3_uri_key`](sqlite3_uri_key.rs) - sqlite3_uri_key.rs
  - [`sqlite3_uri_parameter`](sqlite3_uri_parameter.rs) - sqlite3_uri_parameter.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs
  - [`sqlite3_vfs.xOpen`](sqlite3_vfs.xOpen.rs) - sqlite3_vfs.xOpen.rs

## `create_function.rs`
Requires implementations from:
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_create_function`](sqlite3_create_function.rs) - sqlite3_create_function.rs
  - [`sqlite3_create_window_function`](sqlite3_create_window_function.rs) - sqlite3_create_window_function.rs
  - [`sqlite3_limit`](sqlite3_limit.rs) - sqlite3_limit.rs
  - [`sqlite3_user_data`](sqlite3_user_data.rs) - sqlite3_user_data.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_value_text16`](sqlite3_value_text16.rs) - sqlite3_value_text16.rs
  - [`sqlite3_value_text16be`](sqlite3_value_text16be.rs) - sqlite3_value_text16be.rs
  - [`sqlite3_value_text16le`](sqlite3_value_text16le.rs) - sqlite3_value_text16le.rs

## `create_module.rs`
Requires implementations from:
  - [`sqlite3_create_module`](sqlite3_create_module.rs) - sqlite3_create_module.rs
  - [`sqlite3_drop_modules`](sqlite3_drop_modules.rs) - sqlite3_drop_modules.rs
  - [`sqlite3_module`](sqlite3_module.rs) - sqlite3_module.rs

## `data_count.rs`
Requires implementations from:
  - [`sqlite3_column`](sqlite3_column.rs) - sqlite3_column.rs
  - [`sqlite3_column_count`](sqlite3_column_count.rs) - sqlite3_column_count.rs
  - [`sqlite3_data_count`](sqlite3_data_count.rs) - sqlite3_data_count.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `data_directory.rs`
Requires implementations from:
  - [`sqlite3_data_directory`](sqlite3_data_directory.rs) - sqlite3_data_directory.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs

## `database_file_object.rs`
Requires implementations from:
  - [`sqlite3_database_file_object`](sqlite3_database_file_object.rs) - sqlite3_database_file_object.rs
  - [`sqlite3_file`](sqlite3_file.rs) - sqlite3_file.rs
  - [`sqlite3_file_object`](sqlite3_file_object.rs) - sqlite3_file_object.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs

## `db_cacheflush.rs`
Requires implementations from:
  - [`sqlite3_db_cacheflush`](sqlite3_db_cacheflush.rs) - sqlite3_db_cacheflush.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs

## `db_config.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_db_config`](sqlite3_db_config.rs) - sqlite3_db_config.rs

## `db_filename.rs`
Requires implementations from:
  - [`sqlite3_db_filename`](sqlite3_db_filename.rs) - sqlite3_db_filename.rs
  - [`sqlite3_filename`](sqlite3_filename.rs) - sqlite3_filename.rs
  - [`sqlite3_filename_database`](sqlite3_filename_database.rs) - sqlite3_filename_database.rs
  - [`sqlite3_filename_journal`](sqlite3_filename_journal.rs) - sqlite3_filename_journal.rs
  - [`sqlite3_filename_wal`](sqlite3_filename_wal.rs) - sqlite3_filename_wal.rs
  - [`sqlite3_uri_boolean`](sqlite3_uri_boolean.rs) - sqlite3_uri_boolean.rs
  - [`sqlite3_uri_int64`](sqlite3_uri_int64.rs) - sqlite3_uri_int64.rs
  - [`sqlite3_uri_parameter`](sqlite3_uri_parameter.rs) - sqlite3_uri_parameter.rs

## `db_handle.rs`
Requires implementations from:
  - [`sqlite3_db_handle`](sqlite3_db_handle.rs) - sqlite3_db_handle.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `db_mutex.rs`
Requires implementations from:
  - [`sqlite3_db_mutex`](sqlite3_db_mutex.rs) - sqlite3_db_mutex.rs
  - [`sqlite3_mutex`](sqlite3_mutex.rs) - sqlite3_mutex.rs

## `db_name.rs`
Requires implementations from:
  - [`sqlite3_db_name`](sqlite3_db_name.rs) - sqlite3_db_name.rs
  - [`sqlite3_deserialize`](sqlite3_deserialize.rs) - sqlite3_deserialize.rs
  - [`sqlite3_serialize`](sqlite3_serialize.rs) - sqlite3_serialize.rs

## `db_readonly.rs`
Requires implementations from:
  - [`sqlite3_db_readonly`](sqlite3_db_readonly.rs) - sqlite3_db_readonly.rs

## `db_release_memory.rs`
Requires implementations from:
  - [`sqlite3_db_release_memory`](sqlite3_db_release_memory.rs) - sqlite3_db_release_memory.rs
  - [`sqlite3_release_memory`](sqlite3_release_memory.rs) - sqlite3_release_memory.rs

## `db_status.rs`
Requires implementations from:
  - [`sqlite3_db_status`](sqlite3_db_status.rs) - sqlite3_db_status.rs
  - [`sqlite3_status`](sqlite3_status.rs) - sqlite3_status.rs
  - [`sqlite3_stmt_status`](sqlite3_stmt_status.rs) - sqlite3_stmt_status.rs

## `declare_vtab.rs`
Requires implementations from:
  - [`sqlite3_declare_vtab`](sqlite3_declare_vtab.rs) - sqlite3_declare_vtab.rs

## `deserialize.rs`
Requires implementations from:
  - [`sqlite3_deserialize`](sqlite3_deserialize.rs) - sqlite3_deserialize.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs

## `drop_modules.rs`
Requires implementations from:
  - [`sqlite3_create_module`](sqlite3_create_module.rs) - sqlite3_create_module.rs
  - [`sqlite3_drop_modules`](sqlite3_drop_modules.rs) - sqlite3_drop_modules.rs

## `enable_load_extension.rs`
Requires implementations from:
  - [`sqlite3_db_config`](sqlite3_db_config.rs) - sqlite3_db_config.rs
  - [`sqlite3_enable_load_extension`](sqlite3_enable_load_extension.rs) - sqlite3_enable_load_extension.rs
  - [`sqlite3_load_extension`](sqlite3_load_extension.rs) - sqlite3_load_extension.rs

## `enable_shared_cache.rs`
Requires implementations from:
  - [`sqlite3_enable_shared_cache`](sqlite3_enable_shared_cache.rs) - sqlite3_enable_shared_cache.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open16`](sqlite3_open16.rs) - sqlite3_open16.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs

## `errcode.rs`
Requires implementations from:
  - [`sqlite3_`](sqlite3_.rs) - sqlite3_.rs
  - [`sqlite3_db_mutex`](sqlite3_db_mutex.rs) - sqlite3_db_mutex.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_error_offset`](sqlite3_error_offset.rs) - sqlite3_error_offset.rs
  - [`sqlite3_errstr`](sqlite3_errstr.rs) - sqlite3_errstr.rs
  - [`sqlite3_extended_errcode`](sqlite3_extended_errcode.rs) - sqlite3_extended_errcode.rs
  - [`sqlite3_mutex_enter`](sqlite3_mutex_enter.rs) - sqlite3_mutex_enter.rs
  - [`sqlite3_mutex_leave`](sqlite3_mutex_leave.rs) - sqlite3_mutex_leave.rs

## `exec.rs`
Requires implementations from:
  - [`sqlite3_column_name`](sqlite3_column_name.rs) - sqlite3_column_name.rs
  - [`sqlite3_column_text`](sqlite3_column_text.rs) - sqlite3_column_text.rs
  - [`sqlite3_exec`](sqlite3_exec.rs) - sqlite3_exec.rs
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs

## `expanded_sql.rs`
Requires implementations from:
  - [`sqlite3_expanded_sql`](sqlite3_expanded_sql.rs) - sqlite3_expanded_sql.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_normalized_sql`](sqlite3_normalized_sql.rs) - sqlite3_normalized_sql.rs
  - [`sqlite3_prepare16_v2`](sqlite3_prepare16_v2.rs) - sqlite3_prepare16_v2.rs
  - [`sqlite3_prepare16_v3`](sqlite3_prepare16_v3.rs) - sqlite3_prepare16_v3.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_prepare_v3`](sqlite3_prepare_v3.rs) - sqlite3_prepare_v3.rs
  - [`sqlite3_sql`](sqlite3_sql.rs) - sqlite3_sql.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `extended_result_codes.rs`
Requires implementations from:
  - [`sqlite3_extended_result_codes`](sqlite3_extended_result_codes.rs) - sqlite3_extended_result_codes.rs

## `file.rs`
Requires implementations from:
  - [`sqlite3_file`](sqlite3_file.rs) - sqlite3_file.rs
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs

## `file_control.rs`
Requires implementations from:
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_file`](sqlite3_file.rs) - sqlite3_file.rs
  - [`sqlite3_file_control`](sqlite3_file_control.rs) - sqlite3_file_control.rs
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs

## `filename.rs`
Requires implementations from:
  - [`sqlite3_filename`](sqlite3_filename.rs) - sqlite3_filename.rs
  - [`sqlite3_filename_database`](sqlite3_filename_database.rs) - sqlite3_filename_database.rs
  - [`sqlite3_filename_journal`](sqlite3_filename_journal.rs) - sqlite3_filename_journal.rs
  - [`sqlite3_filename_wal`](sqlite3_filename_wal.rs) - sqlite3_filename_wal.rs
  - [`sqlite3_uri_boolean`](sqlite3_uri_boolean.rs) - sqlite3_uri_boolean.rs
  - [`sqlite3_uri_key`](sqlite3_uri_key.rs) - sqlite3_uri_key.rs
  - [`sqlite3_uri_parameter`](sqlite3_uri_parameter.rs) - sqlite3_uri_parameter.rs

## `filename_database.rs`
Requires implementations from:
  - [`sqlite3_db_filename`](sqlite3_db_filename.rs) - sqlite3_db_filename.rs
  - [`sqlite3_filename`](sqlite3_filename.rs) - sqlite3_filename.rs
  - [`sqlite3_filename_database`](sqlite3_filename_database.rs) - sqlite3_filename_database.rs
  - [`sqlite3_filename_journal`](sqlite3_filename_journal.rs) - sqlite3_filename_journal.rs
  - [`sqlite3_filename_wal`](sqlite3_filename_wal.rs) - sqlite3_filename_wal.rs

## `finalize.rs`
Requires implementations from:
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `free.rs`
Requires implementations from:
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_msize`](sqlite3_msize.rs) - sqlite3_msize.rs
  - [`sqlite3_realloc`](sqlite3_realloc.rs) - sqlite3_realloc.rs

## `free_table.rs`
Requires implementations from:
  - [`sqlite3_column_text`](sqlite3_column_text.rs) - sqlite3_column_text.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_exec`](sqlite3_exec.rs) - sqlite3_exec.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_free_table`](sqlite3_free_table.rs) - sqlite3_free_table.rs
  - [`sqlite3_get_table`](sqlite3_get_table.rs) - sqlite3_get_table.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs

## `get_autocommit.rs`
Requires implementations from:
  - [`sqlite3_get_autocommit`](sqlite3_get_autocommit.rs) - sqlite3_get_autocommit.rs

## `get_auxdata.rs`
Requires implementations from:
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_get_auxdata`](sqlite3_get_auxdata.rs) - sqlite3_get_auxdata.rs
  - [`sqlite3_get_clientdata`](sqlite3_get_clientdata.rs) - sqlite3_get_clientdata.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_set_auxdata`](sqlite3_set_auxdata.rs) - sqlite3_set_auxdata.rs
  - [`sqlite3_set_clientdata`](sqlite3_set_clientdata.rs) - sqlite3_set_clientdata.rs

## `get_clientdata.rs`
Requires implementations from:
  - [`sqlite3_get_auxdata`](sqlite3_get_auxdata.rs) - sqlite3_get_auxdata.rs
  - [`sqlite3_get_clientdata`](sqlite3_get_clientdata.rs) - sqlite3_get_clientdata.rs
  - [`sqlite3_set_auxdata`](sqlite3_set_auxdata.rs) - sqlite3_set_auxdata.rs
  - [`sqlite3_set_clientdata`](sqlite3_set_clientdata.rs) - sqlite3_set_clientdata.rs

## `hard_heap_limit64.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_hard_heap_limit`](sqlite3_hard_heap_limit.rs) - sqlite3_hard_heap_limit.rs
  - [`sqlite3_soft_heap_limit`](sqlite3_soft_heap_limit.rs) - sqlite3_soft_heap_limit.rs

## `index_info.rs`
Requires implementations from:
  - [`sqlite3_declare_vtab`](sqlite3_declare_vtab.rs) - sqlite3_declare_vtab.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_index_constraint`](sqlite3_index_constraint.rs) - sqlite3_index_constraint.rs
  - [`sqlite3_index_constraint_usage`](sqlite3_index_constraint_usage.rs) - sqlite3_index_constraint_usage.rs
  - [`sqlite3_index_info`](sqlite3_index_info.rs) - sqlite3_index_info.rs
  - [`sqlite3_index_orderby`](sqlite3_index_orderby.rs) - sqlite3_index_orderby.rs
  - [`sqlite3_libversion_number`](sqlite3_libversion_number.rs) - sqlite3_libversion_number.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_vtab_collation`](sqlite3_vtab_collation.rs) - sqlite3_vtab_collation.rs
  - [`sqlite3_vtab_distinct`](sqlite3_vtab_distinct.rs) - sqlite3_vtab_distinct.rs
  - [`sqlite3_vtab_rhs_value`](sqlite3_vtab_rhs_value.rs) - sqlite3_vtab_rhs_value.rs

## `initialize.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_initialize`](sqlite3_initialize.rs) - sqlite3_initialize.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_os_end`](sqlite3_os_end.rs) - sqlite3_os_end.rs
  - [`sqlite3_os_init`](sqlite3_os_init.rs) - sqlite3_os_init.rs
  - [`sqlite3_shutdown`](sqlite3_shutdown.rs) - sqlite3_shutdown.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs

## `interrupt.rs`
Requires implementations from:
  - [`sqlite3_interrupt`](sqlite3_interrupt.rs) - sqlite3_interrupt.rs
  - [`sqlite3_is_interrupted`](sqlite3_is_interrupted.rs) - sqlite3_is_interrupted.rs

## `io_methods.rs`
Requires implementations from:
  - [`sqlite3_file`](sqlite3_file.rs) - sqlite3_file.rs
  - [`sqlite3_file_control`](sqlite3_file_control.rs) - sqlite3_file_control.rs
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs
  - [`sqlite3_vfs.xOpen`](sqlite3_vfs.xOpen.rs) - sqlite3_vfs.xOpen.rs

## `keyword_check.rs`
Requires implementations from:
  - [`sqlite3_keyword_check`](sqlite3_keyword_check.rs) - sqlite3_keyword_check.rs
  - [`sqlite3_keyword_count`](sqlite3_keyword_count.rs) - sqlite3_keyword_count.rs
  - [`sqlite3_keyword_name`](sqlite3_keyword_name.rs) - sqlite3_keyword_name.rs

## `last_insert_rowid.rs`
Requires implementations from:
  - [`sqlite3_last_insert_rowid`](sqlite3_last_insert_rowid.rs) - sqlite3_last_insert_rowid.rs
  - [`sqlite3_set_last_insert_rowid`](sqlite3_set_last_insert_rowid.rs) - sqlite3_set_last_insert_rowid.rs

## `libversion.rs`
Requires implementations from:
  - [`sqlite3_libversion`](sqlite3_libversion.rs) - sqlite3_libversion.rs
  - [`sqlite3_libversion_number`](sqlite3_libversion_number.rs) - sqlite3_libversion_number.rs
  - [`sqlite3_sourceid`](sqlite3_sourceid.rs) - sqlite3_sourceid.rs
  - [`sqlite3_version`](sqlite3_version.rs) - sqlite3_version.rs

## `limit.rs`
Requires implementations from:
  - [`sqlite3_limit`](sqlite3_limit.rs) - sqlite3_limit.rs
  - [`sqlite3_set_authorizer`](sqlite3_set_authorizer.rs) - sqlite3_set_authorizer.rs

## `load_extension.rs`
Requires implementations from:
  - [`sqlite3_db_config`](sqlite3_db_config.rs) - sqlite3_db_config.rs
  - [`sqlite3_enable_load_extension`](sqlite3_enable_load_extension.rs) - sqlite3_enable_load_extension.rs
  - [`sqlite3_extension_init`](sqlite3_extension_init.rs) - sqlite3_extension_init.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_load_extension`](sqlite3_load_extension.rs) - sqlite3_load_extension.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs

## `log.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_log`](sqlite3_log.rs) - sqlite3_log.rs
  - [`sqlite3_snprintf`](sqlite3_snprintf.rs) - sqlite3_snprintf.rs

## `mem_methods.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_mem_methods`](sqlite3_mem_methods.rs) - sqlite3_mem_methods.rs
  - [`sqlite3_realloc`](sqlite3_realloc.rs) - sqlite3_realloc.rs
  - [`sqlite3_shutdown`](sqlite3_shutdown.rs) - sqlite3_shutdown.rs

## `memory_highwater.rs`
Requires implementations from:
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_memory_highwater`](sqlite3_memory_highwater.rs) - sqlite3_memory_highwater.rs
  - [`sqlite3_memory_used`](sqlite3_memory_used.rs) - sqlite3_memory_used.rs
  - [`sqlite3_realloc`](sqlite3_realloc.rs) - sqlite3_realloc.rs

## `module.rs`
Requires implementations from:
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_create_module`](sqlite3_create_module.rs) - sqlite3_create_module.rs
  - [`sqlite3_create_module_v2`](sqlite3_create_module_v2.rs) - sqlite3_create_module_v2.rs
  - [`sqlite3_index_info`](sqlite3_index_info.rs) - sqlite3_index_info.rs
  - [`sqlite3_module`](sqlite3_module.rs) - sqlite3_module.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_vtab`](sqlite3_vtab.rs) - sqlite3_vtab.rs
  - [`sqlite3_vtab_cursor`](sqlite3_vtab_cursor.rs) - sqlite3_vtab_cursor.rs

## `mprintf.rs`
Requires implementations from:
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc64`](sqlite3_malloc64.rs) - sqlite3_malloc64.rs
  - [`sqlite3_mprintf`](sqlite3_mprintf.rs) - sqlite3_mprintf.rs
  - [`sqlite3_snprintf`](sqlite3_snprintf.rs) - sqlite3_snprintf.rs
  - [`sqlite3_vmprintf`](sqlite3_vmprintf.rs) - sqlite3_vmprintf.rs
  - [`sqlite3_vsnprintf`](sqlite3_vsnprintf.rs) - sqlite3_vsnprintf.rs

## `mutex.rs`
Requires implementations from:
  - [`sqlite3_mutex`](sqlite3_mutex.rs) - sqlite3_mutex.rs
  - [`sqlite3_mutex_alloc`](sqlite3_mutex_alloc.rs) - sqlite3_mutex_alloc.rs

## `mutex_alloc.rs`
Requires implementations from:
  - [`sqlite3_`](sqlite3_.rs) - sqlite3_.rs
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_initialize`](sqlite3_initialize.rs) - sqlite3_initialize.rs
  - [`sqlite3_mutex`](sqlite3_mutex.rs) - sqlite3_mutex.rs
  - [`sqlite3_mutex_alloc`](sqlite3_mutex_alloc.rs) - sqlite3_mutex_alloc.rs
  - [`sqlite3_mutex_enter`](sqlite3_mutex_enter.rs) - sqlite3_mutex_enter.rs
  - [`sqlite3_mutex_free`](sqlite3_mutex_free.rs) - sqlite3_mutex_free.rs
  - [`sqlite3_mutex_held`](sqlite3_mutex_held.rs) - sqlite3_mutex_held.rs
  - [`sqlite3_mutex_leave`](sqlite3_mutex_leave.rs) - sqlite3_mutex_leave.rs
  - [`sqlite3_mutex_notheld`](sqlite3_mutex_notheld.rs) - sqlite3_mutex_notheld.rs
  - [`sqlite3_mutex_try`](sqlite3_mutex_try.rs) - sqlite3_mutex_try.rs

## `mutex_held.rs`
Requires implementations from:
  - [`sqlite3_mutex`](sqlite3_mutex.rs) - sqlite3_mutex.rs
  - [`sqlite3_mutex_held`](sqlite3_mutex_held.rs) - sqlite3_mutex_held.rs
  - [`sqlite3_mutex_notheld`](sqlite3_mutex_notheld.rs) - sqlite3_mutex_notheld.rs

## `mutex_methods.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_initialize`](sqlite3_initialize.rs) - sqlite3_initialize.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_mutex`](sqlite3_mutex.rs) - sqlite3_mutex.rs
  - [`sqlite3_mutex_alloc`](sqlite3_mutex_alloc.rs) - sqlite3_mutex_alloc.rs
  - [`sqlite3_mutex_enter`](sqlite3_mutex_enter.rs) - sqlite3_mutex_enter.rs
  - [`sqlite3_mutex_free`](sqlite3_mutex_free.rs) - sqlite3_mutex_free.rs
  - [`sqlite3_mutex_held`](sqlite3_mutex_held.rs) - sqlite3_mutex_held.rs
  - [`sqlite3_mutex_leave`](sqlite3_mutex_leave.rs) - sqlite3_mutex_leave.rs
  - [`sqlite3_mutex_methods`](sqlite3_mutex_methods.rs) - sqlite3_mutex_methods.rs
  - [`sqlite3_mutex_notheld`](sqlite3_mutex_notheld.rs) - sqlite3_mutex_notheld.rs
  - [`sqlite3_mutex_try`](sqlite3_mutex_try.rs) - sqlite3_mutex_try.rs
  - [`sqlite3_shutdown`](sqlite3_shutdown.rs) - sqlite3_shutdown.rs

## `next_stmt.rs`
Requires implementations from:
  - [`sqlite3_next_stmt`](sqlite3_next_stmt.rs) - sqlite3_next_stmt.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `open.rs`
Requires implementations from:
  - [`sqlite3_close`](sqlite3_close.rs) - sqlite3_close.rs
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_db_readonly`](sqlite3_db_readonly.rs) - sqlite3_db_readonly.rs
  - [`sqlite3_enable_shared_cache`](sqlite3_enable_shared_cache.rs) - sqlite3_enable_shared_cache.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_errmsg16`](sqlite3_errmsg16.rs) - sqlite3_errmsg16.rs
  - [`sqlite3_extended_result_codes`](sqlite3_extended_result_codes.rs) - sqlite3_extended_result_codes.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_temp_directory`](sqlite3_temp_directory.rs) - sqlite3_temp_directory.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs

## `overload_function.rs`
Requires implementations from:
  - [`sqlite3_overload_function`](sqlite3_overload_function.rs) - sqlite3_overload_function.rs

## `pcache.rs`
Requires implementations from:
  - [`sqlite3_pcache`](sqlite3_pcache.rs) - sqlite3_pcache.rs
  - [`sqlite3_pcache_methods2`](sqlite3_pcache_methods2.rs) - sqlite3_pcache_methods2.rs

## `pcache_methods2.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_initialize`](sqlite3_initialize.rs) - sqlite3_initialize.rs
  - [`sqlite3_pcache`](sqlite3_pcache.rs) - sqlite3_pcache.rs
  - [`sqlite3_pcache*`](sqlite3_pcache*.rs) - sqlite3_pcache*.rs
  - [`sqlite3_pcache_page`](sqlite3_pcache_page.rs) - sqlite3_pcache_page.rs
  - [`sqlite3_shutdown`](sqlite3_shutdown.rs) - sqlite3_shutdown.rs

## `pcache_page.rs`
Requires implementations from:
  - [`sqlite3_pcache_methods2`](sqlite3_pcache_methods2.rs) - sqlite3_pcache_methods2.rs
  - [`sqlite3_pcache_page`](sqlite3_pcache_page.rs) - sqlite3_pcache_page.rs

## `prepare.rs`
Requires implementations from:
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open16`](sqlite3_open16.rs) - sqlite3_open16.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_prepare`](sqlite3_prepare.rs) - sqlite3_prepare.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_prepare_v3`](sqlite3_prepare_v3.rs) - sqlite3_prepare_v3.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `preupdate_blobwrite.rs`
Requires implementations from:
  - [`sqlite3_blob_write`](sqlite3_blob_write.rs) - sqlite3_blob_write.rs
  - [`sqlite3_preupdate_blobwrite`](sqlite3_preupdate_blobwrite.rs) - sqlite3_preupdate_blobwrite.rs
  - [`sqlite3_preupdate_count`](sqlite3_preupdate_count.rs) - sqlite3_preupdate_count.rs
  - [`sqlite3_preupdate_depth`](sqlite3_preupdate_depth.rs) - sqlite3_preupdate_depth.rs
  - [`sqlite3_preupdate_hook`](sqlite3_preupdate_hook.rs) - sqlite3_preupdate_hook.rs
  - [`sqlite3_preupdate_new`](sqlite3_preupdate_new.rs) - sqlite3_preupdate_new.rs
  - [`sqlite3_preupdate_old`](sqlite3_preupdate_old.rs) - sqlite3_preupdate_old.rs
  - [`sqlite3_update_hook`](sqlite3_update_hook.rs) - sqlite3_update_hook.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs

## `profile.rs`
Requires implementations from:
  - [`sqlite3_profile`](sqlite3_profile.rs) - sqlite3_profile.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_trace`](sqlite3_trace.rs) - sqlite3_trace.rs
  - [`sqlite3_trace_v2`](sqlite3_trace_v2.rs) - sqlite3_trace_v2.rs

## `progress_handler.rs`
Requires implementations from:
  - [`sqlite3_prepare`](sqlite3_prepare.rs) - sqlite3_prepare.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_progress_handler`](sqlite3_progress_handler.rs) - sqlite3_progress_handler.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs

## `randomness.rs`
Requires implementations from:
  - [`sqlite3_randomness`](sqlite3_randomness.rs) - sqlite3_randomness.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs

## `release_memory.rs`
Requires implementations from:
  - [`sqlite3_db_release_memory`](sqlite3_db_release_memory.rs) - sqlite3_db_release_memory.rs
  - [`sqlite3_release_memory`](sqlite3_release_memory.rs) - sqlite3_release_memory.rs

## `reset.rs`
Requires implementations from:
  - [`sqlite3_bind_`](sqlite3_bind_.rs) - sqlite3_bind_.rs
  - [`sqlite3_bind_*`](sqlite3_bind_*.rs) - sqlite3_bind_*.rs
  - [`sqlite3_clear_bindings`](sqlite3_clear_bindings.rs) - sqlite3_clear_bindings.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `reset_auto_extension.rs`
Requires implementations from:
  - [`sqlite3_auto_extension`](sqlite3_auto_extension.rs) - sqlite3_auto_extension.rs
  - [`sqlite3_reset_auto_extension`](sqlite3_reset_auto_extension.rs) - sqlite3_reset_auto_extension.rs

## `result_blob.rs`
Requires implementations from:
  - [`sqlite3_bind_text16`](sqlite3_bind_text16.rs) - sqlite3_bind_text16.rs
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_create_function`](sqlite3_create_function.rs) - sqlite3_create_function.rs
  - [`sqlite3_create_function16`](sqlite3_create_function16.rs) - sqlite3_create_function16.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_result_blob`](sqlite3_result_blob.rs) - sqlite3_result_blob.rs
  - [`sqlite3_result_double`](sqlite3_result_double.rs) - sqlite3_result_double.rs
  - [`sqlite3_result_error`](sqlite3_result_error.rs) - sqlite3_result_error.rs
  - [`sqlite3_result_error_code`](sqlite3_result_error_code.rs) - sqlite3_result_error_code.rs
  - [`sqlite3_result_error_nomem`](sqlite3_result_error_nomem.rs) - sqlite3_result_error_nomem.rs
  - [`sqlite3_result_error_toobig`](sqlite3_result_error_toobig.rs) - sqlite3_result_error_toobig.rs
  - [`sqlite3_result_int`](sqlite3_result_int.rs) - sqlite3_result_int.rs
  - [`sqlite3_result_null`](sqlite3_result_null.rs) - sqlite3_result_null.rs
  - [`sqlite3_result_pointer`](sqlite3_result_pointer.rs) - sqlite3_result_pointer.rs
  - [`sqlite3_result_text`](sqlite3_result_text.rs) - sqlite3_result_text.rs
  - [`sqlite3_result_value`](sqlite3_result_value.rs) - sqlite3_result_value.rs
  - [`sqlite3_result_zeroblob`](sqlite3_result_zeroblob.rs) - sqlite3_result_zeroblob.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_value_pointer`](sqlite3_value_pointer.rs) - sqlite3_value_pointer.rs

## `result_subtype.rs`
Requires implementations from:
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_result_subtype`](sqlite3_result_subtype.rs) - sqlite3_result_subtype.rs

## `serialize.rs`
Requires implementations from:
  - [`sqlite3_deserialize`](sqlite3_deserialize.rs) - sqlite3_deserialize.rs
  - [`sqlite3_malloc64`](sqlite3_malloc64.rs) - sqlite3_malloc64.rs
  - [`sqlite3_serialize`](sqlite3_serialize.rs) - sqlite3_serialize.rs

## `set_authorizer.rs`
Requires implementations from:
  - [`sqlite3_limit`](sqlite3_limit.rs) - sqlite3_limit.rs
  - [`sqlite3_prepare`](sqlite3_prepare.rs) - sqlite3_prepare.rs
  - [`sqlite3_prepare16`](sqlite3_prepare16.rs) - sqlite3_prepare16.rs
  - [`sqlite3_prepare16_v2`](sqlite3_prepare16_v2.rs) - sqlite3_prepare16_v2.rs
  - [`sqlite3_prepare16_v3`](sqlite3_prepare16_v3.rs) - sqlite3_prepare16_v3.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_prepare_v3`](sqlite3_prepare_v3.rs) - sqlite3_prepare_v3.rs
  - [`sqlite3_set_authorizer`](sqlite3_set_authorizer.rs) - sqlite3_set_authorizer.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs

## `set_errmsg.rs`
Requires implementations from:
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_set_errmsg`](sqlite3_set_errmsg.rs) - sqlite3_set_errmsg.rs

## `set_last_insert_rowid.rs`
Requires implementations from:
  - [`sqlite3_last_insert_rowid`](sqlite3_last_insert_rowid.rs) - sqlite3_last_insert_rowid.rs
  - [`sqlite3_set_last_insert_rowid`](sqlite3_set_last_insert_rowid.rs) - sqlite3_set_last_insert_rowid.rs

## `setlk_timeout.rs`
Requires implementations from:
  - [`sqlite3_busy_timeout`](sqlite3_busy_timeout.rs) - sqlite3_busy_timeout.rs
  - [`sqlite3_setlk_timeout`](sqlite3_setlk_timeout.rs) - sqlite3_setlk_timeout.rs

## `sleep.rs`
Requires implementations from:
  - [`sqlite3_sleep`](sqlite3_sleep.rs) - sqlite3_sleep.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs

## `snapshot.rs`
Requires implementations from:
  - [`sqlite3_snapshot`](sqlite3_snapshot.rs) - sqlite3_snapshot.rs
  - [`sqlite3_snapshot_cmp`](sqlite3_snapshot_cmp.rs) - sqlite3_snapshot_cmp.rs
  - [`sqlite3_snapshot_free`](sqlite3_snapshot_free.rs) - sqlite3_snapshot_free.rs
  - [`sqlite3_snapshot_get`](sqlite3_snapshot_get.rs) - sqlite3_snapshot_get.rs
  - [`sqlite3_snapshot_open`](sqlite3_snapshot_open.rs) - sqlite3_snapshot_open.rs
  - [`sqlite3_snapshot_recover`](sqlite3_snapshot_recover.rs) - sqlite3_snapshot_recover.rs

## `snapshot_cmp.rs`
Requires implementations from:
  - [`sqlite3_snapshot`](sqlite3_snapshot.rs) - sqlite3_snapshot.rs
  - [`sqlite3_snapshot_cmp`](sqlite3_snapshot_cmp.rs) - sqlite3_snapshot_cmp.rs
  - [`sqlite3_snapshot_get`](sqlite3_snapshot_get.rs) - sqlite3_snapshot_get.rs

## `snapshot_free.rs`
Requires implementations from:
  - [`sqlite3_snapshot`](sqlite3_snapshot.rs) - sqlite3_snapshot.rs
  - [`sqlite3_snapshot_free`](sqlite3_snapshot_free.rs) - sqlite3_snapshot_free.rs

## `snapshot_get.rs`
Requires implementations from:
  - [`sqlite3_snapshot`](sqlite3_snapshot.rs) - sqlite3_snapshot.rs
  - [`sqlite3_snapshot_free`](sqlite3_snapshot_free.rs) - sqlite3_snapshot_free.rs
  - [`sqlite3_snapshot_get`](sqlite3_snapshot_get.rs) - sqlite3_snapshot_get.rs

## `snapshot_open.rs`
Requires implementations from:
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_snapshot`](sqlite3_snapshot.rs) - sqlite3_snapshot.rs
  - [`sqlite3_snapshot_open`](sqlite3_snapshot_open.rs) - sqlite3_snapshot_open.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs

## `snapshot_recover.rs`
Requires implementations from:
  - [`sqlite3_close`](sqlite3_close.rs) - sqlite3_close.rs
  - [`sqlite3_snapshot_open`](sqlite3_snapshot_open.rs) - sqlite3_snapshot_open.rs
  - [`sqlite3_snapshot_recover`](sqlite3_snapshot_recover.rs) - sqlite3_snapshot_recover.rs

## `soft_heap_limit.rs`
Requires implementations from:
  - [`sqlite3_soft_heap_limit`](sqlite3_soft_heap_limit.rs) - sqlite3_soft_heap_limit.rs
  - [`sqlite3_soft_heap_limit64`](sqlite3_soft_heap_limit64.rs) - sqlite3_soft_heap_limit64.rs

## `sqlite3.rs`
Requires implementations from:
  - [`sqlite3_autovacuum_pages`](sqlite3_autovacuum_pages.rs) - sqlite3_autovacuum_pages.rs
  - [`sqlite3_blob_open`](sqlite3_blob_open.rs) - sqlite3_blob_open.rs
  - [`sqlite3_busy_handler`](sqlite3_busy_handler.rs) - sqlite3_busy_handler.rs
  - [`sqlite3_busy_timeout`](sqlite3_busy_timeout.rs) - sqlite3_busy_timeout.rs
  - [`sqlite3_changes`](sqlite3_changes.rs) - sqlite3_changes.rs
  - [`sqlite3_changes64`](sqlite3_changes64.rs) - sqlite3_changes64.rs
  - [`sqlite3_close`](sqlite3_close.rs) - sqlite3_close.rs
  - [`sqlite3_close_v2`](sqlite3_close_v2.rs) - sqlite3_close_v2.rs
  - [`sqlite3_collation_needed`](sqlite3_collation_needed.rs) - sqlite3_collation_needed.rs
  - [`sqlite3_collation_needed16`](sqlite3_collation_needed16.rs) - sqlite3_collation_needed16.rs
  - [`sqlite3_commit_hook`](sqlite3_commit_hook.rs) - sqlite3_commit_hook.rs
  - [`sqlite3_create_collation`](sqlite3_create_collation.rs) - sqlite3_create_collation.rs
  - [`sqlite3_create_collation16`](sqlite3_create_collation16.rs) - sqlite3_create_collation16.rs
  - [`sqlite3_create_collation_v2`](sqlite3_create_collation_v2.rs) - sqlite3_create_collation_v2.rs
  - [`sqlite3_create_function`](sqlite3_create_function.rs) - sqlite3_create_function.rs
  - [`sqlite3_create_function16`](sqlite3_create_function16.rs) - sqlite3_create_function16.rs
  - [`sqlite3_create_function_v2`](sqlite3_create_function_v2.rs) - sqlite3_create_function_v2.rs
  - [`sqlite3_create_module`](sqlite3_create_module.rs) - sqlite3_create_module.rs
  - [`sqlite3_create_module_v2`](sqlite3_create_module_v2.rs) - sqlite3_create_module_v2.rs
  - [`sqlite3_create_window_function`](sqlite3_create_window_function.rs) - sqlite3_create_window_function.rs
  - [`sqlite3_db_cacheflush`](sqlite3_db_cacheflush.rs) - sqlite3_db_cacheflush.rs
  - [`sqlite3_db_config`](sqlite3_db_config.rs) - sqlite3_db_config.rs
  - [`sqlite3_db_filename`](sqlite3_db_filename.rs) - sqlite3_db_filename.rs
  - [`sqlite3_db_mutex`](sqlite3_db_mutex.rs) - sqlite3_db_mutex.rs
  - [`sqlite3_db_name`](sqlite3_db_name.rs) - sqlite3_db_name.rs
  - [`sqlite3_db_readonly`](sqlite3_db_readonly.rs) - sqlite3_db_readonly.rs
  - [`sqlite3_db_release_memory`](sqlite3_db_release_memory.rs) - sqlite3_db_release_memory.rs
  - [`sqlite3_db_status`](sqlite3_db_status.rs) - sqlite3_db_status.rs
  - [`sqlite3_db_status64`](sqlite3_db_status64.rs) - sqlite3_db_status64.rs
  - [`sqlite3_drop_modules`](sqlite3_drop_modules.rs) - sqlite3_drop_modules.rs
  - [`sqlite3_enable_load_extension`](sqlite3_enable_load_extension.rs) - sqlite3_enable_load_extension.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_errmsg16`](sqlite3_errmsg16.rs) - sqlite3_errmsg16.rs
  - [`sqlite3_error_offset`](sqlite3_error_offset.rs) - sqlite3_error_offset.rs
  - [`sqlite3_errstr`](sqlite3_errstr.rs) - sqlite3_errstr.rs
  - [`sqlite3_exec`](sqlite3_exec.rs) - sqlite3_exec.rs
  - [`sqlite3_extended_errcode`](sqlite3_extended_errcode.rs) - sqlite3_extended_errcode.rs
  - [`sqlite3_extended_result_codes`](sqlite3_extended_result_codes.rs) - sqlite3_extended_result_codes.rs
  - [`sqlite3_file_control`](sqlite3_file_control.rs) - sqlite3_file_control.rs
  - [`sqlite3_free_table`](sqlite3_free_table.rs) - sqlite3_free_table.rs
  - [`sqlite3_get_autocommit`](sqlite3_get_autocommit.rs) - sqlite3_get_autocommit.rs
  - [`sqlite3_get_clientdata`](sqlite3_get_clientdata.rs) - sqlite3_get_clientdata.rs
  - [`sqlite3_get_table`](sqlite3_get_table.rs) - sqlite3_get_table.rs
  - [`sqlite3_interrupt`](sqlite3_interrupt.rs) - sqlite3_interrupt.rs
  - [`sqlite3_is_interrupted`](sqlite3_is_interrupted.rs) - sqlite3_is_interrupted.rs
  - [`sqlite3_last_insert_rowid`](sqlite3_last_insert_rowid.rs) - sqlite3_last_insert_rowid.rs
  - [`sqlite3_limit`](sqlite3_limit.rs) - sqlite3_limit.rs
  - [`sqlite3_load_extension`](sqlite3_load_extension.rs) - sqlite3_load_extension.rs
  - [`sqlite3_next_stmt`](sqlite3_next_stmt.rs) - sqlite3_next_stmt.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open16`](sqlite3_open16.rs) - sqlite3_open16.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_overload_function`](sqlite3_overload_function.rs) - sqlite3_overload_function.rs
  - [`sqlite3_prepare`](sqlite3_prepare.rs) - sqlite3_prepare.rs
  - [`sqlite3_prepare16`](sqlite3_prepare16.rs) - sqlite3_prepare16.rs
  - [`sqlite3_prepare16_v2`](sqlite3_prepare16_v2.rs) - sqlite3_prepare16_v2.rs
  - [`sqlite3_prepare16_v3`](sqlite3_prepare16_v3.rs) - sqlite3_prepare16_v3.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_prepare_v3`](sqlite3_prepare_v3.rs) - sqlite3_prepare_v3.rs
  - [`sqlite3_preupdate_blobwrite`](sqlite3_preupdate_blobwrite.rs) - sqlite3_preupdate_blobwrite.rs
  - [`sqlite3_preupdate_count`](sqlite3_preupdate_count.rs) - sqlite3_preupdate_count.rs
  - [`sqlite3_preupdate_depth`](sqlite3_preupdate_depth.rs) - sqlite3_preupdate_depth.rs
  - [`sqlite3_preupdate_hook`](sqlite3_preupdate_hook.rs) - sqlite3_preupdate_hook.rs
  - [`sqlite3_preupdate_new`](sqlite3_preupdate_new.rs) - sqlite3_preupdate_new.rs
  - [`sqlite3_preupdate_old`](sqlite3_preupdate_old.rs) - sqlite3_preupdate_old.rs
  - [`sqlite3_progress_handler`](sqlite3_progress_handler.rs) - sqlite3_progress_handler.rs
  - [`sqlite3_rollback_hook`](sqlite3_rollback_hook.rs) - sqlite3_rollback_hook.rs
  - [`sqlite3_set_authorizer`](sqlite3_set_authorizer.rs) - sqlite3_set_authorizer.rs
  - [`sqlite3_set_clientdata`](sqlite3_set_clientdata.rs) - sqlite3_set_clientdata.rs
  - [`sqlite3_set_errmsg`](sqlite3_set_errmsg.rs) - sqlite3_set_errmsg.rs
  - [`sqlite3_set_last_insert_rowid`](sqlite3_set_last_insert_rowid.rs) - sqlite3_set_last_insert_rowid.rs
  - [`sqlite3_setlk_timeout`](sqlite3_setlk_timeout.rs) - sqlite3_setlk_timeout.rs
  - [`sqlite3_system_errno`](sqlite3_system_errno.rs) - sqlite3_system_errno.rs
  - [`sqlite3_table_column_metadata`](sqlite3_table_column_metadata.rs) - sqlite3_table_column_metadata.rs
  - [`sqlite3_total_changes`](sqlite3_total_changes.rs) - sqlite3_total_changes.rs
  - [`sqlite3_total_changes64`](sqlite3_total_changes64.rs) - sqlite3_total_changes64.rs
  - [`sqlite3_trace_v2`](sqlite3_trace_v2.rs) - sqlite3_trace_v2.rs
  - [`sqlite3_txn_state`](sqlite3_txn_state.rs) - sqlite3_txn_state.rs
  - [`sqlite3_unlock_notify`](sqlite3_unlock_notify.rs) - sqlite3_unlock_notify.rs
  - [`sqlite3_update_hook`](sqlite3_update_hook.rs) - sqlite3_update_hook.rs
  - [`sqlite3_wal_autocheckpoint`](sqlite3_wal_autocheckpoint.rs) - sqlite3_wal_autocheckpoint.rs
  - [`sqlite3_wal_checkpoint`](sqlite3_wal_checkpoint.rs) - sqlite3_wal_checkpoint.rs
  - [`sqlite3_wal_checkpoint_v2`](sqlite3_wal_checkpoint_v2.rs) - sqlite3_wal_checkpoint_v2.rs
  - [`sqlite3_wal_hook`](sqlite3_wal_hook.rs) - sqlite3_wal_hook.rs

## `status.rs`
Requires implementations from:
  - [`sqlite3_db_status`](sqlite3_db_status.rs) - sqlite3_db_status.rs
  - [`sqlite3_status`](sqlite3_status.rs) - sqlite3_status.rs

## `step.rs`
Requires implementations from:
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_prepare`](sqlite3_prepare.rs) - sqlite3_prepare.rs
  - [`sqlite3_prepare16`](sqlite3_prepare16.rs) - sqlite3_prepare16.rs
  - [`sqlite3_prepare16_v2`](sqlite3_prepare16_v2.rs) - sqlite3_prepare16_v2.rs
  - [`sqlite3_prepare16_v3`](sqlite3_prepare16_v3.rs) - sqlite3_prepare16_v3.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_prepare_v3`](sqlite3_prepare_v3.rs) - sqlite3_prepare_v3.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs

## `stmt.rs`
Requires implementations from:
  - [`sqlite3_bind_`](sqlite3_bind_.rs) - sqlite3_bind_.rs
  - [`sqlite3_bind_blob`](sqlite3_bind_blob.rs) - sqlite3_bind_blob.rs
  - [`sqlite3_bind_blob64`](sqlite3_bind_blob64.rs) - sqlite3_bind_blob64.rs
  - [`sqlite3_bind_double`](sqlite3_bind_double.rs) - sqlite3_bind_double.rs
  - [`sqlite3_bind_int`](sqlite3_bind_int.rs) - sqlite3_bind_int.rs
  - [`sqlite3_bind_int64`](sqlite3_bind_int64.rs) - sqlite3_bind_int64.rs
  - [`sqlite3_bind_null`](sqlite3_bind_null.rs) - sqlite3_bind_null.rs
  - [`sqlite3_bind_parameter_count`](sqlite3_bind_parameter_count.rs) - sqlite3_bind_parameter_count.rs
  - [`sqlite3_bind_parameter_index`](sqlite3_bind_parameter_index.rs) - sqlite3_bind_parameter_index.rs
  - [`sqlite3_bind_parameter_name`](sqlite3_bind_parameter_name.rs) - sqlite3_bind_parameter_name.rs
  - [`sqlite3_bind_pointer`](sqlite3_bind_pointer.rs) - sqlite3_bind_pointer.rs
  - [`sqlite3_bind_text`](sqlite3_bind_text.rs) - sqlite3_bind_text.rs
  - [`sqlite3_bind_text16`](sqlite3_bind_text16.rs) - sqlite3_bind_text16.rs
  - [`sqlite3_bind_text64`](sqlite3_bind_text64.rs) - sqlite3_bind_text64.rs
  - [`sqlite3_bind_value`](sqlite3_bind_value.rs) - sqlite3_bind_value.rs
  - [`sqlite3_bind_zeroblob`](sqlite3_bind_zeroblob.rs) - sqlite3_bind_zeroblob.rs
  - [`sqlite3_bind_zeroblob64`](sqlite3_bind_zeroblob64.rs) - sqlite3_bind_zeroblob64.rs
  - [`sqlite3_clear_bindings`](sqlite3_clear_bindings.rs) - sqlite3_clear_bindings.rs
  - [`sqlite3_column_blob`](sqlite3_column_blob.rs) - sqlite3_column_blob.rs
  - [`sqlite3_column_bytes`](sqlite3_column_bytes.rs) - sqlite3_column_bytes.rs
  - [`sqlite3_column_bytes16`](sqlite3_column_bytes16.rs) - sqlite3_column_bytes16.rs
  - [`sqlite3_column_count`](sqlite3_column_count.rs) - sqlite3_column_count.rs
  - [`sqlite3_column_database_name`](sqlite3_column_database_name.rs) - sqlite3_column_database_name.rs
  - [`sqlite3_column_database_name16`](sqlite3_column_database_name16.rs) - sqlite3_column_database_name16.rs
  - [`sqlite3_column_decltype`](sqlite3_column_decltype.rs) - sqlite3_column_decltype.rs
  - [`sqlite3_column_decltype16`](sqlite3_column_decltype16.rs) - sqlite3_column_decltype16.rs
  - [`sqlite3_column_double`](sqlite3_column_double.rs) - sqlite3_column_double.rs
  - [`sqlite3_column_int`](sqlite3_column_int.rs) - sqlite3_column_int.rs
  - [`sqlite3_column_int64`](sqlite3_column_int64.rs) - sqlite3_column_int64.rs
  - [`sqlite3_column_name`](sqlite3_column_name.rs) - sqlite3_column_name.rs
  - [`sqlite3_column_name16`](sqlite3_column_name16.rs) - sqlite3_column_name16.rs
  - [`sqlite3_column_origin_name`](sqlite3_column_origin_name.rs) - sqlite3_column_origin_name.rs
  - [`sqlite3_column_origin_name16`](sqlite3_column_origin_name16.rs) - sqlite3_column_origin_name16.rs
  - [`sqlite3_column_table_name`](sqlite3_column_table_name.rs) - sqlite3_column_table_name.rs
  - [`sqlite3_column_table_name16`](sqlite3_column_table_name16.rs) - sqlite3_column_table_name16.rs
  - [`sqlite3_column_text`](sqlite3_column_text.rs) - sqlite3_column_text.rs
  - [`sqlite3_column_text16`](sqlite3_column_text16.rs) - sqlite3_column_text16.rs
  - [`sqlite3_column_type`](sqlite3_column_type.rs) - sqlite3_column_type.rs
  - [`sqlite3_column_value`](sqlite3_column_value.rs) - sqlite3_column_value.rs
  - [`sqlite3_data_count`](sqlite3_data_count.rs) - sqlite3_data_count.rs
  - [`sqlite3_db_handle`](sqlite3_db_handle.rs) - sqlite3_db_handle.rs
  - [`sqlite3_expanded_sql`](sqlite3_expanded_sql.rs) - sqlite3_expanded_sql.rs
  - [`sqlite3_finalize`](sqlite3_finalize.rs) - sqlite3_finalize.rs
  - [`sqlite3_normalized_sql`](sqlite3_normalized_sql.rs) - sqlite3_normalized_sql.rs
  - [`sqlite3_prepare`](sqlite3_prepare.rs) - sqlite3_prepare.rs
  - [`sqlite3_prepare16`](sqlite3_prepare16.rs) - sqlite3_prepare16.rs
  - [`sqlite3_prepare16_v2`](sqlite3_prepare16_v2.rs) - sqlite3_prepare16_v2.rs
  - [`sqlite3_prepare16_v3`](sqlite3_prepare16_v3.rs) - sqlite3_prepare16_v3.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_prepare_v3`](sqlite3_prepare_v3.rs) - sqlite3_prepare_v3.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_sql`](sqlite3_sql.rs) - sqlite3_sql.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt_busy`](sqlite3_stmt_busy.rs) - sqlite3_stmt_busy.rs
  - [`sqlite3_stmt_explain`](sqlite3_stmt_explain.rs) - sqlite3_stmt_explain.rs
  - [`sqlite3_stmt_isexplain`](sqlite3_stmt_isexplain.rs) - sqlite3_stmt_isexplain.rs
  - [`sqlite3_stmt_readonly`](sqlite3_stmt_readonly.rs) - sqlite3_stmt_readonly.rs
  - [`sqlite3_stmt_scanstatus`](sqlite3_stmt_scanstatus.rs) - sqlite3_stmt_scanstatus.rs
  - [`sqlite3_stmt_scanstatus_reset`](sqlite3_stmt_scanstatus_reset.rs) - sqlite3_stmt_scanstatus_reset.rs
  - [`sqlite3_stmt_scanstatus_v2`](sqlite3_stmt_scanstatus_v2.rs) - sqlite3_stmt_scanstatus_v2.rs
  - [`sqlite3_stmt_status`](sqlite3_stmt_status.rs) - sqlite3_stmt_status.rs

## `stmt_busy.rs`
Requires implementations from:
  - [`sqlite3_next_stmt`](sqlite3_next_stmt.rs) - sqlite3_next_stmt.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt_busy`](sqlite3_stmt_busy.rs) - sqlite3_stmt_busy.rs

## `stmt_explain.rs`
Requires implementations from:
  - [`sqlite3_prepare`](sqlite3_prepare.rs) - sqlite3_prepare.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_prepare_v3`](sqlite3_prepare_v3.rs) - sqlite3_prepare_v3.rs
  - [`sqlite3_reset`](sqlite3_reset.rs) - sqlite3_reset.rs
  - [`sqlite3_sql`](sqlite3_sql.rs) - sqlite3_sql.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt_explain`](sqlite3_stmt_explain.rs) - sqlite3_stmt_explain.rs

## `stmt_isexplain.rs`
Requires implementations from:
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt_isexplain`](sqlite3_stmt_isexplain.rs) - sqlite3_stmt_isexplain.rs

## `stmt_readonly.rs`
Requires implementations from:
  - [`sqlite3_exec`](sqlite3_exec.rs) - sqlite3_exec.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt_readonly`](sqlite3_stmt_readonly.rs) - sqlite3_stmt_readonly.rs

## `stmt_scanstatus.rs`
Requires implementations from:
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt_scanstatus`](sqlite3_stmt_scanstatus.rs) - sqlite3_stmt_scanstatus.rs
  - [`sqlite3_stmt_scanstatus_reset`](sqlite3_stmt_scanstatus_reset.rs) - sqlite3_stmt_scanstatus_reset.rs

## `stmt_scanstatus_reset.rs`
Requires implementations from:
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt_scanstatus`](sqlite3_stmt_scanstatus.rs) - sqlite3_stmt_scanstatus.rs
  - [`sqlite3_stmt_scanstatus_reset`](sqlite3_stmt_scanstatus_reset.rs) - sqlite3_stmt_scanstatus_reset.rs

## `stmt_status.rs`
Requires implementations from:
  - [`sqlite3_db_status`](sqlite3_db_status.rs) - sqlite3_db_status.rs
  - [`sqlite3_status`](sqlite3_status.rs) - sqlite3_status.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt_status`](sqlite3_stmt_status.rs) - sqlite3_stmt_status.rs

## `str.rs`
Requires implementations from:
  - [`sqlite3_str`](sqlite3_str.rs) - sqlite3_str.rs
  - [`sqlite3_str_append`](sqlite3_str_append.rs) - sqlite3_str_append.rs
  - [`sqlite3_str_appendall`](sqlite3_str_appendall.rs) - sqlite3_str_appendall.rs
  - [`sqlite3_str_appendchar`](sqlite3_str_appendchar.rs) - sqlite3_str_appendchar.rs
  - [`sqlite3_str_appendf`](sqlite3_str_appendf.rs) - sqlite3_str_appendf.rs
  - [`sqlite3_str_errcode`](sqlite3_str_errcode.rs) - sqlite3_str_errcode.rs
  - [`sqlite3_str_finish`](sqlite3_str_finish.rs) - sqlite3_str_finish.rs
  - [`sqlite3_str_length`](sqlite3_str_length.rs) - sqlite3_str_length.rs
  - [`sqlite3_str_new`](sqlite3_str_new.rs) - sqlite3_str_new.rs
  - [`sqlite3_str_reset`](sqlite3_str_reset.rs) - sqlite3_str_reset.rs
  - [`sqlite3_str_value`](sqlite3_str_value.rs) - sqlite3_str_value.rs
  - [`sqlite3_str_vappendf`](sqlite3_str_vappendf.rs) - sqlite3_str_vappendf.rs

## `str_append.rs`
Requires implementations from:
  - [`sqlite3_str`](sqlite3_str.rs) - sqlite3_str.rs
  - [`sqlite3_str_append`](sqlite3_str_append.rs) - sqlite3_str_append.rs
  - [`sqlite3_str_appendall`](sqlite3_str_appendall.rs) - sqlite3_str_appendall.rs
  - [`sqlite3_str_appendchar`](sqlite3_str_appendchar.rs) - sqlite3_str_appendchar.rs
  - [`sqlite3_str_appendf`](sqlite3_str_appendf.rs) - sqlite3_str_appendf.rs
  - [`sqlite3_str_errcode`](sqlite3_str_errcode.rs) - sqlite3_str_errcode.rs
  - [`sqlite3_str_new`](sqlite3_str_new.rs) - sqlite3_str_new.rs
  - [`sqlite3_str_reset`](sqlite3_str_reset.rs) - sqlite3_str_reset.rs
  - [`sqlite3_str_vappendf`](sqlite3_str_vappendf.rs) - sqlite3_str_vappendf.rs

## `str_errcode.rs`
Requires implementations from:
  - [`sqlite3_str`](sqlite3_str.rs) - sqlite3_str.rs
  - [`sqlite3_str_errcode`](sqlite3_str_errcode.rs) - sqlite3_str_errcode.rs
  - [`sqlite3_str_length`](sqlite3_str_length.rs) - sqlite3_str_length.rs
  - [`sqlite3_str_value`](sqlite3_str_value.rs) - sqlite3_str_value.rs

## `str_finish.rs`
Requires implementations from:
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc64`](sqlite3_malloc64.rs) - sqlite3_malloc64.rs
  - [`sqlite3_str`](sqlite3_str.rs) - sqlite3_str.rs
  - [`sqlite3_str_finish`](sqlite3_str_finish.rs) - sqlite3_str_finish.rs

## `str_new.rs`
Requires implementations from:
  - [`sqlite3_limit`](sqlite3_limit.rs) - sqlite3_limit.rs
  - [`sqlite3_str`](sqlite3_str.rs) - sqlite3_str.rs
  - [`sqlite3_str_errcode`](sqlite3_str_errcode.rs) - sqlite3_str_errcode.rs
  - [`sqlite3_str_finish`](sqlite3_str_finish.rs) - sqlite3_str_finish.rs
  - [`sqlite3_str_length`](sqlite3_str_length.rs) - sqlite3_str_length.rs
  - [`sqlite3_str_new`](sqlite3_str_new.rs) - sqlite3_str_new.rs

## `strglob.rs`
Requires implementations from:
  - [`sqlite3_strglob`](sqlite3_strglob.rs) - sqlite3_strglob.rs
  - [`sqlite3_stricmp`](sqlite3_stricmp.rs) - sqlite3_stricmp.rs
  - [`sqlite3_strlike`](sqlite3_strlike.rs) - sqlite3_strlike.rs
  - [`sqlite3_strnicmp`](sqlite3_strnicmp.rs) - sqlite3_strnicmp.rs

## `stricmp.rs`
Requires implementations from:
  - [`sqlite3_stricmp`](sqlite3_stricmp.rs) - sqlite3_stricmp.rs
  - [`sqlite3_strnicmp`](sqlite3_strnicmp.rs) - sqlite3_strnicmp.rs

## `strlike.rs`
Requires implementations from:
  - [`sqlite3_strglob`](sqlite3_strglob.rs) - sqlite3_strglob.rs
  - [`sqlite3_stricmp`](sqlite3_stricmp.rs) - sqlite3_stricmp.rs
  - [`sqlite3_strlike`](sqlite3_strlike.rs) - sqlite3_strlike.rs
  - [`sqlite3_strnicmp`](sqlite3_strnicmp.rs) - sqlite3_strnicmp.rs

## `system_errno.rs`
Requires implementations from:
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_system_errno`](sqlite3_system_errno.rs) - sqlite3_system_errno.rs

## `table_column_metadata.rs`
Requires implementations from:
  - [`sqlite3_table_column_metadata`](sqlite3_table_column_metadata.rs) - sqlite3_table_column_metadata.rs

## `temp_directory.rs`
Requires implementations from:
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_mprintf`](sqlite3_mprintf.rs) - sqlite3_mprintf.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_temp_directory`](sqlite3_temp_directory.rs) - sqlite3_temp_directory.rs

## `test_control.rs`
Requires implementations from:
  - [`sqlite3_test_control`](sqlite3_test_control.rs) - sqlite3_test_control.rs

## `threadsafe.rs`
Requires implementations from:
  - [`sqlite3_config`](sqlite3_config.rs) - sqlite3_config.rs
  - [`sqlite3_threadsafe`](sqlite3_threadsafe.rs) - sqlite3_threadsafe.rs

## `total_changes.rs`
Requires implementations from:
  - [`sqlite3_changes`](sqlite3_changes.rs) - sqlite3_changes.rs
  - [`sqlite3_total_changes`](sqlite3_total_changes.rs) - sqlite3_total_changes.rs

## `trace_v2.rs`
Requires implementations from:
  - [`sqlite3_profile`](sqlite3_profile.rs) - sqlite3_profile.rs
  - [`sqlite3_trace`](sqlite3_trace.rs) - sqlite3_trace.rs

## `txn_state.rs`
Requires implementations from:
  - [`sqlite3_txn_state`](sqlite3_txn_state.rs) - sqlite3_txn_state.rs

## `unlock_notify.rs`
Requires implementations from:
  - [`sqlite3_close`](sqlite3_close.rs) - sqlite3_close.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_unlock_notify`](sqlite3_unlock_notify.rs) - sqlite3_unlock_notify.rs
  - [`sqlite3_xxx`](sqlite3_xxx.rs) - sqlite3_xxx.rs

## `update_hook.rs`
Requires implementations from:
  - [`sqlite3_commit_hook`](sqlite3_commit_hook.rs) - sqlite3_commit_hook.rs
  - [`sqlite3_prepare_v2`](sqlite3_prepare_v2.rs) - sqlite3_prepare_v2.rs
  - [`sqlite3_preupdate_hook`](sqlite3_preupdate_hook.rs) - sqlite3_preupdate_hook.rs
  - [`sqlite3_rollback_hook`](sqlite3_rollback_hook.rs) - sqlite3_rollback_hook.rs
  - [`sqlite3_step`](sqlite3_step.rs) - sqlite3_step.rs
  - [`sqlite3_update_hook`](sqlite3_update_hook.rs) - sqlite3_update_hook.rs

## `uri_boolean.rs`
Requires implementations from:
  - [`sqlite3_create_filename`](sqlite3_create_filename.rs) - sqlite3_create_filename.rs
  - [`sqlite3_db_filename`](sqlite3_db_filename.rs) - sqlite3_db_filename.rs
  - [`sqlite3_filename`](sqlite3_filename.rs) - sqlite3_filename.rs
  - [`sqlite3_uri_boolean`](sqlite3_uri_boolean.rs) - sqlite3_uri_boolean.rs
  - [`sqlite3_uri_key`](sqlite3_uri_key.rs) - sqlite3_uri_key.rs
  - [`sqlite3_uri_parameter`](sqlite3_uri_parameter.rs) - sqlite3_uri_parameter.rs

## `user_data.rs`
Requires implementations from:
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_create_function`](sqlite3_create_function.rs) - sqlite3_create_function.rs
  - [`sqlite3_create_function16`](sqlite3_create_function16.rs) - sqlite3_create_function16.rs
  - [`sqlite3_user_data`](sqlite3_user_data.rs) - sqlite3_user_data.rs

## `value.rs`
Requires implementations from:
  - [`sqlite3_bind_value`](sqlite3_bind_value.rs) - sqlite3_bind_value.rs
  - [`sqlite3_column_value`](sqlite3_column_value.rs) - sqlite3_column_value.rs
  - [`sqlite3_result_value`](sqlite3_result_value.rs) - sqlite3_result_value.rs
  - [`sqlite3_threadsafe`](sqlite3_threadsafe.rs) - sqlite3_threadsafe.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_value_blob`](sqlite3_value_blob.rs) - sqlite3_value_blob.rs
  - [`sqlite3_value_bytes`](sqlite3_value_bytes.rs) - sqlite3_value_bytes.rs
  - [`sqlite3_value_bytes16`](sqlite3_value_bytes16.rs) - sqlite3_value_bytes16.rs
  - [`sqlite3_value_double`](sqlite3_value_double.rs) - sqlite3_value_double.rs
  - [`sqlite3_value_dup`](sqlite3_value_dup.rs) - sqlite3_value_dup.rs
  - [`sqlite3_value_encoding`](sqlite3_value_encoding.rs) - sqlite3_value_encoding.rs
  - [`sqlite3_value_free`](sqlite3_value_free.rs) - sqlite3_value_free.rs
  - [`sqlite3_value_frombind`](sqlite3_value_frombind.rs) - sqlite3_value_frombind.rs
  - [`sqlite3_value_int`](sqlite3_value_int.rs) - sqlite3_value_int.rs
  - [`sqlite3_value_int64`](sqlite3_value_int64.rs) - sqlite3_value_int64.rs
  - [`sqlite3_value_nochange`](sqlite3_value_nochange.rs) - sqlite3_value_nochange.rs
  - [`sqlite3_value_numeric_type`](sqlite3_value_numeric_type.rs) - sqlite3_value_numeric_type.rs
  - [`sqlite3_value_pointer`](sqlite3_value_pointer.rs) - sqlite3_value_pointer.rs
  - [`sqlite3_value_subtype`](sqlite3_value_subtype.rs) - sqlite3_value_subtype.rs
  - [`sqlite3_value_text`](sqlite3_value_text.rs) - sqlite3_value_text.rs
  - [`sqlite3_value_text16`](sqlite3_value_text16.rs) - sqlite3_value_text16.rs
  - [`sqlite3_value_text16be`](sqlite3_value_text16be.rs) - sqlite3_value_text16be.rs
  - [`sqlite3_value_text16le`](sqlite3_value_text16le.rs) - sqlite3_value_text16le.rs
  - [`sqlite3_value_type`](sqlite3_value_type.rs) - sqlite3_value_type.rs
  - [`sqlite3_vtab_rhs_value`](sqlite3_vtab_rhs_value.rs) - sqlite3_vtab_rhs_value.rs

## `value_blob.rs`
Requires implementations from:
  - [`sqlite3_bind`](sqlite3_bind.rs) - sqlite3_bind.rs
  - [`sqlite3_bind_pointer`](sqlite3_bind_pointer.rs) - sqlite3_bind_pointer.rs
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_result_pointer`](sqlite3_result_pointer.rs) - sqlite3_result_pointer.rs
  - [`sqlite3_stmt`](sqlite3_stmt.rs) - sqlite3_stmt.rs
  - [`sqlite3_stmt*`](sqlite3_stmt*.rs) - sqlite3_stmt*.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_value*`](sqlite3_value*.rs) - sqlite3_value*.rs
  - [`sqlite3_value_blob`](sqlite3_value_blob.rs) - sqlite3_value_blob.rs
  - [`sqlite3_value_bytes`](sqlite3_value_bytes.rs) - sqlite3_value_bytes.rs
  - [`sqlite3_value_bytes16`](sqlite3_value_bytes16.rs) - sqlite3_value_bytes16.rs
  - [`sqlite3_value_double`](sqlite3_value_double.rs) - sqlite3_value_double.rs
  - [`sqlite3_value_frombind`](sqlite3_value_frombind.rs) - sqlite3_value_frombind.rs
  - [`sqlite3_value_int`](sqlite3_value_int.rs) - sqlite3_value_int.rs
  - [`sqlite3_value_nochange`](sqlite3_value_nochange.rs) - sqlite3_value_nochange.rs
  - [`sqlite3_value_numeric_type`](sqlite3_value_numeric_type.rs) - sqlite3_value_numeric_type.rs
  - [`sqlite3_value_pointer`](sqlite3_value_pointer.rs) - sqlite3_value_pointer.rs
  - [`sqlite3_value_text`](sqlite3_value_text.rs) - sqlite3_value_text.rs
  - [`sqlite3_value_text16`](sqlite3_value_text16.rs) - sqlite3_value_text16.rs
  - [`sqlite3_value_type`](sqlite3_value_type.rs) - sqlite3_value_type.rs
  - [`sqlite3_vtab_nochange`](sqlite3_vtab_nochange.rs) - sqlite3_vtab_nochange.rs

## `value_dup.rs`
Requires implementations from:
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_value_dup`](sqlite3_value_dup.rs) - sqlite3_value_dup.rs
  - [`sqlite3_value_free`](sqlite3_value_free.rs) - sqlite3_value_free.rs

## `value_encoding.rs`
Requires implementations from:
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_value_bytes`](sqlite3_value_bytes.rs) - sqlite3_value_bytes.rs
  - [`sqlite3_value_bytes16`](sqlite3_value_bytes16.rs) - sqlite3_value_bytes16.rs
  - [`sqlite3_value_encoding`](sqlite3_value_encoding.rs) - sqlite3_value_encoding.rs
  - [`sqlite3_value_text`](sqlite3_value_text.rs) - sqlite3_value_text.rs
  - [`sqlite3_value_text16`](sqlite3_value_text16.rs) - sqlite3_value_text16.rs
  - [`sqlite3_value_text16be`](sqlite3_value_text16be.rs) - sqlite3_value_text16be.rs
  - [`sqlite3_value_text16le`](sqlite3_value_text16le.rs) - sqlite3_value_text16le.rs
  - [`sqlite3_value_type`](sqlite3_value_type.rs) - sqlite3_value_type.rs

## `value_subtype.rs`
Requires implementations from:
  - [`sqlite3_result_subtype`](sqlite3_result_subtype.rs) - sqlite3_result_subtype.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_value_subtype`](sqlite3_value_subtype.rs) - sqlite3_value_subtype.rs

## `vfs.rs`
Requires implementations from:
  - [`sqlite3_file`](sqlite3_file.rs) - sqlite3_file.rs
  - [`sqlite3_filename`](sqlite3_filename.rs) - sqlite3_filename.rs
  - [`sqlite3_io_methods`](sqlite3_io_methods.rs) - sqlite3_io_methods.rs
  - [`sqlite3_open`](sqlite3_open.rs) - sqlite3_open.rs
  - [`sqlite3_open16`](sqlite3_open16.rs) - sqlite3_open16.rs
  - [`sqlite3_open_v2`](sqlite3_open_v2.rs) - sqlite3_open_v2.rs
  - [`sqlite3_syscall_ptr`](sqlite3_syscall_ptr.rs) - sqlite3_syscall_ptr.rs
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs
  - [`sqlite3_vfs_find`](sqlite3_vfs_find.rs) - sqlite3_vfs_find.rs
  - [`sqlite3_vfs_register`](sqlite3_vfs_register.rs) - sqlite3_vfs_register.rs
  - [`sqlite3_vfs_unregister`](sqlite3_vfs_unregister.rs) - sqlite3_vfs_unregister.rs

## `vfs_find.rs`
Requires implementations from:
  - [`sqlite3_vfs`](sqlite3_vfs.rs) - sqlite3_vfs.rs
  - [`sqlite3_vfs_find`](sqlite3_vfs_find.rs) - sqlite3_vfs_find.rs
  - [`sqlite3_vfs_register`](sqlite3_vfs_register.rs) - sqlite3_vfs_register.rs
  - [`sqlite3_vfs_unregister`](sqlite3_vfs_unregister.rs) - sqlite3_vfs_unregister.rs

## `vtab.rs`
Requires implementations from:
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_module`](sqlite3_module.rs) - sqlite3_module.rs
  - [`sqlite3_mprintf`](sqlite3_mprintf.rs) - sqlite3_mprintf.rs
  - [`sqlite3_vtab`](sqlite3_vtab.rs) - sqlite3_vtab.rs

## `vtab_collation.rs`
Requires implementations from:
  - [`sqlite3_declare_vtab`](sqlite3_declare_vtab.rs) - sqlite3_declare_vtab.rs
  - [`sqlite3_index_info`](sqlite3_index_info.rs) - sqlite3_index_info.rs
  - [`sqlite3_vtab_collation`](sqlite3_vtab_collation.rs) - sqlite3_vtab_collation.rs

## `vtab_config.rs`
Requires implementations from:
  - [`sqlite3_vtab_config`](sqlite3_vtab_config.rs) - sqlite3_vtab_config.rs

## `vtab_cursor.rs`
Requires implementations from:
  - [`sqlite3_vtab`](sqlite3_vtab.rs) - sqlite3_vtab.rs
  - [`sqlite3_vtab_cursor`](sqlite3_vtab_cursor.rs) - sqlite3_vtab_cursor.rs

## `vtab_distinct.rs`
Requires implementations from:
  - [`sqlite3_index_info`](sqlite3_index_info.rs) - sqlite3_index_info.rs
  - [`sqlite3_vtab_distinct`](sqlite3_vtab_distinct.rs) - sqlite3_vtab_distinct.rs

## `vtab_in.rs`
Requires implementations from:
  - [`sqlite3_index_info`](sqlite3_index_info.rs) - sqlite3_index_info.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_vtab_in`](sqlite3_vtab_in.rs) - sqlite3_vtab_in.rs
  - [`sqlite3_vtab_in_first`](sqlite3_vtab_in_first.rs) - sqlite3_vtab_in_first.rs
  - [`sqlite3_vtab_in_next`](sqlite3_vtab_in_next.rs) - sqlite3_vtab_in_next.rs

## `vtab_in_first.rs`
Requires implementations from:
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_vtab_in`](sqlite3_vtab_in.rs) - sqlite3_vtab_in.rs
  - [`sqlite3_vtab_in_first`](sqlite3_vtab_in_first.rs) - sqlite3_vtab_in_first.rs
  - [`sqlite3_vtab_in_next`](sqlite3_vtab_in_next.rs) - sqlite3_vtab_in_next.rs

## `vtab_nochange.rs`
Requires implementations from:
  - [`sqlite3_context`](sqlite3_context.rs) - sqlite3_context.rs
  - [`sqlite3_result_xxxxx`](sqlite3_result_xxxxx.rs) - sqlite3_result_xxxxx.rs
  - [`sqlite3_value_nochange`](sqlite3_value_nochange.rs) - sqlite3_value_nochange.rs
  - [`sqlite3_vtab_nochange`](sqlite3_vtab_nochange.rs) - sqlite3_vtab_nochange.rs

## `vtab_on_conflict.rs`
Requires implementations from:
  - [`sqlite3_vtab_on_conflict`](sqlite3_vtab_on_conflict.rs) - sqlite3_vtab_on_conflict.rs

## `vtab_rhs_value.rs`
Requires implementations from:
  - [`sqlite3_index_info`](sqlite3_index_info.rs) - sqlite3_index_info.rs
  - [`sqlite3_value`](sqlite3_value.rs) - sqlite3_value.rs
  - [`sqlite3_vtab_rhs_value`](sqlite3_vtab_rhs_value.rs) - sqlite3_vtab_rhs_value.rs

## `wal_autocheckpoint.rs`
Requires implementations from:
  - [`sqlite3_wal_autocheckpoint`](sqlite3_wal_autocheckpoint.rs) - sqlite3_wal_autocheckpoint.rs
  - [`sqlite3_wal_hook`](sqlite3_wal_hook.rs) - sqlite3_wal_hook.rs

## `wal_checkpoint.rs`
Requires implementations from:
  - [`sqlite3_wal_checkpoint`](sqlite3_wal_checkpoint.rs) - sqlite3_wal_checkpoint.rs
  - [`sqlite3_wal_checkpoint_v2`](sqlite3_wal_checkpoint_v2.rs) - sqlite3_wal_checkpoint_v2.rs

## `wal_checkpoint_v2.rs`
Requires implementations from:
  - [`sqlite3_errcode`](sqlite3_errcode.rs) - sqlite3_errcode.rs
  - [`sqlite3_errmsg`](sqlite3_errmsg.rs) - sqlite3_errmsg.rs

## `wal_hook.rs`
Requires implementations from:
  - [`sqlite3_wal_autocheckpoint`](sqlite3_wal_autocheckpoint.rs) - sqlite3_wal_autocheckpoint.rs
  - [`sqlite3_wal_checkpoint_v2`](sqlite3_wal_checkpoint_v2.rs) - sqlite3_wal_checkpoint_v2.rs
  - [`sqlite3_wal_hook`](sqlite3_wal_hook.rs) - sqlite3_wal_hook.rs

## `win32_set_directory.rs`
Requires implementations from:
  - [`sqlite3_data_directory`](sqlite3_data_directory.rs) - sqlite3_data_directory.rs
  - [`sqlite3_free`](sqlite3_free.rs) - sqlite3_free.rs
  - [`sqlite3_malloc`](sqlite3_malloc.rs) - sqlite3_malloc.rs
  - [`sqlite3_temp_directory`](sqlite3_temp_directory.rs) - sqlite3_temp_directory.rs
  - [`sqlite3_win32_set_directory`](sqlite3_win32_set_directory.rs) - sqlite3_win32_set_directory.rs
  - [`sqlite3_win32_set_directory16`](sqlite3_win32_set_directory16.rs) - sqlite3_win32_set_directory16.rs
  - [`sqlite3_win32_set_directory8`](sqlite3_win32_set_directory8.rs) - sqlite3_win32_set_directory8.rs
