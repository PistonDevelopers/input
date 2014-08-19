//! Back-end agnostic mouse buttons.

use {Device, DeviceID, ElementID, Event, Timestamp};

/// An object that represents a mouse.
pub trait MouseDevice: Device {
    /// Returns the position of the cursor on the surface.
    ///
    /// The value is the (x, y) coordinates in pixels relative to the top-left hand corner
    ///  of the surface.
    fn get_cursor_position(&self) -> (i16, i16);
}

/// An event triggered by a mouse device.
#[deriving(Clone, Show)]
pub enum MouseEvent {
    /// Pressed a mouse button.
    MousePress {
        /// When the event happened.
        pub timestamp: Timestamp,

        /// Which device triggered this event.
        pub device: DeviceID,

        /// Which button triggered this event.
        pub element: ElementID,

        /// The meaning of the button if known.
        pub button: Option<Button>,
    },

    /// Released a mouse button.
    MouseRelease {
        /// When the event happened.
        pub timestamp: Timestamp,

        /// Which device triggered this event.
        pub device: DeviceID,

        /// Which button triggered this event.
        pub element: ElementID,

        /// The meaning of the button if known.
        pub button: Option<Button>,
    },

    /// Moved mouse cursor.
    MouseMove {
        /// When the event happened.
        pub timestamp: Timestamp,
        /// Which device triggered this event.
        pub device: DeviceID,
        /// Which axis triggered this event.
        pub element: ElementID,
        /// x in window coordinates.
        pub x: f64,
        /// y in window coordinates.
        pub y: f64,
        /// x in drawing coordinates.
        pub draw_x: f64,
        /// y in drawing coordinates.
        pub draw_y: f64,
        /// Delta x in window coordinates.
        pub delta_x: f64,
        /// Delta y in window coordinates.
        pub delta_y: f64,
        /// Delta x in drawing coordinates.
        pub draw_delta_x: f64,
        /// Delta y in drawing coordinates.
        pub draw_delta_y: f64,
    },

    /// Scrolled mouse.
    MouseScroll {
        /// When the event happened.
        pub timestamp: Timestamp,
        /// Which device triggered this event.
        pub device: DeviceID,
        /// Which axis triggered this event.
        pub element: ElementID,
        /// x.
        pub x: f64,
        /// y.
        pub y: f64,
    }
}

impl Event for MouseEvent {
    fn get_timestamp(&self) -> &Timestamp {
        match self {
            &MousePress{ref timestamp, ..} => timestamp,
            &MouseRelease{ref timestamp, ..} => timestamp,
            &MouseMove{ref timestamp, ..} => timestamp,
            &MouseScroll{ref timestamp, ..} => timestamp
        }
    }

    fn get_device_id(&self) -> &DeviceID {
        match self {
            &MousePress{ref device, ..} => device,
            &MouseRelease{ref device, ..} => device,
            &MouseMove{ref device, ..} => device,
            &MouseScroll{ref device, ..} => device
        }
    }

    fn get_element_id(&self) -> &ElementID {
        match self {
            &MousePress{ref element, ..} => element,
            &MouseRelease{ref element, ..} => element,
            &MouseMove{ref element, ..} => element,
            &MouseScroll{ref element, ..} => element
        }
    }

    fn get_element_value(&self) -> f32 {
        match self {
            &MousePress{..} => 1.0,
            &MouseRelease{..} => 0.0,
            &MouseMove{delta_x, delta_y, ..} =>
                if delta_x != 0.0 { delta_x as f32 } else { delta_y as f32 },
            &MouseScroll{x, y, ..} =>
                if x != 0.0 { x as f32 } else { y as f32 },
        }
    }
}

/// Trait for events that can be turned into `MouseEvent`s.
pub trait ToMouseEvent: Event {
    /// Turns the event into a mouse event.
    fn to_mouse_event(&self) -> Option<MouseEvent>;
}

impl ToMouseEvent for MouseEvent {
    fn to_mouse_event(&self) -> Option<MouseEvent> {
        Some(self.clone())
    }
}

/// Represent a mouse button.
#[deriving(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Show)]
pub enum Button {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Extra mouse button number 1.
    X1,
    /// Extra mouse button number 2.
    X2,
}
