pub mod converting;
pub mod nodes;
pub mod visitor;

/// Enums used in various routines
pub enum TreeTraversalMode {
    /// Used when emit_... is invoked before children are visited
    Enter,
    /// Used when emit_... is invoked before children are visited  
    Exit,
}

#[derive(PartialEq, Eq)]
pub enum TraversalResult {
    /// Returned when the visitor should continue tree traversal
    Continue,
    /// Returned when the visitor should skip the children and exit traversal
    SkipChildren,
}
