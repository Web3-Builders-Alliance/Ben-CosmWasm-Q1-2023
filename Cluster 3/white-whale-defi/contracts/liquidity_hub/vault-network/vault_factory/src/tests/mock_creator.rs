use cosmwasm_std::{MessageInfo, testing::mock_info};

/// Creates a mock creator
pub fn mock_creator() -> MessageInfo {
    mock_info("creator", &[])
}
