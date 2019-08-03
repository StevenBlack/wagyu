use crate::cli::option;

use clap::{AppSettings};


// Format
// (name, about, options, settings)

pub const HD: (&str, &str, &[(&str, &[&'static str], &[&'static str])], &[AppSettings]) = (
    "hd",
    "Generates an HD wallet (include -h for more options)",
    &[option::BITCOIN_FORMAT, option::BITCOIN_NETWORK, option::COUNT, option::DERIVATION, option::PASSWORD_GENERATE, option::WORD_COUNT],
    &[AppSettings::DisableHelpSubcommand, AppSettings::DisableVersion],
);

pub const IMPORT: (&str, &str, &[(&str, &[&'static str], &[&'static str])], &[AppSettings]) = (
    "import",
    "Imports a wallet (include -h for more options)",
    &[option::ADDRESS, option::BITCOIN_FORMAT, option::BITCOIN_NETWORK, option::PRIVATE, option::PUBLIC],
    &[AppSettings::DisableHelpSubcommand, AppSettings::DisableVersion, AppSettings::ArgRequiredElseHelp],
);

pub const IMPORT_HD: (&str, &str, &[(&str, &[&'static str], &[&'static str])], &[AppSettings]) = (
    "import-hd",
    "Imports an HD wallet (include -h for more options)",
    &[
        option::BITCOIN_FORMAT,
        option::BITCOIN_NETWORK,
        option::DERIVATION_IMPORT,
        option::EXTENDED_PUBLIC,
        option::EXTENDED_PRIVATE,
        option::INDEX,
        option::MNEMONIC,
        option::PASSWORD_IMPORT,
    ],
    &[AppSettings::DisableHelpSubcommand, AppSettings::DisableVersion, AppSettings::ArgRequiredElseHelp],
);