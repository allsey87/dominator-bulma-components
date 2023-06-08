use std::sync::Arc;

use dominator::{html, Dom, clone, BoxFragment, events, animation::{MutableAnimation, Percentage}};
use dominator_bulma::{columns, column, button, icon};
use futures_signals::signal::{Mutable, SignalExt};

pub struct Component {
    current_index: Mutable<isize>,
    slides: Vec<BoxFragment>,
    transition: MutableAnimation,
}

impl Component {

    pub fn new(slides: Vec<BoxFragment>) -> Component {
        Component {
            current_index: Mutable::new(0),
            slides,
            transition: MutableAnimation::new_with_initial(500.0, Percentage::new(0.5))
        }
    }

    fn render_slide(
        this: &Arc<Component>,
        current_index: isize,
        offset: isize,
    ) -> Dom {
        let len = this.slides.len();
        let mut index = current_index + offset;
        while index < 0 {
            index += len as isize
        }
        let index = index % len as isize;
        let content = this.slides.get(index as usize).unwrap();
        let transform = this.transition.signal()
            .map(move |percent| percent.range_inclusive(-100.0, 100.0) + (offset * 100) as f64)
                .map(|value| format!("translate({value}%, 0)"));
        let gradient = this.transition.signal()
            .map(move |percent| percent.range_inclusive(-100.0, 100.0) + (offset * 100) as f64)
                .map(|value| format!("linear-gradient(270deg, \
                    rgba(0,0,0,0) {}%, \
                    rgba(0,0,0,1) {}%, \
                    rgba(0,0,0,1) {}%, \
                    rgba(0,0,0,0) {}%)",
                    value - 10.0, value + 0.0, value + 100.0, value + 110.0));
        column!({
            .apply_if(offset != 0, |dom| { dom.class("is-overlay") })
            .style_signal("-webkit-mask-image", gradient)
            .style_signal("transform", transform)
            .fragment(&content)
        })
    }

    pub fn render(this: &Arc<Component>) -> Dom {
        columns!("is-mobile", "is-vcentered", "is-relative", {
            .future(this.transition.signal().for_each(clone!(this => move |value: Percentage| {
                let value = value.into_f64();
                if value == 1.0 {
                    this.transition.jump_to(Percentage::new(0.5));
                    let mut current_index = this.current_index.lock_mut();
                    *current_index -= 1;
                }
                if value == 0.0 {
                    this.transition.jump_to(Percentage::new(0.5));
                    let mut current_index = this.current_index.lock_mut();
                    *current_index += 1;
                }
                async {}
            })))
            .child(column!("is-narrow", {
                .style("z-index", "1")
                .child(button!("is-rounded", {
                    .event(clone!(this => move |_: events::Click| {
                        this.transition.animate_to(Percentage::new(1.0))
                    }))
                    .child(icon!("is-medium", {
                        .child(html!("i", {
                            .class("mdi")
                            .class("mdi-24px")
                            .class("mdi-arrow-left-bold")
                        }))
                    }))
                }))
            }))
            .child(column!({
                .child(columns!("is-relative", {
                    .children_signal_vec(this.current_index.signal()
                        .map(clone!(this => move |index| vec![
                            Self::render_slide(&this, index, -2),
                            Self::render_slide(&this, index, -1),
                            Self::render_slide(&this, index, 0),
                            Self::render_slide(&this, index, 1),
                            Self::render_slide(&this, index, 2),
                        ]))
                        .to_signal_vec()
                    )
                }))
            }))
            .child(column!("is-narrow", {
                .style("z-index", "1")
                .child(button!("is-rounded", {
                    .event(clone!(this => move |_: events::Click| {
                        this.transition.animate_to(Percentage::new(0.0))
                    }))
                    .child(icon!("is-medium", {
                        .child(html!("i", {
                            .class("mdi")
                            .class("mdi-24px")
                            .class("mdi-arrow-right-bold")
                        }))
                    }))
                }))
            }))
        })
    }
}