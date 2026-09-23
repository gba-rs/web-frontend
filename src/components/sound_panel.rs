use yew::prelude::*;
use yew::{html, Component, Context, Html, NodeRef};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const WIDTH: u32 = 460;
const HEIGHT: u32 = 100;
const COL_LEFT: &str = "#60d0ff";
const COL_RIGHT: &str = "#ff8040";
const COL_CENTER: &str = "#303840";
const COL_BG: &str = "#101418";

pub struct SoundPanel {
    canvas_ref: NodeRef,
    props: SoundPanelProp,
}

#[derive(Properties, Clone, PartialEq)]
pub struct SoundPanelProp {
    pub samples: Vec<i16>,
}

impl Component for SoundPanel {
    type Message = ();
    type Properties = SoundPanelProp;

    fn create(ctx: &Context<Self>) -> Self {
        SoundPanel {
            canvas_ref: NodeRef::default(),
            props: ctx.props().clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.canvas_ref.clone()} width={WIDTH.to_string()} height={HEIGHT.to_string()}></canvas>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        self.draw();
    }
}

impl SoundPanel {
    fn draw(&self) {
        let canvas = match self.canvas_ref.cast::<HtmlCanvasElement>() {
            Some(c) => c,
            None => return,
        };
        let ctx = match canvas.get_context("2d").ok().flatten().and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok()) {
            Some(c) => c,
            None => return,
        };

        ctx.set_fill_style_str(COL_BG);
        ctx.fill_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);

        let mid = HEIGHT as f64 / 2.0;
        ctx.set_stroke_style_str(COL_CENTER);
        ctx.begin_path();
        ctx.move_to(0.0, mid);
        ctx.line_to(WIDTH as f64, mid);
        ctx.stroke();

        let frame_count = self.props.samples.len() / 2;
        if frame_count < 2 {
            return;
        }

        self.plot_channel(&ctx, 0, frame_count, COL_LEFT);
        self.plot_channel(&ctx, 1, frame_count, COL_RIGHT);
    }

    fn plot_channel(&self, ctx: &CanvasRenderingContext2d, channel_offset: usize, frame_count: usize, color: &str) {
        ctx.set_stroke_style_str(color);
        ctx.begin_path();
        let half_h = HEIGHT as f64 / 2.0;
        for col in 0..WIDTH as usize {
            let frame_idx = col * frame_count / WIDTH as usize;
            let sample = self.props.samples[frame_idx * 2 + channel_offset] as f64;
            let normalized = sample / (i16::MAX as f64);
            let x = col as f64;
            let y = half_h - normalized * half_h;
            if col == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();
    }
}
