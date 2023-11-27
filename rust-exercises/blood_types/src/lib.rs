use std::cmp::{Ord, Ordering};
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(fmt::Error),
        }
    }
}

impl FromStr for RhFactor {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(fmt::Error),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        let antigen_cmp = self.antigen.cmp(&other.antigen);
        if antigen_cmp == Ordering::Equal {
            return self.rh_factor.cmp(&other.rh_factor);
        }
        antigen_cmp
    }
}

impl FromStr for BloodType {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen_str = &s[0..s.len() - 1];
        let rh_str = &s[s.len() - 1..];

        let antigen = antigen_str.parse()?;
        let rh_factor = rh_str.parse()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl std::fmt::Debug for BloodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            format!("{:?}", self.antigen),
            if self.rh_factor == RhFactor::Positive {
                "+"
            } else {
                "-"
            }
        )
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Positive {
            return false;
        }

        match (&self.antigen, &other.antigen) {
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::AB, _) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();

        for antigen in &[Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for rh_factor in &[RhFactor::Positive, RhFactor::Negative] {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if self.can_receive_from(&donor) {
                    donors.push(donor);
                }
            }
        }

        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();

        for antigen in &[Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for rh_factor in &[RhFactor::Positive, RhFactor::Negative] {
                let recipient = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if recipient.can_receive_from(&self) {
                    recipients.push(recipient);
                }
            }
        }

        recipients
    }
}
