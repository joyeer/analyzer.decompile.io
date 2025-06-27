
/// This module defines the control flow graph (CFG) for a Java method.
pub(crate) struct BasicBlock {
    offset_start_at: usize,
    offset_end_at: usize,
}

/// The control flow graph (CFG) for a Java method.
pub(crate) struct ControlFlowGraph {
    pub blocks: Vec<BasicBlock>,
    pub edges: Vec<(usize, usize)>,
}

impl ControlFlowGraph {
    /// Creates a new control flow graph.
    pub fn new() -> Self {
        ControlFlowGraph {
            blocks: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Adds a block to the control flow graph.
    pub fn add_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }

    /// Adds an edge to the control flow graph.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }
}