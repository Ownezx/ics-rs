/*
The property is defined by the following notation:

  status     = "STATUS" statparam] ":" statvalue CRLF

  statparam  = *(";" xparam)

  statvalue  = "TENTATIVE"           ;Indicates event is
                                     ;tentative.
             / "CONFIRMED"           ;Indicates event is
                                     ;definite.
             / "CANCELLED"           ;Indicates event was
                                     ;cancelled.
     ;Status values for a "VEVENT"
  statvalue  =/ "NEEDS-ACTION"       ;Indicates to-do needs action.
             / "COMPLETED"           ;Indicates to-do completed.
             / "IN-PROCESS"          ;Indicates to-do in process of
             / "CANCELLED"           ;Indicates to-do was cancelled.
     ;Status values for "VTODO".

  statvalue  =/ "DRAFT"              ;Indicates journal is draft.
             / "FINAL"               ;Indicates journal is final.
             / "CANCELLED"           ;Indicates journal is removed.
     ;Status values for "VJOURNAL".
*/

pub enum VTodoStatus {
    NeedsAction,
    Completed,
    InProgress,
    Cancelled,
}

pub enum VEventStatus {
    Tentative,
    Confirmed,
    Cancelled,
}

pub enum VJournal {
    Draft,
    Final,
    Cancelled,
}
