// ucqcf_engine/src/task.rs

//! Defines the core `Task` unit that is processed by the CEE.

use uuid::Uuid;

/// The type of cryptographic operation to be performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    Encrypt,
    Decrypt,
    KeyGeneration,
    Sign,
    Verify,
}

/// The security domain of the task, which dictates isolation and resource allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityDomain {
    Defense,
    Industrial,
    User,
}

/// The latency requirements for the task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatencyClass {
    Realtime,
    Batch,
}

/// The preferred hardware affinity for the task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Affinity {
    Cpu,
    Fpga,
    Tee,
}

/// A schedulable execution unit for the Core Execution Engine.
#[derive(Debug, Clone)]
pub struct Task {
    /// A unique identifier for the task.
    pub id: Uuid,
    /// The type of operation to be performed.
    pub task_type: TaskType,
    /// The security domain of the task.
    pub security_domain: SecurityDomain,
    /// The latency requirements for the task.
    pub latency_class: LatencyClass,
    /// The preferred hardware affinity for the task.
    pub affinity: Affinity,
    // A placeholder for the actual data to be processed.
    pub data: Vec<u8>,
}
