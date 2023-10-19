use cargo_metadata::Metadata;
use clippy_utils::diagnostics::span_lint;
use rustc_lint::LateContext;
use rustc_span::source_map::DUMMY_SP;

use super::INVALID_CRATE_NAME;

pub(super) fn check(cx: &LateContext<'_>, metadata: &Metadata) {
    for package in &metadata.packages {
        if !is_valid_crate_name(&package.name) {
            span_lint(
                cx,
                INVALID_CRATE_NAME,
                DUMMY_SP,
                &format!("crate name `{}` not start with ylong_ or huawei_ .", package.name),
            );
        }
    }
}

fn is_valid_crate_name(name: &str) -> bool {
    let prefix_names = vec!["ylong_", "huawei_"];
    prefix_names.iter().any(|p| name.starts_with(p))
}
