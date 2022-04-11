/// Dom events
#[derive(Debug, Copy, Clone)]
pub enum Event {
    Abort,
    AfterPrint,
    AnimationEnd,
    AnimationIteration,
    AnimationStart,
    BeforePrint,
    BeforeUnload,
    Blur,
    CanPlay,
    CanPlayThrough,
    Change,
    Click,
    ContextMenu,
    Copy,
    Cut,
    DoubleClick,
    Drag,
    DragEnd,
    DragEnter,
    DragLeave,
    DragOver,
    DragStart,
    Drop,
    DurationChange,
    Ended,
    Error,
    Focus,
    FocusIn,
    FocusOut,
    FullscreenChange,
    FullscreenError,
    HashChange,
    Input,
    Invalid,
    KeyDown,
    KeyPress,
    KeyUp,
    Load,
    LoadedData,
    LoadedMetadata,
    LoadStart,
    Message,
    MouseDown,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseOver,
    MouseOut,
    MouseUp,
    MouseWheel,
    Offline,
    Online,
    Open,
    PageHide,
    PageShow,
    Paste,
    Pause,
    Play,
    Playing,
    PopState,
    Progress,
    RateChange,
    Resize,
    Reset,
    Scroll,
    Search,
    Seeked,
    Seeking,
    Select,
    Show,
    Stalled,
    Storage,
    Submit,
    Suspend,
    TimeUpdate,
    Toggle,
    TouchCancel,
    TouchEnd,
    TouchMove,
    TouchStart,
    Transitionend,
    Unload,
    VolumeChange,
    Waiting,
    Wheel,
}

impl Event {
    pub fn to_str(&self) -> &'static str {
        match self {
            Event::Abort => "abort",
            Event::AfterPrint => "afterprint",
            Event::AnimationEnd => "animationend",
            Event::AnimationIteration => "animationiteration",
            Event::AnimationStart => "animationstart",
            Event::BeforePrint => "beforeprint",
            Event::BeforeUnload => "beforeunload",
            Event::Blur => "blur",
            Event::CanPlay => "canplay",
            Event::CanPlayThrough => "canplaythrough",
            Event::Change => "change",
            Event::Click => "click",
            Event::ContextMenu => "contextmenu",
            Event::Copy => "copy",
            Event::Cut => "cut",
            Event::DoubleClick => "dblclick",
            Event::Drag => "drag",
            Event::DragEnd => "dragend",
            Event::DragEnter => "dragenter",
            Event::DragLeave => "dragleave",
            Event::DragOver => "dragover",
            Event::DragStart => "dragstart",
            Event::Drop => "drop",
            Event::DurationChange => "durationchange",
            Event::Ended => "ended",
            Event::Error => "error",
            Event::Focus => "focus",
            Event::FocusIn => "focusin",
            Event::FocusOut => "focusout",
            Event::FullscreenChange => "fullscreenchange",
            Event::FullscreenError => "fullscreenerror",
            Event::HashChange => "hashchange",
            Event::Input => "input",
            Event::Invalid => "invalid",
            Event::KeyDown => "keydown",
            Event::KeyPress => "keypress",
            Event::KeyUp => "keyup",
            Event::Load => "load",
            Event::LoadedData => "loadeddata",
            Event::LoadedMetadata => "loadedmetadata",
            Event::LoadStart => "loadstart",
            Event::Message => "message",
            Event::MouseDown => "mousedown",
            Event::MouseEnter => "mouseenter",
            Event::MouseLeave => "mouseleave",
            Event::MouseMove => "mousemove",
            Event::MouseOver => "mouseover",
            Event::MouseOut => "mouseout",
            Event::MouseUp => "mouseup",
            Event::MouseWheel => "mousewheel",
            Event::Offline => "offline",
            Event::Online => "online",
            Event::Open => "open",
            Event::PageHide => "pagehide",
            Event::PageShow => "pageshow",
            Event::Paste => "paste",
            Event::Pause => "pause",
            Event::Play => "play",
            Event::Playing => "playing",
            Event::PopState => "popstate",
            Event::Progress => "progress",
            Event::RateChange => "ratechange",
            Event::Resize => "resize",
            Event::Reset => "reset",
            Event::Scroll => "scroll",
            Event::Search => "search",
            Event::Seeked => "seeked",
            Event::Seeking => "seeking",
            Event::Select => "select",
            Event::Show => "show",
            Event::Stalled => "stalled",
            Event::Storage => "storage",
            Event::Submit => "submit",
            Event::Suspend => "suspend",
            Event::TimeUpdate => "timeupdate",
            Event::Toggle => "toggle",
            Event::TouchCancel => "touchcancel",
            Event::TouchEnd => "touchend",
            Event::TouchMove => "touchmove",
            Event::TouchStart => "touchstart",
            Event::Transitionend => "transitionend",
            Event::Unload => "unload",
            Event::VolumeChange => "volumechange",
            Event::Waiting => "waiting",
            Event::Wheel => "wheel",
        }
    }    
}

/// Dom styles
#[derive(Debug, Copy, Clone)]
pub enum Style {
    AlignContent,
    AlignItems,
    AlignSelf,
    Animation,
    AnimationDelay,
    AnimationDirection,
    AnimationDuration,
    AnimationFillMode,
    AnimationIterationCount,
    AnimationName,
    AnimationTimingFunction,
    AnimationPlayState,
    Background,
    BackgroundAttachment,
    BackgroundColor,
    BackgroundImage,
    BackgroundPosition,
    BackgroundRepeat,
    BackgroundClip,
    BackgroundOrigin,
    BackgroundSize,
    BackfaceVisibility,
    Border,
    BorderBottom,
    BorderBottomColor,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    BorderBottomStyle,
    BorderBottomWidth,
    BorderCollapse,
    BorderColor,
    BorderImage,
    BorderImageOutset,
    BorderImageRepeat,
    BorderImageSlice,
    BorderImageSource,
    BorderImageWidth,
    BorderLeft,
    BorderLeftColor,
    BorderLeftStyle,
    BorderLeftWidth,
    BorderRadius,
    BorderRight,
    BorderRightColor,
    BorderRightStyle,
    BorderRightWidth,
    BorderSpacing,
    BorderStyle,
    BorderTop,
    BorderTopColor,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderTopStyle,
    BorderTopWidth,
    BorderWidth,
    Bottom,
    BoxDecorationBreak,
    BoxShadow,
    BoxSizing,
    CaptionSide,
    CaretColor,
    Clear,
    Clip,
    Color,
    ColumnCount,
    ColumnFill,
    ColumnGap,
    ColumnRule,
    ColumnRuleColor,
    ColumnRuleStyle,
    ColumnRuleWidth,
    Columns,
    ColumnSpan,
    ColumnWidth,
    Content,
    CounterIncrement,
    CounterReset,
    Cursor,
    Direction,
    Display,
    EmptyCells,
    Filter,
    Flex,
    FlexBasis,
    FlexDirection,
    FlexFlow,
    FlexGrow,
    FlexShrink,
    FlexWrap,
    CssFloat,
    Font,
    FontFamily,
    FontSize,
    FontStyle,
    FontVariant,
    FontWeight,
    FontSizeAdjust,
    FontStretch,
    HangingPunctuation,
    Height,
    Hyphens,
    Icon,
    ImageOrientation,
    Isolation,
    JustifyContent,
    Left,
    LetterSpacing,
    LineHeight,
    ListStyle,
    ListStyleImage,
    ListStylePosition,
    ListStyleType,
    Margin,
    MarginBottom,
    MarginLeft,
    MarginRight,
    MarginTop,
    MaxHeight,
    MaxWidth,
    MinHeight,
    MinWidth,
    NavDown,
    NavIndex,
    NavLeft,
    NavRight,
    NavUp,
    ObjectFit,
    ObjectPosition,
    Opacity,
    Order,
    Orphans,
    Outline,
    OutlineColor,
    OutlineOffset,
    OutlineStyle,
    OutlineWidth,
    Overflow,
    OverflowX,
    OverflowY,
    Padding,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    PaddingTop,
    PageBreakAfter,
    PageBreakBefore,
    PageBreakInside,
    Perspective,
    PerspectiveOrigin,
    Position,
    Quotes,
    Resize,
    Right,
    ScrollBehavior,
    TableLayout,
    TabSize,
    TextAlign,
    TextAlignLast,
    TextDecoration,
    TextDecorationColor,
    TextDecorationLine,
    TextDecorationStyle,
    TextIndent,
    TextJustify,
    TextOverflow,
    TextShadow,
    TextTransform,
    Top,
    Transform,
    TransformOrigin,
    TransformStyle,
    Transition,
    TransitionProperty,
    TransitionDuration,
    TransitionTimingFunction,
    TransitionDelay,
    UnicodeBidi,
    UserSelect,
    VerticalAlign,
    Visibility,
    WhiteSpace,
    Width,
    WordBreak,
    WordSpacing,
    WordWrap,
    Widows,
    ZIndex,
}

impl Style {
    pub fn to_str(&self) -> &'static str {
        match self {
            Style::AlignContent => "align-content",
            Style::AlignItems => "align-items",
            Style::AlignSelf => "align-self",
            Style::Animation => "animation",
            Style::AnimationDelay => "animation-delay",
            Style::AnimationDirection => "animation-direction",
            Style::AnimationDuration => "animation-duration",
            Style::AnimationFillMode => "animation-fill-mode",
            Style::AnimationIterationCount => "animation-iteration-count",
            Style::AnimationName => "animation-name",
            Style::AnimationTimingFunction => "animation-timing-function",
            Style::AnimationPlayState => "animation-play-state",
            Style::Background => "background",
            Style::BackgroundAttachment => "background-attachment",
            Style::BackgroundColor => "background-color",
            Style::BackgroundImage => "background-image",
            Style::BackgroundPosition => "background-position",
            Style::BackgroundRepeat => "background-repeat",
            Style::BackgroundClip => "background-clip",
            Style::BackgroundOrigin => "background-origin",
            Style::BackgroundSize => "background-size",
            Style::BackfaceVisibility => "backface-visibility",
            Style::Border => "border",
            Style::BorderBottom => "border-bottom",
            Style::BorderBottomColor => "border-bottom-color",
            Style::BorderBottomLeftRadius => "border-bottom-left-radius",
            Style::BorderBottomRightRadius => "border-bottom-right-radius",
            Style::BorderBottomStyle => "border-bottom-style",
            Style::BorderBottomWidth => "border-bottom-width",
            Style::BorderCollapse => "border-collapse",
            Style::BorderColor => "border-color",
            Style::BorderImage => "border-image",
            Style::BorderImageOutset => "border-image-outset",
            Style::BorderImageRepeat => "border-image-repeat",
            Style::BorderImageSlice => "border-image-slice",
            Style::BorderImageSource => "border-image-source",
            Style::BorderImageWidth => "border-image-width",
            Style::BorderLeft => "border-left",
            Style::BorderLeftColor => "border-left-color",
            Style::BorderLeftStyle => "border-left-style",
            Style::BorderLeftWidth => "border-left-width",
            Style::BorderRadius => "border-radius",
            Style::BorderRight => "border-right",
            Style::BorderRightColor => "border-right-color",
            Style::BorderRightStyle => "border-right-style",
            Style::BorderRightWidth => "border-right-width",
            Style::BorderSpacing => "border-spacing",
            Style::BorderStyle => "border-style",
            Style::BorderTop => "border-top",
            Style::BorderTopColor => "border-top-color",
            Style::BorderTopLeftRadius => "border-top-left-radius",
            Style::BorderTopRightRadius => "border-top-right-radius",
            Style::BorderTopStyle => "border-top-style",
            Style::BorderTopWidth => "border-top-width",
            Style::BorderWidth => "border-width",
            Style::Bottom => "bottom",
            Style::BoxDecorationBreak => "box-decoration-break",
            Style::BoxShadow => "box-shadow",
            Style::BoxSizing => "box-sizing",
            Style::CaptionSide => "caption-side",
            Style::CaretColor => "caret-color",
            Style::Clear => "clear",
            Style::Clip => "clip",
            Style::Color => "color",
            Style::ColumnCount => "column-count",
            Style::ColumnFill => "column-fill",
            Style::ColumnGap => "column-gap",
            Style::ColumnRule => "column-rule",
            Style::ColumnRuleColor => "column-rule-color",
            Style::ColumnRuleStyle => "column-rule-style",
            Style::ColumnRuleWidth => "column-rule-width",
            Style::Columns => "columns",
            Style::ColumnSpan => "column-span",
            Style::ColumnWidth => "column-width",
            Style::Content => "content",
            Style::CounterIncrement => "counter-increment",
            Style::CounterReset => "counter-reset",
            Style::Cursor => "cursor",
            Style::Direction => "direction",
            Style::Display => "display",
            Style::EmptyCells => "empty-cells",
            Style::Filter => "filter",
            Style::Flex => "flex",
            Style::FlexBasis => "flex-basis",
            Style::FlexDirection => "flex-direction",
            Style::FlexFlow => "flex-flow",
            Style::FlexGrow => "flex-grow",
            Style::FlexShrink => "flex-shrink",
            Style::FlexWrap => "flex-wrap",
            Style::CssFloat => "css-float",
            Style::Font => "font",
            Style::FontFamily => "font-family",
            Style::FontSize => "font-size",
            Style::FontStyle => "font-style",
            Style::FontVariant => "font-variant",
            Style::FontWeight => "font-weight",
            Style::FontSizeAdjust => "font-size-adjust",
            Style::FontStretch => "font-stretch",
            Style::HangingPunctuation => "hanging-punctuation",
            Style::Height => "height",
            Style::Hyphens => "hyphens",
            Style::Icon => "icon",
            Style::ImageOrientation => "image-orientation",
            Style::Isolation => "isolation",
            Style::JustifyContent => "justify-content",
            Style::Left => "left",
            Style::LetterSpacing => "letter-spacing",
            Style::LineHeight => "line-height",
            Style::ListStyle => "list-style",
            Style::ListStyleImage => "list-style-image",
            Style::ListStylePosition => "list-style-position",
            Style::ListStyleType => "list-style-type",
            Style::Margin => "margin",
            Style::MarginBottom => "margin-bottom",
            Style::MarginLeft => "margin-left",
            Style::MarginRight => "margin-right",
            Style::MarginTop => "margin-top",
            Style::MaxHeight => "max-height",
            Style::MaxWidth => "max-width",
            Style::MinHeight => "min-height",
            Style::MinWidth => "min-width",
            Style::NavDown => "nav-down",
            Style::NavIndex => "nav-index",
            Style::NavLeft => "nav-left",
            Style::NavRight => "nav-right",
            Style::NavUp => "nav-up",
            Style::ObjectFit => "object-fit",
            Style::ObjectPosition => "object-position",
            Style::Opacity => "opacity",
            Style::Order => "order",
            Style::Orphans => "orphans",
            Style::Outline => "outline",
            Style::OutlineColor => "outline-color",
            Style::OutlineOffset => "outline-offset",
            Style::OutlineStyle => "outline-style",
            Style::OutlineWidth => "outline-width",
            Style::Overflow => "overflow",
            Style::OverflowX => "overflow-x",
            Style::OverflowY => "overflow-y",
            Style::Padding => "padding",
            Style::PaddingBottom => "padding-bottom",
            Style::PaddingLeft => "padding-left",
            Style::PaddingRight => "padding-right",
            Style::PaddingTop => "padding-top",
            Style::PageBreakAfter => "page-break-after",
            Style::PageBreakBefore => "page-break-before",
            Style::PageBreakInside => "page-break-inside",
            Style::Perspective => "perspective",
            Style::PerspectiveOrigin => "perspective-origin",
            Style::Position => "position",
            Style::Quotes => "quotes",
            Style::Resize => "resize",
            Style::Right => "right",
            Style::ScrollBehavior => "scroll-behavior",
            Style::TableLayout => "table-layout",
            Style::TabSize => "tab-size",
            Style::TextAlign => "text-align",
            Style::TextAlignLast => "text-align-last",
            Style::TextDecoration => "text-decoration",
            Style::TextDecorationColor => "text-decoration-color",
            Style::TextDecorationLine => "text-decoration-line",
            Style::TextDecorationStyle => "text-decoration-style",
            Style::TextIndent => "text-indent",
            Style::TextJustify => "text-justify",
            Style::TextOverflow => "text-overflow",
            Style::TextShadow => "text-shadow",
            Style::TextTransform => "text-transform",
            Style::Top => "top",
            Style::Transform => "transform",
            Style::TransformOrigin => "transform-origin",
            Style::TransformStyle => "transform-style",
            Style::Transition => "transition",
            Style::TransitionProperty => "transition-property",
            Style::TransitionDuration => "transition-duration",
            Style::TransitionTimingFunction => "transition-timing-function",
            Style::TransitionDelay => "transition-delay",
            Style::UnicodeBidi => "unicode-bidi",
            Style::UserSelect => "user-select",
            Style::VerticalAlign => "vertical-align",
            Style::Visibility => "visibility",
            Style::WhiteSpace => "white-space",
            Style::Width => "width",
            Style::WordBreak => "word-break",
            Style::WordSpacing => "word-spacing",
            Style::WordWrap => "word-wrap",
            Style::Widows => "widows",
            Style::ZIndex => "zIndex",
        }
    }    
}

/// Element types
#[derive(Debug, Copy, Clone)]
pub enum WidgetType {
    Address,
    Article,
    Aside,
    Footer,
    Header,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Main,
    Nav,
    Section,
    Blockquote,
    Dd,
    Div,
    Dl,
    Dt,
    Figcaption,
    Figure,
    Hr,
    Li,
    Ol,
    P,
    Pre,
    Ul,
    A,
    Abbr,
    B,
    Bdi,
    Bdo,
    Br,
    Cite,
    Code,
    Data,
    Dfn,
    Em,
    I,
    Kbd,
    Mark,
    Q,
    Rp,
    Rt,
    Ruby,
    S,
    Samp,
    Small,
    Span,
    Strong,
    Sub,
    Sup,
    Time,
    U,
    Var,
    Wbr,
    Area,
    Audio,
    Img,
    Map,
    Track,
    Video,
    Embed,
    Iframe,
    Object,
    Param,
    Picture,
    Portal,
    Source,
    Svg,
    Math,
    Canvas,
    Noscript,
    Script,
    Del,
    Ins,
    Caption,
    Col,
    Colgroup,
    Table,
    Tbody,
    Td,
    Tfoot,
    Th,
    Thead,
    Tr,
    Button,
    Datalist,
    Fieldset,
    Form,
    Input,
    Label,
    Legend,
    Meter,
    Optgroup,
    Option,
    Output,
    Progress,
    Select,
    Textarea,
    Details,
    Dialog,
    Menu,
    Summary,
    Slot,
    Template,
}

impl WidgetType {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            WidgetType::Address => "address",
            WidgetType::Article => "article",
            WidgetType::Aside => "aside",
            WidgetType::Footer => "footer",
            WidgetType::Header => "header",
            WidgetType::H1 => "h1",
            WidgetType::H2 => "h2",
            WidgetType::H3 => "h3",
            WidgetType::H4 => "h4",
            WidgetType::H5 => "h5",
            WidgetType::H6 => "h6",
            WidgetType::Main => "main",
            WidgetType::Nav => "nav",
            WidgetType::Section => "section",
            WidgetType::Blockquote => "blockquote",
            WidgetType::Dd => "dd",
            WidgetType::Div => "div",
            WidgetType::Dl => "dl",
            WidgetType::Dt => "dt",
            WidgetType::Figcaption => "figcaption",
            WidgetType::Figure => "figure",
            WidgetType::Hr => "hr",
            WidgetType::Li => "li",
            WidgetType::Ol => "ol",
            WidgetType::P => "p",
            WidgetType::Pre => "pre",
            WidgetType::Ul => "ul",
            WidgetType::A => "a",
            WidgetType::Abbr => "abbr",
            WidgetType::B => "b",
            WidgetType::Bdi => "bdi",
            WidgetType::Bdo => "bdo",
            WidgetType::Br => "br",
            WidgetType::Cite => "cite",
            WidgetType::Code => "code",
            WidgetType::Data => "data",
            WidgetType::Dfn => "dfn",
            WidgetType::Em => "em",
            WidgetType::I => "i",
            WidgetType::Kbd => "kbd",
            WidgetType::Mark => "mark",
            WidgetType::Q => "q",
            WidgetType::Rp => "rp",
            WidgetType::Rt => "rt",
            WidgetType::Ruby => "ruby",
            WidgetType::S => "s",
            WidgetType::Samp => "samp",
            WidgetType::Small => "small",
            WidgetType::Span => "span",
            WidgetType::Strong => "strong",
            WidgetType::Sub => "sub",
            WidgetType::Sup => "sup",
            WidgetType::Time => "time",
            WidgetType::U => "u",
            WidgetType::Var => "var",
            WidgetType::Wbr => "wbr",
            WidgetType::Area => "area",
            WidgetType::Audio => "audio",
            WidgetType::Img => "img",
            WidgetType::Map => "map",
            WidgetType::Track => "track",
            WidgetType::Video => "video",
            WidgetType::Embed => "embed",
            WidgetType::Iframe => "iframe",
            WidgetType::Object => "object",
            WidgetType::Param => "param",
            WidgetType::Picture => "picture",
            WidgetType::Portal => "portal",
            WidgetType::Source => "source",
            WidgetType::Svg => "svg",
            WidgetType::Math => "math",
            WidgetType::Canvas => "canvas",
            WidgetType::Noscript => "noscript",
            WidgetType::Script => "script",
            WidgetType::Del => "del",
            WidgetType::Ins => "ins",
            WidgetType::Caption => "caption",
            WidgetType::Col => "col",
            WidgetType::Colgroup => "colgroup",
            WidgetType::Table => "table",
            WidgetType::Tbody => "tbody",
            WidgetType::Td => "td",
            WidgetType::Tfoot => "tfoot",
            WidgetType::Th => "th",
            WidgetType::Thead => "thead",
            WidgetType::Tr => "tr",
            WidgetType::Button => "button",
            WidgetType::Datalist => "datalist",
            WidgetType::Fieldset => "fieldset",
            WidgetType::Form => "form",
            WidgetType::Input => "input",
            WidgetType::Label => "label",
            WidgetType::Legend => "legend",
            WidgetType::Meter => "meter",
            WidgetType::Optgroup => "optgroup",
            WidgetType::Option => "option",
            WidgetType::Output => "output",
            WidgetType::Progress => "progress",
            WidgetType::Select => "select",
            WidgetType::Textarea => "textarea",
            WidgetType::Details => "details",
            WidgetType::Dialog => "dialog",
            WidgetType::Menu => "menu",
            WidgetType::Summary => "summary",
            WidgetType::Slot => "slot",
            WidgetType::Template => "template",
        }
    }
}

