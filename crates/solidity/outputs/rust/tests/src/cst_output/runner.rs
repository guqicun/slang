use std::str::FromStr;

use anyhow::Result;
use codegen_utils::context::CodegenContext;
use semver::Version;
use solidity_rust_lib::generated::{
    kinds::ProductionKind,
    language::{Language, ParserOutput},
};
use solidity_testing_utils::cst_snapshots::ParserOutputTestSnapshotExtensions;

use crate::cst_output::generated::BREAKING_VERSIONS;

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let test_dir = codegen
            .repo_root
            .join("crates/solidity/testing/snapshots/cst_output")
            .join(parser_name)
            .join(test_name);

        let input_path = test_dir.join("input.sol");
        let source_id = input_path
            .strip_prefix(&codegen.repo_root)?
            .to_str()
            .unwrap();

        let source = &std::fs::read_to_string(&input_path)?;

        let mut last_output: Option<ParserOutput> = None;

        for version in BREAKING_VERSIONS {
            let version = Version::parse(version)?;
            let snapshot_path = test_dir.join(format!("generated/{version}.snap"));

            let output = Language::new(version)
                .get_parser(
                    ProductionKind::from_str(parser_name)
                        .expect(format!("No such parser: {}", parser_name).as_str()),
                )
                .parse(&source);

            if let Some(last_output) = &last_output {
                if &output == last_output {
                    // Skip versions that produce the same output.
                    continue;
                }
            }

            let snapshot = output.to_test_snapshot(source_id, source)?;
            codegen.write_file(&snapshot_path, &snapshot)?;

            last_output = Some(output);
        }

        return Ok(());
    });
}