use crate::{Event, Model};
use yew::prelude::*;
use idiom_solitaire::SolitaireMode;

impl Model {
    pub fn format_mode(&self) -> u32 {
        match self.solver.mode {
            SolitaireMode::Character => 0,
            SolitaireMode::Pinyin => 1,
            SolitaireMode::Tone => 2,
        }
    }

    pub fn qr_code_view(&self) -> Html {
        let qr = "";
        return html! {
        <div class="form-group">
            <label class="col-sm-2">{"QR_CODE:"}</label>
            <div class="col-sm-10">{qr}</div>
        </div>
        };
    }

    pub fn form_view(&self) -> Html {
        html! {
        <form class="form-horizontal">
            {self.qr_code_view()}
            <div class="form-group">
                <label class="col-sm-2">{"Text:"}</label>
                <div class="col-sm-10">
                    <textarea class="form-control" rows="3"
                        value=self.input
                        oninput=self.link.callback(|input: InputData| Event::Input(input.value))
                    />
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Image:"}</label>
                <div class="col-sm-10">
                    <input type="file" multiple=true
                        onchange=self.link.callback(|input: ChangeData| Event::Files(input))
                    />
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Output Size:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="range" min="80" max="640" step="20"
                            value=self.output_size
                            onchange=self.link.callback(|input: ChangeData| Event::OutputSize(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1">{self.output_size}</label>
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
            <div class="form-group">
                <label class="col-sm-2">{"Enhanced:"}</label>
                <div class="col-sm-10">
                    <input type="checkbox"
                        checked=self.enhanced
                        onchange=self.link.callback(|input: ChangeData| Event::EnhanceMode(input))
                    />
                 </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Background:"}</label>
                <div class="col-sm-10">
                    <div class="form-control-static">
                        <input type="color"
                            onchange=self.link.callback(|input: ChangeData| Event::LightColor(input))
                        />
                    </div>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Foreground:"}</label>
                <div class="col-sm-10">
                    <div class="form-control-static">
                        <input type="color"
                            onchange=self.link.callback(|input: ChangeData| Event::DarkColor(input))
                        />
                    </div>
                </div>
            </div>
        </form>
        }
    }
}
