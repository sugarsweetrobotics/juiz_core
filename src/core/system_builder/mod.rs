mod setup_plugins;
mod setup_objects;
mod cleanup_objects;
mod containers;
mod processes;
mod brokers;
mod ecs;
mod components;
mod connections;
mod subsystems;
mod topics;

mod http_broker;
mod ipc_broker;
mod local_broker;

pub(crate) use setup_plugins::setup_plugins;
pub(crate) use setup_objects::setup_objects;
pub(crate) use cleanup_objects::cleanup_objects;
use uuid::Uuid;

use crate::{brokers::{broker_proxy::TopicBrokerProxy, SystemBrokerProxy}, result::JuizResult};

use super::system::System;


pub(crate) fn setup_topic_synchronization(system: &mut System) -> JuizResult<()> {
    log::trace!("setup_topic_synchronization() called");
    let mut should_request_subscribe_topic_names : Vec<String> = Vec::new();
    let mut should_request_publish_topic_names : Vec<String> = Vec::new();
    system.core_broker().lock_mut().and_then(|mut cb| {
        let system_uuid = Uuid::parse_str(cb.system_uuid()?.as_str().unwrap())?;
        for (topic_name, topic) in cb.store_mut().topics.iter() {
            if topic.num_local_publishers().unwrap() > 0 {
                should_request_subscribe_topic_names.push(topic_name.to_owned());

            }
            if topic.num_local_subscribers().unwrap() > 0 {
                should_request_publish_topic_names.push(topic_name.to_owned());

            }
        }
        for topic_name in should_request_subscribe_topic_names.iter() {
            let result = cb.topic_request_subscribe(topic_name, Some(system_uuid))?;
        }
        for topic_name in should_request_publish_topic_names.iter() {
            let result = cb.topic_request_publish(topic_name, Some(system_uuid))?;
        }

        log::trace!("setup_topic_synchronization() exit");
        Ok(())
    })
}

