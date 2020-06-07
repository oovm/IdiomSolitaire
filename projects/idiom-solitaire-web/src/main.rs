#![recursion_limit = "1024"]


use yew::{
    html,
    prelude::*,
    services::reader::{FileData, ReaderService, ReaderTask},
    Component, ComponentLink, Html, ShouldRender,
};
use idiom_solitaire::{SolitaireSolver};

mod form;

pub enum Event {
    Input(String),
    Length(ChangeData),
    Mode(ChangeData),
    Files(ChangeData),
    Loaded(FileData),
}

#[derive(Debug)]
pub struct Model {
    link: ComponentLink<Self>,
    tasks: Vec<ReaderTask>,
    input: String,
    solver: SolitaireSolver,
    length: usize
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            tasks: vec![],
            input: String::new(),
            solver: Default::default(),
            length: 1
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(s) => {
                self.input = s;
            }
            Event::Files(ChangeData::Files(f)) => {
                let task = ReaderService::new().read_file(f.get(0).unwrap(), self.link.callback(Event::Loaded)).unwrap();
                self.tasks.push(task)
            }
            Event::Loaded(data) => {
                let _ = data;
                //ConsoleService::log(&format!("{:?}", data));
                //self.image = data.content
            }
            _ => (),
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <main class="container-fluid">
            <div class="page-header">
                <h1>{"成语接龙"}</h1>
                <span>
                <iframe
                    src="https://ghbtns.com/github-btn.html?user=GalAster&repo=IdiomSolitaire&type=star&count=true&size=large"
                    frameborder="0" scrolling="0" width="170" height="30" title="GitHub" loading="lazy"
                />
                </span>
            </div>
            {self.form_view()}
        </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
