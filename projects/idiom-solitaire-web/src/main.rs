#![recursion_limit = "1024"]

use idiom_solitaire::{Idiom, SolitaireMode, SolitaireSolver};
use std::str::FromStr;
use yew::{
    html,
    prelude::*,
    services::reader::{FileData, ReaderService, ReaderTask},
    Component, ComponentLink, Html, ShouldRender,
};

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
    output: Vec<Idiom>,
    length: usize,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut solver = SolitaireSolver::default();
        solver.mode = SolitaireMode::Tone;
        solver.load(include_bytes!("../../external/database.csv")).unwrap();
        Self { link, tasks: vec![], input: String::from("耗子尾汁"), solver, output: vec![], length: 1 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(s) => {
                self.input = s;
                self.resolve();
            }
            Event::Length(ChangeData::Value(s)) => {
                if let Ok(o) = usize::from_str(&s) {
                    self.length = o
                }
                self.resolve();
            }
            Event::Mode(ChangeData::Select(s)) => {
                self.solver.mode = match s.value().as_ref() {
                    "2" => SolitaireMode::Sound,
                    "1" => SolitaireMode::Tone,
                    _ => SolitaireMode::Character,
                };
                self.resolve()
            }
            Event::Files(ChangeData::Files(f)) => {
                let task = ReaderService::new().read_file(f.get(0).unwrap(), self.link.callback(Event::Loaded)).unwrap();
                self.tasks.push(task)
            }
            Event::Loaded(data) => {
                let _ = data;
                // ConsoleService::log(&format!("{:?}", data));
                // self.image = data.content
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

impl Model {
    pub fn resolve(&mut self) {
        if self.solver.dict.0.is_empty() {
            "GG";
        }
        self.output = self.solver.solve_chain(&self.input, self.length)
    }
    pub fn load_default_dict(&mut self) {
        unimplemented!()
    }
}

fn main() {
    yew::start_app::<Model>();
}
