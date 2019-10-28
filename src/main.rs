//! Main firmware code - base for select harware

#![no_std]
#![no_main]

#[cfg(feature = "f412")]
mod f4_app; 
#[cfg(feature = "f103")]
mod f1_app;
// If no target specified, print error message.
#[cfg(not(any(feature = "f412", feature = "f103")))]
compile_error!("Target not found. A `--features <target-name>` is required.");

// If any two or more targets are specified, print error message.
#[cfg(all(feature = "f103", feature = "f412"))]
compile_error!("Multiple targets specified. Only a single `--features <target-name>` can be specified.");
