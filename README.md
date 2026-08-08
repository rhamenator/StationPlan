# StationPlan

A generic field-station scheduling core derived from `E:\MDOT\SCHEDULE` while
removing agency names, fixed areas/counties, and highway-specific schemas. It
prioritizes visits, respects scheduling windows, assigns bounded crews, and
fails when work cannot fit.

```powershell
cargo test
cargo run
```

Next slices: dates/calendars, crew skills, travel matrices, pickup windows,
remarks/status history, rescheduling, CSV import/export, and route optimization
using the compatibility corpus graph algorithms.
