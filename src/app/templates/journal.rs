use crate::app::templates::NavigationBar;
use crate::data::users::ConnectedProfile;
use crate::dates::WithTimeZone;
use askama::Template;
use askama_web::WebTemplate;
use chrono::NaiveDate;

#[derive(Template, WebTemplate)]
#[template(path = "journal/journal_of_day_page.html")]
pub struct JournalOfDayPage {
    navigation_bar: NavigationBar,
    url_param_with_day: String,
}

impl JournalOfDayPage {
    pub fn from(connected_profile: ConnectedProfile, day: Option<NaiveDate>) -> Self {
        let url_param_with_day = match day {
            Some(day) => format!("?day={}", day.to_string()),
            None => String::new(),
        };

        Self {
            navigation_bar: NavigationBar {
                profile: connected_profile.into(),
            },
            url_param_with_day,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "journal/day_navigation.html")]
pub struct DayNavigation {
    previous_day: Option<NaiveDate>,
    day: NaiveDate,
    next_day: Option<NaiveDate>,
}

impl DayNavigation {
    pub fn from(connected_profile: ConnectedProfile, day: Option<NaiveDate>) -> Self {
        let today = connected_profile.today();

        let day = day.unwrap_or(today);

        let previous_day = if day > today {
            Some(today)
        } else {
            day.pred_opt()
        };

        let next_day = if day < today { day.succ_opt() } else { None };

        Self {
            previous_day,
            day,
            next_day,
        }
    }
}
