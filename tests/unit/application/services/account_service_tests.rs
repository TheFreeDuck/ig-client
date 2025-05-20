use ig_client::application::models::account::{
    Account, AccountActivity, AccountBalance, AccountInfo, PageData, Positions, TransactionHistory,
    TransactionMetadata, WorkingOrders,
};

#[test]
fn test_account_info_structure() {
    // Test the AccountInfo structure with a sample account
    let account_info = AccountInfo {
        accounts: vec![Account {
            account_id: "ACCOUNT-123".to_string(),
            account_name: "Test Account".to_string(),
            currency: "EUR".to_string(),
            balance: AccountBalance {
                balance: 1000.0,
                deposit: 1000.0,
                profit_loss: 0.0,
                available: 1000.0,
            },
            account_type: "CFD".to_string(),
            preferred: true,
            status: "ENABLED".to_string(),
        }],
    };

    assert_eq!(account_info.accounts.len(), 1);
    assert_eq!(account_info.accounts[0].account_id, "ACCOUNT-123");
    assert_eq!(account_info.accounts[0].account_name, "Test Account");
    assert_eq!(account_info.accounts[0].currency, "EUR");
    assert_eq!(account_info.accounts[0].balance.balance, 1000.0);
    assert_eq!(account_info.accounts[0].account_type, "CFD");
    assert!(account_info.accounts[0].preferred);
    assert_eq!(account_info.accounts[0].status, "ENABLED");
}

#[test]
fn test_positions_structure() {
    // Test the Positions structure
    let positions = Positions {
        positions: Vec::new(),
    };

    assert!(positions.positions.is_empty());
}

#[test]
fn test_working_orders_structure() {
    // Test the WorkingOrders structure
    let working_orders = WorkingOrders {
        working_orders: Vec::new(),
    };

    assert!(working_orders.working_orders.is_empty());
}

#[test]
fn test_account_activity_structure() {
    // Test the AccountActivity structure
    let activity = AccountActivity {
        activities: Vec::new(),
        metadata: None,
    };

    assert!(activity.activities.is_empty());
}

#[test]
fn test_transaction_history_structure() {
    // Test the TransactionHistory structure
    let history = TransactionHistory {
        transactions: Vec::new(),
        metadata: TransactionMetadata {
            page_data: PageData {
                page_size: 10,
                page_number: 1,
                total_pages: 1,
            },
            size: 0,
        },
    };

    assert!(history.transactions.is_empty());
    assert_eq!(history.metadata.page_data.page_size, 10);
    assert_eq!(history.metadata.page_data.page_number, 1);
    assert_eq!(history.metadata.page_data.total_pages, 1);
    assert_eq!(history.metadata.size, 0);
}
