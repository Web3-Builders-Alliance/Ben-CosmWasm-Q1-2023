use cosmwasm_std::{Addr, MessageInfo, testing::mock_info};

/// Creates a mock creator
pub fn mock_creator() -> MessageInfo {
    mock_info("creator", &[])
}

pub fn mock_admin() -> Addr {
    Addr::unchecked("mock_admin")
}
