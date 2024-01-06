#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum Tz {
    /// Universal Standard Time (+00:00) and Western European Time (+00:00)
    #[default]
    UtcWet = 0,

    /// British Summer Time (+01:00) and Central European Time (+01:00)
    BstCet = 3600,

    /// Central European Summer Time (+02:00) and Eastern European Time (+02:00)
    CestEet = 7200,

    /// Eastern European Summer Time (+03:00) and Arabian Standard Time (+03:00)
    EestAst = 10800,

    /// Indian Standard Time (+05:30)
    Ist = 19800,

    /// Japan Standard Time (+09:00) and Korea Standard Time (+09:00)
    JstKst = 32400,

    /// China Standard Time (+08:00), Australian Western Standard Time (+08:00), Singapore Standard Time (+08:00) and Hong Kong Time (+08:00)
    CstAwstSstHkt = 28800,

    /// Australian Central Standard Time (+09:30)
    Acst = 34200,

    /// Australian Eastern Standard Time (+10:00), Chamorro Standard Time (+10:00)
    AestChst = 36000,

    /// Lord Howe Standard Time (+10:30)
    Lwst = 37800,

    /// New Zealand Standard Time (+12:00) and Fiji Time (+12:00)
    NzstFjt = 43200,

    /// Samoa Standard Time (-11:00)
    Sast = -39600,

    /// Hawaii-Aleutian Standard Time (-10:00)
    Hast = -36000,

    /// Alaska Standard Time (-09:00)
    Alst = -32400,

    /// Pacific Standard Time (-08:00)
    Pst = -28800,

    /// Mountain Standard Time (-07:00)
    Mst = -25200,

    /// Central Standard Time (-06:00)
    Censt = -21600,

    /// Eastern Standard Time (-05:00)
    Est = -18000,

    /// Atlantic Standard Time (-04:00) and Chile Time (-04:00)
    AtstClt = -14400,

    /// Newfoundland Standard Time (-03:30)
    Nst = -12600,

    /// Brazil Time (-03:00), Fernando de Noronha Time (-02:00), Argentina Time (-03:00) and Uruguay Time (-03:00)
    BtAtArtUyt = -10800,

    /// Indochina Time (+07:00) and Western Indonesian Time (+07:00)
    IctWib = 25200,
}

impl core::fmt::Display for Tz {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Tz {
    /// Returns the offset in seconds from UTC.
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::Tz;
    /// println!("{}", Tz::UtcWet.offset()); // 0
    /// println!("{}", Tz::BstCet.offset()); // 3600
    /// println!("{}", Tz::CestEet.offset()); // 7200
    /// ```
    pub fn offset(&self) -> i32 {
        *self as i32
    }

    /// Returns the offset in seconds from UTC as a string.
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::Tz;
    /// println!("{}", Tz::UtcWet.offset_str()); // +00:00
    /// println!("{}", Tz::BstCet.offset_str()); // +01:00
    /// println!("{}", Tz::CestEet.offset_str()); // +02:00
    /// ```
    pub fn offset_str(&self) -> String {
        let offset = self.offset();
        let sign = if offset < 0 { '-' } else { '+' };
        let offset = offset.abs();
        let hours = offset / 3600;
        let minutes = (offset % 3600) / 60;
        format!("{}{:02}:{:02}", sign, hours, minutes)
    }

    /// Returns the name of the timezone.
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::Tz;
    /// println!("{}", Tz::UtcWet.name()); // UTC/WET
    /// println!("{}", Tz::BstCet.name()); // BST/CET
    /// println!("{}", Tz::CestEet.name()); // CEST/EET
    /// ```
    pub fn name(&self) -> String {
        match self {
            Tz::UtcWet => "UTC/WET".to_string(),
            Tz::BstCet => "BST/CET".to_string(),
            Tz::CestEet => "CEST/EET".to_string(),
            Tz::EestAst => "EEST/AST".to_string(),
            Tz::Ist => "IST".to_string(),
            Tz::JstKst => "JST/KST".to_string(),
            Tz::CstAwstSstHkt => "CST/AWST/SST/HKT".to_string(),
            Tz::Acst => "ACST".to_string(),
            Tz::AestChst => "AEST/CHST".to_string(),
            Tz::Lwst => "LWST".to_string(),
            Tz::NzstFjt => "NZST/FJT".to_string(),
            Tz::Sast => "SAST".to_string(),
            Tz::Hast => "HAST".to_string(),
            Tz::Alst => "ALST".to_string(),
            Tz::Pst => "PST".to_string(),
            Tz::Mst => "MST".to_string(),
            Tz::Censt => "CENST".to_string(),
            Tz::Est => "EST".to_string(),
            Tz::AtstClt => "ATST/CLT".to_string(),
            Tz::Nst => "NST".to_string(),
            Tz::BtAtArtUyt => "BT/AT".to_string(),
            Tz::IctWib => "ICT/WIB".to_string(),
        }
    }

    /// Returns the timezone from the name.
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::Tz;
    /// println!("{:?}", Tz::from_name("UTC/WET")); // Some(UtcWet)
    /// println!("{:?}", Tz::from_name("BST/CET")); // Some(BstCet)
    /// println!("{:?}", Tz::from_name("CEST/EET")); // Some(CestEet)
    /// println!("{:?}", Tz::from_name("Life? Don't talk to me about life!")); // None
    /// ```
    pub fn from_name<T: ToString>(name: T) -> Option<Self> {
        match name.to_string().as_str() {
            "UTC/WET" => Some(Tz::UtcWet),
            "BST/CET" => Some(Tz::BstCet),
            "CEST/EET" => Some(Tz::CestEet),
            "EEST/AST" => Some(Tz::EestAst),
            "IST" => Some(Tz::Ist),
            "JST/KST" => Some(Tz::JstKst),
            "CST/AWST/SST/HKT" => Some(Tz::CstAwstSstHkt),
            "ACST" => Some(Tz::Acst),
            "AEST/CHST" => Some(Tz::AestChst),
            "LWST" => Some(Tz::Lwst),
            "NZST/FJT" => Some(Tz::NzstFjt),
            "SAST" => Some(Tz::Sast),
            "HAST" => Some(Tz::Hast),
            "ALST" => Some(Tz::Alst),
            "PST" => Some(Tz::Pst),
            "MST" => Some(Tz::Mst),
            "CENST" => Some(Tz::Censt),
            "EST" => Some(Tz::Est),
            "ATST/CLT" => Some(Tz::AtstClt),
            "NST" => Some(Tz::Nst),
            "BT/AT" => Some(Tz::BtAtArtUyt),
            "ICT/WIB" => Some(Tz::IctWib),
            _ => None,
        }
    }

    /// Returns the timezone from the offset.
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::Tz;
    /// println!("{:?}", Tz::from_offset(0)); // Some(UtcWet)
    /// println!("{:?}", Tz::from_offset(3600)); // Some(BstCet)
    /// println!("{:?}", Tz::from_offset(7200)); // Some(CestEet)
    /// println!("{:?}", Tz::from_offset(123456)); // None
    /// ```
    pub fn from_offset(offset: i32) -> Option<Self> {
        match offset {
            0 => Some(Tz::UtcWet),
            3600 => Some(Tz::BstCet),
            7200 => Some(Tz::CestEet),
            10800 => Some(Tz::EestAst),
            19800 => Some(Tz::Ist),
            32400 => Some(Tz::JstKst),
            28800 => Some(Tz::CstAwstSstHkt),
            34200 => Some(Tz::Acst),
            36000 => Some(Tz::AestChst),
            37800 => Some(Tz::Lwst),
            43200 => Some(Tz::NzstFjt),
            -39600 => Some(Tz::Sast),
            -36000 => Some(Tz::Hast),
            -32400 => Some(Tz::Alst),
            -28800 => Some(Tz::Pst),
            -25200 => Some(Tz::Mst),
            -21600 => Some(Tz::Censt),
            -18000 => Some(Tz::Est),
            -14400 => Some(Tz::AtstClt),
            -12600 => Some(Tz::Nst),
            -10800 => Some(Tz::BtAtArtUyt),
            25200 => Some(Tz::IctWib),
            _ => None,
        }
    }

    pub fn from_offset_str(offset: &str) -> Option<Self> {
        let offset = offset.split(":").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>();
        Self::from_offset((offset[0] * 3600) + (offset[1] * 60))
    }

    /// Offsets the provided struct by the timezone.
    /// 
    /// # Examples
    /// ```rust
    /// use thetime::{Time, System, Tz};
    /// println!("{:?}", Tz::Acst.offset_struct(System::now()));
    /// ```
    pub fn offset_struct<T: crate::Time>(&self, time: T) -> T {
        time.change_tz(self.offset_str())
    }
}