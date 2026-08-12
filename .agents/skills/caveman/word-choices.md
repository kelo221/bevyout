# STE word choices — replacement table

The official ASD-STE100 dictionary approves ~900 words and lists ~1,200
unapproved words with alternatives (free spec: https://www.asd-ste100.org).
This table covers the unapproved words that most often appear in software
documentation and AI-generated prose. The principle behind every row: the
shortest common word with one meaning wins.

## Verbs

| Do not write | Write |
|---|---|
| utilize, leverage, employ | use |
| ensure, verify, confirm, validate (as "check") | make sure that |
| facilitate | help, make easier |
| perform, execute, conduct (an action) | do (or the specific verb: run, install) |
| commence, initiate | start |
| terminate, conclude | stop, end |
| implement (as "do") | do, install, add |
| obtain, acquire | get |
| require (as instruction) | must: "You must…" / "X is necessary" |
| attempt | try |
| demonstrate | show |
| modify, alter | change |
| assist | help |
| follow (as "comply with") | obey ("Obey the safety instructions") |
| enable/allow (as "make possible") | let, or state directly what happens |

## Connectors and phrases

| Do not write | Write |
|---|---|
| prior to | before |
| subsequent to, following (as preposition) | after |
| in order to | to |
| in the event that | if |
| with regard to, regarding | about, for |
| due to the fact that | because |
| furthermore, additionally, moreover | also, and |
| however (mid-sentence pileups) | but |
| it is important/worth noting that | (delete — state the fact) |
| as well as | and |
| a number of, a variety of | some, many, or the count |
| approximately | about (or the exact number) |

## Adjectives and adverbs (mostly: delete)

| Do not write | Write |
|---|---|
| appropriate, applicable, relevant | the specific thing ("the correct port") |
| seamlessly, easily, simply, quickly | (delete) |
| robust, powerful, comprehensive | (delete, or the measurable fact) |
| optimal | best |
| sufficient | enough |
| numerous | many |
| additional | more |
| potentially, possibly (in warnings) | (delete — a warning states what WILL occur) |

## One word, one part of speech — examples from the standard

- **oil**: noun only. "The oil is dirty" ✓ — "Oil the valve" ✗ (write
  "Apply oil to the valve" or use a technical verb).
- **close**: verb only, two meanings: move together ("Close the door") or
  operate a circuit breaker. Not "close the meeting" (write "end the
  meeting").
- **test**: pick noun or verb once per document and hold it ("do a test" vs
  "test the pump" — not both).

When a needed technical term is not in this table, keep it: domain
terminology, product names, commands, and identifiers are technical names and
are always allowed.
