use shadcn_rs::*;
use yew::prelude::*;

use crate::components::{ComponentPage, Example};

#[function_component(AttachmentPage)]
pub fn attachment_page() -> Html {
    let attached = use_state(|| true);
    let status = use_state(|| AttachmentStatus::Default);
    let remove = {
        let attached = attached.clone();
        Callback::from(move |_: MouseEvent| attached.set(false))
    };
    let restore = {
        let attached = attached.clone();
        Callback::from(move |_: MouseEvent| attached.set(true))
    };
    let advance = {
        let status = status.clone();
        Callback::from(move |_: MouseEvent| {
            status.set(match *status {
                AttachmentStatus::Default => AttachmentStatus::Uploading,
                AttachmentStatus::Uploading => AttachmentStatus::Processing,
                AttachmentStatus::Processing => AttachmentStatus::Done,
                AttachmentStatus::Done => AttachmentStatus::Error,
                AttachmentStatus::Error => AttachmentStatus::Default,
            });
        })
    };
    let examples = vec![Example {
        title: "File lifecycle",
        description: "Cycle through upload states, remove the sample file, and attach it again. This demo uses a local placeholder file.",
        demo: html! {
            <div class="demo-stack">
                <AttachmentGroup>
                    if *attached {
                        <Attachment status={*status}>
                            <AttachmentMedia><span>{ "📄" }</span></AttachmentMedia>
                            <AttachmentContent>
                                <AttachmentTitle>{ "project-brief.pdf" }</AttachmentTitle>
                                <AttachmentDescription><>{ format!("128 KB · {:?}", *status) }</></AttachmentDescription>
                            </AttachmentContent>
                            <AttachmentActions>
                                <AttachmentAction aria_label="Remove attachment" onclick={remove}>{ "×" }</AttachmentAction>
                            </AttachmentActions>
                        </Attachment>
                    }
                    <AttachmentTrigger onclick={restore} aria_label="Attach sample file">{ "+ Attach sample" }</AttachmentTrigger>
                </AttachmentGroup>
                <Button onclick={advance} disabled={!*attached}>{ "Next upload state" }</Button>
            </div>
        },
        code: r#"<Attachment status={AttachmentStatus::Done}>
    <AttachmentMedia><span>{ "📄" }</span></AttachmentMedia>
    <AttachmentContent>
        <AttachmentTitle>{ "project-brief.pdf" }</AttachmentTitle>
        <AttachmentDescription>{ "128 KB · Ready" }</AttachmentDescription>
    </AttachmentContent>
</Attachment>"#,
    }];
    html! { <ComponentPage name="Attachment" description="File previews with status, actions, and attachment controls." {examples} /> }
}
