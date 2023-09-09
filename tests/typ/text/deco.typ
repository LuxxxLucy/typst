// Test text decorations.

---
#let red = rgb("fc0030")

// Basic strikethrough.
#strike[Statements dreamt up by the utterly deranged.]

// Move underline down.
#underline(offset: 5pt)[Further below.]

// Different color.
#underline(stroke: red, evade: false)[Critical information is conveyed here.]

// Inherits font color.
#text(fill: red, underline[Change with the wind.])

// Both over- and underline.
#overline(underline[Running amongst the wolves.])

---
#let redact = strike.with(stroke: 10pt, extent: 0.05em)
#let highlight-custom = strike.with(stroke: 10pt + rgb("abcdef88"), extent: 0.05em)

// Abuse thickness and transparency for redacting and highlighting stuff.
Sometimes, we work #redact[in secret].
There might be #highlight-custom[redacted] things.
 underline()

This is the built-in #highlight[highlight with default color]. We can also specify
a customized value #highlight(fill: rgb("abcdef88"))[to highlight]. Notice color
difference with the #highlight-custom[redacted] above.

Default highlight:

#highlight[ace], #highlight[base],#highlight[super], #highlight[phone]

#let highlight-tight = highlight.with(top-edge: "x-height", bottom-edge: "baseline")

A tighter highlight:

#highlight-tight[ace], #highlight-tight[base],#highlight-tight[super], #highlight-tight[phone]

---
// Test stroke folding.
#set underline(stroke: 2pt, offset: 2pt)
#underline(text(red, [DANGER!]))
