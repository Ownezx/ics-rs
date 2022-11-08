pub mod cal_adress;
pub mod class;
pub mod status;
pub mod uri;

pub enum Property {
    // Time properties
    DTstamp,
    Completed,
    Created,
    DTStart,
    LastModification,
    RecurrenceID,
    ExDate,
    RDate,

    // String properties
    UID,
    Description,
    Location,
    Summary,
    Comment,
    Related,
    Resources,
    Categories,

    // Cal adress properties
    Organizer,
    Attendee,
    Contact,

    // Integer properties
    PercentComplete,
    Priority,
    Sequence,

    // Status,
    Status,

    // URI properties
    URL,
    Attach,

    // Others
    Geo,
    Class,
}

impl Property {
    pub fn get_property_from_identifier(identifier: String) -> Property {
        match identifier.as_str() {
            "ORGANIZER" => Property::Organizer,
            "PERCENT-COMPLETE" => Property::PercentComplete,
            "PRIORITY" => Property::Priority,
            "SEQUENCE" => Property::Sequence,
            "STATUS" => Property::Status,
            "URL" => Property::URL,
            "ATTACH" => Property::Attach,
            "GEO" => Property::Geo,
            "CLASS" => Property::Class,
            (_) => {
                panic!("Unknown identifier")
            }
        }
    }
}
