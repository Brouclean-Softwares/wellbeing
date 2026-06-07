use crate::data::users::ConnectedProfile;
use crate::languages::Translator;
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "shared/modal_button.html")]
pub struct ModalButton {
    button_level: &'static str,
    button_icon: &'static str,
    button_name: String,
    modal_id: &'static str,
    modal_title: String,
    modal_content: String,
    form_action: &'static str,
    cancel_text: String,
}

impl ModalButton {
    pub fn from(
        connected_profile: ConnectedProfile,
        button_level: &'static str,
        button_icon: &'static str,
        button_name: String,
        modal_id: &'static str,
        modal_title: String,
        modal_content: String,
        form_action: &'static str,
    ) -> ModalButton {
        ModalButton {
            button_level,
            button_icon,
            button_name,
            modal_id,
            modal_title,
            modal_content,
            form_action,
            cancel_text: connected_profile.translate("cancel"),
        }
    }

    pub fn delete_button_from(
        connected_profile: ConnectedProfile,
        delete_url: &'static str,
        element_id: i64,
    ) -> ModalButton {
        ModalButton::from(
            connected_profile.clone(),
            "danger",
            "trash",
            connected_profile.translate("delete"),
            "delete",
            connected_profile.translate("delete_confirmation_title"),
            DeleteModalButtonContent {
                connected_profile,
                element_id,
            }
            .render()
            .unwrap(),
            delete_url,
        )
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "shared/delete_modal_button_content.html")]
struct DeleteModalButtonContent {
    connected_profile: ConnectedProfile,
    element_id: i64,
}
