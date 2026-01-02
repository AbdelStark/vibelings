Look at the example output structure carefully. You need event_name, date, and sessions array.

---

Each session needs 6 required fields: id, title, speaker, time_slot, duration_minutes, and track.

---

The id field must match the pattern "SXXX" where XXX is a 3-digit number (e.g., "S001", "S002").

---

The track field must be exactly one of: "technical", "workshop", or "keynote" - no other values are accepted.

---

The time_slot must be in 24-hour format (HH:MM). Valid examples: "09:00", "14:30", "23:59". Invalid: "9:00", "2:30pm".

---

The date must be in ISO format (YYYY-MM-DD). Example: "2025-06-15". Don't use other date formats.

---

You need at least 2 sessions in the array. The maximum is 10 sessions.

---

Duration must be an integer between 15 and 120 (inclusive). Common values: 30, 45, 60, 90.
