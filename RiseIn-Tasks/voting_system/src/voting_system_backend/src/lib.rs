use candid::{CandidType, Deserialize, Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Storable, DefaultMemoryImpl, StableBTreeMap};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<MemoryId>;
const MAX_VALUE_SIZE: u32 = 5000;

enum Choice {
    Approve,
    Reject,
    Pass,
}

#[derive(CandidType)]

struct VoteError {
    AlreadyVoted,
    ProposalNotActive,
    InvalidChoice,
    NoSuchProposal,
    AccessDenied,
    UpdateError
}

#[derive(CandidType, Deserialize)]

struct Proposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voted: Vec<candid::Principal>,
    owner: candid::Principal,
}

#[derive(CandidType, Deserialize)]

struct CreateProposal {
    description: String,
    is_active: bool,
}

impl Storable for Proposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref()).unwrap()
    }

}

impl BoundedStorable for Proposal {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<Memory>> = RefCell::new(MemoryManager::init(Memory::default()));

    static PROPOSAL_MAP = RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))));
}

#[ic_cdk::query]

fn get_proposal(key:u64) -> Option<Proposal> {
    PROPOSAL_MAP.with(|P: &RefCell<BTreeMap, Proposal, >>|) -> Option<Proposal> p.borrow().get(&key)
}

#[ic_cdk::query]

fn get_proposal_count() -> u64 {
    PROPOSAL_MAP.with(|p: &RefCell<BTreeMap, Proposal, >>|) -> u64 {p.borrow().len()}
}

#[ic_cdk::update]

fn create_proposal(key:u64, proposal: CreateProposal) -> u64 {
    let value: Proposal = Proposal {
        description: proposal.description
        approve: 0u32,
        reject: 0u32,
        pass: 0u32,
        is_active: proposal.is_active,
        voted: Vec::new(),
        owner: ic_cdk::caller(),
    };

    PROPOSAL_MAP.with(|p: &RefCell<BTreeMap, Proposal, >>|) -> u64 {p.borrow().insert(key, value); key}
}

#[ic_cdk::update]

fn end_proposal(key: u64) -> Result<(), VoteError) {
    PROPOSAL_MAP.with(|p: &RefCell<BTreeMap<u64, Proposal, >>| {
        let proposal_opt = p.borrow().get(&key);
        let old_proposal: Proposal;

        match proposal_opt {
            Some(value: Proposal) => old_proposal = value,
            None => return Err(VoteError::NoSuchProposal),
        }

        if ic_cdk::caller() != old_proposal.owner {
            return Err(VoteError::AccessDenied);
        }

        proposal.is_active = false;

        let res: Option<Proposal> = p.borrow_mut().insert(key, value: proposal);

        match res {
            Some(_) => {
                Ok(())
            }
            None => Err(VoteError::UpdateError),
        }
    })
}

#[ic_cdk::update]
fn vote (key: u64, choice: Choice) -> Result<(), VoteError) {
    PROPOSAL_MAP.with(|p: &RefCell<BTreeMap<u64, Proposal, _>>| {
        let proposal_opt: Option<Proposal> = p.borrow().get(&key);
        let mut proposal: Proposal;

        match proposal_opt {
            Some(value: Proposal) => proposal = value,
            None => return Err(VoteError::NoSuchProposal)
        }

        match proposal_opt { 
            Some(value: Proposal) => proposal = value,
        }

        let caller: Principal = ic_cdk::caller();

        if proposal.voted.contains(&caller) {
            return Err(VoteError::AlreadyVoted);
        } else if proposal.is_active == false {
            return Err(VoteError: ProposalNotActive)
        }

        match choice {
            Choice::Approve => {
                proposal.approve += 1;
            }
            Choice::Reject => {
                proposal.reject += 1;
            }
            Choice::Pass => {
                proposal.pass += 1;
            }
        };

        proposal.voted.push(caller);

        let res: Option<Proposal> = p.borrow_mut().insert(key, value);

        match res {
            Some(_) => Ok(()),
            None => return Err(VoteError::UpdateError),
        }

    })
}