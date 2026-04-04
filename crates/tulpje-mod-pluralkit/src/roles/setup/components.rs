use twilight_model::channel::message::{
    Component,
    component::{Label, TextDisplay, TextInput, TextInputStyle},
};

pub(super) fn prompt_role_suffix_component(
    custom_id: &str,
    existing_suffix: &str,
) -> Vec<Component> {
    vec![
        TextDisplay {
            id: None,
            content: String::from(
                "Would you like member roles to have a suffix? \
                For example `(Member)` behind their name. \
                Please enter one below if so",
            ),
        }
        .into(),
        Label {
            id: None,
            label: "Role Suffix".into(),
            description: None,
            component: Box::new(
                TextInput {
                    id: None,
                    required: Some(false),
                    placeholder: None,

                    custom_id: String::from(custom_id),
                    value: Some(existing_suffix.into()),
                    style: TextInputStyle::Short,
                    max_length: Some(30),
                    min_length: None,

                    #[expect(deprecated, reason = "can't initialise otherwise")]
                    label: None,
                }
                .into(),
            ),
        }
        .into(),
    ]
}
