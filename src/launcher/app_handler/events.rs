// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

/// This is an event subset to be used by external codes to communicate with the App Handling
/// module. Observer registration facilities may be availed for notifications.
pub enum AppHandlerEvent {
    /// Request Launcher to add and app to manage.
    AddApp(event_data::AppDetail),
    /// Request Launcher to remove a previously added app.
    RemoveApp(::routing::NameType),
    /// Request Launcher to activate/start an app.
    ActivateApp(::routing::NameType),
    /// Modify settings for an app
    ModifyAppSettings(event_data::ModifyAppSettings),
    /// Register an observer to receive notifications about status of adding of an app.
    RegisterAppAddObserver(::observer::AppHandlerObserver),
    /// Register an observer to receive notifications about status of removal of an app.
    RegisterAppRemoveObserver(::observer::AppHandlerObserver),
    /// Register an observer to receive notifications about an app being activated. Note however
    /// that a successful activation does not necessarily translate into successfully managed
    /// session. It just means that app has been started. It can still crash after start, fail
    /// authentication etc. Register obeservers in IPC module to get a more fine grained
    /// information.
    RegisterAppActivateObserver(::observer::AppHandlerObserver),
    /// Register an observer to receive notifications about status of modification of an app.
    RegisterAppModifyObserver(::observer::AppHandlerObserver),
    /// Obtain all apps currently being managed by Launcher.
    GetAllManagedApps(::std::sync::mpsc::Sender<Result<Vec<event_data::ManagedApp>, ::errors::LauncherError>>),
    /// Gracefully exit the app handling module. After a call to this Launcher will no longer cater
    /// to any requests handled by this module. This is essentially Launcher-close scenario and
    /// Launcher must be restarted to be functional again.
    Terminate,
}

impl ::std::fmt::Debug for AppHandlerEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl From<event_data::AppDetail> for AppHandlerEvent {
    fn from(data: event_data::AppDetail) -> Self {
        AppHandlerEvent::AddApp(data)
    }
}

impl From<event_data::ModifyAppSettings> for AppHandlerEvent {
    fn from(data: event_data::ModifyAppSettings) -> Self {
        AppHandlerEvent::ModifyAppSettings(data)
    }
}

/// Data that will be transferred during specific events to get a fine-grained detail about various
/// parameters.
pub mod event_data {
    /// Details of applications. Contains fields required as manadatory parameters of the
    /// associated event.
    #[derive(Debug, Clone)]
    pub struct AppDetail {
        /// Absolute path to the app binary on this machine.
        pub absolute_path    : String,
        /// If this app is allowed to have access to `SAFEDrive`.
        pub safe_drive_access: bool,
    }

    /// Representation of an app currently managed by Launcher.
    #[derive(Debug, Clone)]
    pub struct ManagedApp {
        /// Unique id given to the app. This will be consistent across all machines.
        pub id               : ::routing::NameType,
        /// Name of this app. Unless specifically changed it will be the name of the binary added
        /// to Launcher in the first machine for this app.
        pub name             : String,
        /// If the app was added to this machine, this will contain the absolute path to the
        /// application binary. Otherwise it will be `None` indicating that app was added to
        /// Launcher but not yet on this machine.
        pub local_path       : Option<String>,
        /// Number of machines this app is currently added to Launcher in.
        pub reference_count  : u32,
        /// If this app is allowed to have access to `SAFEDrive`.
        pub safe_drive_access: bool,
    }

    /// Representation of settings modification data. Optional fields should be filled with some
    /// data only if it is required to modify that parameter, otherwise fill in `None`.
    #[derive(Debug, Clone)]
    pub struct ModifyAppSettings {
        /// Unique id given to the app. This will be consistent across all machines.
        pub id               : ::routing::NameType,
        /// App name for this app stored in Launcher.
        pub name             : Option<String>,
        /// Modify local binary absolute path.
        pub local_path       : Option<String>,
        /// If this app is allowed to have access to `SAFEDrive`.
        pub safe_drive_access: Option<bool>,
    }
}
