# reflowable schedule idea

Events exist in a 2D space:
- location (room/hall/tent)
- time

You see basically this in schedule.mch2022.org. It's a big 2D field with events
placed in room-columns at their appropriate times.

Two linear projections of this are:
- events in room ordered by time
- events at time ordered by room

Each event has:
- room
- time (start - end, duration)
- abstract/description
- track
- speaker
- type (workshop/talk/...)

Event cross-ref:
- overlapping events (time)
- events in same track
- event in events-by-room projection
- event in events-by-time projection
