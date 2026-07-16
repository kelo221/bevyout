# Bevyout Discord Bot

This is separate from the Forgejo `forgejo-delibot` service. It receives GitHub
pull-request events, loads the personality profiles from `personalities.json`,
asks Gemini for one original notification, and returns a Discord-ready payload
to the GitHub workflow. The workflow posts that payload to Discord because the
VM's Azure egress is rejected by Discord's edge network.

The service reloads `personalities.json` for every request, so adding a profile
does not require changing the GitHub workflow. The production deployment lives
at `/opt/bevyout-discord-bot` and listens only on `127.0.0.1:8978`; Caddy exposes
the authenticated endpoint at `/api/bevyout-discord` over the existing HTTPS
listener.

Required environment variables:

- `BOT_AUTH_TOKEN`: shared bearer token used by GitHub Actions.
- `GEMINI_API_KEY`: Gemini API key.
- `GEMINI_MODEL`: defaults to `gemini-3.1-flash-lite`.
- `PORT`: defaults to `8978`.
