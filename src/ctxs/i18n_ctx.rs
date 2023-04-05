use std::fmt::Display;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default, Copy)]
pub enum I18nLocale {
    #[default]
    Zhcn,
    En,
}

impl Display for I18nLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nLocale::En => "en",
                I18nLocale::Zhcn => "zh-cn",
            }
        )
    }
}

impl Reducible for I18nLocale {
    type Action = I18nLocale;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        rust_i18n::set_locale(action.to_string().as_str());
        Rc::new(action)
    }
}
pub type I18nLocaleContext = UseReducerHandle<I18nLocale>;
