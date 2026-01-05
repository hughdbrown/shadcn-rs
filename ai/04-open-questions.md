# Open Questions & Clarifications - RESOLVED

This document tracked critical decisions needed before implementation. All questions have been answered and decisions are documented below.

## Decisions Summary

1. **CSS Strategy**: Ship complete CSS file with library
2. **Positioning**: Simple CSS for v1.0, enhance later
3. **Animations**: CSS animations/transitions only
4. **Icons**: Create Rust icon library (port Lucide icons)
5. **Validation**: Simple validator functions/closures
6. **Virtual Scrolling**: Add in later version (post v1.0)
7. **Portal**: Use web-sys to append to document.body
8. **Browser Support**: Last 2 versions of evergreen browsers
9. **Bundle Size**: No specific limits, optimize as needed
10. **Component API**: Multiple components (compound pattern)
11. **Error Handling**: Panic for dev errors, callbacks for user errors
12. **TypeScript**: Add in later version (post v1.0)
13. **Charts**: Full-featured like recharts
14. **Touch Gestures**: Drawer, Carousel, Tabs, Dialog all support touch
15. **Phasing**: Tier 1 sequential, then parallelize Tier 2+
16. **Publishing**: Wait until all components done for v1.0

## Detailed Questions & Answers

### 1. CSS Generation Strategy

**Question:** How should we generate and distribute the CSS file?

**Options:**
a) Hand-write CSS file, include in library as static asset
b) Use a CSS generator/build step (like Tailwind CLI)
c) Embed CSS as const str in Rust code
d) Let users bring their own CSS (provide reference implementation)

**Recommendation:** Option (a) - Hand-write CSS, include as static asset
- Most straightforward for initial implementation
- Users include CSS file via `<link>` tag in HTML
- Can migrate to Tailwind CLI later if needed

**DECIDED:** Ship complete CSS file with library. Users include via `<link rel="stylesheet" href="shadcn-rs.css">`. CSS will be bundled with the crate and can be served from the build output. Users can override any styles with their own CSS.

---

### 2. Positioning/Floating Logic for Overlays

**Question:** How should we handle complex positioning for Popover, Tooltip, Dropdown, etc.?

**Background:** shadcn/ui uses Radix UI which uses Floating UI for positioning. We need a Rust/WASM equivalent.

**Options:**
a) Port Floating UI logic to Rust
b) Use simpler CSS-based positioning (position: absolute with transforms)
c) Bind to Floating UI via JS interop
d) Use web-sys to calculate positions manually

**Recommendation:** Start with (b) CSS-based positioning for MVP, consider (a) or (d) for v1.0
- CSS-based is simpler and covers 80% of use cases
- Can add advanced positioning later

**DECIDED:** Simple CSS-based positioning for v1.0. Use `position: absolute` with transforms and basic positioning logic (top, right, bottom, left). This covers 80% of use cases. Advanced positioning with collision detection, auto-flipping, and viewport awareness will be added in v1.1+.

---

### 3. Animation Strategy

**Question:** How should we handle component animations (Dialog slide-in, Accordion expand, etc.)?

**Options:**
a) CSS animations/transitions only
b) Use web-sys to manipulate styles
c) Use gloo-timers for animation frames
d) Bind to JS animation library

**Recommendation:** Option (a) - CSS animations
- Performant, declarative
- Works well with WASM
- Matches shadcn/ui approach

**DECIDED:** CSS animations and transitions only. Use CSS classes for animations (slide-in, fade, expand, etc.) triggered by state changes. This is performant, declarative, and matches shadcn/ui's approach.

---

### 4. Icon Support

**Question:** How should users add icons to components (Button, Alert, etc.)?

**Background:** shadcn/ui uses Lucide React icons. We need a Rust solution.

**Options:**
a) Accept `Html` as icon prop, users provide their own SVG
b) Create Rust icon library (port Lucide icons)
c) Accept icon name as string, render from embedded SVG set
d) Recommend external icon library (if one exists)

**DECIDED:** Create a Rust icon library by porting Lucide icons. This will be a separate crate (`shadcn-icons` or similar) that provides all Lucide icons as Yew components. Components will accept `icon: Option<Html>` props for flexibility, and users can use the icon library or provide their own SVG.

Example usage:
```rust
use shadcn_icons::Check;
html! {
    <Button icon={html! { <Check /> }}>
        { "Click me" }
    </Button>
}
```

---

### 5. Form Validation System

**Question:** How should Form component handle validation?

**Background:** shadcn/ui integrates with React Hook Form or Zod. We need a Rust approach.

**Options:**
a) Simple validator functions (closures)
b) Create validation framework similar to validator.rs
c) Don't include validation, let users handle it
d) Use existing Rust validation crate (if compatible with WASM)

**DECIDED:** Simple validator functions using callbacks. Components accept `validate: Option<Callback<String, Result<(), String>>>` props. This is flexible and doesn't require a heavy framework.

Example:
```rust
let validate = Callback::from(|value: String| {
    if value.is_empty() {
        Err("Field is required".to_string())
    } else {
        Ok(())
    }
});

html! {
    <Input validate={validate} />
}
```

---

### 6. Virtual Scrolling for Large Lists

**Question:** Should Select/Combobox support virtual scrolling for performance with large lists?

**Options:**
a) Yes, implement virtual scrolling
b) No, document performance limits (e.g., 1000 items max)
c) Add in later version

**DECIDED:** Add virtual scrolling in a later version (post v1.0). Most use cases have fewer than 100 items. For v1.0, document performance recommendations (e.g., max 500-1000 items). Virtual scrolling can be added in v1.1+ if there's demand.

---

### 7. Portal Implementation

**Question:** How should Portal render components outside the DOM hierarchy?

**Background:** Needed for Dialog, Popover, etc. to render at document.body level.

**Options:**
a) Use web-sys to manually append to document.body
b) Use Yew's create_portal (if available)
c) Use gloo-utils for DOM manipulation

**DECIDED:** Use web-sys to manually append elements to `document.body`. This is the standard approach and works reliably. Implementation will use `web_sys::window()?.document()?.body()?.append_child()` to render components outside their parent hierarchy.

---

### 8. Testing Infrastructure Details

**Question:** What specific web-sys features do we need for wasm-bindgen-test?

**Action needed:** Create list of required web-sys features for:
- Event handling (MouseEvent, KeyboardEvent, InputEvent, etc.)
- DOM queries (querySelector, etc.)
- Element properties (className, innerHTML, etc.)

---

### 9. Browser Support Matrix

**Question:** What minimum browser versions should we support?

**DECIDED:** Support last 2 versions of evergreen browsers:
- Chrome/Edge: Last 2 versions
- Firefox: Last 2 versions
- Safari: Last 2 versions
- Mobile Safari: iOS 14+
- Chrome Mobile: Last 2 versions

This provides wide coverage while allowing use of modern web features.

---

### 10. Bundle Size Targets

**Question:** What are acceptable bundle size limits?

**DECIDED:** No specific hard limits. Optimize bundle size as needed during development. We'll measure actual sizes and optimize where beneficial, but won't constrain ourselves with arbitrary limits. Focus on reasonable performance rather than hitting specific numbers.

---

### 11. Component Granularity

**Question:** Should compound components (Card, Dialog, etc.) be:
a) Single component with sub-components as separate exports
b) Single component with builder pattern
c) Multiple components that work together

**Example:**

Option (a) - Current recommendation:
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

Option (b):
```rust
html! {
    <Card
        header={html! { "Title" }}
        content={html! { "Content" }}
    />
}
```

**DECIDED:** Multiple separate components (compound pattern). This matches shadcn/ui's API and provides maximum flexibility:
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

---

### 12. Error Handling Strategy

**Question:** How should components handle errors?

**Options:**
a) Panic on critical errors, log warnings for non-critical
b) Return Result types (doesn't work well with Yew)
c) Use error callback props
d) Use global error boundary

**DECIDED:** Panic for developer errors, use callbacks for user errors.
- Developer errors (invalid props, configuration mistakes): Use `panic!` or `assert!` with helpful messages
- User errors (validation failures, input errors): Use error callbacks that return `Result<(), String>`

Example:
```rust
// Developer error - panic
assert!(!items.is_empty(), "Select requires at least one item");

// User error - callback
if let Some(validator) = &props.validate {
    match validator.emit(value.clone()) {
        Err(e) => set_error(Some(e)),
        Ok(_) => set_error(None),
    }
}
```

---

### 13. TypeScript Definitions

**Question:** Should we generate TypeScript definitions for better IDE support?

**Background:** Users of the WASM module might benefit from TS types.

**Options:**
a) Yes, use wasm-bindgen to generate .d.ts files
b) No, this is a Rust library
c) Add in later version

**DECIDED:** Add TypeScript definitions in a later version (post v1.0). This is not critical for the initial release. If users request it, we can generate .d.ts files using wasm-bindgen in v1.1+.

---

### 14. Chart Component Complexity

**Question:** How complex should Chart components be?

**Options:**
a) Basic charts only (Bar, Line, Pie) with limited customization
b) Full-featured like recharts
c) Very basic, recommend external charting library

**DECIDED:** Full-featured charts like recharts. Implement comprehensive charting capabilities including:
- Bar Chart (vertical, horizontal, stacked)
- Line Chart (single, multi-line, area)
- Pie Chart and Donut Chart
- Area Chart
- Customizable axes, legends, tooltips
- Responsive sizing
- Interactive features (hover, click)

This will be a significant component but provides complete charting functionality for users.

---

### 15. Mobile Touch Gestures

**Question:** Should components like Drawer, Carousel support touch gestures?

**Options:**
a) Yes, implement swipe/drag gestures
b) No, click/tap only
c) Add in later version

**DECIDED:** Implement touch gesture support for:
- **Drawer**: Swipe up/down to close
- **Carousel**: Swipe left/right to navigate between items
- **Tabs**: Swipe to switch between tabs
- **Dialog**: Swipe down to close (mobile-friendly dismissal)

These components will use touch event listeners (touchstart, touchmove, touchend) to detect and handle swipe gestures.

---

## Implementation Readiness Checklist

All decisions have been made. Ready to begin implementation! ✅

- [x] **CSS Strategy** - Ship complete CSS file with library
- [x] **Positioning Logic** - Simple CSS for v1.0, enhance later
- [x] **Animation Strategy** - CSS animations/transitions only
- [x] **Icon Support** - Port Lucide icons to Rust
- [x] **Form Validation** - Simple callback-based validation
- [x] **Portal Implementation** - Use web-sys to append to document.body
- [x] **web-sys Features** - Will be identified during implementation
- [x] **Browser Support** - Last 2 versions of evergreen browsers
- [x] **Bundle Size Targets** - No hard limits, optimize as needed
- [x] **Component API** - Multiple components (compound pattern)
- [x] **Error Handling** - Panic for dev errors, callbacks for user errors
- [x] **Chart Scope** - Full-featured like recharts
- [x] **Touch Gestures** - Drawer, Carousel, Tabs, Dialog
- [x] **TypeScript** - Add in later version
- [x] **Virtual Scrolling** - Add in later version
- [x] **Publishing** - Wait until all components done for v1.0

## Additional Considerations - DECIDED

### Phased Implementation

**DECIDED:** Implement Tier 1 components sequentially to establish consistent patterns, coding style, and best practices. After Tier 1 is complete and patterns are established, parallelize work on Tier 2+ components.

### Early Feedback

**DECIDED:** Do NOT publish early versions. Wait until all components are implemented and v1.0 is ready. This prevents API churn and ensures a polished first impression.

### Documentation-First Approach

**DECIDED:** Write documentation alongside implementation. For each component:
1. Design the API (props, variants, etc.)
2. Write rustdoc comments documenting the API
3. Implement the component
4. Write tests
5. Create example

This ensures documentation is accurate and helps clarify design decisions.

---

## Next Steps - READY TO BEGIN! ✅

All questions have been answered. The project is ready for implementation:

1. ✅ All 15 critical questions answered
2. ✅ Additional considerations decided
3. 🔄 Update PRD with decisions (in progress)
4. 🔄 Update architecture document (in progress)
5. 🔄 Update implementation tasks (in progress)
6. ⏭️  Begin Phase 0: Project Setup

## Key Additions to Scope

Based on decisions, we will also create:

- **shadcn-icons** crate: Port of Lucide icons for Rust/Yew
- Full-featured charting system (comprehensive, not basic)
- Touch gesture support utilities for mobile interactions
- Advanced positioning system (v1.1+)
- Virtual scrolling (v1.1+)
- TypeScript definitions (v1.1+ if requested)
