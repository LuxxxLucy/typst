use kurbo::{BezPath, Line, ParamCurve};
use ttf_parser::{GlyphId, OutlineBuilder};

use super::{BottomEdge, BottomEdgeMetric, TextElem, TopEdge, TopEdgeMetric};
use crate::prelude::*;

/// Underlines text.
///
/// ## Example { #example }
/// ```example
/// This is #underline[important].
/// ```
///
/// Display: Underline
/// Category: text
#[element(Show)]
pub struct UnderlineElem {
    /// How to stroke the line.
    ///
    /// See the [line's documentation]($func/line.stroke) for more details. If
    /// set to `{auto}`, takes on the text's color and a thickness defined in
    /// the current font.
    ///
    /// ```example
    /// Take #underline(
    ///   stroke: 1.5pt + red,
    ///   offset: 2pt,
    ///   [care],
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Smart<PartialStroke>,

    /// The position of the line relative to the baseline, read from the font
    /// tables if `{auto}`.
    ///
    /// ```example
    /// #underline(offset: 5pt)[
    ///   The Tale Of A Faraway Line I
    /// ]
    /// ```
    #[resolve]
    pub offset: Smart<Length>,

    /// The amount by which to extend the line beyond (or within if negative)
    /// the content.
    ///
    /// ```example
    /// #align(center,
    ///   underline(extent: 2pt)[Chapter 1]
    /// )
    /// ```
    #[resolve]
    pub extent: Length,

    /// Whether the line skips sections in which it would collide with the
    /// glyphs.
    ///
    /// ```example
    /// This #underline(evade: true)[is great].
    /// This #underline(evade: false)[is less great].
    /// ```
    #[default(true)]
    pub evade: bool,

    /// The content to underline.
    #[required]
    pub body: Content,
}

impl Show for UnderlineElem {
    #[tracing::instrument(name = "UnderlineElem::show", skip_all)]
    fn show(&self, _: &mut Vt, styles: StyleChain) -> SourceResult<Content> {
        let (stroke, offset, evade) = (
            self.stroke(styles).unwrap_or_default(),
            self.offset(styles),
            self.evade(styles),
        );
        Ok(self.body().styled(TextElem::set_deco(Decoration {
            line: DecoLine::Underline(stroke, offset, evade),
            extent: self.extent(styles),
        })))
    }
}

/// Adds a line over text.
///
/// ## Example { #example }
/// ```example
/// #overline[A line over text.]
/// ```
///
/// Display: Overline
/// Category: text
#[element(Show)]
pub struct OverlineElem {
    /// How to stroke the line.
    ///
    /// See the [line's documentation]($func/line.stroke) for more details. If
    /// set to `{auto}`, takes on the text's color and a thickness defined in
    /// the current font.
    ///
    /// ```example
    /// #set text(fill: olive)
    /// #overline(
    ///   stroke: green.darken(20%),
    ///   offset: -12pt,
    ///   [The Forest Theme],
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Smart<PartialStroke>,

    /// The position of the line relative to the baseline. Read from the font
    /// tables if `{auto}`.
    ///
    /// ```example
    /// #overline(offset: -1.2em)[
    ///   The Tale Of A Faraway Line II
    /// ]
    /// ```
    #[resolve]
    pub offset: Smart<Length>,

    /// The amount by which to extend the line beyond (or within if negative)
    /// the content.
    ///
    /// ```example
    /// #set overline(extent: 4pt)
    /// #set underline(extent: 4pt)
    /// #overline(underline[Typography Today])
    /// ```
    #[resolve]
    pub extent: Length,

    /// Whether the line skips sections in which it would collide with the
    /// glyphs.
    ///
    /// ```example
    /// #overline(
    ///   evade: false,
    ///   offset: -7.5pt,
    ///   stroke: 1pt,
    ///   extent: 3pt,
    ///   [Temple],
    /// )
    /// ```
    #[default(true)]
    pub evade: bool,

    /// The content to add a line over.
    #[required]
    pub body: Content,
}

impl Show for OverlineElem {
    #[tracing::instrument(name = "OverlineElem::show", skip_all)]
    fn show(&self, _: &mut Vt, styles: StyleChain) -> SourceResult<Content> {
        let (stroke, offset, evade) = (
            self.stroke(styles).unwrap_or_default(),
            self.offset(styles),
            self.evade(styles),
        );
        Ok(self.body().styled(TextElem::set_deco(Decoration {
            line: DecoLine::Overline(stroke, offset, evade),
            extent: self.extent(styles),
        })))
    }
}

/// Strikes through text.
///
/// ## Example { #example }
/// ```example
/// This is #strike[not] relevant.
/// ```
///
/// Display: Strikethrough
/// Category: text
#[element(Show)]
pub struct StrikeElem {
    /// How to stroke the line.
    ///
    /// See the [line's documentation]($func/line.stroke) for more details. If
    /// set to `{auto}`, takes on the text's color and a thickness defined in
    /// the current font.
    ///
    /// _Note:_ Please don't use this for real redaction as you can still
    /// copy paste the text.
    ///
    /// ```example
    /// This is #strike(stroke: 1.5pt + red)[very stricken through]. \
    /// This is #strike(stroke: 10pt)[redacted].
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Smart<PartialStroke>,

    /// The position of the line relative to the baseline. Read from the font
    /// tables if `{auto}`.
    ///
    /// This is useful if you are unhappy with the offset your font provides.
    ///
    /// ```example
    /// #set text(font: "Inria Serif")
    /// This is #strike(offset: auto)[low-ish]. \
    /// This is #strike(offset: -3.5pt)[on-top].
    /// ```
    #[resolve]
    pub offset: Smart<Length>,

    /// The amount by which to extend the line beyond (or within if negative)
    /// the content.
    ///
    /// ```example
    /// This #strike(extent: -2pt)[skips] parts of the word.
    /// This #strike(extent: 2pt)[extends] beyond the word.
    /// ```
    #[resolve]
    pub extent: Length,

    /// The content to strike through.
    #[required]
    pub body: Content,
}

impl Show for StrikeElem {
    #[tracing::instrument(name = "StrikeElem::show", skip_all)]
    fn show(&self, _: &mut Vt, styles: StyleChain) -> SourceResult<Content> {
        let (stroke, offset) =
            (self.stroke(styles).unwrap_or_default(), self.offset(styles));
        Ok(self.body().styled(TextElem::set_deco(Decoration {
            // Note that we do not support evade option for strikethrough.
            line: DecoLine::Strikethrough(stroke, offset),
            extent: self.extent(styles),
        })))
    }
}

/// Highlight text with a background color.
///
/// ## Example { #example }
/// ```example
/// This is #highlight[important].
/// ```
///
/// Display: Highlight
/// Category: text
#[element(Show)]
pub struct HighlightElem {
    /// The color to highlight the text with.
    /// (Default: 0xffff5f)
    ///
    /// ```example
    /// This is #highlight(fill: blue)[with blue].
    /// ```
    #[default(Color::Rgba(RgbaColor::new(0xFF, 0xFF, 0x5F, 0xFF)).into())]
    pub fill: Paint,

    /// The top end of the background rectangle. Note that top edge will update
    /// to be always higher than the glyph's bounding box.
    /// (default: "ascender")
    ///
    /// ```example
    /// #let highlight-default = highlight.with(top-edge: "ascender")
    /// #let highlight-tight = highlight.with(top-edge: "x-height")
    ///
    /// #highlight-default[a], #highlight-default[ai], #highlight-default[aib]
    ///
    /// #highlight-tight[a], #highlight-tight[ai], #highlight-tight[aib]
    /// ```
    #[default(TopEdge::Metric(TopEdgeMetric::Ascender))]
    pub top_edge: TopEdge,

    /// The bottom end of the background rectangle. Note that top edge will update
    /// to be always lower than the glyph's bounding box.
    /// (default: "descender")
    ///
    /// ```example
    /// #let highlight-default = highlight.with(bottom-edge: "descender")
    /// #let highlight-tight = highlight.with(bottom-edge: "baseline")
    ///
    /// #highlight-default[a], #highlight-default[ah]
    ///
    /// #highlight-tight[a], #highlight-tight[ah]
    /// ```
    #[default(BottomEdge::Metric(BottomEdgeMetric::Descender))]
    pub bottom_edge: BottomEdge,

    /// The amount by which to extend the background to the sides beyond
    /// (or within if negative) the content.
    ///
    /// ```example
    /// A long #highlight(extent: 4pt)[background]. \
    /// ```
    #[resolve]
    pub extent: Length,

    /// The content that should be highlighted.
    #[required]
    pub body: Content,
}

impl Show for HighlightElem {
    #[tracing::instrument(name = "HighlightElem::show", skip_all)]
    fn show(&self, _: &mut Vt, styles: StyleChain) -> SourceResult<Content> {
        Ok(self.body().styled(TextElem::set_deco(Decoration {
            line: DecoLine::Highlight(
                self.fill(styles),
                self.top_edge(styles),
                self.bottom_edge(styles),
            ),
            extent: self.extent(styles),
        })))
    }
}

/// Defines a line-based decoration that is positioned over, under or on top of text,
/// or highlights the text with a background.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Decoration {
    pub line: DecoLine,
    pub extent: Abs,
}

impl Fold for Decoration {
    type Output = Vec<Self>;

    fn fold(self, mut outer: Self::Output) -> Self::Output {
        outer.insert(0, self);
        outer
    }
}

cast! {
    type Decoration: "decoration",
}

/// A kind of decorative line.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DecoLine {
    Underline(PartialStroke<Abs>, Smart<Abs>, bool),
    Strikethrough(PartialStroke<Abs>, Smart<Abs>),
    Overline(PartialStroke<Abs>, Smart<Abs>, bool),
    Highlight(Paint, TopEdge, BottomEdge),
}

/// Add line decorations to a single run of shaped text.
pub(super) fn decorate(
    frame: &mut Frame,
    styles: StyleChain,
    deco: &Decoration,
    text: &TextItem,
    shift: Abs,
    pos: Point,
) {
    let font_metrics = text.font.metrics();
    let width = text.width();

    if let DecoLine::Highlight(fill, top_edge, bottom_edge) = &deco.line {
        // return the top/bottom edge of the text given the metric of the font
        fn get_top_bottom(
            text: &TextItem,
            top_edge: TopEdge,
            bottom_edge: BottomEdge,
            styles: StyleChain,
        ) -> (Abs, Abs) {
            let mut top = top_edge.resolve(styles, &text.font, None);
            let mut bottom = bottom_edge.resolve(styles, &text.font, None);
            // loop over the glyphs to extract the max/min for top and bottom edge
            for g in text.glyphs.iter() {
                if let Some(bb) =
                    text.font.ttf().glyph_bounding_box(ttf_parser::GlyphId(g.id))
                {
                    top = top.max(text.font.to_em(bb.y_max).resolve(styles));
                    bottom = bottom.min(text.font.to_em(bb.y_min).resolve(styles));
                }
            }
            (top, bottom)
        }
        let (top, bottom) = get_top_bottom(text, *top_edge, *bottom_edge, styles);
        let bg = Geometry::Rect(Size::new(width + 2.0 * deco.extent, top - bottom))
            .filled(fill.clone());
        let offset = (-top) - shift;
        let origin = Point::new(pos.x - deco.extent, pos.y + offset);
        frame.prepend(origin, FrameItem::Shape(bg, Span::detached()));
        return;
    }

    let (stroke, metrics, offset, evade) = match &deco.line {
        DecoLine::Strikethrough(s, o) => (s, font_metrics.strikethrough, o, false),
        DecoLine::Overline(s, o, e) => (s, font_metrics.overline, o, *e),
        DecoLine::Underline(s, o, e) => (s, font_metrics.underline, o, *e),
        _ => return,
    };

    let offset = offset.unwrap_or(-metrics.position.resolve(styles)) - shift;
    let stroke = stroke.clone().unwrap_or(Stroke {
        paint: text.fill.clone(),
        thickness: metrics.thickness.resolve(styles),
        ..Stroke::default()
    });

    let gap_padding = 0.08 * text.size;
    let min_width = 0.162 * text.size;

    let start = pos.x - deco.extent;
    let end = pos.x + (width + 2.0 * deco.extent);

    let mut push_segment = |from: Abs, to: Abs| {
        let origin = Point::new(from, pos.y + offset);
        let target = Point::new(to - from, Abs::zero());

        if target.x >= min_width || !evade {
            let shape = Geometry::Line(target).stroked(stroke.clone());
            frame.push(origin, FrameItem::Shape(shape, Span::detached()));
        }
    };

    if !evade {
        push_segment(start, end);
        return;
    }

    let line = Line::new(
        kurbo::Point::new(pos.x.to_raw(), offset.to_raw()),
        kurbo::Point::new((pos.x + width).to_raw(), offset.to_raw()),
    );

    let mut x = pos.x;
    let mut intersections = vec![];

    for glyph in text.glyphs.iter() {
        let dx = glyph.x_offset.resolve(styles) + x;
        let mut builder =
            BezPathBuilder::new(font_metrics.units_per_em, text.size, dx.to_raw());

        let bbox = text.font.ttf().outline_glyph(GlyphId(glyph.id), &mut builder);
        let path = builder.finish();

        x += glyph.x_advance.resolve(styles);

        // Only do the costly segments intersection test if the line
        // intersects the bounding box.
        let intersect = bbox.map_or(false, |bbox| {
            let y_min = -text.font.to_em(bbox.y_max).resolve(styles);
            let y_max = -text.font.to_em(bbox.y_min).resolve(styles);
            offset >= y_min && offset <= y_max
        });

        if intersect {
            // Find all intersections of segments with the line.
            intersections.extend(
                path.segments()
                    .flat_map(|seg| seg.intersect_line(line))
                    .map(|is| Abs::raw(line.eval(is.line_t).x)),
            );
        }
    }

    // Add start and end points, taking padding into account.
    intersections.push(start - gap_padding);
    intersections.push(end + gap_padding);
    // When emitting the decorative line segments, we move from left to
    // right. The intersections are not necessarily in this order, yet.
    intersections.sort();

    for edge in intersections.windows(2) {
        let l = edge[0];
        let r = edge[1];

        // If we are too close, don't draw the segment
        if r - l < gap_padding {
            continue;
        } else {
            push_segment(l + gap_padding, r - gap_padding);
        }
    }
}

/// Builds a kurbo [`BezPath`] for a glyph.
struct BezPathBuilder {
    path: BezPath,
    units_per_em: f64,
    font_size: Abs,
    x_offset: f64,
}

impl BezPathBuilder {
    fn new(units_per_em: f64, font_size: Abs, x_offset: f64) -> Self {
        Self {
            path: BezPath::new(),
            units_per_em,
            font_size,
            x_offset,
        }
    }

    fn finish(self) -> BezPath {
        self.path
    }

    fn p(&self, x: f32, y: f32) -> kurbo::Point {
        kurbo::Point::new(self.s(x) + self.x_offset, -self.s(y))
    }

    fn s(&self, v: f32) -> f64 {
        Em::from_units(v, self.units_per_em).at(self.font_size).to_raw()
    }
}

impl OutlineBuilder for BezPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.move_to(self.p(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(self.p(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path.quad_to(self.p(x1, y1), self.p(x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path.curve_to(self.p(x1, y1), self.p(x2, y2), self.p(x, y));
    }

    fn close(&mut self) {
        self.path.close_path();
    }
}
