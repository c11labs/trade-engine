use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::cmp::PartialEq;

pub trait IntoInner {
    fn into_inner(self) -> f64;
}

#[derive(Debug, Copy, Clone)]
pub struct LongPrice(pub f64);

impl IntoInner for LongPrice {
    fn into_inner(self) -> f64 {
        self.0
    }
}

impl PartialEq for LongPrice {
    fn eq(&self, other: &Self) -> bool {
        if OrderedFloat(self.into_inner()) == OrderedFloat(other.into_inner()) {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for LongPrice {}

impl PartialOrd for LongPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for LongPrice {
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
pub struct ShortPrice(pub f64);

impl IntoInner for ShortPrice {
    fn into_inner(self) -> f64 {
        self.0
    }
}

impl PartialEq for ShortPrice {
    fn eq(&self, other: &Self) -> bool {
        if OrderedFloat(self.into_inner()) == OrderedFloat(other.into_inner()) {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for ShortPrice {}

impl PartialOrd for ShortPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ShortPrice {
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
