use semver::{BuildMetadata, Version};

use crate::{
    analyzer::version_strategy::{
        context::Context, date::DateParts, traits::VersionStrategy,
    },
    result::Result,
};

#[derive(Default)]
pub struct DateWithTimeMicroVersionStrategy;

impl VersionStrategy for DateWithTimeMicroVersionStrategy {
    fn calculate_next_version(&self, ctx: &Context) -> Result<Version> {
        let parts = DateParts::now();
        let mut version = parts.version(ctx.config.version_type);
        // Padded to the six digits the microsecond field can hold, for the
        // same lexical-sort reason as the time segments.
        version.build = BuildMetadata::new(&format!(
            "{}.{:06}",
            parts.time_build_metadata(),
            parts.micro
        ))?;
        Ok(version)
    }
}
