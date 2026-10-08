# 24 — Dependency, version, and licensing policy

## Selected baseline
Effect 4.0.2 and published GPUI 0.2.2 are the observed baseline selections for the scaffolds, not claims of tested compatibility. Effect 4 was released on 30 September 2026. The reference TypeScript core was compiled with the installed TypeScript 5.8.3 and run on Node 22.16.0; those are preparation-environment facts, not a recommendation to ship that exact Node patch indefinitely. [S01–S05.]

Install selected packages on a connected development machine, commit real lockfiles, and record exact toolchain/native SDK versions. Do not invent a lockfile or checksum. Avoid wildcard dependencies, mutable Git branches, and unreviewed latest container tags. Container images used for repeatable environments should be digest-pinned after selection and scanning.

## Upgrade process
Use a scheduled dependency review owned by the engineering team. Apply security updates with urgency appropriate to exposure. For GPUI, replay input/accessibility/platform/performance tests. For Effect, check core versus unstable-module status and run interruption/resource/error tests. For agent adapters, run conformance and policy-negative cases. For workflow engines, run history replay/version migration and failure recovery tests.

An adapter interface does not make live workflow migration automatic. A native package bump does not prove Windows/Linux behavior. Document compatibility and rollback before rollout.

## Licensing and provenance
The checked GPUI package metadata declares Apache-2.0; do not generalize that to every Zed repository component. Review each copied or depended-on component and its actual license. [S06.] Prefer dependencies over copying large source fragments; preserve notices when reuse requires it. The handoff's original scaffold code is small and does not bundle framework source, fonts, SDK binaries, or external articles.

The user has not selected an open-source license for the future product. Do not silently add a permissive or copyleft product license or claim trademark clearance. Track third-party notices, generated code provenance, media assets, and connector dependencies. Get qualified review for distribution and commercial licensing decisions.

## Software supply chain
Generate an SBOM, scan dependencies and executable artifacts, verify publisher/signature where available, isolate package-install scripts during evaluation, and keep signing credentials out of developer workspaces. Connector updates and environment images can introduce executable code and require the same care as application dependencies.

## Package verification limits
External npm access failed in the preparation environment, and Rust/Cargo were not installed. The Effect and GPUI scaffolds therefore have no dependency-install, compile, or native-run evidence. Their first build tickets explicitly close those gaps. The dependency-free core, contract validation, and SQLite reference tests are separately recorded. No security audit or production deployment occurred.
