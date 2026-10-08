# 29 — Accessibility, internationalization, and inclusive native quality

## Acceptance position
Accessibility is a foundation-stage product requirement. Use WCAG 2.2 as a web acceptance reference and apply equivalent native usability principles with platform-specific assistive-technology testing. This document is not a conformance certificate. [S18.]

## Text and input
Test Latin and non-Latin scripts, Chinese/Japanese/Korean IMEs, right-to-left and mixed-direction text, combining marks, grapheme clusters, bidirectional controls, long unbroken strings, and pasted structured content. Enter must not send while composition is active. Cursor movement, delete/backspace, selection, copy/paste, undo, and mention insertion must respect the text model. Code views and message text may need different directionality rules.

Store UTC event times and preserve named timezones for schedules. Render dates/numbers using user locale. Avoid ambiguous relative times in critical audit/approval views; include an exact timestamp on inspection. Daylight-saving changes must not duplicate or silently skip a scheduled consequential action; define the policy and test it.

## Semantic interface
Every actionable element has a stable role, label, focus state, and keyboard path. Channels and messages expose navigable structure. Status changes are announced with appropriate urgency without speaking every token. A review package identifies version, evidence status, and the decision being requested. Color is never the only difference between failed, passed, pending, or stale.

Dialogs trap focus appropriately, restore it on close, and do not discard drafts on Escape. Provide visible focus rings and meaningful disabled-state explanations. Scrolling, opening threads, navigation history, and context actions work without a pointer. Do not place a terminal-only control in the path of ordinary review.

## Visual and motion settings
Support sufficient contrast, text scaling, comfortable/compact density, reduced motion, and system theme behavior. Test truncation and reflow under enlarged text and narrow windows. Avoid automatic scroll jumps while users read. Animated run indicators do not need continuous GPU redraw when nothing useful changes.

## Platform matrix
Record the selected GPUI version, OS version, assistive technology, input method, display scaling, and known limitations. Run native tests on the platforms actually claimed supported. Do not extrapolate accessibility from a framework announcement or a single macOS test. Browser review needs its own keyboard/screen-reader and responsive-layout tests.

## International product behavior
Names and channel titles allow Unicode under safe normalization rules. Do not use display-name text as a principal identifier. Search behavior must be tested across relevant languages and tokenization, with exact identifiers still working. Localization keys separate UI text from code; agent-generated content remains clearly distinguished from trusted application labels. Approval wording must be unambiguous after translation.

## Gate
A user relying on keyboard and screen reader can open a channel, compose/recover a draft, create a task, inspect evidence, request changes, and accept a version-bound result. Unusable input composition, inaccessible approval controls, unreadable contrast, or lost focus that blocks completion is a release blocker for the affected supported surface.
