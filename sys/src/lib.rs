#![allow(unused_unsafe)] // suppress some warnings generated by autocxx

use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "openvr.h"

    generate!("vr::VR_Init")
    generate_pod!("vr::EVRApplicationType")
    generate!("vr::VR_Shutdown")

    generate!("vr::IVRSystem")
    generate!("vr::VRSystem")

    generate!("vr::IVROverlay")
    generate!("vr::VROverlay")
    generate_pod!("vr::EVROverlayError")
    generate_pod!("vr::VROverlayHandle_t")

    generate!("vr::IVRApplications")
    generate!("vr::VRApplications")
    generate!("vr::k_unMaxApplicationKeyLength")

    generate!("vr::VR_GetVRInitErrorAsSymbol")
    generate_pod!("vr::EVRInitError")

    generate_pod!("vr::ETrackingUniverseOrigin")
    generate!("vr::HmdMatrix34_t")

    generate_pod!("vr::VRTextureBounds_t")
}

//pub use ffi::vr::*;
pub use ffi::vr::*;
pub use ffi::{make_string, ToCppString};
