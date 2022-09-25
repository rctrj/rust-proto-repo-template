pub mod pb {
    // This is required as exported code has some clippy issues
    #![allow(clippy::all)]

    include!(concat!(env!("OUT_DIR"), "/protos.rs"));
}
