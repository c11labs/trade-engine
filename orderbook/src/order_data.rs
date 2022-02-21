#[derive(Debug, Default)]
pub struct OrderData {
    pub order_id: u32,
    pub user_name: String,
    pub security_id: u32,
}

impl OrderData {
    pub fn new(order_id: u32, user_name: String, security_id: u32) -> Self {
        Self {
            order_id,
            user_name,
            security_id,
        }
    }
}
