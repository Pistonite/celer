use std::collections::BTreeMap;

use crate::macros::async_trait;

use super::{PluginError, PluginInstance, PluginResult};

/// Early plugin runtime are ran to initialize the plugin instance list
#[async_trait(auto)]
pub trait EarlyPluginRuntime {
    /// Called when a plugin is attempted to be loaded
    ///
    /// Plugins that wish to define their own loading behavior should implement this method,
    /// such as augment an existing plugin when a duplicate is found, or add immediate or deferred plugins for
    /// tasks depending on other plugins. Note that plugins added here are not visible to the user
    /// through plugin settings to be disabled
    async fn on_load_plugin(&self, instance: PluginInstance, plugins: &mut PluginList) -> PluginResult<()> {
        if !instance.allow_duplicate {
            let id = instance.get_id();
            if plugins.contains_immediate(&id) {
                return Err(PluginError::Duplicate(instance.get_display_name().into_owned()));
            }
        }
        plugins.add_immediate(instance);
        Ok(())
    }
}

pub struct DefaultEarlyPluginRuntime;
impl EarlyPluginRuntime for DefaultEarlyPluginRuntime {}

#[derive(Debug, Default)]
pub struct PluginList {
    /// Plugins that are queued to be added immediately
    immediate: Vec<InstanceEntry>,

    immediate_first: BTreeMap<String, PluginInstance>,

    /// Plugins that are queued to be added after the immediate plugins are added
    ///
    /// In this list, plugins are added in the reverse order. These are also invisible to the user
    /// and other plugins, and cannot be disabled individually
    deferred: Vec<PluginInstance>,
}

#[derive(Debug)]
enum InstanceEntry {
    First(String),
    NotFirst(PluginInstance),
}

impl PluginList {
    pub fn get_first_immediate_by_id(&self, id: &str) -> Option<&PluginInstance> {
        self.immediate_first.get(id)
    }

    pub fn contains_immediate(&self, id: &str) -> bool {
        self.immediate_first.contains_key(id)
    }

    pub fn add_immediate(&mut self, instance: PluginInstance) {
        let id = instance.get_id();
        let id_ref: &str = id.as_ref();
        if self.immediate_first.contains_key(id_ref) {
            self.immediate.push(InstanceEntry::NotFirst(instance));
        } else {
            let id = id.into_owned();
            self.immediate_first.insert(id.clone(), instance);
            self.immediate.push(InstanceEntry::First(id));
        }
    }
}

impl IntoIterator for PluginList {
    type Item = PluginInstance;
    type IntoIter = PluginListIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        PluginListIntoIter {
            immediate_iter: self.immediate.into_iter(),
            immediate_first: self.immediate_first,
            deferred_iter: self.deferred.into_iter().rev(),
        }
    }
}

pub struct PluginListIntoIter {
    immediate_iter: std::vec::IntoIter<InstanceEntry>,
    immediate_first: BTreeMap<String, PluginInstance>,
    deferred_iter: std::iter::Rev<std::vec::IntoIter<PluginInstance>>,
}

impl Iterator for PluginListIntoIter {
    type Item = PluginInstance;

    fn next(&mut self) -> Option<Self::Item> {
        match self.immediate_iter.next() {
            Some(InstanceEntry::First(id)) => {
                let instance = self.immediate_first.remove(&id);
                // SAFETY: the id is guaranteed to be in the map since immediate is private
                // and it can only be manipulated in add_immediate
                let instance = instance.unwrap();
                Some(instance)
            }
            Some(InstanceEntry::NotFirst(instance)) => {
                Some(instance)
            }
            None => self.deferred_iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.immediate_iter.len() + self.deferred_iter.len();
        (len, Some(len))
    }
}
