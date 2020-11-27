use crate::{Event, Model};
use idiom_solitaire::{Idiom, SolitaireMode};
use yew::prelude::*;

impl Model {
    pub fn format_mode(&self) -> u32 {
        match self.solver.mode {
            SolitaireMode::Character => 0,
            SolitaireMode::Sound => 1,
            SolitaireMode::Tone => 2,
        }
    }

    pub fn error_view(&self) -> Html {
        match self.errors {
            Some(s) => {
                html! {
                <div class="form-group">
                    <label class="col-sm-2">{"报错信息:"}</label>
                    <div class="col-sm-10">
                        <label sytle="color:red">{s}</label>
                    </div>
                </div>
                }
            }
            None => html! {},
        }
    }

    pub fn solitaire_view(&self) -> Html {
        if self.output.is_empty() {
            return html! {};
        }
        let items = self.output.iter().map(|i| idiom_view(i)).collect::<Html>();
        return html! {
        <div class="form-group">
            <label class="col-sm-2">{"接龙结果:"}</label>
            <div class="col-sm-10">
                {items}
                <label sytle="color:red">{"?"}</label>
            </div>
        </div>
        };
    }

    pub fn form_view(&self) -> Html {
        // let debug = format!("{:?}", self);
        html! {
        <form class="form-horizontal">//{debug}
            <div class="form-group">
                <label class="col-sm-2">{"初始输入:"}</label>
                <div class="col-sm-10">
                    <textarea class="form-control" rows="3"
                        value=self.input
                        oninput=self.link.callback(|input: InputData| Event::Input(input.value))
                    />
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"本地字典:"}</label>
                <div class="col-sm-10">
                    <input type="file" multiple=true
                        onchange=self.link.callback(|input: ChangeData| Event::Files(input))
                    />
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"接龙长度:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="range" min="1" max="100" step="1"
                            value=self.length
                            onchange=self.link.callback(|input: ChangeData| Event::Length(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1">{self.length}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"接龙模式:"}</label>
                <div class="col-sm-10">
                    <select class="form-control"
                        value=self.format_mode()
                        onchange=self.link.callback(|input: ChangeData| Event::Mode(input))
                    >
                        <option value="2">{"同音"}</option>
                        <option value="1">{"同调"}</option>
                        <option value="0">{"同字"}</option>
                    </select>
                </div>
            </div>
            {self.error_view()}
            {self.solitaire_view()}
        </form>
        }
    }
}

pub fn idiom_view(input: &Idiom) -> Html {
    let text = input.idiom.as_str();
    return html! {<>
    <span class="tooltip">
        {text}
        <div class="tooltiptext">
            <label>{"成语: "}</label>
            <span>{text}</span>
            <br/>
            <label>{"注音: "}</label>
            <span>{input.pinyin.as_str()}</span>
            <br/>
            <label>{"注释: "}</label>
            <span>{input.explanation.as_str()}</span>
        </div>
    </span>
    {" ➞ "}
    </>};
}
