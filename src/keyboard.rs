//! Back-end agnostic keyboard keys.

use std::hash::Hash;
use std::hash::sip::SipState;
use std::num::FromPrimitive;
use std::num::ToPrimitive;
use {Device, DeviceID, ElementID, Event, Timestamp};

/// 
pub trait KeyboardDevice: Device {
    /// Returns the key corresponding to the element.
    ///
    /// Returns `None` if the element doesn't match any `Key` in the enum.
    fn get_mapping(&self, id: &ElementID) -> Option<Key>;
}

/// An event triggered by a keyboard device.
#[deriving(Clone, Show)]
pub enum KeyboardEvent {
    /// Pressed a keyboard key.
    KeyPress {
        /// When the event happened.
        pub timestamp: Timestamp,

        /// Which device triggered this event.
        pub device: DeviceID,

        /// Which element triggered this event.
        pub element: ElementID,

        /// The key that was pressed, or none if unknown.
        pub key: Option<Key>,
    },

    /// Released a keyboard key.
    KeyRelease {
        /// When the event happened.
        pub timestamp: Timestamp,

        /// Which device triggered this event.
        pub device: DeviceID,

        /// Which element triggered this event.
        pub element: ElementID,

        /// The key that was released, or none if unknown.
        pub key: Option<Key>,
    }
}

impl Event for KeyboardEvent {
    fn get_timestamp(&self) -> &Timestamp {
        match self {
            &KeyPress{ref timestamp, ..} => timestamp,
            &KeyRelease{ref timestamp, ..} => timestamp
        }
    }

    fn get_device_id(&self) -> &DeviceID {
        match self {
            &KeyPress{ref device, ..} => device,
            &KeyRelease{ref device, ..} => device
        }
    }

    fn get_element_id(&self) -> &ElementID {
        match self {
            &KeyPress{ref element, ..} => element,
            &KeyRelease{ref element, ..} => element
        }
    }

    fn get_element_value(&self) -> f32 {
        match self {
            &KeyPress{..} => 1.0,
            &KeyRelease{..} => 0.0
        }
    }
}

/// Trait for events that can be turned into `KeyboardEvent`s
pub trait ToKeyboardEvent: Event {
    /// Turns the event into a keyboard event.
    fn to_keyboard_event(&self) -> Option<KeyboardEvent>;
}

impl ToKeyboardEvent for KeyboardEvent {
    fn to_keyboard_event(&self) -> Option<KeyboardEvent> {
        Some(self.clone())
    }
}

/// Represent a keyboard key.
#[allow(missing_doc)]
#[deriving(Clone, Show)]
pub enum Key {
    Unknown                 = 0,
    Backspace               = 8,
    Tab                     = 9,
    Return                  = 13,
    Escape                  = 27,
    Space                   = 32,
    Exclaim                 = 33,
    Quotedbl                = 34,
    Hash                    = 35,
    Dollar                  = 36,
    Percent                 = 37,
    Ampersand               = 38,
    Quote                   = 39,
    LeftParen               = 40,
    RightParen              = 41,
    Asterisk                = 42,
    Plus                    = 43,
    Comma                   = 44,
    Minus                   = 45,
    Period                  = 46,
    Slash                   = 47,
    D0                      = 48,
    D1                      = 49,
    D2                      = 50,
    D3                      = 51,
    D4                      = 52,
    D5                      = 53,
    D6                      = 54,
    D7                      = 55,
    D8                      = 56,
    D9                      = 57,
    Colon                   = 58,
    Semicolon               = 59,
    Less                    = 60,
    Equals                  = 61,
    Greater                 = 62,
    Question                = 63,
    At                      = 64,
    LeftBracket             = 91,
    Backslash               = 92,
    RightBracket            = 93,
    Caret                   = 94,
    Underscore              = 95,
    Backquote               = 96,
    A                       = 97,
    B                       = 98,
    C                       = 99,
    D                       = 100,
    E                       = 101,
    F                       = 102,
    G                       = 103,
    H                       = 104,
    I                       = 105,
    J                       = 106,
    K                       = 107,
    L                       = 108,
    M                       = 109,
    N                       = 110,
    O                       = 111,
    P                       = 112,
    Q                       = 113,
    R                       = 114,
    S                       = 115,
    T                       = 116,
    U                       = 117,
    V                       = 118,
    W                       = 119,
    X                       = 120,
    Y                       = 121,
    Z                       = 122,
    Delete                  = 127,
    CapsLock                = 1073741881,
    F1                      = 1073741882,
    F2                      = 1073741883,
    F3                      = 1073741884,
    F4                      = 1073741885,
    F5                      = 1073741886,
    F6                      = 1073741887,
    F7                      = 1073741888,
    F8                      = 1073741889,
    F9                      = 1073741890,
    F10                     = 1073741891,
    F11                     = 1073741892,
    F12                     = 1073741893,
    PrintScreen             = 1073741894,
    ScrollLock              = 1073741895,
    Pause                   = 1073741896,
    Insert                  = 1073741897,
    Home                    = 1073741898,
    PageUp                  = 1073741899,
    End                     = 1073741901,
    PageDown                = 1073741902,
    Right                   = 1073741903,
    Left                    = 1073741904,
    Down                    = 1073741905,
    Up                      = 1073741906,
    NumLockClear            = 1073741907,
    NumPadDivide            = 1073741908,
    NumPadMultiply          = 1073741909,
    NumPadMinus             = 1073741910,
    NumPadPlus              = 1073741911,
    NumPadEnter             = 1073741912,
    NumPad1                 = 1073741913,
    NumPad2                 = 1073741914,
    NumPad3                 = 1073741915,
    NumPad4                 = 1073741916,
    NumPad5                 = 1073741917,
    NumPad6                 = 1073741918,
    NumPad7                 = 1073741919,
    NumPad8                 = 1073741920,
    NumPad9                 = 1073741921,
    NumPad0                 = 1073741922,
    NumPadPeriod            = 1073741923,
    Application             = 1073741925,
    Power                   = 1073741926,
    NumPadEquals            = 1073741927,
    F13                     = 1073741928,
    F14                     = 1073741929,
    F15                     = 1073741930,
    F16                     = 1073741931,
    F17                     = 1073741932,
    F18                     = 1073741933,
    F19                     = 1073741934,
    F20                     = 1073741935,
    F21                     = 1073741936,
    F22                     = 1073741937,
    F23                     = 1073741938,
    F24                     = 1073741939,
    Execute                 = 1073741940,
    Help                    = 1073741941,
    Menu                    = 1073741942,
    Select                  = 1073741943,
    Stop                    = 1073741944,
    Again                   = 1073741945,
    Undo                    = 1073741946,
    Cut                     = 1073741947,
    Copy                    = 1073741948,
    Paste                   = 1073741949,
    Find                    = 1073741950,
    Mute                    = 1073741951,
    VolumeUp                = 1073741952,
    VolumeDown              = 1073741953,
    NumPadComma             = 1073741957,
    NumPadEqualsAS400       = 1073741958,
    AltErase                = 1073741977,
    Sysreq                  = 1073741978,
    Cancel                  = 1073741979,
    Clear                   = 1073741980,
    Prior                   = 1073741981,
    Return2                 = 1073741982,
    Separator               = 1073741983,
    Out                     = 1073741984,
    Oper                    = 1073741985,
    ClearAgain              = 1073741986,
    CrSel                   = 1073741987,
    ExSel                   = 1073741988,
    NumPad00                = 1073742000,
    NumPad000               = 1073742001,
    ThousandsSeparator      = 1073742002,
    DecimalSeparator        = 1073742003,
    CurrencyUnit            = 1073742004,
    CurrencySubUnit         = 1073742005,
    NumPadLeftParen         = 1073742006,
    NumPadRightParen        = 1073742007,
    NumPadLeftBrace         = 1073742008,
    NumPadRightBrace        = 1073742009,
    NumPadTab               = 1073742010,
    NumPadBackspace         = 1073742011,
    NumPadA                 = 1073742012,
    NumPadB                 = 1073742013,
    NumPadC                 = 1073742014,
    NumPadD                 = 1073742015,
    NumPadE                 = 1073742016,
    NumPadF                 = 1073742017,
    NumPadXor               = 1073742018,
    NumPadPower             = 1073742019,
    NumPadPercent           = 1073742020,
    NumPadLess              = 1073742021,
    NumPadGreater           = 1073742022,
    NumPadAmpersand         = 1073742023,
    NumPadDblAmpersand      = 1073742024,
    NumPadVerticalBar       = 1073742025,
    NumPadDblVerticalBar    = 1073742026,
    NumPadColon             = 1073742027,
    NumPadHash              = 1073742028,
    NumPadSpace             = 1073742029,
    NumPadAt                = 1073742030,
    NumPadExclam            = 1073742031,
    NumPadMemStore          = 1073742032,
    NumPadMemRecall         = 1073742033,
    NumPadMemClear          = 1073742034,
    NumPadMemAdd            = 1073742035,
    NumPadMemSubtract       = 1073742036,
    NumPadMemMultiply       = 1073742037,
    NumPadMemDivide         = 1073742038,
    NumPadPlusMinus         = 1073742039,
    NumPadClear             = 1073742040,
    NumPadClearEntry        = 1073742041,
    NumPadBinary            = 1073742042,
    NumPadOctal             = 1073742043,
    NumPadDecimal           = 1073742044,
    NumPadHexadecimal       = 1073742045,
    LCtrl                   = 1073742048,
    LShift                  = 1073742049,
    LAlt                    = 1073742050,
    LGui                    = 1073742051,
    RCtrl                   = 1073742052,
    RShift                  = 1073742053,
    RAlt                    = 1073742054,
    RGui                    = 1073742055,
    Mode                    = 1073742081,
    AudioNext               = 1073742082,
    AudioPrev               = 1073742083,
    AudioStop               = 1073742084,
    AudioPlay               = 1073742085,
    AudioMute               = 1073742086,
    MediaSelect             = 1073742087,
    Www                     = 1073742088,
    Mail                    = 1073742089,
    Calculator              = 1073742090,
    Computer                = 1073742091,
    AcSearch                = 1073742092,
    AcHome                  = 1073742093,
    AcBack                  = 1073742094,
    AcForward               = 1073742095,
    AcStop                  = 1073742096,
    AcRefresh               = 1073742097,
    AcBookmarks             = 1073742098,
    BrightnessDown          = 1073742099,
    BrightnessUp            = 1073742100,
    DisplaySwitch           = 1073742101,
    KbdIllumToggle          = 1073742102,
    KbdIllumDown            = 1073742103,
    KbdIllumUp              = 1073742104,
    Eject                   = 1073742105,
    Sleep                   = 1073742106,
}


impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        return (*self as i32) == (*other as i32);
    }
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        let (s_id, o_id)  = (*self as i32, *other as i32);
        s_id.partial_cmp(&o_id)
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Key) -> Ordering {
        let (s_id, o_id)  = (*self as i32, *other as i32);
        s_id.cmp(&o_id)
    }
}

impl Key {
    /// Returns an id of the key
    #[inline(always)]
    pub fn code(&self) -> i32 {
        *self as i32
    }
}

impl Hash for Key {
    #[inline(always)]
    fn hash(&self, state: &mut SipState) {
        self.code().hash(state);
    }
}

impl ToPrimitive for Key {
    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        Some(self.code() as i64)
    }

    #[inline(always)]
    fn to_u64(&self) -> Option<u64> {
        Some(self.code() as u64)
    }

    #[inline(always)]
    fn to_int(&self) -> Option<int> {
        Some(self.code() as int)
    }
}

impl FromPrimitive for Key {
    fn from_u64(n: u64) -> Option<Key> {
        match n {
            0 => Some(Unknown),
            8 => Some(Backspace),
            9 => Some(Tab),
            13 => Some(Return),
            27 => Some(Escape),
            32 => Some(Space),
            33 => Some(Exclaim),
            34 => Some(Quotedbl),
            35 => Some(Hash),
            36 => Some(Dollar),
            37 => Some(Percent),
            38 => Some(Ampersand),
            39 => Some(Quote),
            40 => Some(LeftParen),
            41 => Some(RightParen),
            42 => Some(Asterisk),
            43 => Some(Plus),
            44 => Some(Comma),
            45 => Some(Minus),
            46 => Some(Period),
            47 => Some(Slash),
            48 => Some(D0),
            49 => Some(D1),
            50 => Some(D2),
            51 => Some(D3),
            52 => Some(D4),
            53 => Some(D5),
            54 => Some(D6),
            55 => Some(D7),
            56 => Some(D8),
            57 => Some(D9),
            58 => Some(Colon),
            59 => Some(Semicolon),
            60 => Some(Less),
            61 => Some(Equals),
            62 => Some(Greater),
            63 => Some(Question),
            64 => Some(At),
            91 => Some(LeftBracket),
            92 => Some(Backslash),
            93 => Some(RightBracket),
            94 => Some(Caret),
            95 => Some(Underscore),
            96 => Some(Backquote),
            97 => Some(A),
            98 => Some(B),
            99 => Some(C),
            100 => Some(D),
            101 => Some(E),
            102 => Some(F),
            103 => Some(G),
            104 => Some(H),
            105 => Some(I),
            106 => Some(J),
            107 => Some(K),
            108 => Some(L),
            109 => Some(M),
            110 => Some(N),
            111 => Some(O),
            112 => Some(P),
            113 => Some(Q),
            114 => Some(R),
            115 => Some(S),
            116 => Some(T),
            117 => Some(U),
            118 => Some(V),
            119 => Some(W),
            120 => Some(X),
            121 => Some(Y),
            122 => Some(Z),
            127 => Some(Delete),
            1073741881 => Some(CapsLock),
            1073741882 => Some(F1),
            1073741883 => Some(F2),
            1073741884 => Some(F3),
            1073741885 => Some(F4),
            1073741886 => Some(F5),
            1073741887 => Some(F6),
            1073741888 => Some(F7),
            1073741889 => Some(F8),
            1073741890 => Some(F9),
            1073741891 => Some(F10),
            1073741892 => Some(F11),
            1073741893 => Some(F12),
            1073741894 => Some(PrintScreen),
            1073741895 => Some(ScrollLock),
            1073741896 => Some(Pause),
            1073741897 => Some(Insert),
            1073741898 => Some(Home),
            1073741899 => Some(PageUp),
            1073741901 => Some(End),
            1073741902 => Some(PageDown),
            1073741903 => Some(Right),
            1073741904 => Some(Left),
            1073741905 => Some(Down),
            1073741906 => Some(Up),
            1073741907 => Some(NumLockClear),
            1073741908 => Some(NumPadDivide),
            1073741909 => Some(NumPadMultiply),
            1073741910 => Some(NumPadMinus),
            1073741911 => Some(NumPadPlus),
            1073741912 => Some(NumPadEnter),
            1073741913 => Some(NumPad1),
            1073741914 => Some(NumPad2),
            1073741915 => Some(NumPad3),
            1073741916 => Some(NumPad4),
            1073741917 => Some(NumPad5),
            1073741918 => Some(NumPad6),
            1073741919 => Some(NumPad7),
            1073741920 => Some(NumPad8),
            1073741921 => Some(NumPad9),
            1073741922 => Some(NumPad0),
            1073741923 => Some(NumPadPeriod),
            1073741925 => Some(Application),
            1073741926 => Some(Power),
            1073741927 => Some(NumPadEquals),
            1073741928 => Some(F13),
            1073741929 => Some(F14),
            1073741930 => Some(F15),
            1073741931 => Some(F16),
            1073741932 => Some(F17),
            1073741933 => Some(F18),
            1073741934 => Some(F19),
            1073741935 => Some(F20),
            1073741936 => Some(F21),
            1073741937 => Some(F22),
            1073741938 => Some(F23),
            1073741939 => Some(F24),
            1073741940 => Some(Execute),
            1073741941 => Some(Help),
            1073741942 => Some(Menu),
            1073741943 => Some(Select),
            1073741944 => Some(Stop),
            1073741945 => Some(Again),
            1073741946 => Some(Undo),
            1073741947 => Some(Cut),
            1073741948 => Some(Copy),
            1073741949 => Some(Paste),
            1073741950 => Some(Find),
            1073741951 => Some(Mute),
            1073741952 => Some(VolumeUp),
            1073741953 => Some(VolumeDown),
            1073741957 => Some(NumPadComma),
            1073741958 => Some(NumPadEqualsAS400),
            1073741977 => Some(AltErase),
            1073741978 => Some(Sysreq),
            1073741979 => Some(Cancel),
            1073741980 => Some(Clear),
            1073741981 => Some(Prior),
            1073741982 => Some(Return2),
            1073741983 => Some(Separator),
            1073741984 => Some(Out),
            1073741985 => Some(Oper),
            1073741986 => Some(ClearAgain),
            1073741987 => Some(CrSel),
            1073741988 => Some(ExSel),
            1073742000 => Some(NumPad00),
            1073742001 => Some(NumPad000),
            1073742002 => Some(ThousandsSeparator),
            1073742003 => Some(DecimalSeparator),
            1073742004 => Some(CurrencyUnit),
            1073742005 => Some(CurrencySubUnit),
            1073742006 => Some(NumPadLeftParen),
            1073742007 => Some(NumPadRightParen),
            1073742008 => Some(NumPadLeftBrace),
            1073742009 => Some(NumPadRightBrace),
            1073742010 => Some(NumPadTab),
            1073742011 => Some(NumPadBackspace),
            1073742012 => Some(NumPadA),
            1073742013 => Some(NumPadB),
            1073742014 => Some(NumPadC),
            1073742015 => Some(NumPadD),
            1073742016 => Some(NumPadE),
            1073742017 => Some(NumPadF),
            1073742018 => Some(NumPadXor),
            1073742019 => Some(NumPadPower),
            1073742020 => Some(NumPadPercent),
            1073742021 => Some(NumPadLess),
            1073742022 => Some(NumPadGreater),
            1073742023 => Some(NumPadAmpersand),
            1073742024 => Some(NumPadDblAmpersand),
            1073742025 => Some(NumPadVerticalBar),
            1073742026 => Some(NumPadDblVerticalBar),
            1073742027 => Some(NumPadColon),
            1073742028 => Some(NumPadHash),
            1073742029 => Some(NumPadSpace),
            1073742030 => Some(NumPadAt),
            1073742031 => Some(NumPadExclam),
            1073742032 => Some(NumPadMemStore),
            1073742033 => Some(NumPadMemRecall),
            1073742034 => Some(NumPadMemClear),
            1073742035 => Some(NumPadMemAdd),
            1073742036 => Some(NumPadMemSubtract),
            1073742037 => Some(NumPadMemMultiply),
            1073742038 => Some(NumPadMemDivide),
            1073742039 => Some(NumPadPlusMinus),
            1073742040 => Some(NumPadClear),
            1073742041 => Some(NumPadClearEntry),
            1073742042 => Some(NumPadBinary),
            1073742043 => Some(NumPadOctal),
            1073742044 => Some(NumPadDecimal),
            1073742045 => Some(NumPadHexadecimal),
            1073742048 => Some(LCtrl),
            1073742049 => Some(LShift),
            1073742050 => Some(LAlt),
            1073742051 => Some(LGui),
            1073742052 => Some(RCtrl),
            1073742053 => Some(RShift),
            1073742054 => Some(RAlt),
            1073742055 => Some(RGui),
            1073742081 => Some(Mode),
            1073742082 => Some(AudioNext),
            1073742083 => Some(AudioPrev),
            1073742084 => Some(AudioStop),
            1073742085 => Some(AudioPlay),
            1073742086 => Some(AudioMute),
            1073742087 => Some(MediaSelect),
            1073742088 => Some(Www),
            1073742089 => Some(Mail),
            1073742090 => Some(Calculator),
            1073742091 => Some(Computer),
            1073742092 => Some(AcSearch),
            1073742093 => Some(AcHome),
            1073742094 => Some(AcBack),
            1073742095 => Some(AcForward),
            1073742096 => Some(AcStop),
            1073742097 => Some(AcRefresh),
            1073742098 => Some(AcBookmarks),
            1073742099 => Some(BrightnessDown),
            1073742100 => Some(BrightnessUp),
            1073742101 => Some(DisplaySwitch),
            1073742102 => Some(KbdIllumToggle),
            1073742103 => Some(KbdIllumDown),
            1073742104 => Some(KbdIllumUp),
            1073742105 => Some(Eject),
            1073742106 => Some(Sleep),

            _ => Some(Unknown)
        }
    }

    #[inline(always)]
    fn from_i64(n: i64) -> Option<Key> {
        FromPrimitive::from_u64(n as u64)
    }

    #[inline(always)]
    fn from_int(n: int) -> Option<Key> {
        FromPrimitive::from_u64(n as u64)
    }
}


