use {
    crate::Error,
    time::{macros::format_description, Date},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Id {
    pub kind: Kind,
    pub id: u64,
    pub created_on: Date,
    pub edited_on: Option<Date>,
    pub consumed_on: Option<Date>,
    pub consumes: Vec<u64>,
    pub tags: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Active,
    Archived,
    Consumed,
}

impl Kind {
    pub fn parse(s: &str) -> Result<Self, Error> {
        match s {
            "a" => Ok(Kind::Active),
            "arch" => Ok(Kind::Archived),
            "c" => Ok(Kind::Consumed),
            _ => Err(Error::new_parse_error("invalid prefix (must be a or c)")),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Kind::Active => "a".to_string(),
            Kind::Archived => "arch".to_string(),
            Kind::Consumed => "c".to_string(),
        }
    }
}

impl Id {
    /// parse a string of the form:
    /// a,001234,2020-01-21,e2025-02-08,c000234,my,example,tags
    pub fn parse(s: &str) -> Result<Self, Error> {
        let mut parts = s.split(',');
        // first take the prefix
        let kind = parts
            .next()
            .ok_or(Error::new_parse_error("missing prefix"))?;
        let kind = Kind::parse(kind)?;

        // next take the id parse the id
        let id = match parts.next() {
            None => return Err(Error::new_parse_error("missing id")),
            Some(s) => {
                let Ok(id) = s.parse::<u64>() else {
                    return Err(Error::new_parse_error("invalid id"));
                };
                id
            }
        };

        // next take the creation date
        let created_on = match parts.next() {
            None => return Err(Error::new_parse_error("missing creation date")),
            Some(s) => {
                let format = format_description!("[year]-[month]-[day]");
                let Ok(dt) = Date::parse(&s, &format) else {
                    return Err(Error::new_parse_error("invalid creation date parse"));
                };
                dt
            }
        };

        // next take the last edited date
        let edited_on = match parts.next() {
            None => None,
            Some(s) => {
                if s.chars().next().unwrap() == 'e' {
                    let format = format_description!("[year]-[month]-[day]");
                    let Ok(dt) = Date::parse(&s[1..], &format) else {
                        return Err(Error::new_parse_error("invalid last edited date parse"));
                    };
                    Some(dt)
                } else {
                    None
                }
            }
        };

        // next take the consumed date
        let consumed_on = match parts.next() {
            None => None,
            Some(s) => {
                if s.chars().next().unwrap() == 'c' {
                    let format = format_description!("[year]-[month]-[day]");
                    let Ok(dt) = Date::parse(&s[1..], &format) else {
                        return Err(Error::new_parse_error("invalid consumed date parse"));
                    };
                    Some(dt)
                } else {
                    None
                }
            }
        };

        // next get any consumes ids
        let mut consumes = Vec::new();
        while let Some(s) = parts.next() {
            if s.chars().next().unwrap() == 'c' {
                let Ok(id) = s[1..].parse::<u64>() else {
                    break;
                };
                consumes.push(id);
            } else {
                break;
            }
        }

        // all remaining are tags
        let mut tags = Vec::new();
        while let Some(s) = parts.next() {
            tags.push(s.to_string());
        }

        Ok(Self {
            kind,
            id,
            created_on,
            edited_on,
            consumed_on,
            consumes,
            tags,
        })
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&self.kind.to_string());
        s.push(',');
        s.push_str(&self.id.to_string());
        s.push(',');
        let date_format = format_description!("[year]-[month]-[day]");
        let created_on_str = &self.created_on.format(date_format).unwrap().to_string();
        s.push_str(created_on_str);
        if let Some(edited_on) = &self.edited_on {
            s.push_str(",e");
            let edited_on_str = edited_on.format(date_format).unwrap().to_string();
            s.push_str(&edited_on_str);
        }
        if let Some(consumed_on) = &self.consumed_on {
            s.push_str(",c");
            let consumed_on_str = consumed_on.format(date_format).unwrap().to_string();
            s.push_str(&consumed_on_str);
        }
        for consume in &self.consumes {
            s.push_str(",c");
            s.push_str(&consume.to_string());
        }
        for tag in &self.tags {
            s.push_str(",");
            s.push_str(&tag);
        }
        write!(f, "{}", s)
    }
}

pub mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse() {
        let id = Id::parse("a,001234,2020-01-21,e2025-02-08,c000234,my,example,tags").unwrap();
        assert_eq!(id.kind, Kind::Active);
        assert_eq!(id.id, 1234);
        assert_eq!(
            id.created_on,
            Date::from_calendar_date(2020, 1.try_into().unwrap(), 21).unwrap()
        );
        assert_eq!(
            id.edited_on,
            Some(Date::from_calendar_date(2025, 2.try_into().unwrap(), 8).unwrap())
        );
        assert_eq!(
            id.consumed_on,
            Some(Date::from_calendar_date(2000, 2.try_into().unwrap(), 34).unwrap())
        );
        assert_eq!(id.consumes, vec![234]);
        assert_eq!(id.tags, vec!["my".to_string(), "example".to_string()]);
    }

    #[test]
    fn test_display() {
        let id = Id {
            kind: Kind::Active,
            id: 1234,
            created_on: Date::from_calendar_date(2020, 1.try_into().unwrap(), 21).unwrap(),
            edited_on: Some(Date::from_calendar_date(2025, 2.try_into().unwrap(), 8).unwrap()),
            consumed_on: Some(Date::from_calendar_date(2000, 2.try_into().unwrap(), 34).unwrap()),
            consumes: vec![234],
            tags: vec!["my".to_string(), "example".to_string()],
        };
        assert_eq!(
            id.to_string(),
            "a,001234,2020-01-21,e2025-02-08,c000234,my,example,tags"
        );
    }
}
