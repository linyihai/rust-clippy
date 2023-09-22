use super::IMPLICIT_DEPENDENCIES;
use cargo_metadata::Metadata;
use clippy_utils::diagnostics::span_lint;
use if_chain::if_chain;
use rustc_lint::LateContext;
use rustc_span::source_map::DUMMY_SP;
use semver;

pub(super) fn check(cx: &LateContext<'_>, metadata: &Metadata) {
    for dep in &metadata.packages[0].dependencies {
        if_chain! {
            if let Some(ref source) = dep.source;
            if !source.starts_with("git");
            if !is_explicit_version(&dep.req);
            then {
                span_lint(
                    cx,
                    IMPLICIT_DEPENDENCIES,
                    DUMMY_SP,
                    &format!("implicit dependency for `{}`", dep.name),
                );
            }
        }
    }
}

fn is_explicit_version(req: &semver::VersionReq) -> bool {
    if req.comparators.len() != 1 {
        return false;
    }

    let major = req.comparators[0].major;
    let minor = req.comparators[0].minor.unwrap_or_default();
    let patch = req.comparators[0].patch.unwrap_or_default();

    if let Ok(expect_version) = semver::VersionReq::parse(&format!("{major}.{minor}.{patch}")) {
        return req == &expect_version;
    }
    false
}
