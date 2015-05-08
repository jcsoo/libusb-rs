//! This crate provides a safe wrapper around the native `libusb` library.

extern crate libusb_sys as ffi;
extern crate libc;
extern crate time;

pub use ::version::{LibraryVersion,version};
pub use ::error::{UsbResult,UsbError};

pub use ::context::{Context,LogLevel};
pub use ::device_list::{DeviceList};
pub use ::device_ref::{DeviceRef};
pub use ::device_handle::{DeviceHandle};
pub use ::interface_handle::{InterfaceHandle};

pub use ::fields::{Version};
pub use ::device::{Device,Speed};
pub use ::configuration::{Configuration};
pub use ::interface::{Interface,InterfaceSetting};
pub use ::endpoint::{Endpoint,Direction,TransferType,SyncType,UsageType};


#[cfg(test)]
#[macro_use]
mod test_helpers;

mod version;
mod error;

mod context;
mod device_list;
mod device_ref;
mod device_handle;
mod interface_handle;

mod fields;
mod device;
mod configuration;
mod interface;
mod endpoint;