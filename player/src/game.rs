use common::*;

#[inline]
pub fn update_and_render(state: &mut Framebuffer, input: Input) {
    let mut clearColour = 0;

    if input.pressed_this_frame(Button::Left) {
        clearColour = BLUE;
    }

    if input.pressed_this_frame(Button::Right) {
        clearColour = GREEN;
    }

    if input.pressed_this_frame(Button::Up) {
        clearColour = RED;
    }

    if input.pressed_this_frame(Button::Down) {
        clearColour = YELLOW;
    }

    if input.pressed_this_frame(Button::Select) {
        clearColour = PURPLE;
    }

    if input.pressed_this_frame(Button::Start) {
        clearColour = GREY;
    }

    if input.pressed_this_frame(Button::A) {
        clearColour = WHITE;
    }

    if input.pressed_this_frame(Button::B) {
        clearColour = BLACK;
    }

    if clearColour != 0 {
        state.clearTo(clearColour);
    }
}
