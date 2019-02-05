extern crate bincode;
extern crate ring;

use super::hash;
use super::block_header;
use super::transaction;
use super::Block;
use super::hash::Hashable;

pub struct VoterBlock {
    pub header: block_header::BlockHeader,
    pub transactions: Vec<transaction::Transaction>,
    pub metadata: VoterMetadata,
}

impl Block for VoterBlock {
    fn header(&self) -> &block_header::BlockHeader {
        return &self.header;
    }

    fn hash(&self) -> hash::Hash {
        return self.header.hash();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Vote {
    pub level: u64,
    pub hash: hash::Hash,
}

impl std::fmt::Display for Vote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{level={}, hash={}}}", self.level, self.hash)
    }
}

pub struct VoterMetadata {
    pub votes: Vec<Vote>,
    pub parent_links: Vec<hash::Hash>,
}

impl std::fmt::Display for VoterMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "  votes: [\n")?;
        for v in &self.votes {
            write!(f, "    {},\n", v)?;
        }
        write!(f, "  ]\n",)?;
        write!(f, "  parent links: [\n")?;
        for p in &self.parent_links {
            write!(f, "    {},\n", p)?;
        }
        write!(f, "  ]\n",)?;
        write!(f, "}}")
    }
}

impl hash::Hashable for VoterMetadata {
    fn hash(&self) -> hash::Hash {
        let mut ctx = ring::digest::Context::new(&ring::digest::SHA256);
        for v in &self.votes {
            let serialized = bincode::serialize(&v).unwrap();
            ctx.update(&serialized);
        }
        for p in &self.parent_links {
            let serialized = bincode::serialize(&p).unwrap();
            ctx.update(&serialized);
        }
        let digest = ctx.finish();
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].clone_from_slice(digest.as_ref());
        return raw_hash.into();
    }
}

#[cfg(test)]
mod tests {
    use super::super::hash;
    use super::super::hash::Hashable;
    use super::super::Block;
    use super::super::block_header;
    use super::Vote;
    use super::VoterMetadata;
    use super::VoterBlock;

    macro_rules! fake_voter {
        () => {
            VoterBlock {
                header: block_header::BlockHeader {
                    voter_hash: hash::Hash([1; 32]),
                    proposal_hash: hash::Hash([2; 32]),
                    transactions_hash: hash::Hash([3; 32]),
                    nonce: 12345,
                },
                transactions: vec![],
                metadata: VoterMetadata {
                    votes: vec![],
                    parent_links: vec![],
                },
            }
        };
    }


    #[test]
    fn metadata_hash() {
        let metadata = VoterMetadata {
            votes: vec![
                Vote {
                    level: 1,
                    hash: hash::Hash(hex!(
                        "0102010201020102010201020102010201020102010201020102010201020102"
                    )),
                },
                Vote {
                    level: 2,
                    hash: hash::Hash(hex!(
                        "0304030403040304030403040304030403040304030403040304030403040304"
                    )),
                },
            ],
            parent_links: vec![
                hash::Hash(hex!(
                    "0102030405060504010203040506050401020304050605040102030405060504"
                )),
                hash::Hash(hex!(
                    "0403020104030201040302010403020104030201040302010403020104030201"
                )),
            ],
        };
        let hash = metadata.hash();
        println!("{}", metadata);
        let should_be = hash::Hash(hex!(
            "4f4577a4f4662f58def9b1324f91048c26c75000d2184a7fd2f1d7122e6aa931"
        ));
        assert_eq!(hash, should_be);
    }

    #[test]
    fn block_hash() {
        let block = fake_voter!();
        assert_eq!(block.hash(), hash::Hash(hex!("29e6703a080f122e9ac455aedfbe9bd1974492df74f88ad970c07b824d4ea292")));
    }
}
