//! Contains the `InputProvider` trait, which is used to provide input events to widgets.

use events::{ConrodEvent, Scroll, MouseClick, MouseDrag, InputState};
use input::{Input, Button};
use input::keyboard::Key;
use input::mouse::MouseButton;
use position::Point;


/// Trait for something that provides events to be consumed by a widget.
/// Provides a bunch of convenience methods for filtering out specific types of events.
pub trait InputProvider<'a, T: Iterator<Item=&'a ConrodEvent>> {
    /// This is the only method that needs to be implemented.
    /// Just provided a reference to a `Vec<ConrodEvent>` that contains
    /// all the events for this update cycle.
    fn all_events(&'a self) -> T;

    /// Returns the current input state. The returned state is assumed to be up to
    /// date with all of the events so far.
    fn current_state(&self) -> &InputState;

    //////////////////////////////////////////////////
    // Methods that just check the stream of events //
    //////////////////////////////////////////////////

    /// Returns a `String` containing _all_ the text that was entered since
    /// the last update cycle.
    fn text_just_entered(&'a self) -> Option<String> {
        let all_text: String = self.all_events().filter_map(|evt| {
            match *evt {
                ConrodEvent::Raw(Input::Text(ref text)) => Some(text),
                _ => None
            }
        }).fold(String::new(), |acc, item| {
            acc + item
        });

        if all_text.is_empty() {
            None
        } else {
            Some(all_text)
        }
    }

    /// Returns all of the `Key`s that were released since the last update.
    fn keys_just_released(&'a self) -> Vec<Key> {
        use input::Button::Keyboard;

        self.all_events().filter_map(|evt| {
            match *evt {
                ConrodEvent::Raw(Input::Release(Keyboard(key))) => Some(key),
                _ => None
            }
        }).collect::<Vec<Key>>()
    }

    /// Returns all of the keyboard `Key`s that were pressed since the last update.
    fn keys_just_pressed(&'a self) -> Vec<Key> {
        use input::Button::Keyboard;

        self.all_events().filter_map(|evt| {
            match *evt {
                ConrodEvent::Raw(Input::Press(Keyboard(key))) => Some(key),
                _ => None
            }
        }).collect::<Vec<Key>>()
    }

    /// Returns all of the `MouseButton`s that were pressed since the last update.
    fn mouse_buttons_just_pressed(&'a self) -> Vec<MouseButton> {
        self.all_events().filter_map(|evt| {
            match *evt {
                ConrodEvent::Raw(Input::Press(Button::Mouse(button))) => Some(button),
                _ => None
            }
        }).collect::<Vec<MouseButton>>()
    }

    /// Returns all of the `MouseButton`s that were released since the last update.
    fn mouse_buttons_just_released(&'a self) -> Vec<MouseButton> {
        self.all_events().filter_map(|evt| {
            match *evt {
                ConrodEvent::Raw(Input::Release(Button::Mouse(button))) => Some(button),
                _ => None
            }
        }).collect::<Vec<MouseButton>>()
    }

    /// Returns a `Scroll` struct if any scrolling was done since the last update.
    /// If multiple raw scroll events occured since the last update (which could very well
    /// happen if the user is scrolling quickly), then the `Scroll` returned will represent an
    /// aggregate total of all the scrolling.
    fn scroll(&'a self) -> Option<Scroll> {
        self.all_events().filter_map(|evt| {
            match *evt {
                ConrodEvent::Scroll(scroll) => Some(scroll),
                _ => None
            }
        }).fold(None, |maybe_scroll, scroll| {
            if maybe_scroll.is_some() {
                maybe_scroll.map(|acc| {
                    Scroll{
                        x: acc.x + scroll.x,
                        y: acc.y + scroll.y,
                        modifiers: scroll.modifiers
                    }
                })
            } else {
                Some(scroll)
            }
        })
    }

    /// Convenience method to call `mouse_drag`, passing in `MouseButton::Left`.
    /// Saves widgets from having to `use input::mouse::MouseButton` if all they care
    /// about is the left mouse button.
    fn mouse_left_drag(&'a self) -> Option<MouseDrag> {
        self.mouse_drag(MouseButton::Left)
    }

    /// Returns a `MouseDrag` if one has occured involving the given mouse button.
    /// If multiple raw mouse movement events have
    /// occured since the last update (which will happen if the user moves the mouse quickly),
    /// then the returned `MouseDrag` will be only the _most recent_ one, which will contain
    /// the most recent mouse position.
    fn mouse_drag(&'a self, button: MouseButton) -> Option<MouseDrag> {
        self.all_events().filter_map(|evt| {
            match *evt {
                ConrodEvent::MouseDrag(drag_evt) if drag_evt.button == button => Some(drag_evt),
                _ => None
            }
        }).last()
    }

    /// Convenience method to call `mouse_click`, passing in passing in `MouseButton::Left`.
    /// Saves widgets from having to `use input::mouse::MouseButton` if all they care
    /// about is the left mouse button.
    fn mouse_left_click(&'a self) -> Option<MouseClick> {
        self.mouse_click(MouseButton::Left)
    }

    /// Convenience method to call `mouse_click`, passing in passing in `MouseButton::Right`.
    /// Saves widgets from having to `use input::mouse::MouseButton` if all they care
    /// about is the left mouse button.
    fn mouse_right_click(&'a self) -> Option<MouseClick> {
        self.mouse_click(MouseButton::Right)
    }

    /// Returns a `MouseClick` if one has occured with the given mouse button.
    /// A _click_ is determined to have occured if a mouse button was pressed and subsequently
    /// released while the mouse was in roughly the same place.
    fn mouse_click(&'a self, button: MouseButton) -> Option<MouseClick> {
        self.all_events().filter_map(|evt| {
            match *evt {
                ConrodEvent::MouseClick(click) if click.button == button => Some(click),
                _ => None
            }
        }).next()
    }

    /////////////////////////////////////////////////////
    // Methods that just check the current input state //
    /////////////////////////////////////////////////////

    /// Returns true if the given mouse button is currently pressed, otherwise false
    fn mouse_button_currently_pressed(&self, button: MouseButton) -> bool {
        match self.current_state().mouse_buttons.get(button) {
            Some(_) => true,
            _ => false
        }
    }

    /// Convenience method for checking if the Left mouse button is down.
    /// Returns true if the Left mouse button is currently pressed, otherwise false.
    fn mouse_left_button_currently_pressed(&self) -> bool {
        self.mouse_button_currently_pressed(MouseButton::Left)
    }

    /// Convenience method for checking if the Right mouse button is down.
    /// Returns true if the Right mouse button is currently pressed, otherwise false.
    fn mouse_right_button_currently_pressed(&self) -> bool {
        self.mouse_button_currently_pressed(MouseButton::Right)
    }

    /// Convenience method for returning the current mouse position.
    fn current_mouse_position(&self) -> Point {
        self.current_state().mouse_position
    }

}