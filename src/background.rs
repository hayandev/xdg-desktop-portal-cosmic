use std::collections::HashMap;

use crate::{subscription, wayland::WaylandHelper, PortalResponse};
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1;
use tokio::sync::mpsc::Sender;
use zbus::{self, zvariant};

#[derive(zvariant::SerializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
struct NotifyBackgroundResult {
    result: u32,
}

pub struct Background {
    wayland_helper: WaylandHelper,
    // tx: Sender<subscription::Event>,
}

impl Background {
    pub fn new(wayland_helper: WaylandHelper) -> Self {
        Self {
            wayland_helper,
            // tx,
        }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Background")]
impl Background {
    async fn get_app_state(&self) -> Vec<HashMap<String, u32>> {
        let toplevels = self.wayland_helper.toplevels();
        log::info!("get app state : toplevels {:?}", toplevels);
        toplevels
            .iter()
            .map(|(_, top_level_info)| {
                let state: u32 = if top_level_info.state.len() == 1 {
                    if top_level_info
                        .state
                        .contains(&zcosmic_toplevel_handle_v1::State::Minimized)
                    {
                        0
                    } else {
                        2
                    }
                } else {
                    1
                };

                HashMap::from([(top_level_info.app_id.clone(), state)])
            })
            .collect()
    }

    async fn notify_background(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        name: &str,
    ) -> NotifyBackgroundResult {
        log::info!(
            "notify background : handle {:?}, app_id {:?}, name {:?}",
            handle,
            app_id,
            name
        );
        NotifyBackgroundResult { result: 1 }
    }

    #[zbus(signal)]
    async fn running_applications_changed(
        &self,
        signal_ctxt: &zbus::SignalContext<'_>,
    ) -> zbus::Result<()>;

    async fn enable_autostart(&self, app_id: &str, enable: bool, commandline: Vec<&str>, flags: u32) -> bool {
        log::info!("enable autostart : app_id {:?}, enable {:?}, commandline {:?}, flags {:?}", app_id, enable, commandline, flags);
        true
    }
}
