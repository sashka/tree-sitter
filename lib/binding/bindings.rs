/* automatically generated by rust-bindgen */

pub type FILE = [u64; 19usize];
pub type TSSymbol = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSLanguage {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSParser {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSTree {
    _unused: [u8; 0],
}
pub const TSInputEncoding_TSInputEncodingUTF8: TSInputEncoding = 0;
pub const TSInputEncoding_TSInputEncodingUTF16: TSInputEncoding = 1;
pub type TSInputEncoding = u32;
pub const TSSymbolType_TSSymbolTypeRegular: TSSymbolType = 0;
pub const TSSymbolType_TSSymbolTypeAnonymous: TSSymbolType = 1;
pub const TSSymbolType_TSSymbolTypeAuxiliary: TSSymbolType = 2;
pub type TSSymbolType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSPoint {
    pub row: u32,
    pub column: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSRange {
    pub start_point: TSPoint,
    pub end_point: TSPoint,
    pub start_byte: u32,
    pub end_byte: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSInput {
    pub payload: *mut ::std::os::raw::c_void,
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            payload: *mut ::std::os::raw::c_void,
            byte_index: u32,
            position: TSPoint,
            bytes_read: *mut u32,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub encoding: TSInputEncoding,
}
pub const TSLogType_TSLogTypeParse: TSLogType = 0;
pub const TSLogType_TSLogTypeLex: TSLogType = 1;
pub type TSLogType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSLogger {
    pub payload: *mut ::std::os::raw::c_void,
    pub log: ::std::option::Option<
        unsafe extern "C" fn(
            payload: *mut ::std::os::raw::c_void,
            arg1: TSLogType,
            arg2: *const ::std::os::raw::c_char,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSInputEdit {
    pub start_byte: u32,
    pub old_end_byte: u32,
    pub new_end_byte: u32,
    pub start_point: TSPoint,
    pub old_end_point: TSPoint,
    pub new_end_point: TSPoint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSNode {
    pub context: [u32; 4usize],
    pub id: *const ::std::os::raw::c_void,
    pub tree: *const TSTree,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSTreeCursor {
    pub tree: *const ::std::os::raw::c_void,
    pub id: *const ::std::os::raw::c_void,
    pub context: [u32; 2usize],
}
extern "C" {
    pub fn ts_parser_new() -> *mut TSParser;
}
extern "C" {
    pub fn ts_parser_delete(arg1: *mut TSParser);
}
extern "C" {
    pub fn ts_parser_language(arg1: *const TSParser) -> *const TSLanguage;
}
extern "C" {
    pub fn ts_parser_set_language(arg1: *mut TSParser, arg2: *const TSLanguage) -> bool;
}
extern "C" {
    pub fn ts_parser_logger(arg1: *const TSParser) -> TSLogger;
}
extern "C" {
    pub fn ts_parser_set_logger(arg1: *mut TSParser, arg2: TSLogger);
}
extern "C" {
    pub fn ts_parser_print_dot_graphs(arg1: *mut TSParser, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ts_parser_halt_on_error(arg1: *mut TSParser, arg2: bool);
}
extern "C" {
    pub fn ts_parser_parse(arg1: *mut TSParser, arg2: *const TSTree, arg3: TSInput) -> *mut TSTree;
}
extern "C" {
    pub fn ts_parser_parse_string(
        arg1: *mut TSParser,
        arg2: *const TSTree,
        arg3: *const ::std::os::raw::c_char,
        arg4: u32,
    ) -> *mut TSTree;
}
extern "C" {
    pub fn ts_parser_parse_string_encoding(
        arg1: *mut TSParser,
        arg2: *const TSTree,
        arg3: *const ::std::os::raw::c_char,
        arg4: u32,
        arg5: TSInputEncoding,
    ) -> *mut TSTree;
}
extern "C" {
    pub fn ts_parser_enabled(arg1: *const TSParser) -> bool;
}
extern "C" {
    pub fn ts_parser_set_enabled(arg1: *mut TSParser, arg2: bool);
}
extern "C" {
    pub fn ts_parser_timeout_micros(arg1: *const TSParser) -> u64;
}
extern "C" {
    pub fn ts_parser_set_timeout_micros(arg1: *mut TSParser, arg2: u64);
}
extern "C" {
    pub fn ts_parser_reset(arg1: *mut TSParser);
}
extern "C" {
    pub fn ts_parser_set_included_ranges(arg1: *mut TSParser, arg2: *const TSRange, arg3: u32);
}
extern "C" {
    pub fn ts_parser_included_ranges(arg1: *const TSParser, arg2: *mut u32) -> *const TSRange;
}
extern "C" {
    pub fn ts_tree_copy(arg1: *const TSTree) -> *mut TSTree;
}
extern "C" {
    pub fn ts_tree_delete(arg1: *mut TSTree);
}
extern "C" {
    pub fn ts_tree_root_node(arg1: *const TSTree) -> TSNode;
}
extern "C" {
    pub fn ts_tree_edit(arg1: *mut TSTree, arg2: *const TSInputEdit);
}
extern "C" {
    pub fn ts_tree_get_changed_ranges(
        arg1: *const TSTree,
        arg2: *const TSTree,
        arg3: *mut u32,
    ) -> *mut TSRange;
}
extern "C" {
    pub fn ts_tree_print_dot_graph(arg1: *const TSTree, arg2: *mut FILE);
}
extern "C" {
    pub fn ts_tree_language(arg1: *const TSTree) -> *const TSLanguage;
}
extern "C" {
    pub fn ts_node_start_byte(arg1: TSNode) -> u32;
}
extern "C" {
    pub fn ts_node_start_point(arg1: TSNode) -> TSPoint;
}
extern "C" {
    pub fn ts_node_end_byte(arg1: TSNode) -> u32;
}
extern "C" {
    pub fn ts_node_end_point(arg1: TSNode) -> TSPoint;
}
extern "C" {
    pub fn ts_node_symbol(arg1: TSNode) -> TSSymbol;
}
extern "C" {
    pub fn ts_node_type(arg1: TSNode) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn ts_node_string(arg1: TSNode) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ts_node_eq(arg1: TSNode, arg2: TSNode) -> bool;
}
extern "C" {
    pub fn ts_node_is_null(arg1: TSNode) -> bool;
}
extern "C" {
    pub fn ts_node_is_named(arg1: TSNode) -> bool;
}
extern "C" {
    pub fn ts_node_is_missing(arg1: TSNode) -> bool;
}
extern "C" {
    pub fn ts_node_has_changes(arg1: TSNode) -> bool;
}
extern "C" {
    pub fn ts_node_has_error(arg1: TSNode) -> bool;
}
extern "C" {
    pub fn ts_node_parent(arg1: TSNode) -> TSNode;
}
extern "C" {
    pub fn ts_node_child(arg1: TSNode, arg2: u32) -> TSNode;
}
extern "C" {
    pub fn ts_node_named_child(arg1: TSNode, arg2: u32) -> TSNode;
}
extern "C" {
    pub fn ts_node_child_count(arg1: TSNode) -> u32;
}
extern "C" {
    pub fn ts_node_named_child_count(arg1: TSNode) -> u32;
}
extern "C" {
    pub fn ts_node_next_sibling(arg1: TSNode) -> TSNode;
}
extern "C" {
    pub fn ts_node_next_named_sibling(arg1: TSNode) -> TSNode;
}
extern "C" {
    pub fn ts_node_prev_sibling(arg1: TSNode) -> TSNode;
}
extern "C" {
    pub fn ts_node_prev_named_sibling(arg1: TSNode) -> TSNode;
}
extern "C" {
    pub fn ts_node_first_child_for_byte(arg1: TSNode, arg2: u32) -> TSNode;
}
extern "C" {
    pub fn ts_node_first_named_child_for_byte(arg1: TSNode, arg2: u32) -> TSNode;
}
extern "C" {
    pub fn ts_node_descendant_for_byte_range(arg1: TSNode, arg2: u32, arg3: u32) -> TSNode;
}
extern "C" {
    pub fn ts_node_named_descendant_for_byte_range(arg1: TSNode, arg2: u32, arg3: u32) -> TSNode;
}
extern "C" {
    pub fn ts_node_descendant_for_point_range(arg1: TSNode, arg2: TSPoint, arg3: TSPoint)
        -> TSNode;
}
extern "C" {
    pub fn ts_node_named_descendant_for_point_range(
        arg1: TSNode,
        arg2: TSPoint,
        arg3: TSPoint,
    ) -> TSNode;
}
extern "C" {
    pub fn ts_node_edit(arg1: *mut TSNode, arg2: *const TSInputEdit);
}
extern "C" {
    pub fn ts_tree_cursor_new(arg1: TSNode) -> TSTreeCursor;
}
extern "C" {
    pub fn ts_tree_cursor_delete(arg1: *mut TSTreeCursor);
}
extern "C" {
    pub fn ts_tree_cursor_reset(arg1: *mut TSTreeCursor, arg2: TSNode);
}
extern "C" {
    pub fn ts_tree_cursor_current_node(arg1: *const TSTreeCursor) -> TSNode;
}
extern "C" {
    pub fn ts_tree_cursor_goto_parent(arg1: *mut TSTreeCursor) -> bool;
}
extern "C" {
    pub fn ts_tree_cursor_goto_next_sibling(arg1: *mut TSTreeCursor) -> bool;
}
extern "C" {
    pub fn ts_tree_cursor_goto_first_child(arg1: *mut TSTreeCursor) -> bool;
}
extern "C" {
    pub fn ts_tree_cursor_goto_first_child_for_byte(arg1: *mut TSTreeCursor, arg2: u32) -> i64;
}
extern "C" {
    pub fn ts_language_symbol_count(arg1: *const TSLanguage) -> u32;
}
extern "C" {
    pub fn ts_language_symbol_name(
        arg1: *const TSLanguage,
        arg2: TSSymbol,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn ts_language_symbol_for_name(
        arg1: *const TSLanguage,
        arg2: *const ::std::os::raw::c_char,
    ) -> TSSymbol;
}
extern "C" {
    pub fn ts_language_symbol_type(arg1: *const TSLanguage, arg2: TSSymbol) -> TSSymbolType;
}
extern "C" {
    pub fn ts_language_version(arg1: *const TSLanguage) -> u32;
}

pub const TREE_SITTER_LANGUAGE_VERSION: usize = 9;
