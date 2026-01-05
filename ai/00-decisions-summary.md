# shadcn-rs Project Decisions Summary

**Status**: All critical decisions made ✅
**Ready for**: Phase 0 implementation
**Date**: 2026-01-05

## Project Overview

Building a comprehensive UI component library for Rust/WebAssembly that provides shadcn/ui compatible components. This enables developers to build modern web applications using Rust with a familiar component API.

## Key Decisions

### Technical Stack
- **Language**: Rust 2024 edition
- **Framework**: Yew (React-like framework for WASM)
- **Styling**: Complete CSS file shipped with library
- **Testing**: wasm-bindgen-test for UI integration tests
- **Browser Support**: Last 2 versions of evergreen browsers

### Project Scope
- **Components**: All 59 shadcn/ui components across 8 tiers
- **Icons**: Port Lucide icons to Rust/Yew (separate shadcn-icons crate)
- **Charts**: Full-featured charting system (Bar, Line, Area, Pie, Donut)
- **Touch Gestures**: Drawer, Carousel, Tabs, Dialog
- **Publication**: Wait until v1.0 complete (no early releases)

### Architecture Decisions

#### 1. CSS Strategy
**Decision**: Ship complete CSS file with library

Users include: `<link rel="stylesheet" href="shadcn-rs.css">`
- Pre-generated CSS with utility classes
- CSS custom properties for theming
- Light and dark mode support
- Users can override styles with custom CSS

#### 2. Positioning (v1.0)
**Decision**: Simple CSS-based positioning

- Use `position: absolute` with transforms
- Support top, right, bottom, left positions
- Basic viewport awareness
- Advanced features (collision detection, auto-flip) → v1.1+

#### 3. Animations
**Decision**: CSS animations and transitions only

- Pure CSS for all animations
- Class-based triggers (`.enter`, `.exit`, `.active`)
- Common animations: slide, fade, expand, collapse
- Configurable via CSS variables

#### 4. Icons
**Decision**: Create Rust icon library (port Lucide)

- Separate `shadcn-icons` crate
- All Lucide icons as Yew components
- Components accept `icon: Option<Html>` props
- Users can use icon library or custom SVG

Example:
```rust
use shadcn_icons::Check;
html! {
    <Button icon={html! { <Check /> }}>
        { "Click me" }
    </Button>
}
```

#### 5. Form Validation
**Decision**: Simple callback-based validation

- Components accept `validate: Option<Callback<String, Result<(), String>>>`
- Flexible, no heavy framework

Example:
```rust
let validate = Callback::from(|value: String| {
    if value.is_empty() {
        Err("Required".to_string())
    } else {
        Ok(())
    }
});
```

#### 6. Virtual Scrolling
**Decision**: Add in post-v1.0 release

- Most use cases have < 100 items
- Document performance limits for v1.0
- Implement virtual scrolling in v1.1+

#### 7. Portal Implementation
**Decision**: Use web-sys to append to document.body

- Manual DOM manipulation
- `window()?.document()?.body()?.append_child()`
- Clean up on unmount
- Support custom target containers

#### 8. Component API
**Decision**: Multiple components (compound pattern)

```rust
html! {
    <Card>
        <CardHeader>
            <CardTitle>{"Title"}</CardTitle>
        </CardHeader>
        <CardContent>{"Content"}</CardContent>
    </Card>
}
```

#### 9. Error Handling
**Decision**: Panic for dev errors, callbacks for user errors

- Developer errors (invalid props): `panic!` with helpful messages
- User errors (validation): Error callbacks

#### 10. TypeScript Definitions
**Decision**: Add in post-v1.0 release

- Not critical for v1.0
- Can generate .d.ts files in v1.1+ if requested

#### 11. Charts
**Decision**: Full-featured like recharts

Comprehensive charting:
- Bar Chart (vertical, horizontal, stacked)
- Line Chart (single, multi-line)
- Area Chart
- Pie Chart
- Donut Chart
- Customizable axes, legends, tooltips
- Interactive features, responsive sizing

#### 12. Touch Gestures
**Decision**: Implement for key mobile components

Components with touch support:
- **Drawer**: Swipe up/down to close
- **Carousel**: Swipe left/right to navigate
- **Tabs**: Swipe to switch tabs
- **Dialog**: Swipe down to close

Implementation:
- Touch event listeners (touchstart, touchmove, touchend)
- Swipe detection with configurable thresholds

#### 13. Bundle Size
**Decision**: No hard limits, optimize as needed

- Measure and optimize during development
- Focus on reasonable performance vs arbitrary numbers

#### 14. Implementation Phasing
**Decision**: Tier 1 sequential, then parallelize

- Implement Tier 1 (10 components) sequentially
- Establish patterns, coding style, best practices
- Parallelize Tier 2+ once patterns established

#### 15. Publishing Strategy
**Decision**: Launch with v1.0 (no early releases)

- Wait until all components complete
- No alpha/beta versions
- Launch with polished, complete v1.0.0
- Prevents API churn, ensures good first impression

## Workspace Structure

```
shadcn-rs/
├── Cargo.toml                    # Workspace manifest
├── shadcn-rs/                    # Library crate (components)
├── shadcn-icons/                 # Icon library crate
└── shadcn-showcase/              # Showcase binary
```

## Component Tiers

### Tier 1 - Foundational (10 components)
Badge, Button, Label, Separator, Skeleton, Spinner, Kbd, Typography, Avatar, Alert

### Tier 2 - Form Components (8 components)
Input, Textarea, Checkbox, Switch, Radio Group, Native Select, Slider, Progress

### Tier 3 - Layout & Structure (8 components)
Card, Aspect Ratio, Scroll Area, Resizable, Tabs, Table, Empty, Item

### Tier 4 - Interactive (7 components)
Button Group, Input Group, Field, Collapsible, Accordion, Toggle, Toggle Group

### Tier 5 - Overlays & Popups (7 components)
Dialog, Alert Dialog, Popover, Tooltip, Hover Card, Sheet, Drawer

### Tier 6 - Navigation (7 components)
Breadcrumb, Navigation Menu, Menubar, Dropdown Menu, Context Menu, Pagination, Sidebar

### Tier 7 - Advanced Forms (7 components)
Form, Select, Combobox, Command, Input OTP, Date Picker, Calendar

### Tier 8 - Complex (5 components)
Carousel, Data Table, Chart (full-featured), Toast, Sonner

**Total**: 59 components

## Post-v1.0 Features

Features deferred to v1.1+:
- Virtual scrolling for large lists
- Advanced positioning (Floating UI-style)
- TypeScript definitions
- Additional chart types
- Performance optimizations

## Success Criteria

- [x] All critical decisions made
- [ ] All 59 components implemented
- [ ] Icon library with Lucide icons ported
- [ ] Full-featured charting system
- [ ] Touch gestures on mobile components
- [ ] 100% test coverage for component logic
- [ ] Accessibility audit passes (WCAG 2.1 AA)
- [ ] Documentation complete
- [ ] Showcase app demonstrates all components
- [ ] Published at v1.0.0 on crates.io

## Next Steps

1. ✅ All questions answered
2. ✅ PRD updated with decisions
3. ✅ Architecture documented
4. ✅ Implementation tasks updated
5. ⏭️ **Begin Phase 0: Project Setup**

Ready to start coding! 🚀
