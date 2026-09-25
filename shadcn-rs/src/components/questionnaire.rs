//! Questionnaire component
//!
//! A multi-step questionnaire with single-choice, multiple-choice, freeform,
//! and skippable questions.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{
//!     Questionnaire, QuestionnaireProgress, QuestionnaireItem, QuestionnaireTitle,
//!     QuestionnaireDescription, QuestionnaireChoices, QuestionnaireChoice,
//!     QuestionnaireActions, QuestionnairePrevious, QuestionnaireNext, QuestionnaireSubmit,
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Questionnaire total_steps=2>
//!             <QuestionnaireProgress />
//!             <QuestionnaireItem step=0 name="color" required=true>
//!                 <QuestionnaireTitle>{ "What is your favorite color?" }</QuestionnaireTitle>
//!                 <QuestionnaireDescription>{ "Choose one option." }</QuestionnaireDescription>
//!                 <QuestionnaireChoices>
//!                     <QuestionnaireChoice value="blue" selected=true>
//!                         <span>{ "Blue" }</span>
//!                     </QuestionnaireChoice>
//!                     <QuestionnaireChoice value="green">
//!                         <span>{ "Green" }</span>
//!                     </QuestionnaireChoice>
//!                 </QuestionnaireChoices>
//!             </QuestionnaireItem>
//!             <QuestionnaireActions>
//!                 <QuestionnairePrevious />
//!                 <QuestionnaireNext />
//!                 <QuestionnaireSubmit />
//!             </QuestionnaireActions>
//!         </Questionnaire>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Shared context for questionnaire navigation and steps
#[derive(Clone, PartialEq)]
pub struct QuestionnaireContext {
    /// Active question step index (0-indexed)
    pub current_step: usize,
    /// Total number of question steps
    pub total_steps: usize,
    /// Advance to next step
    pub next_step: Callback<()>,
    /// Return to previous step
    pub prev_step: Callback<()>,
    /// Skip current question
    pub skip_step: Callback<()>,
    /// Submit questionnaire
    pub submit: Callback<()>,
}

/// Hook to consume the [`QuestionnaireContext`]
#[hook]
pub fn use_questionnaire() -> QuestionnaireContext {
    use_context::<QuestionnaireContext>().unwrap_or_else(|| QuestionnaireContext {
        current_step: 0,
        total_steps: 1,
        next_step: Callback::noop(),
        prev_step: Callback::noop(),
        skip_step: Callback::noop(),
        submit: Callback::noop(),
    })
}

/// Properties for [`Questionnaire`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireProps {
    /// Total number of steps
    #[prop_or(1)]
    pub total_steps: usize,

    /// Controlled step index
    #[prop_or_default]
    pub current_step: Option<usize>,

    /// Default starting step index
    #[prop_or(0)]
    pub default_step: usize,

    /// Callback when current step changes
    #[prop_or_default]
    pub on_step_change: Option<Callback<usize>>,

    /// Callback on questionnaire submission
    #[prop_or_default]
    pub on_submit: Option<Callback<()>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Questionnaire subcomponents
    pub children: Children,
}

/// Root questionnaire container
#[function_component(Questionnaire)]
pub fn questionnaire(props: &QuestionnaireProps) -> Html {
    let step_state = use_state(|| props.default_step);
    let active_step = props.current_step.unwrap_or(*step_state);

    let next_step = {
        let step_state = step_state.clone();
        let on_step_change = props.on_step_change.clone();
        let total = props.total_steps;
        Callback::from(move |()| {
            let next = (active_step + 1).min(if total > 0 { total - 1 } else { 0 });
            step_state.set(next);
            if let Some(ref cb) = on_step_change {
                cb.emit(next);
            }
        })
    };

    let prev_step = {
        let step_state = step_state.clone();
        let on_step_change = props.on_step_change.clone();
        Callback::from(move |()| {
            let prev = active_step.saturating_sub(1);
            step_state.set(prev);
            if let Some(ref cb) = on_step_change {
                cb.emit(prev);
            }
        })
    };

    let skip_step = {
        let next_step = next_step.clone();
        Callback::from(move |()| {
            next_step.emit(());
        })
    };

    let submit = {
        let on_submit = props.on_submit.clone();
        Callback::from(move |()| {
            if let Some(ref cb) = on_submit {
                cb.emit(());
            }
        })
    };

    let context = QuestionnaireContext {
        current_step: active_step,
        total_steps: props.total_steps,
        next_step,
        prev_step,
        skip_step,
        submit,
    };

    let onsubmit_form = {
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(ref cb) = on_submit {
                cb.emit(());
            }
        })
    };

    let classes = classes!("questionnaire", props.class.clone());

    html! {
        <ContextProvider<QuestionnaireContext> context={context}>
            <form class={classes} onsubmit={onsubmit_form}>
                { props.children.clone() }
            </form>
        </ContextProvider<QuestionnaireContext>>
    }
}

/// Properties for [`QuestionnaireProgress`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireProgressProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Custom progress indicator content
    #[prop_or_default]
    pub children: Option<Children>,
}

/// Progress indicator showing current step completion
#[function_component(QuestionnaireProgress)]
pub fn questionnaire_progress(props: &QuestionnaireProgressProps) -> Html {
    let context = use_questionnaire();
    let classes = classes!("questionnaire-progress", props.class.clone());

    let percent = ((context.current_step + 1) * 100)
        .checked_div(context.total_steps)
        .unwrap_or(100);

    html! {
        <div
            class={classes}
            role="progressbar"
            aria-valuenow={(context.current_step + 1).to_string()}
            aria-valuemin="1"
            aria-valuemax={context.total_steps.to_string()}
        >
            if let Some(ref custom_children) = props.children {
                { custom_children.clone() }
            } else {
                <div
                    class="questionnaire-progress-bar"
                    style={format!("width: {}%;", percent)}
                />
            }
        </div>
    }
}

/// Properties for [`QuestionnaireItem`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireItemProps {
    /// Step index corresponding to this question
    #[prop_or(0)]
    pub step: usize,

    /// Name identifier for form submission
    #[prop_or_default]
    pub name: Option<AttrValue>,

    /// Whether answering is required
    #[prop_or(false)]
    pub required: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Question title, description, and choices
    pub children: Children,
}

/// Container for a single question step
#[function_component(QuestionnaireItem)]
pub fn questionnaire_item(props: &QuestionnaireItemProps) -> Html {
    let context = use_questionnaire();
    let is_active = context.current_step == props.step;

    let classes = classes!(
        "questionnaire-item",
        if is_active {
            "questionnaire-item-active"
        } else {
            ""
        },
        props.class.clone()
    );

    if !is_active {
        return html! {};
    }

    html! {
        <div class={classes} data-name={props.name.clone()} data-required={props.required.to_string()}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`QuestionnaireTitle`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireTitleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Prompt text
    pub children: Children,
}

/// Question prompt heading
#[function_component(QuestionnaireTitle)]
pub fn questionnaire_title(props: &QuestionnaireTitleProps) -> Html {
    let classes = classes!("questionnaire-title", props.class.clone());

    html! {
        <h3 class={classes}>
            { props.children.clone() }
        </h3>
    }
}

/// Properties for [`QuestionnaireDescription`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Guidance text
    pub children: Children,
}

/// Explanatory guidance text for a question
#[function_component(QuestionnaireDescription)]
pub fn questionnaire_description(props: &QuestionnaireDescriptionProps) -> Html {
    let classes = classes!("questionnaire-description", props.class.clone());

    html! {
        <p class={classes}>
            { props.children.clone() }
        </p>
    }
}

/// Properties for [`QuestionnaireChoices`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireChoicesProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Choice items
    pub children: Children,
}

/// Grid or list of selectable choices
#[function_component(QuestionnaireChoices)]
pub fn questionnaire_choices(props: &QuestionnaireChoicesProps) -> Html {
    let classes = classes!("questionnaire-choices", props.class.clone());

    html! {
        <div class={classes} role="radiogroup">
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`QuestionnaireChoice`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireChoiceProps {
    /// Value identifier for this choice
    pub value: AttrValue,

    /// Whether this choice is currently selected
    #[prop_or(false)]
    pub selected: bool,

    /// Click callback
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Choice label and description
    pub children: Children,
}

/// Selectable option button within a question
#[function_component(QuestionnaireChoice)]
pub fn questionnaire_choice(props: &QuestionnaireChoiceProps) -> Html {
    let classes = classes!(
        "questionnaire-choice",
        if props.selected {
            "questionnaire-choice-selected"
        } else {
            ""
        },
        props.class.clone()
    );

    html! {
        <button
            type="button"
            class={classes}
            data-value={props.value.clone()}
            aria-pressed={if props.selected { "true" } else { "false" }}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
        >
            { props.children.clone() }
        </button>
    }
}

/// Properties for [`QuestionnaireInput`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireInputProps {
    /// Input value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Accessible label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Input change handler
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Freeform custom answer input
#[function_component(QuestionnaireInput)]
pub fn questionnaire_input(props: &QuestionnaireInputProps) -> Html {
    let classes = classes!("questionnaire-input", props.class.clone());

    html! {
        <input
            type="text"
            class={classes}
            value={props.value.clone()}
            placeholder={props.placeholder.clone()}
            aria-label={props.aria_label.clone()}
            oninput={props.oninput.clone()}
        />
    }
}

/// Properties for [`QuestionnaireError`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireErrorProps {
    /// Validation error message
    #[prop_or_default]
    pub message: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Error content
    #[prop_or_default]
    pub children: Option<Children>,
}

/// Error validation message for a question
#[function_component(QuestionnaireError)]
pub fn questionnaire_error(props: &QuestionnaireErrorProps) -> Html {
    let classes = classes!("questionnaire-error", props.class.clone());

    html! {
        <div class={classes} role="alert">
            if let Some(ref msg) = props.message {
                { msg }
            } else if let Some(ref custom_children) = props.children {
                { custom_children.clone() }
            }
        </div>
    }
}

/// Properties for [`QuestionnaireActions`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireActionsProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Navigation buttons
    pub children: Children,
}

/// Footer container for questionnaire step actions
#[function_component(QuestionnaireActions)]
pub fn questionnaire_actions(props: &QuestionnaireActionsProps) -> Html {
    let classes = classes!("questionnaire-actions", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`QuestionnairePrevious`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnairePreviousProps {
    /// Custom click callback (defaults to context `prev_step`)
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Button text
    #[prop_or_default]
    pub children: Option<Children>,
}

/// Previous step button
#[function_component(QuestionnairePrevious)]
pub fn questionnaire_previous(props: &QuestionnairePreviousProps) -> Html {
    let context = use_questionnaire();
    let is_first = context.current_step == 0;
    let disabled = props.disabled || is_first;

    let onclick = {
        let custom = props.onclick.clone();
        let prev = context.prev_step;
        Callback::from(move |e: MouseEvent| {
            if let Some(ref cb) = custom {
                cb.emit(e);
            } else {
                prev.emit(());
            }
        })
    };

    let classes = classes!(
        "questionnaire-previous",
        "btn",
        "variant-outline",
        props.class.clone()
    );

    html! {
        <button
            type="button"
            class={classes}
            {onclick}
            disabled={disabled}
        >
            if let Some(ref custom) = props.children {
                { custom.clone() }
            } else {
                { "Back" }
            }
        </button>
    }
}

/// Properties for [`QuestionnaireNext`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireNextProps {
    /// Custom click callback (defaults to context `next_step`)
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Button text
    #[prop_or_default]
    pub children: Option<Children>,
}

/// Next step button
#[function_component(QuestionnaireNext)]
pub fn questionnaire_next(props: &QuestionnaireNextProps) -> Html {
    let context = use_questionnaire();
    let is_last = context.total_steps > 0 && context.current_step + 1 >= context.total_steps;

    let onclick = {
        let custom = props.onclick.clone();
        let next = context.next_step;
        Callback::from(move |e: MouseEvent| {
            if let Some(ref cb) = custom {
                cb.emit(e);
            } else {
                next.emit(());
            }
        })
    };

    if is_last {
        return html! {};
    }

    let classes = classes!(
        "questionnaire-next",
        "btn",
        "variant-primary",
        props.class.clone()
    );

    html! {
        <button
            type="button"
            class={classes}
            {onclick}
            disabled={props.disabled}
        >
            if let Some(ref custom) = props.children {
                { custom.clone() }
            } else {
                { "Next" }
            }
        </button>
    }
}

/// Properties for [`QuestionnaireSkip`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireSkipProps {
    /// Custom click callback (defaults to context `skip_step`)
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Button text
    #[prop_or_default]
    pub children: Option<Children>,
}

/// Skip step button
#[function_component(QuestionnaireSkip)]
pub fn questionnaire_skip(props: &QuestionnaireSkipProps) -> Html {
    let context = use_questionnaire();

    let onclick = {
        let custom = props.onclick.clone();
        let skip = context.skip_step;
        Callback::from(move |e: MouseEvent| {
            if let Some(ref cb) = custom {
                cb.emit(e);
            } else {
                skip.emit(());
            }
        })
    };

    let classes = classes!(
        "questionnaire-skip",
        "btn",
        "variant-ghost",
        props.class.clone()
    );

    html! {
        <button
            type="button"
            class={classes}
            {onclick}
            disabled={props.disabled}
        >
            if let Some(ref custom) = props.children {
                { custom.clone() }
            } else {
                { "Skip" }
            }
        </button>
    }
}

/// Properties for [`QuestionnaireSubmit`]
#[derive(Properties, PartialEq, Clone)]
pub struct QuestionnaireSubmitProps {
    /// Custom click callback (defaults to context `submit`)
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Button text
    #[prop_or_default]
    pub children: Option<Children>,
}

/// Final submit button
#[function_component(QuestionnaireSubmit)]
pub fn questionnaire_submit(props: &QuestionnaireSubmitProps) -> Html {
    let context = use_questionnaire();
    let is_last = context.total_steps == 0 || context.current_step + 1 >= context.total_steps;

    let onclick = {
        let custom = props.onclick.clone();
        let submit = context.submit;
        Callback::from(move |e: MouseEvent| {
            if let Some(ref cb) = custom {
                cb.emit(e);
            } else {
                submit.emit(());
            }
        })
    };

    if !is_last {
        return html! {};
    }

    let classes = classes!(
        "questionnaire-submit",
        "btn",
        "variant-primary",
        props.class.clone()
    );

    html! {
        <button
            type="submit"
            class={classes}
            {onclick}
            disabled={props.disabled}
        >
            if let Some(ref custom) = props.children {
                { custom.clone() }
            } else {
                { "Submit" }
            }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_questionnaire_classes_have_css() {
        let css = include_str!("../../styles/components.css");
        for class in [
            "questionnaire",
            "questionnaire-progress",
            "questionnaire-item",
            "questionnaire-item-active",
            "questionnaire-title",
            "questionnaire-description",
            "questionnaire-choices",
            "questionnaire-choice",
            "questionnaire-choice-selected",
            "questionnaire-input",
            "questionnaire-error",
            "questionnaire-actions",
            "questionnaire-previous",
            "questionnaire-next",
            "questionnaire-skip",
            "questionnaire-submit",
        ] {
            assert!(css.contains(&format!(".{class} {{")), "missing .{class}");
        }
    }
}
