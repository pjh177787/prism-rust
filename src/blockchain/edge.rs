/*
This contains the different type of edges in Prism graph structure.
*/

// todo: Document the edge types.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub enum Edge {
    // Tx edge types
    TransactionToProposerParent, // For tx block(from) mined on a proposer parent(to)
    // Prop edge types
    ProposerToProposerParent, // prop block(from) mined on a proposer parent(to)
    ProposerToProposerReference(u32), // prop block(to) referred by a prop block(from). u32 used for ordering the refs everywhere.
    ProposerToTransactionReference(u32), // tx blocks(to) referred by a prop block(from)
    ProposerToTransactionLeaderReference(u32), // leader prop block(from) which includes the tx block(from) in the ledger.
    ProposerToTransactionReferenceAndLeaderReference(u32), // Both the above two.
    // Voter edge types
    VoterToProposerParent, // voter block(from) mined on a proposer parent(to)
    VoterToProposerVote,   // voter block(from) voting on a proposer block(to)
    VoterToProposerParentAndVote, // Both the above two
    VoterToVoterParent,    // voter block(from) mined on a voter parent(to)

    // Reverse Edges (Not all have to be used)
    TransactionFromProposerParent,
    // Prop edge types
    ProposerFromProposerParent,
    ProposerFromProposerReference(u32),
    ProposerFromTransactionReference(u32),
    ProposerFromTransactionLeaderReference(u32),
    ProposerFromTransactionReferenceAndLeaderReference(u32),
    // Voter edge types
    VoterFromProposerParent,
    VoterFromProposerVote,
    VoterFromProposerParentAndVote, // When a proposer block is both voted and proposer parent
    VoterFromVoterParent,
}

// Make it cleaner?
impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Edge::TransactionToProposerParent => {
                write!(f, "Tx2PropParent");
                Ok(())
            }
            Edge::ProposerToProposerParent => {
                write!(f, "Prop2PropParent");
                Ok(())
            }
            Edge::ProposerToProposerReference(position) => {
                write!(f, "Prop2PropRef");
                Ok(())
            }
            Edge::ProposerToTransactionReference(position) => {
                write!(f, "Prop2TxRef");
                Ok(())
            }
            Edge::ProposerToTransactionLeaderReference(position) => {
                write!(f, "Prop2TxLeaderRef");
                Ok(())
            }
            Edge::ProposerToTransactionReferenceAndLeaderReference(position) => {
                write!(f, "Prop2TxRefAndLeaderRef");
                Ok(())
            }
            Edge::VoterToProposerParent => {
                write!(f, "V2PropParent");
                Ok(())
            }
            Edge::VoterToVoterParent => {
                write!(f, "V2VParent");
                Ok(())
            }
            Edge::VoterToProposerVote => {
                write!(f, "V2PropVote");
                Ok(())
            }
            Edge::VoterToProposerParentAndVote => {
                write!(f, "V2PropParent_and_Vote");
                Ok(())
            }
            /// Reverse Edges
            Edge::TransactionFromProposerParent => {
                write!(f, "TxFromPropParent");
                Ok(())
            }
            Edge::ProposerFromProposerParent => {
                write!(f, "PropFromPropParent");
                Ok(())
            }
            Edge::ProposerFromProposerReference(position) => {
                write!(f, "PropFromPropRef");
                Ok(())
            }
            Edge::ProposerFromTransactionReference(position) => {
                write!(f, "PropFromTxRef");
                Ok(())
            }
            Edge::ProposerFromTransactionLeaderReference(position) => {
                write!(f, "PropFromTxLeaderRef");
                Ok(())
            }
            Edge::ProposerFromTransactionReferenceAndLeaderReference(position) => {
                write!(f, "Prop2TxRefAndLeaderRef");
                Ok(())
            }
            Edge::VoterFromProposerParent => {
                write!(f, "VFromPropParent");
                Ok(())
            }
            Edge::VoterFromVoterParent => {
                write!(f, "VFromVParent");
                Ok(())
            }
            Edge::VoterFromProposerVote => {
                write!(f, "VFromPropVote");
                Ok(())
            }
            Edge::VoterFromProposerParentAndVote => {
                write!(f, "VFromPropParent_and_Vote");
                Ok(())
            }
        }
    }
}

impl Edge {
    // Returns the reverse edge  type
    pub fn reverse_edge(&self) -> Edge {
        match self {
            Edge::TransactionToProposerParent => Edge::TransactionFromProposerParent,
            Edge::ProposerToProposerParent => Edge::ProposerFromProposerParent,
            Edge::ProposerToProposerReference(position) => {
                Edge::ProposerFromProposerReference(*position)
            }
            Edge::ProposerToTransactionReference(position) => {
                Edge::ProposerFromTransactionReference(*position)
            }
            Edge::ProposerToTransactionLeaderReference(position) => {
                Edge::ProposerFromTransactionLeaderReference(*position)
            }
            Edge::ProposerToTransactionReferenceAndLeaderReference(position) => {
                Edge::ProposerFromTransactionReferenceAndLeaderReference(*position)
            }
            Edge::VoterToProposerParent => Edge::VoterFromProposerParent,
            Edge::VoterToVoterParent => Edge::VoterFromVoterParent,
            Edge::VoterToProposerVote => Edge::VoterFromProposerVote,
            Edge::VoterToProposerParentAndVote => Edge::VoterFromProposerParentAndVote,
            /// Reverse Edges
            Edge::TransactionFromProposerParent => Edge::TransactionToProposerParent,
            Edge::ProposerFromProposerParent => Edge::ProposerToProposerParent,
            Edge::ProposerFromProposerReference(position) => {
                Edge::ProposerToProposerReference(*position)
            }
            Edge::ProposerFromTransactionReference(position) => {
                Edge::ProposerToTransactionReference(*position)
            }
            Edge::ProposerFromTransactionLeaderReference(position) => {
                Edge::ProposerToTransactionLeaderReference(*position)
            }
            Edge::ProposerFromTransactionReferenceAndLeaderReference(position) => {
                Edge::ProposerToTransactionReferenceAndLeaderReference(*position)
            }
            Edge::VoterFromProposerParent => Edge::VoterToProposerParent,
            Edge::VoterFromVoterParent => Edge::VoterToVoterParent,
            Edge::VoterFromProposerVote => Edge::VoterToProposerVote,
            Edge::VoterFromProposerParentAndVote => Edge::VoterToProposerParentAndVote,
        }
    }
}
