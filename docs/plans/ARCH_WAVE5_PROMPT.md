# Architecture wave 5 — kickoff prompt

Requested 2026-07-18 from the attached architecture reviews as the fifth
refactoring wave: add extension traits for console providers and content
record resolution.

Wave 5 is issue #147 under architecture epic #142. The user explicitly allows
improving the suggestion, so this wave rejects speculative, unused traits:
each trait must have a built-in production implementation, a production
consumer, and a synthetic test implementation.
