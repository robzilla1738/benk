//! Real text input for the composer: a `EntityInputHandler` implementation so
//! the OS input stack (IME marked text, dictation, clipboard) is authoritative.
//!
//! Model: the buffer is UTF-8 `String`s; the platform contract is UTF-16
//! ranges. All conversions happen in two helpers — never mix them elsewhere.
//! Text is laid out per-paragraph (one `ShapedLine` per `\n`-separated
//! paragraph); long lines clip instead of wrapping — soft-wrap is a documented
//! follow-up (BENK-002), not hidden behavior.
use crate::theme::{TEXT_BODY, theme};
use gpui::prelude::*;
use gpui::{
    App, Bounds, ClipboardItem, Context, Element, ElementId, ElementInputHandler,
    EntityInputHandler, EventEmitter, FocusHandle, Hitbox, Hsla, IntoElement, KeyDownEvent,
    LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Point, ScrollDelta,
    ScrollWheelEvent, ShapedLine, SharedString, Style, StyleRefinement, Styled, TextRun,
    UTF16Selection, UnderlineStyle, Window, actions, div, fill, point, px,
};
use std::ops::Range;

const LINE_HEIGHT: f32 = 20.0;
const FONT_SIZE: f32 = TEXT_BODY;
const PAD_Y: f32 = 12.0;
const PAD_X: f32 = 14.0;
const MAX_LINES: usize = 6;

pub enum ComposerEvent {
    /// Enter pressed on a non-empty buffer outside IME composition.
    Submitted(String),
    /// Text changed (draft capture hook).
    Changed,
}

pub struct Composer {
    text: String,
    placeholder: SharedString,
    single_line: bool,
    /// UTF-16 selection range within `text`.
    selected_utf16: Range<usize>,
    selection_reversed: bool,
    /// Active IME marked (composing) range in UTF-16, if any.
    marked_utf16: Option<Range<usize>>,
    focus_handle: FocusHandle,
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    /// Coalesced edit kind — consecutive character inserts share one undo entry.
    last_op: Option<OpKind>,
    /// Paragraph-shaped lines + their UTF-8 starts, computed at paint.
    lines: Vec<ParagraphLine>,
    last_bounds: Option<Bounds<Pixels>>,
    scroll_offset: Pixels,
    dragging: bool,
    blink_visible: bool,
}

#[derive(Clone)]
struct Snapshot {
    text: String,
    selection: Range<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OpKind {
    Insert,
    Delete,
    Other,
}

struct ParagraphLine {
    start_utf8: usize,
    shaped: ShapedLine,
}

// -- UTF-16 <-> UTF-8 boundary mapping -------------------------------------

/// Byte offset of the `utf16`-th UTF-16 code unit, snapped to a char boundary.
fn utf16_to_utf8(text: &str, utf16: usize) -> usize {
    let mut units = 0;
    for (byte, ch) in text.char_indices() {
        if units >= utf16 {
            return byte;
        }
        units += ch.len_utf16();
        if units > utf16 {
            return byte;
        }
    }
    text.len()
}

fn utf8_to_utf16(text: &str, utf8: usize) -> usize {
    text[..utf8.min(text.len())].encode_utf16().count()
}

impl Composer {
    pub fn new(
        placeholder: impl Into<SharedString>,
        single_line: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            text: String::new(),
            placeholder: placeholder.into(),
            single_line,
            selected_utf16: 0..0,
            selection_reversed: false,
            marked_utf16: None,
            focus_handle: cx.focus_handle(),
            undo: vec![],
            redo: vec![],
            last_op: None,
            lines: vec![],
            last_bounds: None,
            scroll_offset: px(0.),
            dragging: false,
            blink_visible: true,
        }
    }

    pub fn focus_handle(&self) -> FocusHandle {
        self.focus_handle.clone()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// Update the placeholder shown when the buffer is empty (channel switch).
    pub fn set_placeholder(
        &mut self,
        placeholder: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) {
        self.placeholder = placeholder.into();
        cx.notify();
    }

    /// Replace contents (draft restore / programmatic clear).
    pub fn set_text(&mut self, text: &str, cx: &mut Context<Self>) {
        self.text = text.to_string();
        self.selected_utf16 =
            utf8_to_utf16(&self.text, self.text.len())..utf8_to_utf16(&self.text, self.text.len());
        self.marked_utf16 = None;
        self.undo.clear();
        self.redo.clear();
        self.clamp_scroll();
        cx.emit(ComposerEvent::Changed);
        cx.notify();
    }

    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }

    fn cursor_utf16(&self) -> usize {
        if self.selection_reversed {
            self.selected_utf16.start
        } else {
            self.selected_utf16.end
        }
    }

    fn cursor_utf8(&self) -> usize {
        utf16_to_utf8(&self.text, self.cursor_utf16())
    }

    fn selection_utf8(&self) -> Range<usize> {
        utf16_to_utf8(&self.text, self.selected_utf16.start)
            ..utf16_to_utf8(&self.text, self.selected_utf16.end)
    }

    fn set_cursor_utf8(&mut self, byte: usize) {
        let u16 = utf8_to_utf16(&self.text, byte);
        self.selected_utf16 = u16..u16;
        self.selection_reversed = false;
    }

    // -- editing -------------------------------------------------------------

    fn snapshot(&mut self) {
        self.undo.push(Snapshot {
            text: self.text.clone(),
            selection: self.selected_utf16.clone(),
        });
        self.redo.clear();
    }

    /// Push an undo boundary unless this op continues the previous char insert.
    fn begin_op(&mut self, kind: OpKind) {
        if !(kind == OpKind::Insert && self.last_op == Some(OpKind::Insert)) {
            self.snapshot();
        }
        self.last_op = Some(kind);
    }

    fn end_ops(&mut self) {
        self.last_op = None;
    }

    fn replace_utf8(&mut self, range: Range<usize>, with: &str, cx: &mut Context<Self>) {
        self.text.replace_range(range.clone(), with);
        self.set_cursor_utf8(range.start + with.len());
        self.clamp_scroll();
        cx.emit(ComposerEvent::Changed);
        cx.notify();
    }

    fn backspace(&mut self, cx: &mut Context<Self>) {
        self.end_ops();
        let sel = self.selection_utf8();
        if sel.start == sel.end {
            if sel.start == 0 {
                return;
            }
            // Delete one grapheme-ish unit (char boundary walk-back).
            let prev = self.text[..sel.start]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.begin_op(OpKind::Delete);
            self.replace_utf8(prev..sel.end, "", cx);
        } else {
            self.begin_op(OpKind::Delete);
            self.replace_utf8(sel, "", cx);
        }
    }

    fn delete_forward(&mut self, cx: &mut Context<Self>) {
        self.end_ops();
        let sel = self.selection_utf8();
        if sel.start == sel.end {
            let next = self.text[sel.end..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| sel.end + i)
                .unwrap_or(self.text.len());
            if next == sel.end {
                return;
            }
            self.begin_op(OpKind::Delete);
            self.replace_utf8(sel.start..next, "", cx);
        } else {
            self.begin_op(OpKind::Delete);
            self.replace_utf8(sel, "", cx);
        }
    }

    // -- cursor movement ------------------------------------------------------

    fn move_to(&mut self, byte: usize, extend: bool, cx: &mut Context<Self>) {
        self.end_ops();
        let u16 = utf8_to_utf16(&self.text, byte.min(self.text.len()));
        if extend {
            if self.selection_reversed {
                self.selected_utf16.start = u16;
            } else {
                self.selected_utf16.end = u16;
            }
            if self.selected_utf16.is_empty() {
                self.selection_reversed = false;
            } else if !self.selection_reversed && u16 < self.selected_utf16.start {
                self.selected_utf16.start = u16;
                self.selection_reversed = true;
            }
        } else {
            self.selected_utf16 = u16..u16;
            self.selection_reversed = false;
        }
        self.ensure_cursor_visible();
        cx.notify();
    }

    fn move_horizontal(&mut self, dir: i32, by_word: bool, extend: bool, cx: &mut Context<Self>) {
        if !extend && self.selected_utf16.start != self.selected_utf16.end {
            // Collapse to the near/far edge like native editors.
            let edge = if dir < 0 {
                self.selection_utf8().start
            } else {
                self.selection_utf8().end
            };
            self.move_to(edge, false, cx);
            return;
        }
        let cur = self.cursor_utf8();
        let target = if by_word {
            self.word_boundary(cur, dir)
        } else if dir < 0 {
            self.text[..cur]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0)
        } else {
            cur + self.text[cur..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(0)
        };
        self.move_to(target, extend, cx);
    }

    fn word_boundary(&self, from: usize, dir: i32) -> usize {
        let is_word = |c: char| c.is_alphanumeric() || c == '_';
        let bytes = self.text.as_bytes();
        if dir < 0 {
            let mut i = from;
            while i > 0 && !is_word(self.text[..i].chars().next_back().unwrap_or(' ')) {
                i = self.text[..i]
                    .char_indices()
                    .next_back()
                    .map(|(j, _)| j)
                    .unwrap_or(0);
            }
            while i > 0 && is_word(self.text[..i].chars().next_back().unwrap_or(' ')) {
                i = self.text[..i]
                    .char_indices()
                    .next_back()
                    .map(|(j, _)| j)
                    .unwrap_or(0);
            }
            i
        } else {
            let _ = bytes;
            let mut i = from;
            while i < self.text.len() && !is_word(self.text[i..].chars().next().unwrap_or(' ')) {
                i += self.text[i..]
                    .chars()
                    .next()
                    .map(|c| c.len_utf8())
                    .unwrap_or(0);
            }
            while i < self.text.len() && is_word(self.text[i..].chars().next().unwrap_or(' ')) {
                i += self.text[i..]
                    .chars()
                    .next()
                    .map(|c| c.len_utf8())
                    .unwrap_or(0);
            }
            i
        }
    }

    fn move_vertical(&mut self, dir: i32, extend: bool, cx: &mut Context<Self>) {
        // Same paragraph => single line; move to edge. Otherwise hop paragraphs.
        let cur = self.cursor_utf8();
        let para = self
            .lines
            .iter()
            .enumerate()
            .find(|(_, l)| cur >= l.start_utf8 && cur <= l.start_utf8 + l.shaped.len())
            .map(|(i, _)| i)
            .unwrap_or(0);
        let Some(line) = self.lines.get(para) else {
            self.move_to(if dir < 0 { 0 } else { self.text.len() }, extend, cx);
            return;
        };
        let x = line.shaped.x_for_index(cur - line.start_utf8);
        let target = para as i32 + dir;
        if target < 0 {
            self.move_to(0, extend, cx);
            return;
        }
        let Some(next) = self.lines.get(target as usize) else {
            self.move_to(self.text.len(), extend, cx);
            return;
        };
        self.move_to(
            next.start_utf8 + next.shaped.closest_index_for_x(x),
            extend,
            cx,
        );
    }

    fn move_line_edge(
        &mut self,
        start: bool,
        whole_doc: bool,
        extend: bool,
        cx: &mut Context<Self>,
    ) {
        if whole_doc {
            let byte = if start { 0 } else { self.text.len() };
            self.move_to(byte, extend, cx);
            return;
        }
        let cur = self.cursor_utf8();
        let (s, e) = self.paragraph_bounds(cur);
        self.move_to(if start { s } else { e }, extend, cx);
    }

    fn paragraph_bounds(&self, byte: usize) -> (usize, usize) {
        let start = self.text[..byte].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let end = self.text[byte..]
            .find('\n')
            .map(|i| byte + i)
            .unwrap_or(self.text.len());
        (start, end)
    }

    // -- scroll ---------------------------------------------------------------

    fn content_height(&self) -> Pixels {
        px(self.lines.len().max(1) as f32 * LINE_HEIGHT)
    }

    fn clamp_scroll(&mut self) {
        let max = px(0.).max(self.content_height() - px(MAX_LINES as f32 * LINE_HEIGHT));
        if self.scroll_offset > max {
            self.scroll_offset = max;
        }
        if self.scroll_offset < px(0.) {
            self.scroll_offset = px(0.);
        }
    }

    fn ensure_cursor_visible(&mut self) {
        let cur = self.cursor_utf8();
        if let Some(i) = self
            .lines
            .iter()
            .position(|l| cur >= l.start_utf8 && cur <= l.start_utf8 + l.shaped.len())
        {
            let y = px(i as f32 * LINE_HEIGHT);
            let view_h = px(MAX_LINES.min(self.lines.len()) as f32 * LINE_HEIGHT);
            let _ = view_h;
            if y < self.scroll_offset {
                self.scroll_offset = y;
            } else if y + px(LINE_HEIGHT) > self.scroll_offset + px(MAX_LINES as f32 * LINE_HEIGHT)
            {
                self.scroll_offset = y + px(LINE_HEIGHT) - px(MAX_LINES as f32 * LINE_HEIGHT);
            }
        }
    }

    fn on_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) {
        let delta = match event.delta {
            ScrollDelta::Pixels(p) => p.y,
            ScrollDelta::Lines(l) => px(l.y * LINE_HEIGHT),
        };
        self.scroll_offset -= delta;
        self.clamp_scroll();
        cx.notify();
    }

    // -- actions --------------------------------------------------------------

    /// Enter / Send button submit. Never sends while IME composition is active.
    pub fn submit(&mut self, cx: &mut Context<Self>) {
        // Never send while an IME composition is active — Enter commits text.
        if self.marked_utf16.is_some() || self.is_empty() {
            return;
        }
        let text = std::mem::take(&mut self.text);
        self.selected_utf16 = 0..0;
        self.undo.clear();
        self.scroll_offset = px(0.);
        cx.emit(ComposerEvent::Submitted(text));
        cx.emit(ComposerEvent::Changed);
        cx.notify();
    }

    fn undo(&mut self, cx: &mut Context<Self>) {
        if let Some(prev) = self.undo.pop() {
            self.redo.push(Snapshot {
                text: self.text.clone(),
                selection: self.selected_utf16.clone(),
            });
            self.text = prev.text;
            self.selected_utf16 = prev.selection;
            cx.emit(ComposerEvent::Changed);
            cx.notify();
        }
    }

    fn redo(&mut self, cx: &mut Context<Self>) {
        if let Some(next) = self.redo.pop() {
            self.undo.push(Snapshot {
                text: self.text.clone(),
                selection: self.selected_utf16.clone(),
            });
            self.text = next.text;
            self.selected_utf16 = next.selection;
            cx.emit(ComposerEvent::Changed);
            cx.notify();
        }
    }

    fn select_all(&mut self, cx: &mut Context<Self>) {
        self.selected_utf16 = 0..utf8_to_utf16(&self.text, self.text.len());
        cx.notify();
    }

    fn copy_selection(&mut self, cut: bool, cx: &mut Context<Self>) {
        let sel = self.selection_utf8();
        if sel.is_empty() {
            return;
        }
        let text = self.text[sel.clone()].to_string();
        cx.write_to_clipboard(ClipboardItem::new_string(text));
        if cut {
            self.begin_op(OpKind::Other);
            self.replace_utf8(sel, "", cx);
        }
    }

    fn paste(&mut self, cx: &mut Context<Self>) {
        if let Some(item) = cx.read_from_clipboard()
            && let Some(mut text) = item.text()
        {
            if self.single_line {
                text = text.replace('\n', " ");
            }
            self.begin_op(OpKind::Insert);
            self.replace_utf8(self.selection_utf8(), &text, cx);
        }
    }

    // -- rendering helpers ----------------------------------------------------

    fn shape_paragraphs(&self, window: &Window, cx: &App) -> Vec<ParagraphLine> {
        let t = theme(cx);
        let font = window.text_style().font();
        let mut lines = Vec::new();
        for (start, para) in self.text.split('\n').scan(0usize, |off, p| {
            let s = *off;
            *off += p.len() + 1;
            Some((s, p))
        }) {
            let utf8_base = utf8_to_utf16(&self.text, start);
            let runs: Vec<TextRun> = match &self.marked_utf16 {
                Some(marked) => {
                    let ms = utf16_to_utf8(&self.text, marked.start).saturating_sub(start);
                    let me = utf16_to_utf8(&self.text, marked.end).saturating_sub(start);
                    let ms = ms.min(para.len());
                    let me = me.min(para.len()).max(ms);
                    if ms >= me {
                        vec![self.text_run(para.len(), t.text.into(), &font)]
                    } else {
                        let underline = UnderlineStyle {
                            thickness: px(1.),
                            color: Some(t.text.into()),
                            wavy: false,
                        };
                        vec![
                            self.text_run(ms, t.text.into(), &font),
                            TextRun {
                                underline: Some(underline),
                                ..self.text_run(me - ms, t.text.into(), &font)
                            },
                            self.text_run(para.len() - me, t.text.into(), &font),
                        ]
                        .into_iter()
                        .filter(|r| r.len > 0)
                        .collect()
                    }
                }
                None => vec![self.text_run(para.len(), t.text.into(), &font)],
            };
            let _ = utf8_base;
            let shaped = window.text_system().shape_line(
                SharedString::from(para.to_string()),
                px(FONT_SIZE),
                &runs,
                None,
            );
            lines.push(ParagraphLine {
                start_utf8: start,
                shaped,
            });
        }
        if lines.is_empty() {
            lines.push(ParagraphLine {
                start_utf8: 0,
                shaped: window.text_system().shape_line(
                    SharedString::from(""),
                    px(FONT_SIZE),
                    &[self.text_run(0, t.text.into(), &font)],
                    None,
                ),
            });
        }
        lines
    }

    fn text_run(&self, len: usize, color: Hsla, font: &gpui::Font) -> TextRun {
        TextRun {
            len,
            font: font.clone(),
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        }
    }

    /// Locate cursor → (line index, x, y) in element coordinates.
    fn caret_position(&self) -> (usize, Pixels) {
        let cur = self.cursor_utf8();
        for (i, line) in self.lines.iter().enumerate() {
            let end = line.start_utf8 + line.shaped.len();
            if cur <= end || i == self.lines.len() - 1 {
                return (
                    i,
                    line.shaped.x_for_index(cur.saturating_sub(line.start_utf8)),
                );
            }
        }
        (0, px(0.))
    }
}

impl EventEmitter<ComposerEvent> for Composer {}

// -- EntityInputHandler: the OS IME contract ---------------------------------

impl EntityInputHandler for Composer {
    fn text_for_range(
        &mut self,
        range: Range<usize>,
        adjusted: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let start = utf16_to_utf8(&self.text, range.start);
        let end = utf16_to_utf8(&self.text, range.end);
        *adjusted = Some(utf8_to_utf16(&self.text, start)..utf8_to_utf16(&self.text, end));
        Some(self.text[start..end].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.selected_utf16.clone(),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_utf16.clone()
    }

    fn unmark_text(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.marked_utf16 = None;
        cx.notify();
    }

    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range.unwrap_or_else(|| self.selected_utf16.clone());
        let utf8 = utf16_to_utf8(&self.text, range.start)..utf16_to_utf8(&self.text, range.end);
        self.begin_op(OpKind::Insert);
        self.text.replace_range(utf8.clone(), text);
        self.set_cursor_utf8(utf8.start + text.len());
        self.marked_utf16 = None;
        self.clamp_scroll();
        cx.emit(ComposerEvent::Changed);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range
            .or_else(|| self.marked_utf16.clone())
            .unwrap_or_else(|| self.selected_utf16.clone());
        let utf8 = utf16_to_utf8(&self.text, range.start)..utf16_to_utf8(&self.text, range.end);
        self.text.replace_range(utf8.clone(), new_text);
        let insert_end = utf8.start + new_text.len();
        self.marked_utf16 =
            Some(utf8_to_utf16(&self.text, utf8.start)..utf8_to_utf16(&self.text, insert_end));
        if let Some(sel) = new_selected {
            self.selected_utf16 = sel;
        } else {
            let u16 = utf8_to_utf16(&self.text, insert_end);
            self.selected_utf16 = u16..u16;
        }
        cx.emit(ComposerEvent::Changed);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range: Range<usize>,
        element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let utf8_start = utf16_to_utf8(&self.text, range.start);
        let utf8_end = utf16_to_utf8(&self.text, range.end);
        for (i, line) in self.lines.iter().enumerate() {
            let line_end = line.start_utf8 + line.shaped.len();
            if utf8_start >= line.start_utf8 && utf8_start <= line_end {
                let x0 = line.shaped.x_for_index(utf8_start - line.start_utf8);
                let x1 = line
                    .shaped
                    .x_for_index(utf8_end.min(line_end) - line.start_utf8);
                let origin = point(
                    element_bounds.origin.x + px(PAD_X) + x0.min(x1),
                    element_bounds.origin.y + px(PAD_Y) + px(i as f32 * LINE_HEIGHT)
                        - self.scroll_offset,
                );
                return Some(Bounds {
                    origin,
                    size: gpui::size((x1 - x0).abs().max(px(1.)), px(LINE_HEIGHT)),
                });
            }
        }
        Some(Bounds {
            origin: element_bounds.origin,
            size: gpui::size(px(1.), px(LINE_HEIGHT)),
        })
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let bounds = self.last_bounds?;
        let y = point.y - bounds.origin.y - px(PAD_Y) + self.scroll_offset;
        let i = ((f32::from(y) / LINE_HEIGHT).floor().max(0.)) as usize;
        let line = self.lines.get(i).or_else(|| self.lines.last())?;
        let x = point.x - bounds.origin.x - px(PAD_X);
        let local = line.shaped.closest_index_for_x(x.max(px(0.)));
        Some(utf8_to_utf16(&self.text, line.start_utf8 + local))
    }
}

// -- The paintable element ---------------------------------------------------

struct ComposerElement {
    view: gpui::Entity<Composer>,
    style: StyleRefinement,
}

impl Element for ComposerElement {
    type RequestLayoutState = ();
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _global_id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _global_id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.view.update(cx, |v, cx| {
            v.lines = v.shape_paragraphs(window, cx);
            v.last_bounds = Some(bounds);
        });
        Some(window.insert_hitbox(bounds, gpui::HitboxBehavior::Normal))
    }

    fn paint(
        &mut self,
        _global_id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus = self.view.read(cx).focus_handle();
        window.handle_input(
            &focus,
            ElementInputHandler::new(bounds, self.view.clone()),
            cx,
        );

        let t = theme(cx);
        let tertiary = t.text_tertiary;
        let selection_color = t.selection;
        let caret_color = t.caret;
        let selection = self.view.read(cx).selection_utf8();
        let marked = self.view.read(cx).marked_utf16.clone();
        let scroll = self.view.read(cx).scroll_offset;
        let focused = focus.is_focused(window);
        let blink_on = self.view.read(cx).blink_visible;
        let empty = self.view.read(cx).text.is_empty();
        let placeholder = self.view.read(cx).placeholder.clone();

        window.with_content_mask(Some(gpui::ContentMask { bounds }), |window| {
            if empty {
                let font = window.text_style().font();
                let len = placeholder.len();
                let shaped = window.text_system().shape_line(
                    placeholder,
                    px(FONT_SIZE),
                    &[TextRun {
                        len,
                        font,
                        color: tertiary.into(),
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    }],
                    None,
                );
                let _ = shaped.paint(
                    point(bounds.origin.x + px(PAD_X), bounds.origin.y + px(PAD_Y)),
                    px(LINE_HEIGHT),
                    window,
                    cx,
                );
                return;
            }
            // lines were computed in prepaint
            let line_count = self.view.read(cx).lines.len();
            for i in 0..line_count {
                let (start_utf8, len) = self
                    .view
                    .read(cx)
                    .lines
                    .get(i)
                    .map(|l| (l.start_utf8, l.shaped.len()))
                    .unwrap_or((0, 0));
                let origin = point(
                    bounds.origin.x + px(PAD_X),
                    bounds.origin.y + px(PAD_Y) + px(i as f32 * LINE_HEIGHT) - scroll,
                );
                // Selection highlight for this line.
                if selection.start < start_utf8 + len && selection.end > start_utf8 {
                    let local_start = selection.start.saturating_sub(start_utf8);
                    let local_end = (selection.end - start_utf8).min(len);
                    let line_ref = &self.view.read(cx).lines[i].shaped;
                    let x0 = line_ref.x_for_index(local_start);
                    let x1 = line_ref.x_for_index(local_end);
                    window.paint_quad(fill(
                        Bounds {
                            origin: point(origin.x + x0.min(x1), origin.y),
                            size: gpui::size((x1 - x0).abs().max(px(2.)), px(LINE_HEIGHT)),
                        },
                        selection_color,
                    ));
                }
                self.view.update(cx, |v, _cx| {
                    if let Some(line) = v.lines.get(i) {
                        let _ = line.shaped.paint(origin, px(LINE_HEIGHT), window, _cx);
                    }
                });
            }
            // Caret.
            if focused && marked.is_none() && blink_on {
                let (i, x) = self.view.read(cx).caret_position();
                let y = bounds.origin.y + px(PAD_Y) + px(i as f32 * LINE_HEIGHT) - scroll;
                window.paint_quad(fill(
                    Bounds {
                        origin: point(bounds.origin.x + px(PAD_X) + x, y + px(1.)),
                        size: gpui::size(px(1.5), px(LINE_HEIGHT - 4.)),
                    },
                    caret_color,
                ));
            }
        });
    }
}

impl Styled for ComposerElement {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for ComposerElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

// -- Render ------------------------------------------------------------------

impl Render for Composer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = theme(cx);
        let focused = self.focus_handle.is_focused(window);
        let line_count = self.text.split('\n').count().max(1);
        let height = px((line_count.min(MAX_LINES) as f32 * LINE_HEIGHT) + PAD_Y * 2.);
        let scrollable = line_count > MAX_LINES;

        div()
            .id(ElementId::Name(SharedString::from(format!(
                "composer-{}",
                self.placeholder
            ))))
            .key_context("Composer")
            .track_focus(&self.focus_handle)
            .w_full()
            .h(height)
            .relative()
            .rounded(px(crate::theme::R_MD))
            .bg(t.inset)
            .border_1()
            .border_color(if focused {
                t.focus_ring.into()
            } else {
                gpui::transparent_black()
            })
            .cursor_text()
            .on_action::<Enter>(cx.listener(|this, _, _, cx| this.submit(cx)))
            .on_action::<InsertNewline>(cx.listener(|this, _, _, cx| {
                if !this.single_line {
                    this.begin_op(OpKind::Insert);
                    this.replace_utf8(this.selection_utf8(), "\n", cx);
                } else {
                    this.submit(cx);
                }
            }))
            .on_action::<Backspace>(cx.listener(|this, _, _, cx| this.backspace(cx)))
            .on_action::<Delete>(cx.listener(|this, _, _, cx| this.delete_forward(cx)))
            .on_action::<MoveLeft>(
                cx.listener(|this, _, _, cx| this.move_horizontal(-1, false, false, cx)),
            )
            .on_action::<MoveRight>(
                cx.listener(|this, _, _, cx| this.move_horizontal(1, false, false, cx)),
            )
            .on_action::<MoveWordLeft>(
                cx.listener(|this, _, _, cx| this.move_horizontal(-1, true, false, cx)),
            )
            .on_action::<MoveWordRight>(
                cx.listener(|this, _, _, cx| this.move_horizontal(1, true, false, cx)),
            )
            .on_action::<MoveUp>(cx.listener(|this, _, _, cx| this.move_vertical(-1, false, cx)))
            .on_action::<MoveDown>(cx.listener(|this, _, _, cx| this.move_vertical(1, false, cx)))
            .on_action::<SelectLeft>(
                cx.listener(|this, _, _, cx| this.move_horizontal(-1, false, true, cx)),
            )
            .on_action::<SelectRight>(
                cx.listener(|this, _, _, cx| this.move_horizontal(1, false, true, cx)),
            )
            .on_action::<SelectWordLeft>(
                cx.listener(|this, _, _, cx| this.move_horizontal(-1, true, true, cx)),
            )
            .on_action::<SelectWordRight>(
                cx.listener(|this, _, _, cx| this.move_horizontal(1, true, true, cx)),
            )
            .on_action::<SelectUp>(cx.listener(|this, _, _, cx| this.move_vertical(-1, true, cx)))
            .on_action::<SelectDown>(cx.listener(|this, _, _, cx| this.move_vertical(1, true, cx)))
            .on_action::<SelectAll>(cx.listener(|this, _, _, cx| this.select_all(cx)))
            .on_action::<Home>(
                cx.listener(|this, _, _, cx| this.move_line_edge(true, false, false, cx)),
            )
            .on_action::<End>(
                cx.listener(|this, _, _, cx| this.move_line_edge(false, false, false, cx)),
            )
            .on_action::<DocStart>(
                cx.listener(|this, _, _, cx| this.move_line_edge(true, true, false, cx)),
            )
            .on_action::<DocEnd>(
                cx.listener(|this, _, _, cx| this.move_line_edge(false, true, false, cx)),
            )
            .on_action::<SelectToLineStart>(
                cx.listener(|this, _, _, cx| this.move_line_edge(true, false, true, cx)),
            )
            .on_action::<SelectToLineEnd>(
                cx.listener(|this, _, _, cx| this.move_line_edge(false, false, true, cx)),
            )
            .on_action::<SelectToDocStart>(
                cx.listener(|this, _, _, cx| this.move_line_edge(true, true, true, cx)),
            )
            .on_action::<SelectToDocEnd>(
                cx.listener(|this, _, _, cx| this.move_line_edge(false, true, true, cx)),
            )
            .on_action::<Undo>(cx.listener(|this, _, _, cx| this.undo(cx)))
            .on_action::<Redo>(cx.listener(|this, _, _, cx| this.redo(cx)))
            .on_action::<Cut>(cx.listener(|this, _, _, cx| this.copy_selection(true, cx)))
            .on_action::<Copy>(cx.listener(|this, _, _, cx| this.copy_selection(false, cx)))
            .on_action::<Paste>(cx.listener(|this, _, _, cx| this.paste(cx)))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                // Printable keys arrive through the IME path (insertText →
                // replace_text_in_range). While composition is active, swallow
                // everything else so Enter can never accidentally submit.
                if this.marked_utf16.is_some() {
                    cx.stop_propagation();
                }
                let _ = event;
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, window, cx| {
                    window.focus(&this.focus_handle);
                    this.dragging = true;
                    if let Some(u16) = this.character_index_for_point(event.position, window, cx) {
                        let byte = utf16_to_utf8(&this.text, u16);
                        let clicks = event.click_count;
                        if clicks >= 2 {
                            let (s, e) = this.word_or_line_range(byte, clicks >= 3);
                            this.selected_utf16 =
                                utf8_to_utf16(&this.text, s)..utf8_to_utf16(&this.text, e);
                            this.selection_reversed = false;
                        } else {
                            if event.modifiers.shift {
                                let anchor = if this.selection_reversed {
                                    this.selected_utf16.end
                                } else {
                                    this.selected_utf16.start
                                };
                                this.selected_utf16 = anchor.min(u16)..anchor.max(u16);
                                this.selection_reversed = u16 < anchor;
                            } else {
                                this.selected_utf16 = u16..u16;
                                this.selection_reversed = false;
                            }
                        }
                    }
                    this.end_ops();
                    cx.notify();
                }),
            )
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                // `pressed_button` tracks the real drag state — a release
                // outside the element can never leave `dragging` stale.
                if event.pressed_button != Some(MouseButton::Left) {
                    this.dragging = false;
                    return;
                }
                if this.dragging
                    && let Some(u16) = this.character_index_for_point(event.position, _window, cx)
                {
                    let anchor = if this.selection_reversed {
                        this.selected_utf16.end
                    } else {
                        this.selected_utf16.start
                    };
                    this.selected_utf16 = anchor.min(u16)..anchor.max(u16);
                    this.selection_reversed = u16 < anchor;
                    cx.notify();
                }
            }))
            .when(scrollable, |s| {
                s.on_scroll_wheel(cx.listener(|this, event: &ScrollWheelEvent, _w, cx| {
                    this.on_scroll(event, cx);
                }))
            })
            .child(ComposerElement {
                view: cx.entity(),
                style: StyleRefinement::default().size_full(),
            })
    }
}

impl Composer {
    fn word_or_line_range(&self, byte: usize, line: bool) -> (usize, usize) {
        if line {
            return self.paragraph_bounds(byte);
        }
        let is_word = |c: char| c.is_alphanumeric() || c == '_';
        let mut s = byte;
        while s > 0 && is_word(self.text[..s].chars().next_back().unwrap_or(' ')) {
            s = self.text[..s]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
        let mut e = byte;
        while e < self.text.len() && is_word(self.text[e..].chars().next().unwrap_or(' ')) {
            e += self.text[e..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
        }
        (s, e)
    }
}

actions!(
    composer,
    [
        Backspace,
        Copy,
        Cut,
        Delete,
        DocEnd,
        DocStart,
        End,
        Enter,
        Home,
        InsertNewline,
        MoveDown,
        MoveLeft,
        MoveRight,
        MoveUp,
        MoveWordLeft,
        MoveWordRight,
        Paste,
        Redo,
        SelectAll,
        SelectDown,
        SelectLeft,
        SelectRight,
        SelectToDocEnd,
        SelectToDocStart,
        SelectToLineEnd,
        SelectToLineStart,
        SelectUp,
        SelectWordLeft,
        SelectWordRight,
        Undo,
    ]
);
