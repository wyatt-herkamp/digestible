use crate::digestible::internal_macros::impl_for_hashable_hack;
use crate::Digestible;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
impl_for_hashable_hack!(OsStr);
impl_for_hashable_hack!(OsString);
impl_for_hashable_hack!(PathBuf);
impl_for_hashable_hack!(Path);
