use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub struct BidPrice(pub f32);

impl BidPrice {
    pub fn into_inner(self) -> f32 {
        self.0
    }
}

impl PartialEq for BidPrice {
    fn eq(&self, other: &Self) -> bool {
        if OrderedFloat(self.into_inner()) == OrderedFloat(other.into_inner()) {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for BidPrice {}

impl PartialOrd for BidPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for BidPrice {
    fn cmp(&self, other: &Self) -> Ordering {
        let me = OrderedFloat(self.into_inner());
        let them = OrderedFloat(other.into_inner());

        if me > them {
            return Ordering::Less;
        } else if me < them {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AskPrice(pub f32);

impl AskPrice {
    pub fn into_inner(self) -> f32 {
        self.0
    }
}

impl PartialEq for AskPrice {
    fn eq(&self, other: &Self) -> bool {
        if OrderedFloat(self.into_inner()) == OrderedFloat(other.into_inner()) {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for AskPrice {}

impl PartialOrd for AskPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for AskPrice {
    fn cmp(&self, other: &Self) -> Ordering {
        let me = OrderedFloat(self.into_inner());
        let them = OrderedFloat(other.into_inner());

        if me > them {
            return Ordering::Greater;
        } else if me < them {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    }
}
