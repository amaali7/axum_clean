use crate::authorization::policys::ApplicationStoredPolicy;

pub trait PolicyRepository {
    fn load_active_policies(&self) -> Vec<ApplicationStoredPolicy>;
}
