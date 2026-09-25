//! Toast component
//!
//! Imperative toast notifications, modelled on shadcn/ui's Toast (Base UI
//! toast manager) and Sonner.
//!
//! Get a [`ToastHandle`] from [`use_toast`] and call
//! [`add`](ToastHandle::add), [`update`](ToastHandle::update),
//! [`close`](ToastHandle::close), [`promise`](ToastHandle::promise) or one of
//! the shortcuts ([`success`](ToastHandle::success),
//! [`error`](ToastHandle::error), ...).
//!
//! The declarative [`Toast`] component is still available for callers that
//! manage their own list of toasts.

use std::cell::Cell;
use std::fmt;
use std::future::Future;
use std::rc::Rc;

use gloo::timers::callback::Timeout;
use yew::prelude::*;

/// Where the toaster viewport (or a standalone [`Toast`]) sits on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner (default)
    #[default]
    BottomRight,
}

impl ToastPosition {
    /// Suffix used in position CSS classes, e.g. `"top-left"`.
    pub fn as_str(self) -> &'static str {
        match self {
            ToastPosition::TopLeft => "top-left",
            ToastPosition::TopCenter => "top-center",
            ToastPosition::TopRight => "top-right",
            ToastPosition::BottomLeft => "bottom-left",
            ToastPosition::BottomCenter => "bottom-center",
            ToastPosition::BottomRight => "bottom-right",
        }
    }

    /// Whether the position is along the top edge of the screen.
    pub fn is_top(self) -> bool {
        matches!(
            self,
            ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight
        )
    }
}

/// Kind of toast. Controls the icon, the colors and the live-region politeness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastType {
    /// Neutral message
    #[default]
    Default,
    /// Operation succeeded
    Success,
    /// Informational message
    Info,
    /// Something needs attention
    Warning,
    /// Operation failed (announced assertively with `role="alert"`)
    Error,
    /// Work in progress; sticky by default and shows a spinner
    Loading,
}

impl ToastType {
    /// Name used in CSS classes and the `data-type` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            ToastType::Default => "default",
            ToastType::Success => "success",
            ToastType::Info => "info",
            ToastType::Warning => "warning",
            ToastType::Error => "error",
            ToastType::Loading => "loading",
        }
    }

    /// ARIA role: `"alert"` for errors, `"status"` otherwise.
    pub fn role(self) -> &'static str {
        match self {
            ToastType::Error => "alert",
            _ => "status",
        }
    }

    /// `aria-live` politeness matching [`role`](Self::role).
    pub fn aria_live(self) -> &'static str {
        match self {
            ToastType::Error => "assertive",
            _ => "polite",
        }
    }
}

/// How long a toast stays on screen before it closes itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastDuration {
    /// Use the toaster's default duration; loading toasts stay until updated
    /// or closed.
    #[default]
    Auto,
    /// Never close automatically.
    Sticky,
    /// Close after this many milliseconds.
    Millis(u32),
}

impl ToastDuration {
    /// Resolves to milliseconds, or `None` for a toast that never
    /// auto-dismisses.
    ///
    /// ```
    /// use shadcn_rs::{ToastDuration, ToastType};
    ///
    /// assert_eq!(ToastDuration::Auto.resolve(ToastType::Success, 4000), Some(4000));
    /// assert_eq!(ToastDuration::Auto.resolve(ToastType::Loading, 4000), None);
    /// assert_eq!(ToastDuration::Sticky.resolve(ToastType::Info, 4000), None);
    /// assert_eq!(ToastDuration::Millis(0).resolve(ToastType::Info, 4000), None);
    /// ```
    pub fn resolve(self, toast_type: ToastType, default_ms: u32) -> Option<u32> {
        let ms = match self {
            ToastDuration::Auto if toast_type == ToastType::Loading => None,
            ToastDuration::Auto => Some(default_ms),
            ToastDuration::Sticky => None,
            ToastDuration::Millis(ms) => Some(ms),
        };
        ms.filter(|&ms| ms > 0)
    }
}

/// An action button shown on a toast. Clicking it runs `on_click` and closes
/// the toast.
#[derive(Debug, Clone, PartialEq)]
pub struct ToastAction {
    /// Button label
    pub label: AttrValue,
    /// Called when the button is clicked
    pub on_click: Callback<()>,
}

impl ToastAction {
    /// Creates an action button.
    pub fn new(label: impl Into<AttrValue>, on_click: Callback<()>) -> Self {
        Self {
            label: label.into(),
            on_click,
        }
    }
}

/// Everything that describes one toast.
///
/// Build it with the constructors and chainable setters:
///
/// ```
/// use shadcn_rs::{ToastOptions, ToastType};
/// use yew::Callback;
///
/// let options = ToastOptions::new("Event created")
///     .description("Friday, February 10 at 5:57 PM")
///     .with_type(ToastType::Success)
///     .duration(6000)
///     .action("Undo", Callback::from(|()| {}));
/// assert_eq!(options.r#type, ToastType::Success);
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToastOptions {
    /// Title line
    pub title: Option<AttrValue>,
    /// Secondary text under the title
    pub description: Option<AttrValue>,
    /// Kind of toast
    pub r#type: ToastType,
    /// Auto-dismiss behavior
    pub duration: ToastDuration,
    /// Optional action button
    pub action: Option<ToastAction>,
    /// Custom body; replaces the icon, title and description when set
    pub content: Option<Html>,
}

impl ToastOptions {
    /// A default-type toast with a title.
    pub fn new(title: impl Into<AttrValue>) -> Self {
        Self {
            title: Some(title.into()),
            ..Self::default()
        }
    }

    /// A toast whose body is custom markup.
    pub fn custom(content: Html) -> Self {
        Self {
            content: Some(content),
            ..Self::default()
        }
    }

    /// A success toast with a title.
    pub fn success(title: impl Into<AttrValue>) -> Self {
        Self::new(title).with_type(ToastType::Success)
    }

    /// An info toast with a title.
    pub fn info(title: impl Into<AttrValue>) -> Self {
        Self::new(title).with_type(ToastType::Info)
    }

    /// A warning toast with a title.
    pub fn warning(title: impl Into<AttrValue>) -> Self {
        Self::new(title).with_type(ToastType::Warning)
    }

    /// An error toast with a title.
    pub fn error(title: impl Into<AttrValue>) -> Self {
        Self::new(title).with_type(ToastType::Error)
    }

    /// A loading toast with a title. It stays until updated or closed.
    pub fn loading(title: impl Into<AttrValue>) -> Self {
        Self::new(title).with_type(ToastType::Loading)
    }

    /// Sets the description.
    pub fn description(mut self, description: impl Into<AttrValue>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the toast type.
    pub fn with_type(mut self, toast_type: ToastType) -> Self {
        self.r#type = toast_type;
        self
    }

    /// Closes the toast after `ms` milliseconds (`0` means never).
    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = ToastDuration::Millis(ms);
        self
    }

    /// Keeps the toast open until it is closed or updated.
    pub fn sticky(mut self) -> Self {
        self.duration = ToastDuration::Sticky;
        self
    }

    /// Adds an action button.
    pub fn action(mut self, label: impl Into<AttrValue>, on_click: Callback<()>) -> Self {
        self.action = Some(ToastAction::new(label, on_click));
        self
    }

    /// Replaces the icon, title and description with custom markup.
    pub fn content(mut self, content: Html) -> Self {
        self.content = Some(content);
        self
    }
}

/// Identifies a toast created by [`ToastHandle::add`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToastId(u64);

impl ToastId {
    /// The raw numeric id (unique per [`Toaster`]).
    pub fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ToastId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// One toast in the toaster's list.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ToastRecord {
    pub(crate) id: ToastId,
    pub(crate) options: ToastOptions,
    /// Playing its exit transition; removed after that.
    pub(crate) closing: bool,
    /// Bumped by every update so the auto-dismiss timer restarts.
    pub(crate) version: u32,
}

/// Messages understood by [`ToastState`].
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ToastMsg {
    Add(ToastId, ToastOptions),
    Update(ToastId, ToastOptions),
    Close(ToastId),
    CloseAll,
    Remove(ToastId),
}

/// Pure toast list state. Toasts are stored oldest first.
#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct ToastState {
    pub(crate) toasts: Vec<ToastRecord>,
}

impl ToastState {
    fn find_mut(&mut self, id: ToastId) -> Option<&mut ToastRecord> {
        self.toasts.iter_mut().find(|record| record.id == id)
    }

    /// Applies one message.
    pub(crate) fn apply(&mut self, msg: ToastMsg) {
        match msg {
            ToastMsg::Add(id, options) => {
                if self.toasts.iter().all(|record| record.id != id) {
                    self.toasts.push(ToastRecord {
                        id,
                        options,
                        closing: false,
                        version: 0,
                    });
                }
            }
            ToastMsg::Update(id, options) => {
                if let Some(record) = self.find_mut(id).filter(|record| !record.closing) {
                    record.options = options;
                    record.version = record.version.wrapping_add(1);
                }
            }
            ToastMsg::Close(id) => {
                if let Some(record) = self.find_mut(id) {
                    record.closing = true;
                }
            }
            ToastMsg::CloseAll => {
                for record in &mut self.toasts {
                    record.closing = true;
                }
            }
            ToastMsg::Remove(id) => self.toasts.retain(|record| record.id != id),
        }
    }

    /// The toasts to render, newest first: at most `limit` open toasts plus
    /// any that are playing their exit transition. Older open toasts wait
    /// until a slot frees up.
    pub(crate) fn visible(&self, limit: usize) -> Vec<&ToastRecord> {
        let mut open = 0;
        self.toasts
            .iter()
            .rev()
            .filter(|record| {
                if record.closing {
                    true
                } else if open < limit {
                    open += 1;
                    true
                } else {
                    false
                }
            })
            .collect()
    }
}

impl Reducible for ToastState {
    type Action = ToastMsg;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();
        next.apply(action);
        Rc::new(next)
    }
}

#[derive(Clone)]
struct ToastDispatch {
    dispatcher: UseReducerDispatcher<ToastState>,
    next_id: Rc<Cell<u64>>,
}

/// Imperative handle to the nearest [`Toaster`], returned by [`use_toast`].
///
/// Cheap to clone and safe to move into callbacks and async blocks. Without
/// a `Toaster` above it the handle is detached: calls log a console warning
/// and do nothing.
#[derive(Clone, Default)]
pub struct ToastHandle {
    inner: Option<ToastDispatch>,
}

impl PartialEq for ToastHandle {
    fn eq(&self, other: &Self) -> bool {
        match (&self.inner, &other.inner) {
            (Some(a), Some(b)) => {
                a.dispatcher == b.dispatcher && Rc::ptr_eq(&a.next_id, &b.next_id)
            }
            (None, None) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for ToastHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ToastHandle")
            .field("attached", &self.inner.is_some())
            .finish()
    }
}

impl ToastHandle {
    pub(crate) fn new(
        dispatcher: UseReducerDispatcher<ToastState>,
        next_id: Rc<Cell<u64>>,
    ) -> Self {
        Self {
            inner: Some(ToastDispatch {
                dispatcher,
                next_id,
            }),
        }
    }

    fn dispatch(&self, msg: ToastMsg) {
        match &self.inner {
            Some(inner) => inner.dispatcher.dispatch(msg),
            None => gloo::console::warn!("shadcn-rs: use_toast() called outside a <Toaster>"),
        }
    }

    /// Whether this handle is connected to a mounted [`Toaster`].
    pub fn is_attached(&self) -> bool {
        self.inner.is_some()
    }

    /// Shows a toast and returns its id.
    pub fn add(&self, options: ToastOptions) -> ToastId {
        let id = match &self.inner {
            Some(inner) => {
                let id = inner.next_id.get();
                inner.next_id.set(id + 1);
                ToastId(id)
            }
            None => ToastId(0),
        };
        self.dispatch(ToastMsg::Add(id, options));
        id
    }

    /// Replaces the content of an open toast and restarts its timer. Does
    /// nothing if the toast has already closed.
    pub fn update(&self, id: ToastId, options: ToastOptions) {
        self.dispatch(ToastMsg::Update(id, options));
    }

    /// Closes a toast.
    pub fn close(&self, id: ToastId) {
        self.dispatch(ToastMsg::Close(id));
    }

    /// Closes every toast.
    pub fn close_all(&self) {
        self.dispatch(ToastMsg::CloseAll);
    }

    /// Shows a default toast with a title.
    pub fn message(&self, title: impl Into<AttrValue>) -> ToastId {
        self.add(ToastOptions::new(title))
    }

    /// Shows a success toast with a title.
    pub fn success(&self, title: impl Into<AttrValue>) -> ToastId {
        self.add(ToastOptions::success(title))
    }

    /// Shows an info toast with a title.
    pub fn info(&self, title: impl Into<AttrValue>) -> ToastId {
        self.add(ToastOptions::info(title))
    }

    /// Shows a warning toast with a title.
    pub fn warning(&self, title: impl Into<AttrValue>) -> ToastId {
        self.add(ToastOptions::warning(title))
    }

    /// Shows an error toast with a title.
    pub fn error(&self, title: impl Into<AttrValue>) -> ToastId {
        self.add(ToastOptions::error(title))
    }

    /// Shows a sticky loading toast with a title. Update or close it when
    /// the work finishes.
    pub fn loading(&self, title: impl Into<AttrValue>) -> ToastId {
        self.add(ToastOptions::loading(title))
    }

    /// Shows a loading toast while `future` runs, then turns the same toast
    /// into a success or error toast.
    ///
    /// ```rust,no_run
    /// use shadcn_rs::{use_toast, Button, PromiseMessages, ToastOptions};
    /// use yew::prelude::*;
    ///
    /// async fn save() -> Result<u32, String> {
    ///     Ok(3)
    /// }
    ///
    /// #[function_component(SaveButton)]
    /// fn save_button() -> Html {
    ///     let toast = use_toast();
    ///     let onclick = Callback::from(move |_: MouseEvent| {
    ///         toast.promise(
    ///             save(),
    ///             PromiseMessages::new("Saving...", "Saved", "Could not save")
    ///                 .success_with(|n: &u32| ToastOptions::success(format!("Saved {n} files")))
    ///                 .error_with(|e: &String| ToastOptions::error("Could not save").description(e.clone())),
    ///         );
    ///     });
    ///     html! { <Button {onclick}>{ "Save" }</Button> }
    /// }
    /// ```
    pub fn promise<T, E, F>(&self, future: F, messages: PromiseMessages<T, E>) -> ToastId
    where
        F: Future<Output = Result<T, E>> + 'static,
        T: 'static,
        E: 'static,
    {
        let PromiseMessages {
            loading,
            success,
            error,
        } = messages;
        let id = self.add(loading);
        let handle = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let options = match future.await {
                Ok(value) => success(&value),
                Err(err) => error(&err),
            };
            handle.update(id, options);
        });
        id
    }
}

type PromiseFormatter<V> = Box<dyn FnOnce(&V) -> ToastOptions>;

/// Toasts shown by [`ToastHandle::promise`] while pending, on success and on
/// error.
pub struct PromiseMessages<T, E> {
    loading: ToastOptions,
    success: PromiseFormatter<T>,
    error: PromiseFormatter<E>,
}

impl<T: 'static, E: 'static> PromiseMessages<T, E> {
    /// Plain titles for the loading, success and error toasts.
    pub fn new(
        loading: impl Into<AttrValue>,
        success: impl Into<AttrValue>,
        error: impl Into<AttrValue>,
    ) -> Self {
        let success = success.into();
        let error = error.into();
        Self {
            loading: ToastOptions::loading(loading),
            success: Box::new(move |_: &T| ToastOptions::success(success)),
            error: Box::new(move |_: &E| ToastOptions::error(error)),
        }
    }

    /// Replaces the loading toast.
    pub fn loading_options(mut self, options: ToastOptions) -> Self {
        self.loading = options;
        self
    }

    /// Builds the success toast from the resolved value.
    pub fn success_with(mut self, f: impl FnOnce(&T) -> ToastOptions + 'static) -> Self {
        self.success = Box::new(f);
        self
    }

    /// Builds the error toast from the error value.
    pub fn error_with(mut self, f: impl FnOnce(&E) -> ToastOptions + 'static) -> Self {
        self.error = Box::new(f);
        self
    }
}

impl<T, E> fmt::Debug for PromiseMessages<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PromiseMessages")
            .field("loading", &self.loading)
            .finish_non_exhaustive()
    }
}

/// Returns the [`ToastHandle`] of the nearest [`Toaster`].
///
/// Outside a `Toaster` the handle is detached (see [`ToastHandle`]).
#[hook]
pub fn use_toast() -> ToastHandle {
    use_context::<ToastHandle>().unwrap_or_default()
}

/// How long the exit transition runs before a closed toast leaves the DOM.
const EXIT_MS: u32 = 200;

/// Toaster properties
#[derive(Properties, PartialEq, Clone)]
pub struct ToasterProps {
    /// Corner or edge the toasts stack from
    #[prop_or_default]
    pub position: ToastPosition,

    /// Most toasts shown at once; older ones wait until a slot frees up
    #[prop_or(3)]
    pub limit: usize,

    /// Default auto-dismiss time in milliseconds for [`ToastDuration::Auto`]
    #[prop_or(4000)]
    pub duration: u32,

    /// Tint the whole toast with its type color instead of only the icon
    #[prop_or(false)]
    pub rich_colors: bool,

    /// Show a close button on every toast
    #[prop_or(true)]
    pub close_button: bool,

    /// Gap between toasts in pixels
    #[prop_or(14)]
    pub gap: u32,

    /// Accessible name of the notifications region
    #[prop_or(AttrValue::Static("Notifications"))]
    pub label: AttrValue,

    /// Additional CSS classes for the viewport
    #[prop_or_default]
    pub class: Classes,

    /// The app; every descendant can call [`use_toast`]
    #[prop_or_default]
    pub children: Children,
}

/// Toaster: owns the toast list and renders the toast viewport
///
/// Mount it once near the root of the app, wrapping the components that
/// show toasts. They get a [`ToastHandle`] from [`use_toast`].
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::{use_toast, Button, ToastOptions, ToastPosition, Toaster};
///
/// #[function_component(SaveButton)]
/// fn save_button() -> Html {
///     let toast = use_toast();
///     let onclick = Callback::from(move |_: MouseEvent| {
///         toast.add(ToastOptions::success("Changes saved").description("Just now"));
///     });
///     html! { <Button {onclick}>{ "Save" }</Button> }
/// }
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Toaster position={ToastPosition::TopCenter} limit={5}>
///             <SaveButton />
///         </Toaster>
///     }
/// }
/// ```
///
/// # Accessibility
/// - The viewport is a labelled region (`aria-label`, default "Notifications")
/// - Each toast is `role="status"` / `aria-live="polite"`; errors use
///   `role="alert"` / `aria-live="assertive"`
/// - Auto-dismiss pauses while the pointer is over a toast
/// - Close buttons have an `aria-label`
#[function_component(Toaster)]
pub fn toaster(props: &ToasterProps) -> Html {
    let state = use_reducer(ToastState::default);
    let next_id = use_memo((), |()| Cell::new(1_u64));
    let hovered = use_state(|| false);

    let handle = ToastHandle::new(state.dispatcher(), next_id);

    let on_close = {
        let handle = handle.clone();
        Callback::from(move |id: ToastId| handle.close(id))
    };
    let on_remove = {
        let dispatcher = state.dispatcher();
        Callback::from(move |id: ToastId| dispatcher.dispatch(ToastMsg::Remove(id)))
    };
    let on_hover = {
        let hovered = hovered.clone();
        Callback::from(move |value: bool| hovered.set(value))
    };

    let position = props.position;
    let classes = classes!(
        "toaster",
        format!("toaster-{}", position.as_str()),
        props.rich_colors.then_some("toaster-rich-colors"),
        props.class.clone()
    );
    let style = format!("--toast-gap: {}px", props.gap);

    let toasts = state
        .visible(props.limit)
        .into_iter()
        .map(|record| {
            let duration = record
                .options
                .duration
                .resolve(record.options.r#type, props.duration);
            html! {
                <ToastItem
                    key={record.id.get()}
                    record={record.clone()}
                    {duration}
                    paused={*hovered}
                    close_button={props.close_button}
                    on_close={on_close.clone()}
                    on_remove={on_remove.clone()}
                    on_hover={on_hover.clone()}
                />
            }
        })
        .collect::<Html>();

    html! {
        <ContextProvider<ToastHandle> context={handle}>
            { props.children.clone() }
            <section class="toaster-region" aria-label={props.label.clone()}>
                <ol class={classes} style={style} data-position={position.as_str()}>
                    { toasts }
                </ol>
            </section>
        </ContextProvider<ToastHandle>>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ToastItemProps {
    record: ToastRecord,
    duration: Option<u32>,
    paused: bool,
    close_button: bool,
    on_close: Callback<ToastId>,
    on_remove: Callback<ToastId>,
    on_hover: Callback<bool>,
}

/// Remaining auto-dismiss time, keyed by the toast version and duration it
/// was computed for.
#[derive(Default)]
struct TimerState {
    key: Option<(u32, u32)>,
    remaining_ms: f64,
}

#[function_component(ToastItem)]
fn toast_item(props: &ToastItemProps) -> Html {
    let ToastItemProps {
        record,
        duration,
        paused,
        close_button,
        on_close,
        on_remove,
        on_hover,
    } = props.clone();
    let id = record.id;
    let timer_state = use_mut_ref(TimerState::default);

    // Auto-dismiss. Pausing keeps the time left; an update restarts it.
    {
        let on_close = on_close.clone();
        use_effect_with(
            (duration, paused, record.version, record.closing),
            move |&(duration, paused, version, closing)| {
                let mut running: Option<(Timeout, f64)> = None;
                if let (Some(ms), false) = (duration, closing) {
                    let mut timer = timer_state.borrow_mut();
                    if timer.key != Some((version, ms)) {
                        timer.key = Some((version, ms));
                        timer.remaining_ms = f64::from(ms);
                    }
                    if !paused {
                        let left = timer.remaining_ms.max(0.0) as u32;
                        let timeout = Timeout::new(left, move || on_close.emit(id));
                        running = Some((timeout, js_sys::Date::now()));
                    }
                }
                move || {
                    if let Some((timeout, started)) = running {
                        drop(timeout);
                        timer_state.borrow_mut().remaining_ms -= js_sys::Date::now() - started;
                    }
                }
            },
        );
    }

    // Remove from the list once the exit transition has played.
    use_effect_with(record.closing, move |&closing| {
        let timeout = closing.then(|| Timeout::new(EXIT_MS, move || on_remove.emit(id)));
        move || drop(timeout)
    });

    let options = &record.options;
    let toast_type = options.r#type;
    let classes = classes!(
        "toast",
        format!("toast-{}", toast_type.as_str()),
        record.closing.then_some("toast-closing")
    );

    let onmouseenter = {
        let on_hover = on_hover.clone();
        Callback::from(move |_: MouseEvent| on_hover.emit(true))
    };
    let onmouseleave = Callback::from(move |_: MouseEvent| on_hover.emit(false));

    let action = options.action.clone().map(|action| {
        let on_close = on_close.clone();
        let label = action.label.clone();
        let onclick = Callback::from(move |_: MouseEvent| {
            action.on_click.emit(());
            on_close.emit(id);
        });
        html! {
            <button type="button" class="toast-action" {onclick}>{ label }</button>
        }
    });

    let close = close_button.then(|| {
        let onclick = Callback::from(move |_: MouseEvent| on_close.emit(id));
        html! {
            <button type="button" class="toast-close" aria-label="Close notification" {onclick}>
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                    <path d="M18 6 6 18" />
                    <path d="m6 6 12 12" />
                </svg>
            </button>
        }
    });

    let body = match &options.content {
        Some(content) => html! { <div class="toast-content">{ content.clone() }</div> },
        None => html! {
            <>
                { toast_icon(toast_type) }
                <div class="toast-content">
                    if let Some(title) = &options.title {
                        <div class="toast-title">{ title.clone() }</div>
                    }
                    if let Some(description) = &options.description {
                        <div class="toast-description">{ description.clone() }</div>
                    }
                </div>
            </>
        },
    };
    let has_buttons = action.is_some() || close.is_some();

    html! {
        <li
            class={classes}
            role={toast_type.role()}
            aria-live={toast_type.aria_live()}
            aria-atomic="true"
            data-type={toast_type.as_str()}
            data-state={if record.closing { "closed" } else { "open" }}
            {onmouseenter}
            {onmouseleave}
        >
            { body }
            if has_buttons {
                <div class="toast-actions">
                    { action }
                    { close }
                </div>
            }
        </li>
    }
}

/// Lucide icon for a toast type (none for [`ToastType::Default`]).
fn toast_icon(toast_type: ToastType) -> Html {
    let paths = match toast_type {
        ToastType::Default => return Html::default(),
        ToastType::Success => html! {
            <>
                <circle cx="12" cy="12" r="10" />
                <path d="m9 12 2 2 4-4" />
            </>
        },
        ToastType::Info => html! {
            <>
                <circle cx="12" cy="12" r="10" />
                <path d="M12 16v-4" />
                <path d="M12 8h.01" />
            </>
        },
        ToastType::Warning => html! {
            <>
                <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" />
                <path d="M12 9v4" />
                <path d="M12 17h.01" />
            </>
        },
        ToastType::Error => html! {
            <>
                <circle cx="12" cy="12" r="10" />
                <path d="m15 9-6 6" />
                <path d="m9 9 6 6" />
            </>
        },
        ToastType::Loading => html! { <path d="M21 12a9 9 0 1 1-6.219-8.56" /> },
    };
    html! {
        <span class="toast-icon" aria-hidden="true">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                { paths }
            </svg>
        </span>
    }
}

/// Toast variant (for the standalone [`Toast`] component)
#[derive(Debug, Clone, PartialEq)]
pub enum ToastVariant {
    /// Default variant
    Default,
    /// Success variant
    Success,
    /// Warning variant
    Warning,
    /// Error variant
    Error,
    /// Info variant
    Info,
}

/// Toast component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ToastProps {
    /// Toast variant
    #[prop_or(ToastVariant::Default)]
    pub variant: ToastVariant,

    /// Toast position
    #[prop_or(ToastPosition::TopRight)]
    pub position: ToastPosition,

    /// Toast title
    #[prop_or_default]
    pub title: Option<AttrValue>,

    /// Toast description
    #[prop_or_default]
    pub description: Option<AttrValue>,

    /// Auto-dismiss duration in milliseconds (0 = no auto-dismiss)
    #[prop_or(5000)]
    pub duration: u32,

    /// Action button text
    #[prop_or_default]
    pub action: Option<AttrValue>,

    /// Action button click handler
    #[prop_or_default]
    pub on_action: Option<Callback<MouseEvent>>,

    /// Close handler
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    #[prop_or_default]
    pub children: Children,
}

/// Standalone toast component
///
/// Renders one fixed-position toast that the caller shows and hides itself.
/// Most apps should mount a [`Toaster`] and use [`use_toast`] instead.
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::{Toast, ToastPosition, ToastVariant};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let show_toast = use_state(|| true);
///     let on_close = {
///         let show_toast = show_toast.clone();
///         Callback::from(move |()| show_toast.set(false))
///     };
///
///     html! {
///         if *show_toast {
///             <Toast
///                 variant={ToastVariant::Success}
///                 position={ToastPosition::TopRight}
///                 title="Success!"
///                 description="Your changes have been saved."
///                 duration={3000}
///                 on_close={Some(on_close)}
///             />
///         }
///     }
/// }
/// ```
///
/// # Accessibility
/// - role="status" for non-critical messages
/// - role="alert" for error messages
/// - Live region for screen reader announcements
#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let ToastProps {
        variant,
        position,
        title,
        description,
        duration,
        action,
        on_action,
        on_close,
        class,
        children,
    } = props.clone();

    // Auto-dismiss timer.
    // Note: the timer depends on `duration` so it only re-runs when duration changes.
    // This is correct when the component is unmounted/remounted (common case),
    // since a new component instance always runs its effects on mount.
    {
        let on_close = on_close.clone();
        use_effect_with(duration, move |&duration| {
            let handle = if duration > 0 {
                let timeout = Timeout::new(duration, move || {
                    if let Some(cb) = on_close.as_ref() {
                        cb.emit(());
                    }
                });
                Some(timeout)
            } else {
                None
            };
            move || drop(handle)
        });
    }

    let variant_class = match variant {
        ToastVariant::Default => "toast-default",
        ToastVariant::Success => "toast-success",
        ToastVariant::Warning => "toast-warning",
        ToastVariant::Error => "toast-error",
        ToastVariant::Info => "toast-info",
    };

    let classes: Classes = vec![
        Classes::from("toast"),
        Classes::from("toast-standalone"),
        Classes::from(variant_class),
        Classes::from(format!("toast-{}", position.as_str())),
        class,
    ]
    .into_iter()
    .collect();

    let (role, aria_live) = match variant {
        ToastVariant::Error => ("alert", "assertive"),
        _ => ("status", "polite"),
    };

    let close_handler = on_close.map(|cb| {
        Callback::from(move |_: MouseEvent| {
            cb.emit(());
        })
    });

    let has_children = children.iter().count() > 0;

    html! {
        <div class={classes} role={role} aria-live={aria_live} aria-atomic="true">
            if has_children {
                { children }
            } else {
                <div class="toast-content">
                    if let Some(title_text) = title {
                        <div class="toast-title">
                            { title_text }
                        </div>
                    }
                    if let Some(desc_text) = description {
                        <div class="toast-description">
                            { desc_text }
                        </div>
                    }
                </div>
            }
            <div class="toast-actions">
                if let Some(action_text) = action {
                    <button
                        type="button"
                        class="toast-action"
                        onclick={on_action}
                    >
                        { action_text }
                    </button>
                }
                <button
                    type="button"
                    class="toast-close"
                    onclick={close_handler}
                    aria-label="Close"
                >
                    { "×" }
                </button>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u64) -> ToastId {
        ToastId(n)
    }

    fn state_with(n: u64) -> ToastState {
        let mut state = ToastState::default();
        for i in 1..=n {
            state.apply(ToastMsg::Add(
                id(i),
                ToastOptions::new(format!("toast {i}")),
            ));
        }
        state
    }

    fn visible_ids(state: &ToastState, limit: usize) -> Vec<u64> {
        state.visible(limit).iter().map(|r| r.id.get()).collect()
    }

    #[test]
    fn add_appends_in_order() {
        let state = state_with(3);
        let ids: Vec<u64> = state.toasts.iter().map(|r| r.id.get()).collect();
        assert_eq!(ids, vec![1, 2, 3]);
        assert!(state.toasts.iter().all(|r| !r.closing && r.version == 0));
    }

    #[test]
    fn add_ignores_duplicate_id() {
        let mut state = state_with(1);
        state.apply(ToastMsg::Add(id(1), ToastOptions::new("dup")));
        assert_eq!(state.toasts.len(), 1);
        assert_eq!(
            state.toasts[0].options.title,
            Some(AttrValue::from("toast 1"))
        );
    }

    #[test]
    fn update_replaces_options_and_bumps_version() {
        let mut state = state_with(2);
        state.apply(ToastMsg::Update(id(2), ToastOptions::success("done")));
        let record = &state.toasts[1];
        assert_eq!(record.options.title, Some(AttrValue::from("done")));
        assert_eq!(record.options.r#type, ToastType::Success);
        assert_eq!(record.version, 1);
        assert_eq!(state.toasts[0].version, 0);
    }

    #[test]
    fn update_unknown_or_closing_is_noop() {
        let mut state = state_with(1);
        let before = state.clone();
        state.apply(ToastMsg::Update(id(9), ToastOptions::new("x")));
        assert_eq!(state, before);

        state.apply(ToastMsg::Close(id(1)));
        state.apply(ToastMsg::Update(id(1), ToastOptions::new("late")));
        assert_eq!(
            state.toasts[0].options.title,
            Some(AttrValue::from("toast 1"))
        );
    }

    #[test]
    fn close_marks_then_remove_deletes() {
        let mut state = state_with(2);
        state.apply(ToastMsg::Close(id(1)));
        assert!(state.toasts[0].closing);
        assert_eq!(state.toasts.len(), 2);

        state.apply(ToastMsg::Remove(id(1)));
        let ids: Vec<u64> = state.toasts.iter().map(|r| r.id.get()).collect();
        assert_eq!(ids, vec![2]);
    }

    #[test]
    fn close_all_marks_every_toast() {
        let mut state = state_with(3);
        state.apply(ToastMsg::CloseAll);
        assert!(state.toasts.iter().all(|r| r.closing));
    }

    #[test]
    fn visible_is_newest_first_and_respects_limit() {
        let state = state_with(5);
        assert_eq!(visible_ids(&state, 3), vec![5, 4, 3]);
        assert_eq!(visible_ids(&state, 10), vec![5, 4, 3, 2, 1]);
        assert!(visible_ids(&state, 0).is_empty());
    }

    #[test]
    fn closing_toasts_free_a_slot_but_stay_visible() {
        let mut state = state_with(4);
        state.apply(ToastMsg::Close(id(4)));
        // 4 is animating out and no longer counts toward the limit.
        assert_eq!(visible_ids(&state, 3), vec![4, 3, 2, 1]);

        state.apply(ToastMsg::Remove(id(4)));
        assert_eq!(visible_ids(&state, 3), vec![3, 2, 1]);
    }

    #[test]
    fn reducer_matches_apply() {
        let state = Rc::new(state_with(1));
        let next = state.clone().reduce(ToastMsg::Close(id(1)));
        assert!(next.toasts[0].closing);
        assert!(!state.toasts[0].closing);
    }

    #[test]
    fn duration_resolution() {
        assert_eq!(
            ToastDuration::Auto.resolve(ToastType::Default, 4000),
            Some(4000)
        );
        assert_eq!(ToastDuration::Auto.resolve(ToastType::Loading, 4000), None);
        assert_eq!(
            ToastDuration::Sticky.resolve(ToastType::Success, 4000),
            None
        );
        assert_eq!(
            ToastDuration::Millis(1500).resolve(ToastType::Loading, 4000),
            Some(1500)
        );
        assert_eq!(
            ToastDuration::Millis(0).resolve(ToastType::Info, 4000),
            None
        );
    }

    #[test]
    fn options_builders() {
        let options = ToastOptions::warning("Careful")
            .description("details")
            .sticky()
            .action("Undo", Callback::noop());
        assert_eq!(options.r#type, ToastType::Warning);
        assert_eq!(options.description, Some(AttrValue::from("details")));
        assert_eq!(options.duration, ToastDuration::Sticky);
        assert_eq!(
            options.action.map(|a| a.label),
            Some(AttrValue::from("Undo"))
        );
        assert_eq!(ToastOptions::loading("x").r#type, ToastType::Loading);
        assert!(
            ToastOptions::custom(html! { <b>{ "hi" }</b> })
                .content
                .is_some()
        );
    }

    #[test]
    fn type_roles() {
        assert_eq!(ToastType::Error.role(), "alert");
        assert_eq!(ToastType::Error.aria_live(), "assertive");
        assert_eq!(ToastType::Success.role(), "status");
        assert_eq!(ToastType::Loading.aria_live(), "polite");
    }

    #[test]
    fn position_helpers() {
        assert_eq!(ToastPosition::default(), ToastPosition::BottomRight);
        assert_eq!(ToastPosition::TopCenter.as_str(), "top-center");
        assert!(ToastPosition::TopLeft.is_top());
        assert!(!ToastPosition::BottomCenter.is_top());
    }

    #[test]
    fn detached_handle_is_harmless() {
        let handle = ToastHandle::default();
        assert!(!handle.is_attached());
        assert_eq!(handle, ToastHandle::default());
    }

    #[test]
    fn test_toast_default() {
        let props = ToastProps {
            variant: ToastVariant::Default,
            position: ToastPosition::TopRight,
            title: None,
            description: None,
            duration: 5000,
            action: None,
            on_action: None,
            on_close: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, ToastVariant::Default);
        assert_eq!(props.position, ToastPosition::TopRight);
        assert_eq!(props.duration, 5000);
    }

    #[test]
    fn test_toast_with_action() {
        let props = ToastProps {
            variant: ToastVariant::Info,
            position: ToastPosition::TopLeft,
            title: None,
            description: None,
            duration: 5000,
            action: Some(AttrValue::from("Undo")),
            on_action: None,
            on_close: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.action, Some(AttrValue::from("Undo")));
    }
}
