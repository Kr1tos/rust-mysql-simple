pub static PAYLOAD_OFFSET: uint = 4;
pub static MAX_PACKET_LEN: uint = 16777219;
pub static MAX_PAYLOAD_LEN: uint = 16777215;

pub static UTF8_GENERAL_CI: u8 = 33u8;

/// Status flags
pub static SERVER_STATUS_IN_TRANS: u16 = 0x0001;
pub static SERVER_STATUS_AUTOCOMMIT: u16 = 0x0002;
pub static SERVER_MORE_RESULTS_EXISTS: u16 = 0x0008;
pub static SERVER_STATUS_NO_GOOD_INDEX_USED: u16 = 0x0010;
pub static SERVER_STATUS_NO_INDEX_USED: u16 = 0x0020;
pub static SERVER_STATUS_CURSOR_EXISTS: u16 = 0x0040;
pub static SERVER_STATUS_LAST_ROW_SENT: u16 = 0x0080;
pub static SERVER_STATUS_DB_DROPPED: u16 = 0x0100;
pub static SERVER_STATUS_NO_BACKSLASH_ESCAPES: u16 = 0x0200;
pub static SERVER_STATUS_METADATA_CHANGED: u16 = 0x0400;
pub static SERVER_QUERY_WAS_SLOW: u16 = 0x0800;
pub static SERVER_PT_OUT_PARAMS: u16 = 0x1000;

/// Capability flags
pub static CLIENT_LONG_PASSWORD: u32 = 0x00000001;
pub static CLIENT_FOUND_ROWS: u32 = 0x00000002;
pub static CLIENT_LONG_FLAG: u32 = 0x00000004;
pub static CLIENT_CONNECT_WITH_DB: u32 = 0x00000008;
pub static CLIENT_NO_SCHEMA: u32 = 0x00000010;
pub static CLIENT_COMPRESS: u32 = 0x00000020;
pub static CLIENT_ODBC: u32 = 0x00000040;
pub static CLIENT_LOCAL_FILES: u32 = 0x00000080;
pub static CLIENT_IGNORE_SPACE: u32 = 0x00000100;
pub static CLIENT_PROTOCOL_41: u32 = 0x00000200;
pub static CLIENT_INTERACTIVE: u32 = 0x00000400;
pub static CLIENT_SSL: u32 = 0x00000800;
pub static CLIENT_IGNORE_SIGPIPE: u32 = 0x00001000;
pub static CLIENT_TRANSACTIONS: u32 = 0x00002000;
pub static CLIENT_RESERVED: u32 = 0x00004000;
pub static CLIENT_SECURE_CONNECTION: u32 = 0x00008000;
pub static CLIENT_MULTI_STATEMENTS: u32 = 0x00010000;
pub static CLIENT_MULTI_RESULTS: u32 = 0x00020000;
pub static CLIENT_PS_MULTI_RESULTS: u32 = 0x00040000;
pub static CLIENT_PLUGIN_AUTH: u32 = 0x00080000;
pub static CLIENT_CONNECT_ATTRS: u32 = 0x00100000;
pub static CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA: u32 = 0x00200000;

/// Commands
pub static COM_SLEEP: u8 = 0x00u8;
pub static COM_QUIT: u8 = 0x01_u8;
pub static COM_INIT_DB: u8 = 0x02_u8;
pub static COM_QUERY: u8 = 0x03_u8;
pub static COM_FIELD_LIST: u8 = 0x04_u8;
pub static COM_CREATE_DB: u8 = 0x05_u8;
pub static COM_DROP_DB: u8 = 0x06_u8;
pub static COM_REFRESH: u8 = 0x07_u8;
pub static COM_SHUTDOWN: u8 = 0x08_u8;
pub static COM_STATISTICS: u8 = 0x09_u8;
pub static COM_PROCESS_INFO: u8 = 0x0a_u8;
pub static COM_CONNECT: u8 = 0x0b_u8;
pub static COM_PROCESS_KILL: u8 = 0x0c_u8;
pub static COM_DEBUG: u8 = 0x0d_u8;
pub static COM_PING: u8 = 0x0e_u8;
pub static COM_TIME: u8 = 0x0f_u8;
pub static COM_DELAYED_INSERT: u8 = 0x10_u8;
pub static COM_CHANGE_USER: u8 = 0x11_u8;
pub static COM_BINLOG_DUMP: u8 = 0x12_u8;
pub static COM_TABLE_DUMP: u8 = 0x13_u8;
pub static COM_CONNECT_OUT: u8 = 0x14_u8;
pub static COM_REGISTER_SLAVE: u8 = 0x15_u8;
pub static COM_STMT_PREPARE: u8 = 0x16_u8;
pub static COM_STMT_EXECUTE: u8 = 0x17_u8;
pub static COM_STMT_SEND_LONG_DATA: u8 = 0x18_u8;
pub static COM_STMT_CLOSE: u8 = 0x19_u8;
pub static COM_STMT_RESET: u8 = 0x1a_u8;
pub static COM_SET_OPTION: u8 = 0x1b_u8;
pub static COM_STMT_FETCH: u8 = 0x1c_u8;
pub static COM_DAEMON: u8 = 0x1d_u8;
pub static COM_BINLOG_DUMP_GTID: u8 = 0x1e_u8;
pub static COM_RESET_CONNECTION: u8 = 0x1f_u8;

/// Text protocol column types
pub static MYSQL_TYPE_DECIMAL: u8 = 0x00_u8;
pub static MYSQL_TYPE_TINY: u8 = 0x01_u8;
pub static MYSQL_TYPE_SHORT: u8 = 0x02_u8;
pub static MYSQL_TYPE_LONG: u8 = 0x03_u8;
pub static MYSQL_TYPE_FLOAT: u8 = 0x04_u8;
pub static MYSQL_TYPE_DOUBLE: u8 = 0x05_u8;
pub static MYSQL_TYPE_NULL: u8 = 0x06_u8;
pub static MYSQL_TYPE_TIMESTAMP: u8 = 0x07_u8;
pub static MYSQL_TYPE_LONGLONG: u8 = 0x08_u8;
pub static MYSQL_TYPE_INT24: u8 = 0x09_u8;
pub static MYSQL_TYPE_DATE: u8 = 0x0a_u8;
pub static MYSQL_TYPE_TIME: u8 = 0x0b_u8;
pub static MYSQL_TYPE_DATETIME: u8 = 0x0c_u8;
pub static MYSQL_TYPE_YEAR: u8 = 0x0d_u8;
pub static MYSQL_TYPE_VARCHAR: u8 = 0x0f_u8;
pub static MYSQL_TYPE_BIT: u8 = 0x10_u8;
pub static MYSQL_TYPE_NEWDECIMAL: u8 = 0xf6_u8;
pub static MYSQL_TYPE_ENUM: u8 = 0xf7_u8;
pub static MYSQL_TYPE_SET: u8 = 0xf8_u8;
pub static MYSQL_TYPE_TINY_BLOB: u8 = 0xf9_u8;
pub static MYSQL_TYPE_MEDIUM_BLOB: u8 = 0xfa_u8;
pub static MYSQL_TYPE_LONG_BLOB: u8 = 0xfb_u8;
pub static MYSQL_TYPE_BLOB: u8 = 0xfc_u8;
pub static MYSQL_TYPE_VAR_STRING: u8 = 0xfd_u8;
pub static MYSQL_TYPE_STRING: u8 = 0xfe_u8;
pub static MYSQL_TYPE_GEOMETRY: u8 = 0xff_u8;

/// Column flags
pub static NOT_NULL_FLAG: u16 = 1u16;
pub static PRI_KEY_FLAG: u16 = 2u16;
pub static UNIQUE_KEY_FLAG: u16 = 4u16;
pub static MULTIPLE_KEY_FLAG: u16 = 8u16;
pub static BLOB_FLAG: u16 = 16u16;
pub static UNSIGNED_FLAG: u16 = 32u16;
pub static ZEROFILL_FLAG: u16 = 64u16;
pub static BINARY_FLAG: u16 = 128u16;
pub static ENUM_FLAG: u16 = 256u16;
pub static AUTO_INCREMENT_FLAG: u16 = 512u16;
pub static TIMESTAMP_FLAG: u16 = 1024u16;
pub static SET_FLAG: u16 = 2048u16;
pub static NO_DEFAULT_VALUE_FLAG: u16 = 4096u16;
pub static ON_UPDATE_NOW_FLAG: u16 = 8192u16;
pub static NUM_FLAG: u16 = 32768u16;
pub static PART_KEY_FLAG: u16 = 16384u16;
pub static GROUP_FLAG: u16 = 32768u16;
// pub static UNIQUE_FLAG: u16 = 65536u16;