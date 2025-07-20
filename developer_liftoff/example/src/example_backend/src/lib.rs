#[ic_cdk::query]
#[derive(candid::CandidType, serde::Serialize, serde::Deserialize)]
struct Exam {
    out_of: u8,
    course_name: String,
    curve: u8,
}

type memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE : U32 = 100;