use shadcn_rs::*;
use yew::prelude::*;

use crate::components::{ComponentPage, Example};

#[function_component(QuestionnairePage)]
pub fn questionnaire_page() -> Html {
    let goal = use_state(|| AttrValue::from(""));
    let name = use_state(String::new);
    let submitted = use_state(|| 0usize);
    let oninput = {
        let name = name.clone();
        Callback::from(move |event: InputEvent| {
            name.set(
                event
                    .target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            )
        })
    };
    let on_submit = {
        let submitted = submitted.clone();
        Callback::from(move |()| submitted.set(*submitted + 1))
    };
    let examples = vec![Example {
        title: "Plan your next project",
        description: "Choose a project, move between steps, and submit a local summary. No data is sent anywhere.",
        demo: html! {
            <div class="demo-stack">
                <Questionnaire total_steps={2} {on_submit}>
                    <QuestionnaireProgress />
                    <QuestionnaireItem step={0}>
                        <QuestionnaireTitle>{ "What are you building?" }</QuestionnaireTitle>
                        <QuestionnaireDescription>{ "Select a starting point." }</QuestionnaireDescription>
                        <QuestionnaireChoices>
                            { for ["Dashboard", "Chat app", "Design system"].into_iter().map(|value| {
                                let goal = goal.clone();
                                let selected = *goal == value;
                                html! { <QuestionnaireChoice {value} {selected} onclick={Callback::from(move |_: MouseEvent| goal.set(value.into()))}><>{ value }</></QuestionnaireChoice> }
                            }) }
                        </QuestionnaireChoices>
                        if goal.is_empty() {
                            <QuestionnaireError message="Choose a project or use Skip." />
                        }
                    </QuestionnaireItem>
                    <QuestionnaireItem step={1}>
                        <QuestionnaireTitle>{ "Give it a name" }</QuestionnaireTitle>
                        <QuestionnaireDescription>{ "Optional — you can change it later." }</QuestionnaireDescription>
                        <QuestionnaireInput value={(*name).clone()} placeholder="My next project" aria_label="Project name" {oninput} />
                    </QuestionnaireItem>
                    <QuestionnaireActions>
                        <QuestionnairePrevious />
                        <QuestionnaireSkip />
                        <QuestionnaireNext disabled={goal.is_empty()} />
                        <QuestionnaireSubmit />
                    </QuestionnaireActions>
                </Questionnaire>
                if *submitted > 0 {
                    <p role="status">{ format!("Saved: {} · {} ({} submission{})", if name.is_empty() { "Untitled" } else { &name }, if goal.is_empty() { "No template" } else { &goal }, *submitted, if *submitted == 1 { "" } else { "s" }) }</p>
                }
            </div>
        },
        code: r#"<Questionnaire total_steps={2} on_submit={on_submit}>
    <QuestionnaireProgress />
    <QuestionnaireItem step={0}><QuestionnaireTitle>{ "Choose a project" }</QuestionnaireTitle></QuestionnaireItem>
    <QuestionnaireItem step={1}><QuestionnaireInput aria_label="Project name" /></QuestionnaireItem>
    <QuestionnaireActions>
        <QuestionnairePrevious /><QuestionnaireNext /><QuestionnaireSubmit />
    </QuestionnaireActions>
</Questionnaire>"#,
    }];
    html! { <ComponentPage name="Questionnaire" description="A multi-step form with choices, navigation, validation, and a completion summary." {examples} /> }
}
