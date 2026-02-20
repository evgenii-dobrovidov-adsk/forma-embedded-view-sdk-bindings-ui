#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextLevel {
    H1,
    H2,
    H3,
    P,
    Code,
}

impl TextLevel {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::P => "p",
            Self::Code => "code",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertType {
    Error,
    Warning,
    Info,
}

impl AlertType {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Text,
    Number,
    Email,
    Password,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTimeLocal,
    Month,
    Week,
    Color,
    Range,
    Hidden,
}

impl InputType {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Email => "email",
            Self::Password => "password",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Search => "search",
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::Color => "color",
            Self::Range => "range",
            Self::Hidden => "hidden",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Outlined,
    Flat,
    Solid,
}

impl ButtonVariant {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Outlined => "outlined",
            Self::Flat => "flat",
            Self::Solid => "solid",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}
