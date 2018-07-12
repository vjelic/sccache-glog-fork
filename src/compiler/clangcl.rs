// Copyright 2016 Mozilla Foundation
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_imports,dead_code,unused_variables)]

use ::compiler::{
    msvc,
    Cacheable,
    CompilerArguments,
    write_temp_file,
};
use compiler::args::*;
use compiler::c::{CCompilerImpl, CCompilerKind, Language, ParsedArguments};
use compiler::msvc::MSVCArgAttribute::*;
use futures::future::{self, Future};
use futures_cpupool::CpuPool;
use mock_command::{
    CommandCreator,
    CommandCreatorSync,
    RunCommand,
};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{
    self,
    Write,
};
use std::path::Path;
use std::process;
use util::{run_input_output, OsStrExt};

use errors::*;

/// A unit struct on which to implement `CCompilerImpl`.
#[derive(Clone, Debug)]
pub struct ClangCl;

impl CCompilerImpl for ClangCl {
    fn kind(&self) -> CCompilerKind { CCompilerKind::ClangCl }
    fn parse_arguments(&self,
                       arguments: &[OsString],
                       cwd: &Path) -> CompilerArguments<ParsedArguments>
    {
        msvc::parse_arguments(arguments, &msvc::ARGS[..])
    }

    fn preprocess<T>(&self,
                     creator: &T,
                     executable: &Path,
                     parsed_args: &ParsedArguments,
                     cwd: &Path,
                     env_vars: &[(OsString, OsString)])
                     -> SFuture<process::Output> where T: CommandCreatorSync
    {
        msvc::preprocess(creator, executable, parsed_args, cwd, env_vars, "")
    }

    fn compile<T>(&self,
                  creator: &T,
                  executable: &Path,
                  parsed_args: &ParsedArguments,
                  cwd: &Path,
                  env_vars: &[(OsString, OsString)])
                  -> SFuture<(Cacheable, process::Output)>
        where T: CommandCreatorSync
    {
        msvc::compile(creator, executable, parsed_args, cwd, env_vars)
    }
}

// TODO: add some unit tests
