// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use std::fs;
use std::io;
//use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use compile::Profile;
use decl_provider::SelfProvider;
use ocamlrep::rc::RcOc;
use parser_core_types::source_text::SourceText;
use relative_path::Prefix;
use relative_path::RelativePath;

use crate::compile::SingleFileOpts;
use crate::FileOpts;

#[derive(Parser, Debug)]
pub struct Opts {
    /// Output file. Creates it if necessary
    #[clap(short = 'o')]
    output_file: Option<PathBuf>,

    #[clap(flatten)]
    files: FileOpts,

    #[clap(flatten)]
    single_file_opts: SingleFileOpts,

    /// Skip emitting 'standard' builtins.
    #[clap(long)]
    no_builtins: bool,
}

pub fn run(opts: Opts) -> Result<()> {
    let files = opts.files.gather_input_files()?;

    let mut stdout = io::stdout();

    for path in files {
        let pre_alloc = bumpalo::Bump::default();
        let content = fs::read(&path)?;
        let unit = compile_php_file(&pre_alloc, &path, &content, &opts.single_file_opts)?;

        textual::textual_writer(&mut stdout, &path, unit, opts.no_builtins)?;
    }

    Ok(())
}

pub(crate) fn compile_php_file<'a, 'arena>(
    alloc: &'arena bumpalo::Bump,
    path: &'a Path,
    content: &[u8],
    single_file_opts: &'a SingleFileOpts,
) -> Result<ir::Unit<'arena>> {
    let filepath = RelativePath::make(Prefix::Dummy, path.to_path_buf());
    let source_text = SourceText::make(RcOc::new(filepath.clone()), content);
    let env = crate::compile::native_env(filepath, single_file_opts);
    let mut profile = Profile::default();
    let decl_arena = bumpalo::Bump::new();
    let decl_provider = SelfProvider::wrap_existing_provider(
        None,
        env.to_decl_parser_options(),
        source_text.clone(),
        &decl_arena,
    );
    let unit = compile::unit_from_text(alloc, source_text, &env, decl_provider, &mut profile)?;
    let ir = bc_to_ir::bc_to_ir(&unit, path);

    Ok(ir)
}
