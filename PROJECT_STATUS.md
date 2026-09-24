# Project Status - shadcn-rs

**Last Updated**: 2026-04-17
**Current Phase**: Broad component coverage complete, implementation polish in progress

## Snapshot

- Workspace crates: `shadcn-rs`, `shadcn-icons`, `shadcn-showcase`
- Component modules in `shadcn-rs`: 60
- Showcase component pages: 59 plus shared direction/provider utilities
- Current verification baseline: `cargo check` and `cargo test`

## What Exists

- Core infrastructure is in place: CSS variables, base styles, utility helpers, hooks, and portal support.
- The library exports a full shadcn/ui-style component surface rather than a partial starter set.
- The showcase application covers the exported components and provides live examples for the library surface.

## Current Focus

The highest-priority work is implementation depth and polish rather than breadth:

1. Finish complex components so they behave like real widgets, not shells.
2. Tighten accessibility and focus management for overlay components.
3. Expand the icon crate beyond the minimal starter set.
4. Keep docs aligned with the actual repository state.

## Recent Direction

- `Chart` is being moved from placeholder output toward real SVG rendering.
- `DataTable` is being upgraded to use filtering, sorting, pagination, and selection behaviorally.
- `Carousel` is being upgraded to use controlled/uncontrolled slide state, navigation, autoplay, and indicators.
- `Calendar` and `DatePicker` are being aligned so the picker uses the custom calendar rather than native browser date UI.
- Modal-family components are being tightened around focus trapping and focus restoration.

## Known Gaps

- Some advanced components still need interaction polish and stronger behavioral tests.
- The icon crate is not yet a full Lucide port.
- Documentation is still catching up to the actual implementation level in the repo.

## Success Criteria For Near-Term Polish

- Complex components should use their public props meaningfully.
- Overlay components should match their accessibility claims.
- Showcase examples should demonstrate real behavior, not only surface API shape.
- Status docs should reflect the live repository instead of the initial scaffold phase.
