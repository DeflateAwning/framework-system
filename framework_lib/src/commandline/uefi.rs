use crate::commandline::Cli;

use std::uefi;

use core::convert::TryInto;
use uefi::guid::{Guid, SHELL_PARAMETERS_GUID};
use uefi::shell::ShellParameters as UefiShellParameters;
use uefi_std::ffi;
use uefi_std::proto::Protocol;

pub struct ShellParameters(pub &'static mut UefiShellParameters);

impl Protocol<UefiShellParameters> for ShellParameters {
    fn guid() -> Guid {
        SHELL_PARAMETERS_GUID
    }

    fn new(inner: &'static mut UefiShellParameters) -> Self {
        ShellParameters(inner)
    }
}

// Convert C array of UCS-2 strings to vector of UTF-8 strings
// Basically UEFI to Rust
fn array_nstr(wstr_array: *const *const u16, size: usize) -> Vec<String> {
    let mut strings = vec![];

    for i in 0..size {
        let str = unsafe { ffi::nstr(*wstr_array.offset(i.try_into().unwrap())) };
        strings.push(str);
    }

    strings
}

/// Get commandline arguments from UEFI environment
pub fn get_args() -> Vec<String> {
    let image_handle = std::handle();

    if let Ok(parameters) = ShellParameters::handle_protocol(image_handle) {
        let ptr: *const *const u16 = parameters.0.Argv;
        let size: usize = parameters.0.Argc;

        array_nstr(ptr, size)
    } else {
        // TODO: Maybe handle errors properly
        vec![]
    }
}

pub fn parse(args: &[String]) -> Cli {
    let mut cli = Cli {
        versions: false,
        esrt: false,
        power: false,
        pdports: false,
        privacy: false,
        pd_bin: None,
        ec_bin: None,
        test: false,
        help: false,
        allupdate: false,
        info: false,
        raw_command: vec![],
    };

    if args.len() == 1 {
        cli.help = true;
    }

    for arg in args {
        if arg == "-v" || arg == "--versions" {
            cli.versions = true;
        } else if arg == "--esrt" {
            cli.esrt = true;
        } else if arg == "--power" {
            cli.power = true;
        } else if arg == "--pdports" {
            cli.pdports = true;
        } else if arg == "--allupdate" {
            cli.allupdate = true;
        } else if arg == "--info" {
            cli.info = true;
        } else if arg == "-t" || arg == "--test" {
            cli.test = true;
        } else if arg == "-h" || arg == "--help" {
            cli.help = true;
        } else if arg == "--privacy" {
            cli.privacy = true;
        } else if arg == "--raw-command" {
            cli.raw_command = args[1..].to_vec();
        }
    }

    cli
}