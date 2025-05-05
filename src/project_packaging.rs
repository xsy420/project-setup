use std::fmt::Debug;
use strum::Display;

#[derive(Debug, Clone, Default, Display)]
#[allow(dead_code)]
enum ProjectPackaging {
    #[default]
    NotNeed,
    Alpine,
    ArchLinux,
    Fedora,
    Gentoo,
    Nix,
    Ubuntu,
}
