use std::sync::Arc;
use swc::config::{IsModule, SourceMapsConfig};
use swc_common::{errors::Handler, Mark, SourceFile, SourceMap, GLOBALS};
use swc_ecma_ast::{EsVersion, Program};
use swc_ecma_parser::Syntax;
use swc_ecma_transforms::{resolver, typescript};
use swc_ecma_visit::FoldWith;

fn parse(c: &swc::Compiler, fm: Arc<SourceFile>) -> Program {
    let handler = Handler::with_emitter_writer(Box::new(std::io::stderr()), Some(c.cm.clone()));

    let comments = c.comments().clone();

    c.parse_js(
        fm,
        &handler,
        EsVersion::EsNext,
        Syntax::Typescript(Default::default()),
        IsModule::Bool(false),
        Some(&comments),
    )
    .unwrap()
}

fn as_es(c: &swc::Compiler, fm: Arc<SourceFile>) -> Program {
    let program = parse(c, fm);
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();

    program
        .fold_with(&mut resolver(unresolved_mark, top_level_mark, true))
        .fold_with(&mut typescript::strip(top_level_mark))
}

fn to_js(c: &swc::Compiler, fm: Arc<SourceFile>) -> String {
    let program = as_es(&c, fm);

    let output = c
        .print(
            &program,
            None,
            None,
            false,
            EsVersion::Es2020,
            SourceMapsConfig::Bool(true),
            &Default::default(),
            None,
            false,
            None,
            false,
            false,
            Default::default(),
        )
        .unwrap();

    output.code
}

pub fn read_script_file(path: &str) -> String {
    let file = std::path::Path::new(path)
        .canonicalize()
        .unwrap_or_else(|_| {
            eprintln!("Error: Invalid file path");
            std::process::exit(1);
        });

    let cm = Arc::<SourceMap>::default();

    let c = swc::Compiler::new(cm.clone());

    let fm = cm.load_file(&file).expect("failed to load file");

    GLOBALS.set(&Default::default(), || to_js(&c, fm.clone()))
}
