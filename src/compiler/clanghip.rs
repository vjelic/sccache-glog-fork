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
    gcc,
    Cacheable,
    CompilerArguments,
    write_temp_file,
};
use compiler::args::*;
use compiler::c::{CCompilerImpl, CCompilerKind, Language, ParsedArguments};
use compiler::gcc::GCCArgAttribute::*;
use futures::future::{self, Future};
use futures_cpupool::CpuPool;
use mock_command::{
    CommandCreator,
    CommandCreatorSync,
    RunCommand,
};
use std::ffi::OsString;
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
pub struct ClangHIP;

impl CCompilerImpl for ClangHIP {
    fn kind(&self) -> CCompilerKind { CCompilerKind::ClangHIP }
    fn parse_arguments(&self,
                       arguments: &[OsString],
                       cwd: &Path) -> CompilerArguments<ParsedArguments>
    {
        gcc::parse_arguments(arguments, cwd, (&gcc::ARGS[..], &ARGS[..]))
    }

    fn preprocess<T>(&self,
                     creator: &T,
                     executable: &Path,
                     parsed_args: &ParsedArguments,
                     cwd: &Path,
                     env_vars: &[(OsString, OsString)])
                     -> SFuture<process::Output> where T: CommandCreatorSync
    {
        gcc::preprocess(creator, executable, parsed_args, cwd, env_vars, self.kind())
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
        gcc::compile(creator, executable, parsed_args, cwd, env_vars, self.kind())
    }
}

static ARGS: [(ArgInfo, gcc::GCCArgAttribute); 8] = [
    take_arg!("--serialize-diagnostics", String, Separated, PassThrough),
    take_arg!("--target", String, Separated, PassThrough),
    take_arg!("-Xclang", String, Separated, PassThrough),
    flag!("-fcxx-modules", TooHard),
    flag!("-fmodules", TooHard),
    take_arg!("-gcc-toolchain", String, Separated, PassThrough),
    take_arg!("-include-pch", Path, CanBeSeparated, PreprocessorArgument),
    take_arg!("-target", String, Separated, PassThrough),
];

