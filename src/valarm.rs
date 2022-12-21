/*
alarmc     = "BEGIN" ":" "VALARM" CRLF
                    (audioprop / dispprop / emailprop)
                    "END" ":" "VALARM" CRLF

       audioprop  = *(
                  ;
                  ; 'action' and 'trigger' are both REQUIRED,
                  ; but MUST NOT occur more than once.
                  ;
                  action / trigger /
                  ;
                  ; 'duration' and 'repeat' are both OPTIONAL,
                  ; and MUST NOT occur more than once each;
                  ; but if one occurs, so MUST the other.
                  ;
                  duration / repeat /
                  ;
                  ; The following is OPTIONAL,
                  ; but MUST NOT occur more than once.
                  ;
                  attach /
                  ;
                  ; The following is OPTIONAL,
                  ; and MAY occur more than once.
                  ;
                  x-prop / iana-prop
                  ;
                  )

       dispprop   = *(
                  ;
                  ; The following are REQUIRED,
                  ; but MUST NOT occur more than once.
                  ;
                  action / description / trigger /
                  ;
                  ; 'duration' and 'repeat' are both OPTIONAL,
                  ; and MUST NOT occur more than once each;
                  ; but if one occurs, so MUST the other.
                  ;
                  duration / repeat /
                  ;
                  ; The following is OPTIONAL,
                  ; and MAY occur more than once.
                  ;
                  x-prop / iana-prop
                  ;
                  )

       emailprop  = *(
                  ;
                  ; The following are all REQUIRED,
                  ; but MUST NOT occur more than once.
                  ;
                  action / description / trigger / summary /
                  ;
                  ; The following is REQUIRED,
                  ; and MAY occur more than once.
                  ;
                  attendee /
                  ;
                  ; 'duration' and 'repeat' are both OPTIONAL,
                  ; and MUST NOT occur more than once each;
                  ; but if one occurs, so MUST the other.
                  ;
                  duration / repeat /
                  ;
                  ; The following are OPTIONAL,
                  ; and MAY occur more than once.
                  ;
                  attach / x-prop / iana-prop
                  ;
                  )
 */
