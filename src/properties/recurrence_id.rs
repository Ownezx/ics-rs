use crate::untested_property;

untested_property!(
    RecurrenceID,
    "RECURRENCE-ID",
    "used in conjunction with the 'UID' and 'SEQUENCE' property to identify a specific instance of a recurring 'VEVENT', 'VTODO' or 'VJOURNAL' calendar component. The property value is the effective value of the 'DTSTART' property of the recurrence instance."
);
