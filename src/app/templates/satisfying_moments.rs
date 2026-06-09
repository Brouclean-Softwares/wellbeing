use crate::app::templates::NavigationBar;
use crate::app::templates::shared::ModalButton;
use crate::data::satisfying_moments::SatisfyingMoment;
use crate::data::users::ConnectedProfile;
use crate::languages::Translator;
use askama::Template;
use askama_web::WebTemplate;
use chrono::NaiveDate;

#[derive(Template, WebTemplate)]
#[template(path = "satisfying_moments/add_moment_modal_button_content.html")]
struct AddMomentModalButtonContent {
    connected_profile: ConnectedProfile,
    day: NaiveDate,
}

#[derive(Template, WebTemplate)]
#[template(path = "satisfying_moments/satisfying_moments_of_day.html")]
pub struct SatisfyingMomentsOfDay {
    connected_profile: ConnectedProfile,
    satisfying_moments: Vec<SatisfyingMoment>,
    add_modal_button: ModalButton,
}

impl SatisfyingMomentsOfDay {
    pub fn from(
        connected_profile: ConnectedProfile,
        day: NaiveDate,
        satisfying_moments: Vec<SatisfyingMoment>,
    ) -> Self {
        let add_modal_button = ModalButton::from(
            connected_profile.clone(),
            "primary",
            "plus",
            connected_profile.translate("add"),
            "add",
            connected_profile.translate("satisfying_moment_new"),
            AddMomentModalButtonContent {
                connected_profile: connected_profile.clone(),
                day,
            }
            .render()
            .unwrap(),
            "/satisfying_moments/new",
        );

        Self {
            connected_profile,
            satisfying_moments,
            add_modal_button,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "satisfying_moments/satisfying_moment_card.html")]
pub struct SatisfyingMomentCard {
    connected_profile: ConnectedProfile,
    satisfying_moment: SatisfyingMoment,
    with_date: bool,
}

impl SatisfyingMomentCard {
    pub fn from(
        connected_profile: ConnectedProfile,
        satisfying_moment: SatisfyingMoment,
        with_date: bool,
    ) -> Self {
        Self {
            connected_profile,
            satisfying_moment,
            with_date,
        }
    }

    pub fn from_without_date(
        connected_profile: ConnectedProfile,
        satisfying_moment: SatisfyingMoment,
    ) -> Self {
        Self::from(connected_profile, satisfying_moment, false)
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "satisfying_moments/satisfying_moment_page.html")]
pub struct SatisfyingMomentPage {
    navigation_bar: NavigationBar,
    connected_profile: ConnectedProfile,
    satisfying_moment: SatisfyingMoment,
    edit_mode: bool,
    delete_button: ModalButton,
}

impl SatisfyingMomentPage {
    pub fn from(
        connected_profile: ConnectedProfile,
        satisfying_moment: SatisfyingMoment,
        edit_mode: bool,
    ) -> Self {
        let delete_button = ModalButton::delete_button_from(
            connected_profile.clone(),
            "/satisfying_moments/delete",
            satisfying_moment.id,
        );

        Self {
            navigation_bar: NavigationBar::from(connected_profile.clone()),
            connected_profile,
            satisfying_moment,
            edit_mode,
            delete_button,
        }
    }
}
