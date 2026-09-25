# New upstream components: Yew port spec

Researched 2026-09-24. Covers the six components that shadcn/ui added and shadcn-rs does not have yet:
**Attachment, Bubble, Marker, Message, Message Scroller, Questionnaire**.

## Sources

| What | Where | Status |
| --- | --- | --- |
| Docs pages (Base UI flavor) | `https://ui.shadcn.com/docs/components/base/{attachment,bubble,marker,message,message-scroller}` | Loaded through firecrawl. The code blocks come back empty because they are rendered on the client, so props and prose came from these pages but no source did. |
| Docs page `/docs/components/base/questionnaire` | firecrawl | **Failed** (firecrawl rate limit). I used the same page's MDX source from GitHub instead (`apps/v4/content/docs/components/base/questionnaire.mdx`). |
| `/docs/react/message-scroller`, `/docs/react/questionnaire` | MDX source from GitHub (`apps/v4/content/docs/react/*.mdx`) | Loaded. These pages hold the full API reference for the two headless primitives. |
| Component source | `github.com/shadcn-ui/ui` `apps/v4/registry/bases/base/ui/<name>.tsx` | Loaded, all 6. |
| Visual rules (`cn-*` classes) | `apps/v4/registry/styles/style-vega.css`, compared against `style-nova.css` | Loaded. Bubble, Attachment, Marker and Message are the same in both styles. Questionnaire and Message Scroller use tighter spacing in Nova, noted below. The docs previews use `base-nova`. |
| Headless scroller internals | `packages/react/src/message-scroller/{README.md,PERFORMANCE.md,geometry.ts,use-message-scroller-controller.ts,components.tsx}` | Loaded, read in part. |

Every upstream file also exists in `aria/` and `radix/` versions. The APIs match; only the underlying primitive library differs. This spec follows the `base` version.

## House conventions this spec assumes

These come from `shadcn-rs/src/components/{item,empty,button}.rs` and `ai/05-component-patterns.md`:

- `#[derive(Properties, PartialEq, Clone)]` props, `#[function_component(Name)]`, `#[prop_or_default] pub class: Classes`, `children: Children`.
- Optional text uses `Option<AttrValue>`. Optional handlers use `Option<Callback<T>>`.
- Enums implement `.class() -> &'static str`, like `Variant` and `Size`. The new enums below should do the same, and each should live in its component file unless it is shared.
- Styles are static CSS in `shadcn-rs/styles/components.css`, using flat kebab-case class names (`.item`, `.item-content`) and `hsl(var(--color-*))` tokens. Radius tokens are `--radius`, `--radius-sm`, `--radius-md`.
- Compound parts share state through `ContextProvider` plus a `#[hook] use_*_context()`.
- Tests are `wasm_bindgen_test` in the browser, plus plain `#[test]` for pure logic.

### Upstream patterns to translate or drop, for all six

| Upstream (React) | Yew port |
| --- | --- |
| `render` prop (Base UI `useRender`/`mergeProps`) for polymorphic elements | **Drop.** Use an `href: Option<AttrValue>` to render `<a>`, and `onclick: Option<Callback<MouseEvent>>` to render `<button type="button">`. Otherwise render the default tag. Yew's dynamic tag syntax `html!{ <@{tag} ...> }` handles this cleanly. |
| `cva` + `cn` + Tailwind `@apply` | Plain CSS classes in `components.css`. Variant and size enums map to modifier classes. |
| `data-slot="..."` attributes that the CSS selectors depend on | Unnecessary because we style by class. Keep `data-state`, `data-align` and similar only where CSS needs parent state (for example `.attachment[data-state=uploading] .attachment-title`). |
| `IconPlaceholder` (picks among lucide, tabler and others) | Use a `shadcn-icons` component (`ArrowDown`, `Check`) or an inline SVG. |
| `"use client"`, SSR/hydration notes, `suppressHydrationWarning`, the inline "scroll to end before hydration" `<script>` | **Drop.** Yew here is CSR only (`yew` features `["csr"]`). |
| `React.useId` | `crate::utils::generate_id`. |
| `className` on every part | `class: Classes` on every part. |
| `motion` animation examples, AI SDK and TanStack helpers | **Drop.** They are out of scope. |

### Shared CSS utilities to add first (one-time, about 1.5 h, counted below under Attachment)

None of these exist in `styles/utilities.css` today. I checked with `grep`.

- `.sr-only` (visually hidden). Needed for the MessageScrollerButton label and for icon-only actions.
- `.shimmer`: `background: linear-gradient(90deg, currentColor 40%, color-mix(in srgb, currentColor 35%, transparent) 50%, currentColor 60%) 0 0 / 200% 100%; -webkit-background-clip: text; background-clip: text; color: transparent; animation: shimmer var(--shimmer-duration, 2s) linear infinite;` with `@keyframes shimmer { from { background-position: 100% 0 } to { background-position: -100% 0 } }`. Also add `@media (prefers-reduced-motion: reduce) { .shimmer { animation: none; color: inherit; } }`. Upstream ships `shimmer` in the `shadcn` npm package, not in the component. Used by Attachment titles, Marker content and Bubble streaming.
- `.scroll-fade-x` and `.scroll-fade-b`: edge fades using `mask-image: linear-gradient(...)`. Used by AttachmentGroup and the MessageScroller viewport.
- `.scrollbar-none`: `scrollbar-width: none` plus `::-webkit-scrollbar { display: none }`.

---

## 1. Marker

**Purpose:** an inline conversation marker. Covers a status line ("Thinking..."), a system note, a row with a bottom border, or a labeled separator ("Today").
**Composes:** Spinner (optional, for status markers) and shimmer (optional). Otherwise nothing.

### Parts and props

| Part | Prop | Rust type | Default |
| --- | --- | --- | --- |
| `Marker` | `variant` | `MarkerVariant { Default, Border, Separator }` | `Default` |
| | `role` | `Option<AttrValue>` (usually `"status"`) | `None` |
| | `href` | `Option<AttrValue>` (renders `<a>`) | `None` |
| | `onclick` | `Option<Callback<MouseEvent>>` (renders `<button type="button">`) | `None` |
| | `aria_label` | `Option<AttrValue>` | `None` |
| | `class`, `children` | `Classes`, `Children` | |
| `MarkerIcon` | `class`, `children` | | always `aria-hidden="true"` |
| `MarkerContent` | `class`, `children` | | |

Events: only `onclick` when interactive.

### DOM, ARIA and keyboard

```html
<div class="marker marker-separator" role="status"?>   <!-- or <a>/<button> -->
  <span class="marker-icon" aria-hidden="true">…</span>
  <span class="marker-content">Today</span>
</div>
```

- The component is presentational by default. The user sets `role="status"` for streaming or progress markers.
- **Do not** add `role="separator"` to the separator variant. Upstream says so explicitly: a separator's contents are presentational, so the label would never be announced. The divider lines are `::before` and `::after` pseudo-elements.
- Keyboard: native only (when rendered as `<a>` or `<button>`).

### CSS (`.marker*`)

- `.marker`: `display:flex; width:100%; align-items:center; gap:.5rem; min-height:1rem; font-size:.875rem; color:hsl(var(--color-muted-foreground)); text-align:left;`. Size child SVGs at `1rem`.
- `a.marker` and `.marker-content a`: underline, `text-underline-offset:3px`, and `color:hsl(var(--color-foreground))` on hover.
- `.marker-separator::before, ::after`: `content:""; height:1px; flex:1; min-width:0; background:hsl(var(--color-border));`, with `margin-right:.25rem` on the `::before` and `margin-left:.25rem` on the `::after`. Also `.marker-separator .marker-content { flex:none; text-align:center; }`.
- `.marker-border`: `border-bottom:1px solid hsl(var(--color-border)); padding-bottom:.5rem;`.
- `.marker-icon`: `flex-shrink:0; width:1rem; height:1rem;`.
- `.marker-content`: `min-width:0; overflow-wrap:anywhere;`.

**Estimate:** 3 h in total (1 h component, 1 h showcase page, 1 h tests and docs).
**React-only items dropped:** the `render` prop and the `markerVariants` export (a cva helper). In Rust, `MarkerVariant::class()` fills that role.

---

## 2. Bubble

**Purpose:** the framed surface of one chat message, with variants, start or end alignment, grouping, and reactions.
**Composes:** nothing required. The docs show it combined with Collapsible (show more), Tooltip (read receipt) and Popover (error details). All three already exist locally.
Upstream deliberately scopes Bubble to the surface. Avatar, name, timestamp and actions belong to **Message**.

### Parts and props

| Part | Prop | Rust type | Default |
| --- | --- | --- | --- |
| `BubbleGroup` | `class`, `children` | | |
| `Bubble` | `variant` | `BubbleVariant { Default, Secondary, Muted, Tinted, Outline, Ghost, Destructive }` | `Default` |
| | `align` | `BubbleAlign { Start, End }`, shared with Message, so define it once as `ChatAlign` | `Start` |
| | `class`, `children` | | |
| `BubbleContent` | `href` | `Option<AttrValue>` | `None` |
| | `onclick` | `Option<Callback<MouseEvent>>` | `None` |
| | `class`, `children` | | |
| `BubbleReactions` | `side` | `ReactionSide { Top, Bottom }` | `Bottom` |
| | `align` | `ChatAlign` | `End` |
| | `aria_label` | `Option<AttrValue>` | `None` |
| | `role` | `Option<AttrValue>` (docs recommend `"img"` for a static emoji row) | `None` |
| | `class`, `children` | | |

Note that `Variant` in `crate::types` is missing `Muted` and `Tinted`, so define a separate `BubbleVariant`.

### DOM, ARIA and keyboard

```html
<div class="bubble-group">
  <div class="bubble bubble-secondary" data-align="end">
    <div class="bubble-content">text</div>            <!-- or <a>/<button> -->
    <div class="bubble-reactions bubble-reactions-bottom bubble-reactions-end"
         role="img" aria-label="Reactions: thumbs up, fire">👍🔥</div>
  </div>
</div>
```

- The bubble is presentational. Conversation-level semantics come from the container (MessageScrollerContent `role="log"`).
- A static reactions row should be `role="img"` with a descriptive `aria-label`, so screen readers announce it once and not glyph by glyph ("plus eight"). Interactive reactions should be buttons, each with an `aria-label`.
- An interactive bubble must render as a real `<a>` or `<button>`, with a visible focus ring (see CSS).

### CSS (`.bubble*`)

- `.bubble`: `position:relative; display:flex; flex-direction:column; gap:.25rem; width:fit-content; min-width:0; max-width:80%;`. Add `.bubble[data-align=end] { align-self:flex-end; }` and `.bubble-ghost { max-width:100%; }`. Also `.message[data-align=end] .bubble { align-self:flex-end; }`.
- `.bubble-content`: `width:fit-content; max-width:100%; min-width:0; overflow:hidden; overflow-wrap:anywhere; border-radius:.75rem; border:1px solid transparent; padding:.5rem .75rem; font-size:.875rem; line-height:1.625;`. For `button.bubble-content` add `text-align:left`. Add a `:focus-visible` ring (`border-color:hsl(var(--color-ring)); box-shadow:0 0 0 3px hsl(var(--color-ring)/.5)`). In an end-aligned bubble, `.bubble-content` gets `align-self:flex-end`.
- Variant colors, applied to `.bubble-<v> > .bubble-content`:
  - default: `background:hsl(var(--color-primary)); color:hsl(var(--color-primary-foreground))`, and `hsl(var(--color-primary)/.8)` on interactive hover.
  - secondary: `--color-secondary` / `--color-secondary-foreground`.
  - muted: `--color-muted` background.
  - tinted: upstream uses `oklch(from var(--primary) 0.93 calc(c*0.4) h)`, or `0.3` lightness in dark mode. Local tokens are HSL triplets, so approximate it with `background: color-mix(in srgb, hsl(var(--color-primary)) 10%, hsl(var(--color-background)))` and `color:hsl(var(--color-foreground))`. That formula works in both themes. Nothing needs to change in `variables.css`.
  - outline: `background:hsl(var(--color-background)); border-color:hsl(var(--color-border))`, with `hsl(var(--color-muted))` on hover.
  - ghost: `border-radius:0; background:transparent; padding:0;`.
  - destructive: `background:hsl(var(--color-destructive)/.1); color:hsl(var(--color-destructive))`, or `/.2` in dark mode.
- `.bubble-reactions`: `position:absolute; z-index:10; display:flex; align-items:center; gap:.25rem; border-radius:9999px; background:hsl(var(--color-muted)); box-shadow:0 0 0 3px hsl(var(--color-background)); padding:.125rem .375rem; font-size:.875rem;`. Side and align modifiers:
  - `-bottom { bottom:0; transform:translateY(75%) }`
  - `-top { top:0; transform:translateY(-75%) }`
  - `-start { left:.75rem }`
  - `-end { right:.75rem }`
  - When the row holds buttons, `padding:0`.
- `.bubble-group`: `display:flex; flex-direction:column; min-width:0; gap:.5rem;`.

**Estimate:** 5 h (2 h component and CSS, 1.5 h showcase with every variant plus the Collapsible, Tooltip and Popover combinations, 1.5 h tests).
**React-only items dropped:** the `render` prop on BubbleContent (replaced by `href` and `onclick`) and the oklch relative-color syntax.

---

## 3. Message

**Purpose:** the layout row around one chat message: avatar, start or end alignment, header (sender name) and footer (status and actions).
**Composes:** Avatar, used inside `MessageAvatar`; Bubble, the surface inside `MessageContent`; Button, for footer actions; Marker, for typing and status rows; Attachment.

### Parts and props

| Part | Prop | Rust type | Default |
| --- | --- | --- | --- |
| `MessageGroup` | `class`, `children` | | |
| `Message` | `align` | `ChatAlign { Start, End }` | `Start` |
| | `class`, `children` | | |
| `MessageAvatar` | `class`, `children` | (may be empty, to keep grouped messages aligned) | |
| `MessageContent` | `class`, `children` | | |
| `MessageHeader` | `class`, `children` | always aligned to start | |
| `MessageFooter` | `class`, `children` | follows `align` | |

There are no events; actions are ordinary Buttons that the user places in the footer.

### DOM, ARIA and keyboard

```html
<div class="message" data-align="end">       <!-- flex-direction: row-reverse when end -->
  <div class="message-avatar"><Avatar/></div>
  <div class="message-content">
    <div class="message-header">Olivia</div>
    <div class="bubble …">…</div>
    <div class="message-footer">Delivered</div>
  </div>
</div>
```

- The component is presentational. Icon-only footer actions need an `aria-label`. For in-progress text, use `<Marker role="status">`.

### CSS (`.message*`)

- `.message`: `position:relative; display:flex; width:100%; min-width:0; gap:.5rem; font-size:.875rem;`. Add `.message[data-align=end] { flex-direction:row-reverse; }`.
- `.message-avatar`: `display:flex; width:fit-content; min-width:2rem; flex-shrink:0; align-self:flex-end; align-items:center; justify-content:center; overflow:hidden; border-radius:9999px; background:hsl(var(--color-muted));`.
  - When the message has a footer, upstream shifts the avatar up by `-translate-y-8` (2rem) so it lines up with the bubble and not the footer. Plain CSS can do this with `.message:has(.message-footer) .message-avatar { transform:translateY(-2rem); }`. `:has()` is supported in all current engines.
  - Alternative: a `has_footer: bool` prop on `Message` that adds a class.
- `.message-content`: `display:flex; flex-direction:column; width:100%; min-width:0; gap:.625rem; overflow-wrap:anywhere;`. Add `.message[data-align=end] .message-content > * { align-self:flex-end; }`, then override the header back to `align-self:flex-start`.
- `.message-header` and `.message-footer`: `display:flex; align-items:center; max-width:100%; min-width:0; padding:0 .75rem; font-size:.75rem; font-weight:500; color:hsl(var(--color-muted-foreground));`. Add `.message:has(.bubble-ghost) :is(.message-header,.message-footer) { padding-inline:0 }` and `.message[data-align=end] .message-footer { justify-content:flex-end; }`.
- `.message-group`: `display:flex; flex-direction:column; min-width:0; gap:.5rem;`.

**Estimate:** 3 h (1 h component and CSS, 1 h showcase built as a chat thread that also exercises Bubble, Marker and Attachment, 1 h tests).
**React-only items dropped:** none of note. This one is a thin layout wrapper.

---

## 4. Attachment

**Purpose:** shows a file or image attachment (media, name, metadata) with an upload state and optional actions. Used in composers, threads and upload lists.
**Composes:** Button (AttachmentAction is a ghost Button), Spinner (in the media slot while uploading), and optionally Dialog (the target of the trigger).

### Parts and props

| Part | Prop | Rust type | Default |
| --- | --- | --- | --- |
| `Attachment` | `state` | `AttachmentState { Idle, Uploading, Processing, Error, Done }` | `Done` |
| | `size` | `AttachmentSize { Default, Sm, Xs }` | `Default` |
| | `orientation` | `Orientation { Horizontal, Vertical }` (check whether Separator or Resizable already has one first) | `Horizontal` |
| | `class`, `children` | | |
| `AttachmentMedia` | `variant` | `AttachmentMediaVariant { Icon, Image }` | `Icon` |
| | `class`, `children` | (an icon or an `<img>`) | |
| `AttachmentContent` | `class`, `children` | | |
| `AttachmentTitle` | `class`, `children` | `<span>`; shimmers while `Uploading` or `Processing` | |
| `AttachmentDescription` | `class`, `children` | `<span>` | |
| `AttachmentActions` | `class`, `children` | | |
| `AttachmentAction` | `aria_label` | `AttrValue` (**required**, since the action is icon-only) | |
| | `onclick` | `Option<Callback<MouseEvent>>` | `None` |
| | `variant` | `Variant` | `Variant::Ghost` |
| | `class`, `children` | | |
| `AttachmentTrigger` | `href` | `Option<AttrValue>` (renders `<a>`, otherwise `<button type="button">`) | `None` |
| | `onclick` | `Option<Callback<MouseEvent>>` | `None` |
| | `aria_label` | `AttrValue` (**required**, since the trigger has no text) | |
| | `class` | | |
| `AttachmentGroup` | `aria_label` | `Option<AttrValue>` | `None` |
| | `focusable` | `bool`. If true, adds `tabindex="0" role="group"` for a scrollable row of presentational items. | `false` |
| | `class`, `children` | | |

Events: `AttachmentAction.onclick` (remove, download, retry) and `AttachmentTrigger.onclick` (open a preview Dialog).

### DOM, ARIA and keyboard

```html
<div class="attachment attachment-default attachment-horizontal" data-state="uploading">
  <div class="attachment-media attachment-media-icon">…</div>
  <div class="attachment-content">
    <span class="attachment-title">design-system.zip</span>
    <span class="attachment-description">Uploading · 64%</span>
  </div>
  <div class="attachment-actions"><button class="btn btn-ghost btn-icon-xs" aria-label="Remove design-system.zip">×</button></div>
  <button class="attachment-trigger" type="button" aria-label="Open research-summary.pdf preview"></button>
</div>
```

- The trigger is `position:absolute; inset:0; z-index:10`, and the actions sit at `z-index:20`. Both stay separately focusable and clickable, with neither blocking the other.
- The error state is shown by color, so the failure reason must also be in `AttachmentDescription`. Upstream says this.
- The group scrolls horizontally with snap. When items are interactive, Tab reaches off-screen items. Otherwise use `focusable=true` plus `aria-label`.

### Browser behavior and file handling

Upstream Attachment **does not handle files**. It is purely presentational: state, percent and name all come from the host. Keep it that way. Only the showcase page needs actual file input, and that page could demonstrate:
- `<input type="file" multiple>` read with `HtmlInputElement::files()`, which returns `FileList`. For each `File`, read `name()`, `size()` and `type_()`.
- Image previews via `web_sys::Url::create_object_url_with_blob(&file)`, with a matching `Url::revoke_object_url` in the cleanup of a `use_effect_with`.
- A simulated upload through `gloo::timers::callback::Interval`, cycling `Idle → Uploading(n%) → Processing → Done | Error`.
- New `web-sys` features for the showcase only: `File`, `FileList`, `Blob`, `Url`. `DataTransfer` is already enabled if drag and drop is wanted.
- A small helper for byte formatting (`format_file_size(u64) -> String`, giving "820 KB" or "1.1 MB") could live in `utils`. It is easy to test natively.

### CSS (`.attachment*`)

- `.attachment`: `position:relative; display:flex; flex-wrap:wrap; flex-shrink:0; width:fit-content; max-width:100%; min-width:0; border:1px solid hsl(var(--color-border)); border-radius:.75rem; background:hsl(var(--color-card, var(--color-background))); transition:background-color .15s;`. Plus:
  - `:focus-within { box-shadow:0 0 0 1px hsl(var(--color-ring)/.5) }`
  - `:has(> .attachment-trigger):hover { background:hsl(var(--color-muted)/.5) }`
  - `[data-state=idle] { border-style:dashed }`
  - `[data-state=error] { border-color:hsl(var(--color-destructive)/.3) }`
- Orientation:
  - `-horizontal { align-items:center; min-width:10rem }`
  - `-vertical { flex-direction:column; width:6rem }`, widening to `7.5rem` when it has content (`:has(.attachment-content)`).
- Sizes (gap / padding with content / padding with only media / font):
  - default: `.5rem` / `.5rem .625rem` / `.5rem` / `.875rem`
  - sm: `.625rem` / `.375rem .5rem` / `.375rem` / `.75rem`
  - xs: `.375rem` / `.25rem .375rem` / `.25rem` / `.75rem`, with radius `.5rem`
- `.attachment-media`: `position:relative; display:flex; aspect-ratio:1; flex-shrink:0; align-items:center; justify-content:center; overflow:hidden; width:2.5rem; border-radius:.5rem; background:hsl(var(--color-muted)); color:hsl(var(--color-foreground));`. Width is `2rem` at sm and `1.75rem` at xs, with `.375rem` radius at xs. Vertical orientation uses `width:100%`. SVG size is 1rem, 0.875rem at xs, and 1.5rem when vertical. In the error state, background is `hsl(var(--color-destructive)/.1)` and color is `--color-destructive`.
- `.attachment-media-image img`: `width:100%; aspect-ratio:1; object-fit:cover;`. `.attachment-media-image` has `opacity:.6` unless `data-state` is `idle` or `done`.
- `.attachment-content`: `flex:1; min-width:0; max-width:100%; line-height:1.25;`, with `padding-inline:.25rem` when vertical.
- `.attachment-title`: `display:block; font-weight:500;` and truncated (`overflow:hidden; text-overflow:ellipsis; white-space:nowrap`). Apply `.shimmer` rules under `[data-state=uploading]` and `[data-state=processing]`.
- `.attachment-description`: `display:block; margin-top:.125rem; font-size:.75rem; color:hsl(var(--color-muted-foreground));`, truncated. In the error state the color is `hsl(var(--color-destructive)/.8)`.
- `.attachment-actions`: `position:relative; z-index:20; display:flex; flex-shrink:0; align-items:center;`. When vertical, `position:absolute; top:.75rem; right:.75rem; gap:.25rem`.
- `.attachment-trigger`: `position:absolute; inset:0; z-index:10; outline:none; background:transparent; border:0; cursor:pointer;`.
- `.attachment-group`: `display:flex; min-width:0; gap:.75rem; padding-block:.25rem; overflow-x:auto; overscroll-behavior-x:contain; scroll-snap-type:x mandatory; scroll-padding-inline:.25rem;`, plus `.scroll-fade-x .scrollbar-none`. Add `.attachment-group > .attachment { flex:none; scroll-snap-align:start; }`.
- **Button gap:** upstream uses the Button sizes `icon-xs` and `icon-sm`. The local `Button` has `Size::{Xs..Xl}` but no square icon sizes. Add a `btn-icon` modifier (`width = height; padding:0`), or a `square: bool` prop on Button. Message Scroller needs the same thing.

**Estimate:** 7 h (1.5 h for the shared utilities (shimmer, sr-only, scroll-fade, scrollbar-none), 2.5 h component and CSS, 2 h showcase with a file-picker demo and simulated upload, 1 h tests).
**React-only items dropped:** the `render` prop on AttachmentTrigger (replaced by `href` and `onclick`), `mergeProps`, and passing Button props through with `...props`.

---

## 5. Message Scroller

**Purpose:** the chat transcript scroll container. It anchors new turns near the top, follows streamed output only while the reader is at the live edge, opens saved threads at the last turn, preserves position when history is prepended, and can jump to any message.
**Composes:** Button (MessageScrollerButton, `secondary` variant, icon-sm) and an ArrowDown icon. The messages inside are Message, Bubble and Marker, but the scroller does not depend on them.

By far the largest item. Upstream it is a separate headless package (`@shadcn/react/message-scroller`, about 1,400 lines across its controller and geometry files). The registry component is a thin styled wrapper around it. Upstream's design rule is that the scroll hot path stays **out of React state**: scroll position, anchoring and follow mode are imperative, and they are mirrored to `data-*` attributes so rows never re-render. The Yew port should copy this. Keep the controller in `Rc<RefCell<Controller>>` held in context, and never in `use_state`.

### Parts and props

| Part | Prop | Rust type | Default |
| --- | --- | --- | --- |
| `MessageScrollerProvider` (no DOM) | `auto_scroll` | `bool` | `false` |
| | `default_scroll_position` | `DefaultScrollPosition { Start, End, LastAnchor }` | `End` |
| | `scroll_edge_threshold` | `f64` px | `8.0` |
| | `scroll_margin` | `f64` px | `0.0` |
| | `scroll_previous_item_peek` | `f64` px | `64.0` |
| | `children` | | |
| `MessageScroller` (frame) | `class`, `children` | | |
| `MessageScrollerViewport` | `preserve_scroll_on_prepend` | `bool` | `true` |
| | `aria_label` | `AttrValue` | `"Messages"` |
| | `id` | `Option<AttrValue>` | `None` |
| | `class`, `children` | | |
| `MessageScrollerContent` | `aria_busy` | `bool` (set while a turn streams) | `false` |
| | `class`, `spacer_class` | `Classes` | |
| | `children` | | |
| `MessageScrollerItem` | `message_id` | `Option<AttrValue>` | `None` |
| | `scroll_anchor` | `bool` | `false` |
| | `class`, `children` | | |
| `MessageScrollerButton` | `direction` | `ScrollDirection { Start, End }` | `End` |
| | `smooth` | `bool` (maps to native `ScrollBehavior`) | `true` |
| | `variant` | `Variant` | `Secondary` |
| | `class`, `children` | Optional. Defaults to an ArrowDown icon plus an sr-only "Scroll to end" or "Scroll to start"; the arrow rotates 180° for `Start`. | |

Hooks, which replace the React hooks one for one:

```rust
#[hook] fn use_message_scroller() -> MessageScrollerHandle;
// MessageScrollerHandle: Clone + PartialEq
//   fn scroll_to_message(&self, id: &str, opts: ScrollOptions) -> bool
//   fn scroll_to_end(&self, opts: ScrollOptions) -> bool
//   fn scroll_to_start(&self, opts: ScrollOptions) -> bool
// ScrollOptions { align: ScrollAlign {Start,Center,End,Nearest}=Start, smooth: bool=false, scroll_margin: Option<f64> }

#[hook] fn use_message_scroller_scrollable() -> Scrollable;          // { start: bool, end: bool }
#[hook] fn use_message_scroller_visibility() -> VisibilityState;     // { current_anchor_id: Option<AttrValue>, visible_message_ids: Vec<AttrValue> }
```

Commands return `false` when they cannot be applied: the viewport is not mounted yet, or the target id is not mounted and cannot be queued. `scroll_to_message` can queue a target before any items exist, which supports permalinks. Once rows have mounted, an unknown id returns `false`.

`Scrollable` and `VisibilityState` have to be reactive for sibling UI, so each gets its own `UseStateHandle`, or better a small subscriber list inside the context. Changes to them are **coalesced to one `requestAnimationFrame`**.

Visibility is pay-for-use. The `IntersectionObserver` is created only while `use_message_scroller_visibility` has at least one subscriber; keep a reference count in the controller.

### DOM, ARIA and keyboard

```html
<div class="message-scroller" data-scrollable="start end" data-autoscrolling?>
  <div class="message-scroller-viewport" role="region" aria-label="Messages" tabindex="0"
       data-scrollable="…" data-pending-scroll?>
    <div class="message-scroller-content" role="log" aria-relevant="additions" aria-busy?>
      <div class="message-scroller-item" data-message-id="m1" data-scroll-anchor="true">…</div>
      …
      <div class="message-scroller-spacer" aria-hidden="true" style="height:Npx"></div>
    </div>
  </div>
  <button class="btn … message-scroller-button" data-direction="end" data-active="false"
          inert tabindex="-1"><svg/><span class="sr-only">Scroll to end</span></button>
</div>
```

- The viewport is a focusable, labelled region, so keyboard users can scroll with the arrow, PageUp/PageDown, Home and End keys.
- The content is `role="log"` with `aria-relevant="additions"`. New rows are announced, but token-by-token text mutations are not.
- When the button has nothing to scroll toward, it gets `inert`, `tabindex=-1` and `data-active="false"`. In Yew, write `inert={(!active).then_some(AttrValue::Static(""))}`.

### Browser behavior and how to build it (web-sys + gloo)

New `web-sys` features: `ResizeObserver`, `ResizeObserverEntry`, `MutationObserver`, `MutationObserverInit`, `MutationRecord`, `IntersectionObserver`, `IntersectionObserverEntry`, `IntersectionObserverInit`, `ScrollToOptions`, `ScrollBehavior`, `WheelEvent`, `PointerEvent`, `HtmlCollection`. Use `wasm_bindgen::closure::Closure` for the observer callbacks. `gloo::events::EventListener` (with `EventListenerOptions::run_in_passive()`) covers `scroll`, `wheel`, `touchstart` and `keydown`. `gloo::render::request_animation_frame` handles coalescing.

The controller is a state machine whose mode field takes one of the upstream values: `following-bottom`, `free-scrolling`, `anchored-to-message` or `settling-jump`.

1. **Opening position.** The first time Content has at least one item, apply `default_scroll_position` once, in `use_effect` (after the DOM commit, before paint).
   - `End` sets `scrollTop = scrollHeight`.
   - `LastAnchor` finds the last item with `data-scroll-anchor=true`, then scrolls it to `scroll_margin` below the top. It falls back to `End` when no anchor exists or when the anchored turn already fits.
   - While pending, the viewport carries `data-pending-scroll`, and CSS applies `visibility:hidden` so the reader never sees the jump. Remove the attribute once the position is applied, or right away if the transcript is empty.
2. **Follow the live edge (`auto_scroll`).** A `ResizeObserver` on Content fires as a streamed reply grows. If mode is `following-bottom`, set `scrollTop = scrollHeight - clientHeight`.
   - The mode arms whenever the reader reaches the end (within `scroll_edge_threshold`).
   - It is released by any user-initiated scroll away: `wheel` with `deltaY < 0`, `touchstart` followed by movement, scroll keys in `keydown` (ArrowUp, PageUp, Home, Space+Shift), or a `scroll` event in which `scrollTop` decreased while no programmatic scroll was in flight (this catches scrollbar drags).
   - Upstream also stops on text selection and similar signals. Content growth must never release follow mode.
   - Set a `programmatic_scroll` flag, and the `data-autoscrolling` attribute, while `scroll_to_end` runs with smooth behavior. Clear it on `scrollend`, or after a 500 ms `gloo::timers::Timeout` fallback, because Safari does not reliably fire `scrollend`.
   - While following, publish `end = false` so the jump button does not flicker on every chunk.
3. **Anchor a new turn.** A `MutationObserver` (`childList: true`) on Content detects appended items. When the newly appended item is an unhandled `scroll_anchor`, put it near the top of the viewport. That can require scrolling past the current end of the content, so keep a **tail spacer** element as the last child of Content:
   - `spacer_height = max(0, viewport.clientHeight - (content_bottom - anchor.offsetTop) - scroll_margin - scroll_previous_item_peek)`
   - `scrollTop = anchor.offsetTop - scroll_margin - scroll_previous_item_peek`
   - Set mode to `anchored-to-message`. As the reply streams in and uses up the spacer (recomputed in the ResizeObserver), reach the end and hand back to `following-bottom`.
   - Track handled anchor ids in a `HashSet`, so re-renders never re-anchor.
   - The spacer math is pure: write it as `fn(f64, …) -> f64` in a `geometry.rs` and unit-test it natively. Upstream does the same (`geometry.test.ts`).
4. **Preserve position on prepend.** On every scroll, cheaply remember the first visible item (its `data-message-id` and its `offsetTop - scrollTop`). When the MutationObserver reports nodes added *before* that item, restore `scrollTop = item.offsetTop - saved_offset` in the same callback. MutationObserver callbacks run as microtasks, before paint, so nothing flickers.
   - Do **not** rely on CSS `overflow-anchor` alone. Browser support differs (historically missing in Safari), and upstream implements this by hand. Set `overflow-anchor:none` on the viewport so the browser's version does not fight ours.
5. **Scroll state.** On a rAF-coalesced `scroll` or resize, compute `start = scrollTop > threshold` and `end = scrollHeight - clientHeight - scrollTop > threshold`, with spacer height excluded from `scrollHeight`. Write `data-scrollable="start end"` on both root and viewport with `set_attribute` (no Yew re-render), then notify the `Scrollable` subscribers only if the value changed.
6. **Visibility.** Run an `IntersectionObserver` with `root = viewport` over the items that have a `message_id`.
   - `visible_message_ids` lists the intersecting ids in document order.
   - `current_anchor_id` is the last anchor at or above the reading line (the top edge plus `scroll_margin`), and it stays set after scrolling past it.
7. **`scroll_to_message`.** Look up `[data-message-id="…"]` inside Content (keep a `HashMap<String, Element>` that the MutationObserver refreshes). Compute the target from `align` and `scroll_margin`, then call `viewport.scroll_to_with_scroll_to_options`. This switches mode to `settling-jump`, and an explicit jump releases follow mode.
8. **Item registration.** Mark items with `data-message-id` and `data-scroll-anchor`, and have the controller read the DOM. Items do **not** register through context. This keeps rows free of re-renders, as upstream does.
9. **Cleanup.** Every observer, listener and pending rAF or timeout has to be disconnected in the effect's destructor. Hold them in the controller as `Option<…>` and drop them there.

### CSS (`.message-scroller*`)

- `.message-scroller`: `position:relative; display:flex; flex-direction:column; width:100%; height:100%; min-height:0; overflow:hidden;`. It fills its parent, so the parent must have a height.
- `.message-scroller-viewport`: `width:100%; height:100%; min-height:0; min-width:0; overflow-y:auto; overscroll-behavior:contain; contain:content; scrollbar-width:thin; scrollbar-gutter:stable; overflow-anchor:none;`, plus `.scroll-fade-b`. Add `[data-pending-scroll] { visibility:hidden }` and `[data-autoscrolling] { scrollbar-color:transparent transparent }`.
- `.message-scroller-content`: `display:flex; flex-direction:column; height:max-content; min-height:100%; gap:2rem;` (Nova uses `1.5rem`).
- `.message-scroller-item`: `min-width:0; flex-shrink:0; content-visibility:auto; contain-intrinsic-size:auto 10rem;`. This lets the browser skip painting far-off rows while keeping them in the DOM for find-in-page, copy and screen readers.
- `.message-scroller-spacer`: `flex-shrink:0; pointer-events:none;`. Subtract the content `gap` from the computed height, as upstream does with `spacerGapRef`.
- `.message-scroller-button`:
  - `position:absolute; left:50%; transform:translateX(-50%); border-color:hsl(var(--color-border)); background:hsl(var(--color-background)); color:hsl(var(--color-foreground)); transition:transform .2s, opacity .2s, scale .2s;`, with `hsl(var(--color-muted))` on hover.
  - `[data-direction=end] { bottom:1rem }`, `[data-direction=start] { top:1rem }`, and `[data-direction=start] svg { rotate:180deg }`.
  - `[data-active=false]`: `pointer-events:none; opacity:0; scale:.95; transition-duration:.4s;`, with `translate:0 100%` for end and `0 -100%` for start.
  - `[data-active=true]`: `opacity:1; scale:1; translate:0 0; transition-timing-function:cubic-bezier(.23,1,.32,1);`.
  - RTL: `[dir=rtl] & { transform:translateX(50%) }`.

### Tests

- Native `#[test]` on the geometry functions: spacer height, edge detection, `current_anchor` selection from a list of `(top, is_anchor)`, and alignment targets.
- `wasm_bindgen_test` in the browser. Mount into a fixed 300 px container, append items, and assert `scrollTop` and the `data-scrollable` / `data-active` attributes after awaiting `gloo::timers::future::TimeoutFuture::new(32)` (two frames). Cover:
  - each opening position
  - prepend preservation
  - an anchored append
  - follow, then release on `WheelEvent`
  - `scroll_to_message` returning false for an unknown id

**Estimate:** 32 h. That breaks down as:
- 6 h for the geometry module and its native tests
- 12 h for the controller state machine and observers
- 4 h for the components, context and hooks
- 3 h for CSS
- 4 h for the showcase: streaming simulation with `Interval`, Load History, Jump-to menu driven by visibility, and anchor-role toggle
- 3 h for browser tests

The risk is high. Most of the time will go to tuning edge cases (smooth-scroll release, spacer handoff), so plan an extra day of buffer.
**React-only items dropped:**
- SSR "avoid flash" inline script and `suppressHydrationWarning`, because the port is CSR-only. The `data-pending-scroll` hiding stays.
- `render` on the Button
- motion/`motion.create` item animations. A CSS `@keyframes` entrance on `.message-scroller-item` with `prefers-reduced-motion` handling is enough. Animate only transform and opacity, never height or margin.
- virtualization guidance
- StrictMode double-mount notes
- AI SDK and TanStack helpers

---

## 6. Questionnaire

**Purpose:** a multi-step form, one question (fieldset) at a time. Supports single choice (radio), multiple choice (checkbox), freeform input, optional or skippable items, answer shortcut keys, validation, and Previous/Skip/Next/Submit navigation.
**Composes:** Button styles (`buttonVariants` on the nav buttons), a native radio or checkbox under a custom indicator, a Check icon, and Input styling. The docs also compose it with Card and Dialog, which are only examples. Upstream separates the headless `@shadcn/react/questionnaire` package from the styled wrapper.

Upstream scope: Questionnaire owns ordered items, the active item, answers, validation, progress and navigation. **The host** owns close and cancel, persistence, transport and branching.

### Parts and props

| Part | Prop | Rust type | Default |
| --- | --- | --- | --- |
| `Questionnaire` (renders `<form novalidate>`) | `items` | `Vec<QuestionnaireItemDef>` (**recommended required** in the port, see below) | |
| | `item` | `Option<AttrValue>` (controlled active item name) | `None` |
| | `default_item` | `Option<AttrValue>` | first enabled |
| | `on_item_change` | `Option<Callback<AttrValue>>` | `None` |
| | `shortcuts` | `Option<ShortcutKind { Letters, Numbers }>` | `None` |
| | `on_submit` | `Option<Callback<QuestionnaireAnswers>>` (called after every enabled item validates) | `None` |
| | `on_reset` | `Option<Callback<()>>` | `None` |
| | `class`, `children` | | |
| `QuestionnaireProgress` | `aria_label` | `AttrValue` | `"Questionnaire progress"` |
| | `children` | `Option<Children>`. Default is "Question {current} of {total}". For a custom format, `format: Option<Callback<ProgressState, Html>>` stands in for the React render function. | |
| `QuestionnaireItem` (renders `<fieldset>`) | `name` | `AttrValue` (required, unique) | |
| | `required`, `multiple`, `disabled`, `invalid` | `bool` | `false` |
| | `aria_labelledby` | `Option<AttrValue>` (for Card or Dialog title composition) | `None` |
| | `on_status_change` | `Option<Callback<ItemStatus>>`, where `ItemStatus { Unanswered, Answered, Skipped }` | `None` |
| | `class`, `children` | | |
| `QuestionnaireTitle` (renders `<legend>`) | `id`, `class`, `children` | | |
| `QuestionnaireDescription` (renders `<p>`, linked by `aria-describedby`) | `class`, `children` | | |
| `QuestionnaireChoices` (renders `<div>`) | `class`, `children` | | |
| `QuestionnaireChoice` (renders `<label>`, containing a hidden native input, the indicator, the label and the shortcut) | `value` | `AttrValue` (required) | |
| | `checked` | `Option<bool>` (controlled) | `None` |
| | `default_checked`, `disabled` | `bool` | `false` |
| | `onchange` | `Option<Callback<bool>>` | `None` |
| | `class`, `children` | | |
| `QuestionnaireChoiceDescription` (renders `<span>`) | `class`, `children` | | |
| `QuestionnaireInput` (freeform `<input>`) | `input_type` | `AttrValue` | `"text"` |
| | `value` / `default_value` | `Option<AttrValue>` | |
| | `placeholder` | `Option<AttrValue>` | |
| | `aria_label` / `id` | `Option<AttrValue>`. **One is required**, because a placeholder is not a label. | |
| | `disabled` | `bool` | |
| | `oninput` | `Option<Callback<String>>` | |
| `QuestionnaireError` (renders `<p>`, hidden until the item is invalid) | `children` | Optional. Default "Please choose an answer." or similar. **Upstream does not list the default wording in the docs; check `packages/react/src/questionnaire/` before copying.** | |
| `QuestionnaireActions` (styled layout only) | `class`, `children` | | |
| `QuestionnairePrevious` / `QuestionnaireSkip` / `QuestionnaireNext` / `QuestionnaireSubmit` | `variant` | `Variant` (`Outline` for Previous and Skip, `Default` for Next and Submit) | |
| | `size` | `Size` | `Md` |
| | `disabled` | `bool` | `false` |
| | `children` | Default "Previous", "Skip", "Next" or "Submit" | |

Supporting types:

```rust
pub struct QuestionnaireItemDef { pub name: AttrValue, pub required: bool, pub disabled: bool, pub choices: Vec<QuestionnaireChoiceDef> }
pub struct QuestionnaireChoiceDef { pub value: AttrValue, pub disabled: bool }
pub struct QuestionnaireAnswers(pub Vec<(AttrValue, Answer)>);   // item order preserved; avoids adding an indexmap dependency
pub enum Answer { Single(AttrValue), Multiple(Vec<AttrValue>), Text(AttrValue), Skipped }
pub struct ProgressState { pub current: usize, pub total: usize, pub first: bool, pub last: bool }
```

**Port decision: require `items` on the root.** Upstream makes `items` optional and otherwise discovers items by DOM order. In Yew, children registering themselves into parent context on mount causes an extra render pass and ordering problems. With `items` provided, order, progress, shortcuts and validation are all computed from data. Each `QuestionnaireItem` then reads its own definition from context by `name`. The upstream docs recommend passing `items` anyway ("Define the collection once...").

### DOM, ARIA, keyboard

```html
<form class="questionnaire" novalidate data-current="1" data-total="2" data-first data-shortcuts="letters">
  <div class="questionnaire-progress" role="progressbar" aria-label="Questionnaire progress"
       aria-valuemin="1" aria-valuemax="2" aria-valuenow="1" aria-valuetext="Question 1 of 2">Question 1 of 2</div>
  <fieldset class="questionnaire-item" data-active data-status="unanswered" data-required
            tabindex="-1" aria-describedby="desc-id err-id" aria-invalid?>
    <legend class="questionnaire-title">What should we prototype next?</legend>
    <p class="questionnaire-description" id="desc-id">…</p>
    <div class="questionnaire-choices">
      <label class="questionnaire-choice" data-type="radio" data-checked? data-shortcut="A">
        <input type="radio" name="direction" value="delegation" class="questionnaire-choice-input"
               aria-keyshortcuts="A" aria-invalid?/>
        <span class="questionnaire-choice-indicator" aria-hidden="true">
          <span class="questionnaire-choice-indicator-dot"/><svg class="questionnaire-choice-indicator-check"/>
        </span>
        <span class="questionnaire-choice-label">Delegation <span class="questionnaire-choice-description">…</span></span>
        <span class="questionnaire-shortcut">A</span>
      </label>
      <div class="questionnaire-input-wrapper"><input class="questionnaire-input" name="direction" aria-label="Another answer"/></div>
    </div>
    <p class="questionnaire-error" id="err-id" hidden>…</p>
  </fieldset>
  <fieldset class="questionnaire-item" hidden inert>…</fieldset>   <!-- inactive items stay mounted, hidden + inert -->
  <div class="questionnaire-actions">
    <button type="button" class="btn … questionnaire-previous" data-hidden hidden>Previous</button>
    <button type="button" class="… questionnaire-skip">Skip</button>
    <button type="button" class="… questionnaire-next" aria-keyshortcuts="Enter">Next</button>
    <button type="submit" class="… questionnaire-submit" data-hidden hidden>Submit</button>
  </div>
</form>
```

Accessibility, from the upstream a11y section:
- Each item is a `<fieldset>` whose title is its `<legend>`. With a custom title (such as a Card), use the title's `id` plus `aria-labelledby` on the item.
- The description and the active error are linked through `aria-describedby`. Invalid items and their controls get `aria-invalid`.
- Progress is a named progressbar.
- Fixed choices are **native** radios and checkboxes. The input sits invisible (`opacity:0`) over the whole row, so native semantics, arrow keys and form behavior come for free.
- Inactive items and non-applicable actions are hidden and `inert`.
- Successful navigation focuses the newly active fieldset (`tabindex=-1`). Failed validation focuses the first available answer control.
- Shortcut keys are exposed with `aria-keyshortcuts`.

Keyboard, handled by an `onkeydown` on the `<form>`:

| Key | Behavior |
| --- | --- |
| Tab / Shift+Tab | Native. |
| ArrowUp / ArrowDown | Move to the previous or next answer, from the fieldset, a fixed answer, or an **empty** text input. Native radios also select. |
| ArrowLeft | Previous item, when focus is outside a radio or text entry. |
| ArrowRight | Next item, when the active item is answered or skipped and focus is outside a radio or text entry. |
| Space | Native select, toggle or activate. |
| Enter | Continue from a selected choice or a filled input, or activate a focused action. |
| Cmd/Ctrl+Enter | Validate and continue from anywhere, or submit on the last item. |
| Letter or number (with `shortcuts`) | Select the matching enabled choice of the active item. A shortcut selects but **does not** advance. Letters are A to Z, numbers are 1 to 9, and disabled choices are skipped. |

Shortcuts and arrow handling pause while the user is typing in a text field. If the host calls `prevent_default()` in its own `onkeydown` before ours runs, questionnaire key handling is turned off.

### Behavior, and how to build it

- **State** lives in `use_reducer` in the root: `active: AttrValue`, `answers: HashMap<AttrValue, Answer>`, `skipped: HashSet`, `invalid: HashSet`. It is provided through `ContextProvider<QuestionnaireContext>` (`Rc` state plus a dispatch `Callback<Action>`). Controlled mode is the same pattern as the existing `use_controllable_state` hook. When `item` is `Some`, the reducer calls `on_item_change` in place of mutating `active`.
- **Enabled order** is `items.iter().filter(|i| !i.disabled)`. Disabled items are excluded from progress, navigation, validation and submission.
- **Validation** is pure logic, so give it native unit tests. An item is valid when (required and answered), or (optional and answered or skipped). An external `invalid=true` on the item overrides this.
  - Next validates the active item. Submit validates every enabled item and moves to the first invalid one.
  - On failure, add the item to `invalid`, render the Error, and focus the first answer control. Do this in `use_effect_with(active)` through a `NodeRef` on the fieldset, using `query_selector("input:not([disabled])")`.
- **Nav visibility:**
  - Previous shows when not on the first item.
  - Skip shows when the active item is optional.
  - Next shows when not on the last item.
  - Submit (`type="submit"`) shows on the last item.
  - Buttons stay mounted and toggle `hidden`, `inert` and `data-hidden`/`data-visible`. They stay **enabled** by default, so pressing them can reveal validation errors. Each carries `data-status` for the active item's status.
- **Submit** runs in the form's `onsubmit`: call `e.prevent_default()`, validate everything, then call `on_submit` with a `QuestionnaireAnswers` assembled from state. Upstream reads native `FormData`; building from state is simpler in Rust and needs no `FormData` web-sys feature. Keep the native `name`/`value` attributes on the inputs anyway, so a host can still read `FormData` itself.
- **Reset** (`onreset`) restores the initial item, default answers, skip state and invalid state.
- **`on_status_change`** fires whenever an item moves between unanswered, answered and skipped.
- New web-sys features: `HtmlFormElement` (for `reset()`), and possibly `HtmlFieldSetElement` (the `NodeRef` works as `HtmlElement` too). `KeyboardEvent`, `HtmlInputElement` and `FocusEvent` are already enabled.

### CSS (`.questionnaire*`), Vega values with Nova differences in brackets

- `.questionnaire`: `display:flex; flex-direction:column; width:100%; min-width:0; gap:1.5rem` [Nova `1rem`].
- `.questionnaire-progress`: `width:fit-content; min-width:14ch; min-height:1lh; font-size:.75rem; font-weight:500; color:hsl(var(--color-muted-foreground)); font-variant-numeric:tabular-nums;`.
- `.questionnaire-item`: `display:flex; flex-direction:column; gap:1.25rem` [1rem]`; min-width:0; border:0; padding:0; margin:0; outline:none;`. Add `.questionnaire-item[hidden] { display:none }`. The entrance animation is optional: `.questionnaire-item[data-active] { animation: fade-in .3s, slide-in-up .3s }` reusing the existing keyframes, inside `prefers-reduced-motion: no-preference`.
- `.questionnaire-title`: `font-size:1rem; font-weight:600` [Nova `500; line-height:1.375`]`; text-wrap:pretty; padding:0;`. Upstream adds `margin-bottom:1.25rem` [1rem] when no description follows. Implement that as `.questionnaire-title:not(:has(~ .questionnaire-description))`, or with a `has_description` class set from context.
- `.questionnaire-description`: `font-size:.875rem; color:hsl(var(--color-muted-foreground)); text-wrap:pretty;`.
- `.questionnaire-choices`: `display:grid; min-width:0; gap:.75rem` [.5rem].
- `.questionnaire-choice`:
  - `position:relative; display:flex; align-items:flex-start; gap:.75rem` [.625rem]`; min-height:2.75rem; padding:.875rem 1rem` [.625rem .75rem]`; border:1px solid hsl(var(--color-input)); border-radius:var(--radius-md)` [Nova `.5rem`]`; background:transparent; font-size:.875rem; cursor:pointer; user-select:none; text-align:start; transition:background-color .15s;`
  - `:hover { background:hsl(var(--color-muted)/.5) }`
  - `[data-checked] { border-color:hsl(var(--color-primary)/.4); background:hsl(var(--color-muted)) }`
  - `[data-invalid] { border-color:hsl(var(--color-destructive)) }`
  - `:has(> input:focus-visible) { border-color:hsl(var(--color-ring)); box-shadow:0 0 0 3px hsl(var(--color-ring)/.5) }`
  - `[data-disabled] { pointer-events:none; cursor:not-allowed; opacity:.5 }`
- `.questionnaire-choice-input`: `position:absolute; inset:0; z-index:10; width:100%; height:100%; margin:0; opacity:0; cursor:pointer;`.
- `.questionnaire-choice-indicator`:
  - `position:relative; display:flex; flex-shrink:0; align-items:center; justify-content:center; width:1rem; height:1rem; margin-top:.1rem; border:1px solid hsl(var(--color-input)); border-radius:4px; pointer-events:none;`
  - `[data-type=radio] &`: `border-radius:9999px`.
  - Checked: `background:hsl(var(--color-primary)); border-color:hsl(var(--color-primary)); color:hsl(var(--color-primary-foreground))`.
- `.questionnaire-choice-indicator-dot`: `display:none; width:.5rem; height:.5rem; border-radius:9999px; background:hsl(var(--color-primary-foreground));`, shown only for checked radios.
- `.questionnaire-choice-indicator-check`: `display:none; width:.875rem; height:.875rem;`, shown only for checked checkboxes.
- `.questionnaire-choice-label`: `display:flex; flex:1; min-width:0; flex-direction:column; gap:.25rem` [.125rem]`; line-height:1.375;`.
- `.questionnaire-choice-description`: `color:hsl(var(--color-muted-foreground));`.
- `.questionnaire-shortcut`: `display:none; margin-inline-start:auto; flex-shrink:0; width:1.25rem; height:1.25rem; align-items:center; justify-content:center; border:1px solid hsl(var(--color-input)); border-radius:var(--radius-md); background:hsl(var(--color-background)); color:hsl(var(--color-muted-foreground)); font-family:monospace; font-size:.625rem; font-weight:500; line-height:1; pointer-events:none;`. Add `[data-shortcut] > .questionnaire-shortcut { display:inline-flex }`. The existing `.kbd` styles may already be close enough to reuse.
- `.questionnaire-input-wrapper`: `position:relative; width:100%; min-width:0;`. For `.questionnaire-input`, reuse the existing `.input` rules (height `2.25rem` [2rem], border `--color-input`, focus ring, and `aria-invalid` → destructive border), plus `min-height:2.75rem` on mobile.
- `.questionnaire-error`: `font-size:.875rem; color:hsl(var(--color-destructive));` [Nova `margin-top:.5rem`]. Add `[hidden] { display:none }`.
- `.questionnaire-actions`: `display:grid; grid-template-columns:minmax(0,1fr) auto auto; align-items:center; gap:.5rem; width:100%; min-height:2.75rem;`. Previous goes in column 1 (justify start), Skip in column 2, and Next and Submit both in column 3 (only one is visible at a time). Every action is on row 1 and `justify-self:end` except Previous. On mobile, `min-height:2.75rem` on buttons meets touch-target size.

### Tests

- Native `#[test]`: the validation function, the enabled-order and progress computation, shortcut assignment (letters or numbers, skipping disabled choices), and reducer transitions (next, previous, skip, invalid, reset, controlled mode).
- `wasm_bindgen_test`:
  - progressbar ARIA values
  - inactive fieldset has `hidden` and `inert`
  - Next on an unanswered required item shows the error and focuses the first input
  - a letter shortcut checks the right radio and does not advance
  - Ctrl+Enter on the last item calls `on_submit`
  - Skip visible only on optional items

**Estimate:** 22 h (4 h for types, reducer and validation with native tests, 7 h for components and context, 4 h for keyboard and focus management, 3 h for CSS, 2.5 h for the showcase with single, multiple, freeform, skip, shortcuts, conditional item and Card/Dialog compositions, 1.5 h for browser tests).
**React-only items dropped:**
- `render` props on every part, including the render-function state. Custom progress uses `format: Callback<ProgressState, Html>`, and navigation state is exposed through `data-status`.
- Discovering items from DOM order when `items` is absent (the port requires `items`)
- Server-rendering `items`
- Zod integration. Hosts set `invalid` plus the Error children, which is the same mechanism upstream uses.
- `form.action` and other server-action props
- `React.useId`

---

## Summary

| Component | New files | Depends on (local) | Est. hours |
| --- | --- | --- | --- |
| Marker | `marker.rs`, CSS | Spinner (optional) | 3 |
| Bubble | `bubble.rs`, CSS | none (combines with Collapsible, Tooltip, Popover) | 5 |
| Message | `message.rs`, CSS | Avatar, Bubble, Marker | 3 |
| Attachment | `attachment.rs`, CSS, shared utilities (shimmer, sr-only, scroll-fade, scrollbar-none) | Button (+ `btn-icon` size), Spinner | 7 |
| Message Scroller | `message_scroller/{mod,controller,geometry,hooks}.rs`, CSS, web-sys observer features | Button (+ `btn-icon`), shadcn-icons ArrowDown | 32 |
| Questionnaire | `questionnaire/{mod,state,keyboard}.rs`, CSS | Button styles, Input styles, shadcn-icons Check | 22 |
| **Total** | | | **72 h** |

Suggested order: shared utilities, then Marker, Bubble, Message and Attachment (these four are presentational and give a complete static chat demo), then Questionnaire, then Message Scroller.

Cross-cutting prerequisites:
1. Add a square icon size to `Button` (`btn-icon`, or `icon: bool`).
2. Add the `.sr-only`, `.shimmer`, `.scroll-fade-*` and `.scrollbar-none` utilities.
3. Add a shared `ChatAlign { Start, End }` for Bubble, Message and BubbleReactions, and possibly `Orientation` if one does not exist.
4. Enable the new `web-sys` features listed under Message Scroller, Questionnaire and the Attachment showcase.
5. Register six showcase pages in `shadcn-showcase/src/pages/components/` and `routes.rs`.
