use crate::views::*;
use yew::prelude::*;
use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum SettingsRoute {
    #[at("/settings")]
    Relay,
    #[at("/settings/profile")]
    Profile,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

pub fn switch_main(route: MainRoute) -> Html {
    html!(
        <Layout>
            {
                match route {
                MainRoute::Home => html!(<Home/>),
                MainRoute::Contact => html!(<Contact/>),
                MainRoute::SettingsRoot | MainRoute::Settings =>  html!(<Switch<SettingsRoute> render={switch_settings} />),
                MainRoute::NotFound => html!(<h1>{"Not found"}</h1>),
            }
        }
        </Layout>
    )
}

fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Profile => html! {
                <h1>{"Profile"}</h1>
        },
        SettingsRoute::Relay => html! {
                 <Settings/>
        },
        SettingsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>},
    }
}
