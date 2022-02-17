#[derive(Debug, Default)]
pub struct OrderData {
    pub order_id: u64,
    pub user_name: String,
    pub security_id: u64,
}

impl OrderData {
    pub fn new(order_id: u64, user_name: String, security_id: u64) -> Self {
        Self {
            order_id,
            user_name,
            security_id,
        }
    }
}
