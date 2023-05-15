# ics-rs

## Introduction

This is ispired by the ics and ical-rs crates. It is meant to parse and validate as well as write ics files.

In the long term, the crate should be able to adress VTODOs, VEVENTs, VALARMs and VJOURNAL entries in VCALENDAR files.

## Testing

Several test files can be found in the test/test_files folder, those were gathered from different sources to validate that the parsing is done correctly. To run tests use `cargo test`.

## Features

- [ ] Reading VTODO (Partially implemented)
- [ ] Reading VEVENT
- [ ] Reading VJOURNAL
- [ ] Reading VCALENDAR
- [ ] X-Property support
- [ ] Iana-Property support
- [ ] Writing

## Docs

<https://www.kanzaki.com/docs/ical/>

<https://icalendar.org/RFC-Specifications/iCalendar-RFC-5545/>

<https://www.rfc-editor.org/rfc/rfc5545>

<https://www.rfc-editor.org/rfc/rfc7986>
