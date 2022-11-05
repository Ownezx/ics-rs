pub mod duration_properties;
pub mod geo;
pub mod integer_properties;
pub mod recurrence_id;
pub mod string_properties;
pub mod time_properties;
pub mod uri_properties;
pub mod utc_offset_properties;
pub mod vcalendar;

/// `UID` Property : defines the persistent, globally unique identifier for the calendar component.
pub struct UID {}

/// `EXRULE` Property : defines a rule or repeating pattern for an exception to a recurrence set.
pub struct ExRule {}

/// `REPEAT` Property : defines the number of time the alarm should be repeated, after the initial trigger. (It contains an integer and duration)
pub struct Repat {} // This needs integer and duration together

/// `TRIGGER` Property : specifies when an alarm will trigger.
pub struct Trigger {} // This can be a duration or a time
