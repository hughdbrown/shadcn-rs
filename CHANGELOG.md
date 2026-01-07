# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-07

### Added

#### Components (59 total)

**Tier 1 - Foundational (10 components)**
- Badge - Status indicators with variants
- Button - Interactive button with variants, sizes, loading state
- Label - Form field labels with required indicator
- Separator - Horizontal and vertical dividers
- Skeleton - Loading placeholder animations
- Spinner - Loading spinner with sizes
- Kbd - Keyboard shortcut display
- Typography - Headings, paragraphs, and text styling
- Avatar - User avatars with image, initials, and fallback
- Alert - Dismissible alert messages with variants

**Tier 2 - Form Components (9 components)**
- Input - Text input with types, states, prefix/suffix
- Textarea - Multi-line text with auto-resize, character count
- Checkbox - Checkbox with indeterminate state
- Switch - Toggle switch
- Radio Group - Radio button groups
- Native Select - Browser-native select dropdown
- Slider - Range slider with single/dual handles
- Progress - Progress bar with determinate/indeterminate modes
- Form - Form container with validation support

**Tier 3 - Layout & Structure (8 components)**
- Card - Card container with header, content, footer
- Aspect Ratio - Constrained aspect ratio container
- Scroll Area - Custom scrollbar styling
- Resizable - Resizable panel layout
- Tabs - Tabbed interface
- Table - Data table with header, body, footer
- Empty - Empty state placeholder
- Item - Generic list item

**Tier 4 - Interactive (7 components)**
- Button Group - Grouped buttons
- Input Group - Input with addons
- Field - Form field wrapper with label and error
- Collapsible - Expandable content section
- Accordion - Accordion with single/multiple expand
- Toggle - Toggle button with pressed state
- Toggle Group - Group of toggle buttons

**Tier 5 - Overlays & Popups (7 components)**
- Dialog - Modal dialog with focus trap
- Alert Dialog - Confirmation dialog
- Popover - Floating content panel
- Tooltip - Hover tooltip
- Hover Card - Rich hover preview
- Sheet - Side panel overlay
- Drawer - Mobile-friendly drawer with swipe

**Tier 6 - Navigation (7 components)**
- Breadcrumb - Navigation breadcrumbs
- Navigation Menu - Desktop navigation
- Menubar - Application menubar
- Dropdown Menu - Dropdown with items, checkboxes, radio
- Context Menu - Right-click context menu
- Pagination - Page navigation
- Sidebar - Collapsible sidebar navigation

**Tier 7 - Advanced Forms (7 components)**
- Select - Custom select dropdown
- Combobox - Searchable select
- Command - Command palette
- Input OTP - One-time password input
- Date Picker - Date selection
- Calendar - Date grid with navigation

**Tier 8 - Complex (5 components)**
- Carousel - Image/content carousel with swipe
- Data Table - Sortable, filterable data table
- Chart - Bar, line, area, pie charts
- Toast - Toast notifications
- Sonner - Advanced toast system

#### Core Features
- Type-safe variants, sizes, and colors using Rust enums
- CSS custom properties for theming
- Light and dark mode support
- WCAG 2.1 AA accessibility compliance
- Full keyboard navigation
- Touch gesture support for mobile
- Focus trap for overlays
- Portal rendering for overlays

#### Hooks
- `use_toggle` - Boolean state toggle
- `use_click_outside` - Detect clicks outside element
- `use_escape_key` - Handle Escape key press
- `use_controllable_state` - Controlled/uncontrolled pattern

#### Utilities
- ARIA attribute helpers
- Class name merging utilities
- Portal component for rendering to body
- Touch gesture detection

#### Documentation
- Installation guide
- Theming guide
- Accessibility guide
- API documentation (rustdoc)
- Interactive showcase application

#### Infrastructure
- CI/CD with GitHub Actions
- Clippy linting (warnings as errors)
- Rustfmt formatting
- 325+ unit tests
- Documentation tests

### Fixed
- All clippy warnings resolved
- Code formatted with rustfmt

---

[0.1.0]: https://github.com/hughdbrown/shadcn-rs/releases/tag/v0.1.0
