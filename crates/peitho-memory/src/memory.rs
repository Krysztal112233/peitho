use im::Vector;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SessinMemory {
    session: Uuid,
    memory: Vector<()>,
    history: Vector<()>,
}
