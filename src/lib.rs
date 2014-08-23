#![crate_name = "input"]
#![deny(missing_doc)]
#![feature(globs)]
#![feature(struct_variant)]
#![unstable]

/*!
A flexible structure for user interactions to be used in window frameworks and widgets libraries.

## Example usage

### Video game

```ignore
enum Action {
    MoveLeft,
    MoveRight,
    Shoot,
    Jump
}

let config: HashMap<(input::DeviceID, input::ElementID), Action> = load_config();

for event in implementation.events() {
    let key = (event.get_device_id().clone(), event.get_element_id().clone());

    match config.find(key) {
        Some(MoveLeft) => move_left(),
        Some(MoveRight) => move_right(),
        Some(Shoot) => shoot(),
        Some(Jump) => jump(),
        None => ()
    }
}
```

### User interface

```ignore
fn handle_event<E: input::Event>(event: E) {
    use input::keyboard::ToKeyboardEvent;
    use input::mouse::ToMouseEvent;

    match event.to_keyboard_event() {
        Some(input::keyboard::KeyRelease{ref key, ..}) => {
            dispatch_key(key);
            return;
        },
        _ => ()
    }

    match event.to_mouse_event() {
        Some(input::mouse::MouseMove{ref x, ref y, ..}) => {
            handle_mouse_hover(x, y);
            return;
        },
        Some(input::mouse::MouseRelease{ref button, ..}) if button == Some(input::mouse::Left) => {
            handle_left_click();
            return;
        },
        _ => ()
    }
}
```

## How to implement this interface

Implementation should provide a compile-time way to obtains objects which implement
 `keyboard::KeyboardDevice`, or `mouse::MouseDevice`, or `gamepad::GamepadDevice`, etc.

Devices that are known to always exist (for example a keyboard and a mouse on desktop PC,
 a touch screen on a mobile device, gamepads on console, etc.) should be easy to obtain, for
 example by providing functions like `get_main_keyboard()`, `get_gamepad_1()`, etc.

The way an implementation provides events is not covered by this library. However events
 must implement the `Event` trait in addition to the `keyboard::ToKeyboardEvent`,
 `mouse::ToMouseEvent`, etc. depending on which type of devices may be available on the platform.

Implementations should provide a way for the user to know which device emitted each event (for
 example by providing a `(&Device, Event)` instead of just an `Event`. This is also out of the
 scope of this library.

**/

pub mod keyboard;
pub mod mouse;

/// Represents when an event happened.
/// 
/// The exact value of this and how it can be used remain to be defined.
#[deriving(Show, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Returns the number of nanoseconds since an undefined epoch.
    #[experimental = "May be removed"]
    pub fn as_nanoseconds(&self) -> u64 {
        let &Timestamp(val) = self;
        val
    }
}

/// Object which provides informations about a specific device.
pub trait Device {
    /// Returns the ID of this device.
    fn get_device_id(&self) -> &DeviceID;

    /// Returns the list of elements (ie. buttons and axes) on this device.
    fn get_elements(&self) -> &[Element];

    /// Returns the human-friendly name of the device.
    fn get_human_friendly_name(&self) -> &str;

    /// Returns the value of an element.
    /// 
    /// For absolute axes, the value is within the given range. For relative axes, the value
    ///  is arbitrary. For buttons, the value is either 0 (released) or 1 (pressed).
    fn get_value(&self, &ElementID) -> f32;
}

/// Represents an identifier for a device on the system.
///
/// Each device must have a different `DeviceID`. Two identical devices should provide the same
///  `DeviceID`.
///
/// The `DeviceID` should be persitant. If you store its value in a file, then quit the program,
///  relaunch it, and reload the value, it should still match the same device.
///
/// The exact value is platform-specific and thus implementation-defined.
#[deriving(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Show)]
pub struct DeviceID(pub String);

/// Represents an identifier of an element on a device.
///
/// The exact value is implementation-defined. For keyboards, this is usually the scancode of
///  the key.
#[deriving(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Show)]
pub struct ElementID(pub u64);

/// Trait for an event produced by a device.
pub trait Event {
    /// Returns the moment when the event happened.
    fn get_timestamp(&self) -> &Timestamp;

    /// Return which device triggered this event.
    fn get_device_id(&self) -> &DeviceID;

    /// Return which element triggered this event.
    fn get_element_id(&self) -> &ElementID;

    /// Returns the new value of the element.
    /// 
    /// For absolute axes, the value is within the given range. For relative axes, the value
    ///  is arbitrary. For buttons, the value is either 0 (released) or 1 (pressed).
    fn get_element_value(&self) -> f32;
}

/// An element of a device. For example a button.
pub enum Element {
    /// An axis which produces absolute values.
    ///
    /// Example: the axes of a joystick.
    AbsoluteAxis {
        /// Identifier for this element on the device.
        pub id: ElementID,

        /// The human-friendly name of this element.
        pub name: String,

        /// Minimum and maximum values of the axis.
        pub range: (f32, f32),
    },

    /// An axis which produces relative values.
    ///
    /// Example: an accelerometer, the axes of a mouse.
    RelativeAxis {
        /// Identifier for this element on the device.
        pub id: ElementID,

        /// The human-friendly name of this element.
        pub name: String,
    },

    /// A button which can be either pressed or released.
    ///
    /// Example: mouse buttons, keyboard keys.
    Button {
        /// Identifier for this element on the device.
        pub id: ElementID,

        /// The human-friendly name of this element.
        pub name: String,
    },

    /// An element of unspecified type.
    ///
    /// This can be used for "system" elements that produce events but that are neither axes
    ///  nor buttons.
    UnknownElement {
        /// Identifier for this element on the device.
        pub id: ElementID,
    },
}
