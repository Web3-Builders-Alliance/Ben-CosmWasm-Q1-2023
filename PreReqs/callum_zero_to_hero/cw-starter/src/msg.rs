// messages used by the contract

// import dependencies
use crate::state::{Ballot, Poll};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// define InstantiateMsg struct with a single field called admin which is an Option<String>, so it can be a String or nothing
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

// define ExecuteMsg enumerator with variants for CreatePoll, and Vote
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePoll {
        poll_id: String,
        question: String,
        options: Vec<String>,
    },
    Vote {
        poll_id: String,
        vote: String,
    },
}

// define QueryMsg enumerator with variants for AllPolls, Poll, and Vote
// a user can query all the polls, a specific poll, or a specific user's vote for a a specific poll
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPolls {},
    Poll { poll_id: String },
    Vote { poll_id: String, address: String },
}

// define the response to the AllPolls query as a vector of Polls
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllPollsResponse {
    pub polls: Vec<Poll>,
}

// define the response to the Poll query as an option of a poll's ID or nothing if it does not exist
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PollResponse {
    pub poll: Option<Poll>,
}

// define the repsponse to the Vote query as an option of a ballot or nothing if it does not exist
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct VoteResponse {
    pub vote: Option<Ballot>,
}

// define a blank MigrateMsg struct with no possible enumerations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
