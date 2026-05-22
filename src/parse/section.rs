use crate::parse::{date::DataFormat, extract::ExtractMethod};

pub enum ParseSection {
    AreaOfInterest,
    Title {
        extract_method: ExtractMethod,
    },
    Link {
        extract_method: ExtractMethod,
    },
    Date {
        extract_method: ExtractMethod,
        date_format: DataFormat,
    },
    Author {
        extract_method: ExtractMethod,
    },
    Summary {
        extract_method: ExtractMethod,
    },
}
