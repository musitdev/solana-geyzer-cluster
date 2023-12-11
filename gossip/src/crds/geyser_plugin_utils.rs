use crate::crds::Crds;
use crate::crds::Pubkey;
use crate::crds::VersionedCrdsValue;
use crate::crds_value::CrdsData;

impl Crds {
    /// Notified when an account is updated at runtime, due to transaction activities
    pub fn notify_clusterinfo_update(&self, crd_value: Option<&VersionedCrdsValue>) {
        if let Some(clusterinfo_update_notifier) = &self.clusterinfo_update_notifier {
            if let Some(value) = crd_value {
                if let CrdsData::LegacyContactInfo(ref cluster_info) = value.value.data {
                    let notifier = &clusterinfo_update_notifier.read().unwrap();
                    notifier.notify_clusterinfo_update(cluster_info);
                }
            }
        }
    }

    /// Notified when the AccountsDb is initialized at start when restored
    /// from a snapshot.
    pub fn notify_clusterinfo_remove(&self, pubkey: &Pubkey) {}
}
