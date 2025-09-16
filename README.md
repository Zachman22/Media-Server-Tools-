# Media-Server-Tools

Experimental collection of helpers for managing a self-hosted media server.

## System control

The `system_control` module exposes helpers that build safe `Command`
objects for common administrative tasks such as:

- System updates (apt)
- Firmware updates (fwupdmgr)
- Driver installation and removal
- Shutdown and restart
- Logging off or switching users

These commands are only constructed; the caller is responsible for
spawning them with appropriate privileges.
