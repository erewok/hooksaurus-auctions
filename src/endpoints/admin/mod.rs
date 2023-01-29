use time::format_description::well_known::Rfc3339;

mod handlers;
mod queries;

use crate::db::tables;
pub use handlers::router;
use hooksaurus_core::{AdminRow, Pagination, ToForm};

impl ToForm for tables::address::Address {
    fn to_form(&self) -> String {
        format!(
            r##"
                <div class="uk-margin">
                    <input class="uk-input" type="text" name="street_address1" placeholder="Street address Line 1" required value="{}">
                </div>
                <div class="uk-margin">
                    <input class="uk-input" type="text" name="street_address2" placeholder="Street address Line 2" value="{}">
                </div>
                <div class="uk-margin">
                    <input class="uk-input" type="text" name="street_address3" placeholder="Street address Line 3" value="{}">
                </div>
                <div class="uk-margin">
                    <input class="uk-input" type="text" name="city" placeholder="City" required value="{}">
                </div>
                <div class="uk-margin">
                    <input class="uk-input" type="text" name="state_province_county"
                        placeholder="State, Province, or County" required value="{}">
                </div>
                <div class="uk-margin">
                    <input class="uk-input" type="text" name="postal_code" placeholder="Postal Code" value="{}">
                </div>
                <div class="uk-margin">
                    <input class="uk-input" type="text" name="country_code" placeholder="Country" value="{}">
                </div>
                <div class="uk-margin">
                    <input class="uk-input" type="number" name="latitude" placeholder="Latitude" value="{}">
                </div>
                <div class="uk-margin">
                    <input class="uk-input" type="number" name="longitude" placeholder="longitude" value="{}">
                </div>
        "##,
            self.street_address1,
            self.street_address2
                .clone()
                .unwrap_or_else(|| "".to_string()),
            self.street_address3
                .clone()
                .unwrap_or_else(|| "".to_string()),
            self.city,
            self.state_province_county,
            self.postal_code.clone().unwrap_or_else(|| "".to_string()),
            self.country_code.clone().unwrap_or_else(|| "".to_string()),
            self.latitude.clone().unwrap_or(0.0),
            self.longitude.clone().unwrap_or(0.0),
        )
    }
    fn to_empty_form() -> String {
        r##"
            <div class="uk-margin">
                <input class="uk-input" type="text" name="title" placeholder="Auction Title" required>
            </div>
            <div class="uk-margin">
                <textarea class="uk-textarea" rows="5" placeholder="description" name="description"></textarea>
            </div>
            <div class="uk-margin">
                <input data-uk-datepicker="{format:'DD.MM.YYYY'}" name="start_date_date" required>
                <input data-uk-timepicker="{format:'12h'}" name="start_date_time" required>
            </div>
            <div class="uk-margin">
                <input data-uk-datepicker="{format:'DD.MM.YYYY'}" name="end_date_date" required>
                <input data-uk-timepicker="{format:'12h'}" name="end_date_time" required>
            </div>
            <div class="uk-margin">
                <input class="uk-input" type="text" name="benefits_organization_id"
                    placeholder="Shore Sanctuary" required>
            </div>
        "##.to_string()
    }
}

impl ToForm for tables::auction::Auction {
    fn to_form(&self) -> String {
        format!(
            r##"
            <div class="uk-margin">
                <input class="uk-input" type="text" name="title" placeholder="Auction Title" required value="{}">
            </div>
            <div class="uk-margin">
                <textarea class="uk-textarea" rows="5" placeholder="description" name="description" value="{}"></textarea>
            </div>
            <div class="uk-margin">
                <label class="uk-form-label">Start Date</label>
                <input class="uk-input" type="datetime-local" name="start_date" required value="{}">
            </div>
            <div class="uk-margin">
                <label class="uk-form-label">End Date</label>
                <input class="uk-input" type="datetime-local" name="end_date" required value="{}">
            </div>
            <div class="uk-margin">
                <label class="uk-form-label">Auction Benefits Organization</label>
                <input class="uk-input" type="text" name="benefits_organization_id"
                    placeholder="Shore Sanctuary" required value="{}">
            </div>
        "##,
            self.title,
            self.description,
            self.start_date.format(&Rfc3339).unwrap_or_default(),
            self.end_date.format(&Rfc3339).unwrap_or_default(),
            self.benefits_organization_id
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
        )
    }
    fn to_empty_form() -> String {
        r##"
            <div class="uk-margin">
                <input class="uk-input" type="text" name="title" placeholder="Auction Title" required>
            </div>
            <div class="uk-margin">
                <textarea class="uk-textarea" rows="5" placeholder="description" name="description"></textarea>
            </div>
            <div class="uk-margin">
                <label class="uk-form-label">Start Date</label>
                <input class="uk-input" type="datetime-local" name="start_date" required>
            </div>
            <div class="uk-margin">
                <label class="uk-form-label">End Date</label>
                <input class="uk-input" type="datetime-local" name="end_date" required>
            </div>
            <div class="uk-margin">
                <label class="uk-form-label">Auction Benefits Organization</label>
                <input class="uk-input" type="text" name="benefits_organization_id"
                    placeholder="Shore Sanctuary" required>
            </div>
        "##.to_string()
    }
}
