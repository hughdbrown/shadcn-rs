# Upstream parity research — batch E

Date: 2026-09-24. Upstream source: `https://ui.shadcn.com/docs/components/base/<name>` (Base UI flavor). All 10 pages loaded (HTTP 200).
Firecrawl's rendered markdown drops the code blocks, so I read the raw `.../<name>.md` versions of the same pages. I also read the linked Base UI API references (`https://base-ui.com/react/components/<name>.md`) and, where noted, the upstream registry source (`/r/styles/base-nova/<name>.json`).

Scope notes:
- `render` / `asChild` polymorphism is not compared anywhere. None of our components support it. This is intentional and applies to every component below.
- Tailwind class details and RTL are ignored.
- Our CSS hooks: `.tabs-trigger[data-state="active"]` / `.tabs-content[data-state="inactive"]` (`styles/components.css:1150-1162`), `.tooltip-content` (`:1333`), and no `.tooltip-root` rule exists.

Priority key: **HIGH** = broken or unusable, or an a11y failure. **MED** = a missing API that users will reach for. **LOW** = polish or rarely used.

---

## Spinner (`spinner.rs`)

No meaningful gaps. Upstream is a single `<Spinner />` (a Lucide `LoaderIcon` with `role="status"` and `aria-label="Loading"`), sized with `size-*` classes. Ours has `role="status"` and an `aria_label` prop (default "Loading"), plus `size: Size` and `color`, which is a superset. Small difference: the role sits on a wrapper `<div>` instead of the svg, which is fine. Upstream's docs show inline use inside Button, Badge, InputGroup and Empty, all of which work with any child element.

---

## Switch (`switch.rs`)

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Controlled mode is broken | controlled/uncontrolled | Base UI `checked` (controlled) + `defaultChecked` (uncontrolled) are distinct | `checked: bool` (not `Option<bool>`). `is_checked = if checked { checked } else { *internal }`, so a parent can never force the switch **off**. The click handler toggles internal state even when the switch is controlled | HIGH |
| `onCheckedChange(checked)` | event | Callback receives the new boolean | `onchange: Callback<Event>` gets a synthetic `MouseEvent`/`KeyboardEvent` cast to `Event`. The caller cannot learn the new value | HIGH |
| `aria-invalid` | prop / a11y | Documented "Invalid" example: `<Switch aria-invalid />` + `Field data-invalid` | No `aria_invalid` prop | MED |
| `readOnly` | prop | Base UI `readOnly` (+ `data-readonly`) | Missing | LOW |
| `uncheckedValue` / `inputRef` | prop | Form value when off; ref to hidden input | Missing (only `name`/`value`, and the hidden input renders only when `name` is set) | LOW |
| Hidden `<input>` nested inside `<button>` | behavior / HTML validity | Base UI renders the hidden input as a sibling | Interactive content inside a `<button>` is invalid HTML. `required` validation on it may not surface | LOW |
| `data-checked`/`data-unchecked`/`data-disabled` state attributes | behavior | Styling hooks used by `Field` (`group-has-…`) | Class-based only (`switch-checked`) | LOW |
| Sizes | size | `sm` \| `default` | `Size` enum (Xs…Xl2), which is a superset | — |

---

## Table (`table.rs`)

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `TableCaption` sub-component | sub-component | Composition lists `TableCaption` as a child of `Table` | Only a `caption: Option<AttrValue>` prop on `Table` (text only, no class, no rich children) | LOW |
| Row selection state | prop | Upstream `TableRow` styles `data-[state=selected]`, which Data Table uses for selected rows | `TableRow` has only `class`/`children`: no `selected`/`data_state`, no `onclick` | MED |
| Attribute passthrough on cells/rows | prop | Accepts all native `<td>/<th>/<tr>` props (`scope`, `onClick`, `aria-*`, `key`) | Only `colspan`/`rowspan`/`class`. There is no `scope` on `TableHead` (a11y) and no row click handler | MED |

Everything else matches: Header, Body, Footer, Row, Head, Cell and the scroll-wrapper `div` are all present.

---

## Tabs (`tabs.rs`)

The component has no working behavior. `Tabs` destructures `value`, `default_value` and `on_value_change` into `_`. No context is provided. Triggers have no click handler. The CSS keys on `data-state`, which is never emitted. As a result, every panel is always visible and clicking a trigger does nothing.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Selection state / context | behavior | Root owns the active value; Tab activates on click | No state, no context, no `onclick` on `TabsTrigger` | HIGH |
| `value` / `defaultValue` / `onValueChange` | controlled/uncontrolled + event | All three are wired in Base UI | Props exist but are ignored (`value: _, default_value: _, on_value_change: _`) | HIGH |
| Panel visibility | behavior | Inactive panels are hidden/unmounted (`data-hidden`) | `TabsContent` never sets `data-state`/`hidden`, so all panels render | HIGH |
| `aria-selected` / `data-active` | a11y | The active tab has `aria-selected="true"` | Hard-coded `aria-selected="false"` on every trigger | HIGH |
| Keyboard navigation | behavior / a11y | Arrow keys move focus (Left/Right or Up/Down by orientation), Home/End, roving `tabindex`, `loopFocus` default true | None | HIGH |
| Unique ids / `aria-labelledby` | a11y | Panel labelled by its tab; ids are unique per instance | Panel `id = "tabpanel-{value}"`, which collides across multiple `Tabs` on one page. Triggers have no `id`, and the panel has no `aria-labelledby` | MED |
| `TabsList variant="line"` | variant | `variant: "default" \| "line"` on `TabsList` | `TabsList` has no `variant` | MED |
| Orientation on the list | a11y | `aria-orientation` on tablist, `data-orientation` everywhere | Only on the root `div`. The tablist has no `aria-orientation` | LOW |
| `activateOnFocus` (List) | prop | Default `false` (manual activation) | Missing (moot until keyboard nav exists) | LOW |
| `keepMounted` (Panel) | prop | Keep hidden panel in the DOM | Missing | LOW |
| `Indicator` part | sub-component | Base UI `Tabs.Indicator` (not used by the shadcn wrapper) | Missing | LOW |
| Disabled trigger | prop | `disabled` on the trigger | Present | — |

---

## Textarea (`textarea.rs`)

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `default_value` ignored | controlled/uncontrolled | Native `defaultValue` works | Prop exists but is destructured as `_` and never rendered | MED |
| Auto-grow | behavior | Registry source uses `field-sizing-content min-h-16`, so the textarea grows with its content | Fixed `rows` (default 3) + `resize` enum. No auto-sizing | LOW |
| Native attr passthrough (`dir`, `wrap`, `spellcheck`, `form`, `onpaste`…) | prop | Spreads all `<textarea>` props | Fixed prop list (a broad one) | LOW |

`aria-invalid`, `disabled`, `rows` and `id` for Field pairing are all present.

---

## Toast (`toast.rs` + `sonner.rs`)

**What upstream "Toast" is now:**
- In the **Base UI** flavor, `/docs/components/base/toast` is built on **Base UI Toast**, not Sonner. `/docs/components/base/sonner` and `/docs/components/sonner` both redirect to it.
- It exposes an imperative manager, `const toast = createToastManager()`, with these methods:
  - `toast.add({ title, description, type, timeout, priority, actionProps, onClose, onRemove, data })`, which returns an id
  - `toast.close(id)`
  - `toast.update(id, …)`
  - `toast.promise(p, { loading, success: (data) => …, error })`
- `type` options are `success | info | warning | error | loading`, and each renders an icon.
- `priority: "high"` makes the toast announce assertively.
- `<Toaster />` is mounted once in the layout (Provider default `limit=3`, `timeout=5000`). It provides a Viewport with stacking/expand-on-hover, swipe-to-dismiss (`swipeDirection`) and pause on hover/focus.
- The exported parts are `Toaster, ToastProvider, ToastViewport, Toast, ToastContent, ToastTitle, ToastDescription, ToastAction, ToastClose, useToastManager, createToastManager`.
- In the **Radix** flavor, `/docs/components/radix/toast` says *"The toast component has been deprecated. Use the sonner component instead."* Radix Sonner offers `toast()`, `toast.success/info/warning/error/promise`, and a `position` option.
- Both flavors therefore share one concept: a single mounted Toaster plus an imperative `toast` API with types, action, description and promise.

**Ours:** two parallel *declarative* components and no manager. The caller has to keep its own `Vec` of toasts, render each one, and remove it in `on_close`/`on_dismiss`.
- `toast.rs`: `Toast` with `variant` (no Loading), a per-toast `position`, `duration` 5000, `action` + `on_action`, and `on_close`. Its role is `alert` for errors and `status` otherwise.
- `sonner.rs`: a `Sonner` container (`position`, `expand`, `gap`) and `SonnerToast` with `type` (includes Loading), `dismissible`, `duration` 4000, `action`, and `on_dismiss`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Imperative API (`add`/`close`/`update`) | event / API | `toast.add(opts) -> id`, `toast.close(id)`, `toast.update(id, opts)` callable from anywhere | None. The caller manages the toast list manually | HIGH |
| Single `Toaster` / viewport with stacking + `limit` | sub-component / behavior | One `<Toaster/>`; oldest toasts beyond `limit` (3) are marked `data-limited` and hidden | `Sonner` is a plain flex container with no limit and no stacking. `Toast` positions itself individually | HIGH |
| `toast.promise` | API | Loading → success(data)/error state transitions on one toast | Missing (could use `wasm_bindgen_futures::spawn_local`) | MED |
| Pause timer on hover/focus; window blur | behavior / a11y | Auto-dismiss pauses while hovered or focused | The timer runs regardless (`gloo Timeout`) | MED |
| `priority` → live-region politeness | a11y | `priority: "high"` announces assertively | `toast.rs` puts `role="alert"` + `aria-live="polite"` on the same node for errors. `sonner.rs` always uses `status` | MED |
| `loading` type | variant | Built-in | Present in `SonnerType`, missing in `ToastVariant` | LOW |
| `onRemove` (after exit animation), `data` payload | event | Base UI ToastManagerAddOptions | Missing | LOW |
| Swipe-to-dismiss, F6 viewport hotkey, expand-on-hover | behavior | Base UI Viewport/Root | Missing (`expand` is a static class only) | LOW |
| Per-type icon | behavior | Toaster renders an icon for each `type` | CSS class only, no icon | LOW |

**Recommendation: merge into one module and deprecate the other.** Keep `toast.rs`, whose names match upstream's current Base UI names (`Toaster`, `Toast`, `toast`), and rebuild it around:
- a `ToastProvider`/`Toaster` context that holds `Vec<ToastData>` and applies `limit` + a default `timeout`;
- a `use_toast()` hook returning a handle with `add(ToastOptions) -> ToastId`, `close(id)`, `update(id, …)` and `promise(fut, msgs)`;
- a `ToastType` taken from `SonnerType`, since it includes `Loading`;
- the `position`/`expand`/`gap` behavior taken from `Sonner`, moved onto `Toaster`.

After that, turn `sonner.rs` into `#[deprecated]` type aliases/re-exports (`Sonner` → `Toaster`, `SonnerToast` → `Toast`, `SonnerType` → `ToastType`) for one minor release, then delete it. This is a breaking API change: flag it in the CHANGELOG before the crates.io publish.

---

## Toggle (`toggle.rs`)

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `aria-label` passthrough | a11y | Every upstream example is icon-led and passes `aria-label` | No `aria_label` (or any attr passthrough) prop, so icon-only toggles have no accessible name | HIGH |
| `onPressedChange(pressed)` | event | Callback receives the new bool | `ontoggle: Callback<MouseEvent>`. The caller must invert state itself | MED |
| Controlled mode correctness | controlled/uncontrolled | `pressed` controlled, `defaultPressed` uncontrolled | Display honors `pressed: Option<bool>`, but the click handler flips *internal* state even when controlled, so the internal and external values drift | MED |
| `data-pressed` / `data-state` attribute | behavior | `data-pressed` present when on | Class `toggle-pressed` only (fine for our CSS) | LOW |
| `value` (for group use), `id`, `name` | prop | `value` identifies the toggle inside a group | Missing | LOW |
| Sizes / variants | size / variant | `sm \| default \| lg`; `default \| outline` | `Size` enum + `ToggleVariant::{Default, Outline}` (superset) | — |

---

## Toggle Group (`toggle_group.rs`)

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `onValueChange(groupValue: string[])` | event | Emits the resulting **array** of pressed values | `on_value_change: Callback<AttrValue>` emits only the *clicked item's* value. In Multiple mode the caller cannot tell add from remove, and in Single mode a deselect looks like a select | HIGH |
| `aria-label` on `ToggleGroupItem` | a11y | Every example passes `aria-label` on icon items | No `aria_label` prop, so icon-only items are unnamed | HIGH |
| Controlled `values` (multiple) not synced | controlled/uncontrolled | `value: string[]` fully controlled | Only the single `value` is synced via `use_effect_with`. `values` sets the initial state only | MED |
| `variant` on group → items | variant | `variant="outline"` on `ToggleGroup`, propagated via context | No `variant` on the group or the item. Items always render the default style | MED |
| Keyboard roving focus | behavior / a11y | Arrow keys move between items (respecting orientation), `loopFocus` default true, single tab stop | None; every item is a tab stop. Worse, Single mode uses `role="radiogroup"`/`radio`, which by ARIA pattern *requires* arrow-key navigation. Upstream Base UI uses plain pressed buttons | MED |
| `spacing` | prop | `spacing` (default **2** since 2026-05-17; `0` = connected) | Missing; layout fixed by CSS | LOW |
| API shape | prop | `multiple: bool` + `value/defaultValue: string[]` | `type: Single\|Multiple` + `value`/`default_value`/`values` (Radix-style). Workable, but diverges from Base UI | LOW |
| `default_value` for multiple | controlled/uncontrolled | `defaultValue: string[]` | `default_value` is a single string | LOW |
| Orientation, disabled, size | prop | Present upstream | Present | — |

---

## Tooltip (`tooltip.rs`)

This component is not functional either. `Tooltip` ignores `delay_duration` and `disabled`, and its `_is_open` state is never read. `TooltipContent` always renders. No CSS hides it: `.tooltip-content` has no hidden state, and the root's class `tooltip-root` has no `position: relative` rule, because the CSS styles `.tooltip`. The result is that every tooltip is permanently visible and positioned against an arbitrary ancestor.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Show/hide on hover **and focus**, hide on Escape / pointer leave / blur | behavior / a11y | Base UI opens on hover and keyboard focus, closes on Escape | Always visible; no handlers | HIGH |
| `open` / `defaultOpen` / `onOpenChange` | controlled/uncontrolled + event | Present on Root | Missing | HIGH |
| `aria-describedby` trigger → content, unique id | a11y | Trigger is described by the popup | No ids or linkage | MED |
| Provider delay semantics | behavior | shadcn `TooltipProvider` defaults `delay=0`; Base UI Provider `delay`, `closeDelay`, `timeout` (400 ms, so the next tooltip opens instantly). Trigger `delay` default 600, `closeDelay` 0 | `TooltipProvider` publishes `delay_duration`/`skip_delay_duration` context that nothing consumes. `Tooltip.delay_duration` is ignored | MED |
| Trigger element | behavior / a11y | Trigger renders a focusable `<button>` (or the render target) | `TooltipTrigger` renders a `<div>` wrapper. It is focusable only if the child is, and focus is never listened for anyway | MED |
| Portal + collision avoidance | behavior | Portal + Positioner (`collisionAvoidance`, `collisionPadding`, `sticky`) | Inline absolute CSS; clipped by `overflow:hidden` ancestors; no flip | MED |
| `side` / `sideOffset` / `align` / `alignOffset` | prop | Upstream defaults: `side="top"`, `sideOffset=4`, `align="center"` | `position: Position` (4 sides) only; no offset/align | LOW |
| Arrow | sub-component | `TooltipPrimitive.Arrow` rendered in content | Missing | LOW |
| `disabled`, `closeOnClick` (default true), `disableHoverablePopup`, `trackCursorAxis` | prop | Base UI Root/Trigger | `disabled` exists but is ignored; others missing | LOW |
| Tooltip on a disabled button (wrap in span) | behavior | Documented example | Would work once hover/focus exists, since the trigger is already a wrapper `div` | — |

---

## Typography (`typography.rs`)

Upstream has no component here, only a page of styled plain elements: h1–h4, p, blockquote, table, list, inline code, lead, large, small, muted. Ours is a single `Typography` component with `variant` (H1–H6, P, Blockquote, Code, Lead, Large, Small, Muted) plus `align`/`color`/`weight`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| List style | variant | `ul.my-6.ml-6.list-disc [&>li]:mt-2` | No `List` variant | LOW |
| Table style | variant | Bordered `table` with `even:bg-muted` rows inside an overflow wrapper | No `Table` variant (users can use `Table`) | LOW |
| Element semantics for Large/Small | behavior | `Large` → `<div>`, `Small` → `<small>` | Lead/Large/Small/Muted all render `<p>` | LOW |
| Element override | prop | n/a upstream (you pick the element yourself) | No way to render e.g. an H2-styled `<h1>`; `render`/`as` not supported (see note above) | LOW |

---

## Form (`form.rs`) vs Field

- **Upstream status:** there is no "Form" component any more. `/docs/components/form` and `/docs/components/radix/form` redirect to `/docs/forms`, which has integration guides for React Hook Form, TanStack Form and Formisch. Every guide composes plain `<form>` with the **Field** family: `FieldSet, FieldLegend (variant legend|label), FieldGroup, Field (orientation vertical|horizontal|responsive, data-invalid, role="group"), FieldContent, FieldLabel, FieldTitle, FieldDescription, FieldSeparator, FieldError (children or errors[] → list)`.
- **Ours:** `form.rs` mirrors the *legacy* shadcn react-hook-form wrapper: `Form`, `FormField`, `FormItem`, `FormLabel`, `FormControl`, `FormDescription`, `FormMessage` + `FormMessageType`. `field.rs` exports only one props-driven `Field` (`label`, `help_text`, `error`, `required`), with no sub-components, no orientation and no FieldSet/Group/Error list.
- **Recommendation:** deprecate `FormField`, `FormItem`, `FormLabel`, `FormControl`, `FormDescription` and `FormMessage` with `#[deprecated(note = "use Field…")]` **after** building out the full Field family in `field.rs`. Each old part has a 1:1 successor:
  - FormItem → Field
  - FormLabel → FieldLabel
  - FormDescription → FieldDescription
  - FormMessage → FieldError
  - FormField/FormControl → not needed

  Keep `Form` itself, which is a thin `<form>` wrapper with `onsubmit`/`novalidate` and still useful in Yew, or re-document it as a plain helper. The current single-prop `Field` can stay as a convenience wrapper built on the new parts. Field expansion is **HIGH** because upstream's Switch, Textarea and Toggle Group examples all compose with `Field`/`FieldLabel`/`FieldDescription`/`FieldContent`/`FieldTitle`. Removing the Form* parts is **LOW** urgency.

---

## Totals

| Component | HIGH | MED | LOW |
|---|---|---|---|
| Spinner | 0 | 0 | 0 |
| Switch | 2 | 1 | 4 |
| Table | 0 | 2 | 1 |
| Tabs | 5 | 2 | 4 |
| Textarea | 0 | 1 | 2 |
| Toast/Sonner | 2 | 3 | 4 |
| Toggle | 1 | 2 | 2 |
| Toggle Group | 2 | 3 | 3 |
| Tooltip | 2 | 4 | 3 |
| Typography | 0 | 0 | 4 |
| **Total** | **14** | **18** | **27** |

(Form/Field is assessed separately above: Field expansion HIGH, Form* deprecation LOW.)
