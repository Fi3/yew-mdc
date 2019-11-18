use crate::mdc_sys::MDCRipple;
use yew::prelude::*;

pub struct Button {
    id: String,
    ripple: Option<MDCRipple>,
    props: Props,
}

#[derive(PartialEq, Debug)]
pub enum Style {
    None,
    Raised,
    Unelevated,
    Outlined,
}
impl Default for Style {
    fn default() -> Style {
        Style::None
    }
}
impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Style::*;
        let result = match self {
            None => "",
            Raised => "mdc-button--raised",
            Unelevated => "mdc-button--unelevated",
            Outlined => "mdc-button--outlined",
        };
        write!(f, "{}", result)
    }
}

#[derive(Properties, Debug)]
pub struct Props {
    pub children: Children<Button>,
    pub id: Option<String>,
    #[props(required)]
    pub text: String,
    pub style: Style,
    pub ripple: bool,
    pub trailingicon: bool,
    pub onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

impl Component for Button {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|id| id.to_owned())
            .unwrap_or_else(|| format!("button-{}", crate::next_id()));
        Self {
            id,
            ripple: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if self.props.ripple {
            self.ripple = crate::get_element_by_id(&self.id).map(MDCRipple::new);
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(callback) = &self.props.onclick {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn view(&self) -> Html<Self> {
        let ripple = if self.props.ripple {
            html! {
                <div class="mdc-button__ripple"></div>
            }
        } else {
            html! {}
        };
        let inner = if self.props.trailingicon {
            html! { <>
                { self.props.children.render() }
                <span class="mdc-button__label">{ &self.props.text }</span>
            </> }
        } else {
            html! { <>
                <span class="mdc-button__label">{ &self.props.text }</span>
                { self.props.children.render() }
            </> }
        };
        html! {
            <button class=format!("mdc-button {}", self.props.style)
                    id=self.id
                    onclick=|_| Msg::Clicked>
                { ripple }
                { inner }
            </button>
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}