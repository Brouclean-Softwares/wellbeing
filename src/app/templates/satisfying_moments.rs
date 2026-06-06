use crate::data::satisfying_moments::SatisfyingMoment;
use crate::data::users::ConnectedProfile;
use crate::dates::WithTimeZone;
use crate::languages::Translator;
use askama::Template;
use askama_web::WebTemplate;
use chrono::NaiveDate;

#[derive(Template, WebTemplate)]
#[template(path = "satisfying_moments/satisfying_moments_of_the_day.html")]
pub struct SatisfyingMomentsOfTheDay {
    connected_profile: ConnectedProfile,
    title: String,
    satisfying_moments: Vec<SatisfyingMoment>,
}

impl SatisfyingMomentsOfTheDay {
    pub fn get(
        connected_profile: &ConnectedProfile,
        day: &NaiveDate,
        satisfying_moments: &Vec<SatisfyingMoment>,
    ) -> Self {
        let title = if connected_profile.today().eq(day) {
            connected_profile.translate("satisfying_moments_of_day")
        } else {
            format!(
                "{} - {}",
                connected_profile.translate("satisfying_moments"),
                day
            )
        };

        Self {
            connected_profile: connected_profile.clone(),
            title,
            satisfying_moments: satisfying_moments.clone(),
        }
    }
}
