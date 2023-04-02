use std::str::FromStr;

// the ordering here is just the same order from https://www.wwpdb.org/documentation/file-format-content/format33/sect9.html
#[derive(Debug, PartialEq)]
pub enum CoordinateEnum {
    MODEL,   // start of the model!
    ATOM,    // regular atom
    ANISOU,  // anisotropic temperature factors.
    TER,     // termination of residue
    HETATM,  // atom not in a residue
    ENDMDL,  // end of the model
    UNKNOWN, // default value.
}

pub struct CoordinateRecord {
    record_type: CoordinateEnum, // 1 to 6
    serial: u32,                 // 7 to 11
    name: String,                // 13 to 16
    altLoc: String,              // 17
    resName: String,             // 18 to 20
    chainId: String,             // 22
    resSeq: u32,                 // 23 to 26
    iCode: String,               // 27
    x: f64,                      // 31 to 38
    y: f64,                      // 39 to 46
    z: f64,                      // 47 to 54
    occupancy: f64,              // 55 to 60
    tempFactor: f64,             // 61 to 66
    element: String,             // 77 to 78
    charge: String,              // 79 to 80
    u_0_0: String,               // 29 to 35, U(1,1)
    u_1_1: String,               // 36 to 42, U(2,2)
    u_2_2: String,               // 43 to 49, U(3,3)
    u_0_1: String,               // 50 to 56, U(1,2)
    u_0_2: String,               // 57 to 63, U(1,3)
    u_1_2: String,               // 64 to 70, U(2,3)
}

pub struct CoordinateRecordBuilder<'a> {
    line: &'a str,
    record_type: CoordinateEnum, // 1 to 6
    serial: Option<u32>,         // 7 to 11
    name: Option<String>,        // 13 to 16
    altLoc: Option<String>,      // 17
    resName: Option<String>,     // 18 to 20
    chainId: Option<String>,     // 22
    resSeq: Option<u32>,         // 23 to 26
    iCode: Option<String>,       // 27
    x: Option<f64>,              // 31 to 38
    y: Option<f64>,              // 39 to 46
    z: Option<f64>,              // 47 to 54
    occupancy: Option<f64>,      // 55 to 60
    tempFactor: Option<f64>,     // 61 to 66
    element: Option<String>,     // 77 to 78
    charge: Option<String>,      // 79 to 80
    u_0_0: Option<String>,       // 29 to 35, U(1,1)
    u_1_1: Option<String>,       // 36 to 42, U(2,2)
    u_2_2: Option<String>,       // 43 to 49, U(3,3)
    u_0_1: Option<String>,       // 50 to 56, U(1,2)
    u_0_2: Option<String>,       // 57 to 63, U(1,3)
    u_1_2: Option<String>,       // 64 to 70, U(2,3)
}

impl<'a> CoordinateRecordBuilder<'a> {
    pub fn new(coord: CoordinateEnum, line: &'a str) -> Self {
        Self {
            line: line,
            record_type: coord,
            serial: None,     // 7 to 11
            name: None,       // 13 to 16
            altLoc: None,     // 17
            resName: None,    // 18 to 20
            chainId: None,    // 22
            resSeq: None,     // 23 to 26
            iCode: None,      // 27
            x: None,          // 31 to 38
            y: None,          // 39 to 46
            z: None,          // 47 to 54
            occupancy: None,  // 55 to 60
            tempFactor: None, // 61 to 66
            element: None,    // 77 to 78
            charge: None,     // 79 to 80
            u_0_0: None,      // 29 to 35, U(1,1)
            u_1_1: None,      // 36 to 42, U(2,2)
            u_2_2: None,      // 43 to 49, U(3,3)
            u_0_1: None,      // 50 to 56, U(1,2)
            u_0_2: None,      // 57 to 63, U(1,3)
            u_1_2: None,      // 64 to 70, U(2,3)
        }
    }

    pub fn serial(mut self) -> Self {
        self.serial = Some(u32::from_str(&self.line[10..14]).unwrap());
        self
    }

    pub fn name(mut self) -> Self {
        self.name = Some(self.line[12..16].to_string());
        self
    }

    pub fn altLoc(mut self) -> Self {
        self.altLoc = Some(self.line.chars().nth(16).unwrap().to_string());
        self
    }

    pub fn resName(mut self) -> Self {
        self.resName = Some(self.line[17..20].to_string());
        self
    }

    pub fn chainId(mut self) -> Self {
        self.chainId = Some(self.line.chars().nth(22).unwrap().to_string());
        self
    }

    pub fn resSeq(mut self) -> Self {
        self.resSeq = Some(u32::from_str(&self.line[22..26]).unwrap());
        self
    }

    pub fn iCode(mut self) -> Self {
        self.iCode = Some(self.line.chars().nth(26).unwrap().to_string());
        self
    }

    pub fn x(mut self) -> Self {
        self.x = Some(f64::from_str(&self.line[30..38]).unwrap());
        self
    }

    pub fn y(mut self) -> Self {
        self.y = Some(f64::from_str(&self.line[38..46]).unwrap());
        self
    }

    pub fn z(mut self) -> Self {
        self.z = Some(f64::from_str(&self.line[46..54]).unwrap());
        self
    }

    pub fn occupancy(mut self) -> Self {
        self.occupancy = Some(f64::from_str(&self.line[54..60]).unwrap());
        self
    }

    pub fn tempFactor(mut self) -> Self {
        self.tempFactor = Some(f64::from_str(&self.line[60..66]).unwrap());
        self
    }

    pub fn element(mut self) -> Self {
        self.element = Some(self.line[76..78].to_string());
        self
    }

    pub fn charge(mut self) -> Self {
        self.charge = Some(self.line[78..80].to_string());
        self
    }

    pub fn u_0_0(mut self) -> Self {
        self.u_0_0 = Some(self.line[28..35].to_string());
        self
    }

    pub fn u_1_1(mut self) -> Self {
        self.u_1_1 = Some(self.line[35..42].to_string());
        self
    }

    pub fn u_2_2(mut self) -> Self {
        self.u_2_2 = Some(self.line[42..49].to_string());
        self
    }

    pub fn u_0_1(mut self) -> Self {
        self.u_0_1 = Some(self.line[49..56].to_string());
        self
    }

    pub fn u_0_2(mut self) -> Self {
        self.u_0_2 = Some(self.line[56..63].to_string());
        self
    }

    pub fn u_1_2(mut self) -> Self {
        self.u_1_2 = Some(self.line[63..70].to_string());
        self
    }

    pub fn build(self) -> CoordinateRecord {
        CoordinateRecord {
            record_type: self.record_type,
            serial: self.serial.unwrap_or_else(|| 0),
            name: self.name.unwrap_or_else(|| "".to_string()),
            altLoc: self.altLoc.unwrap_or_else(|| "".to_string()),
            resName: self.resName.unwrap_or_else(|| "".to_string()),
            chainId: self.chainId.unwrap_or_else(|| "".to_string()),
            resSeq: self.resSeq.unwrap_or_else(|| 0),
            iCode: self.iCode.unwrap_or_else(|| "".to_string()),
            x: self.x.unwrap_or_else(|| 0.0),
            y: self.y.unwrap_or_else(|| 0.0),
            z: self.z.unwrap_or_else(|| 0.0),
            occupancy: self.occupancy.unwrap_or_else(|| 0.0),
            tempFactor: self.tempFactor.unwrap_or_else(|| 0.0),
            element: self.element.unwrap_or_else(|| "".to_string()),
            charge: self.charge.unwrap_or_else(|| "".to_string()),
            u_0_0: self.u_0_0.unwrap_or_else(|| "".to_string()),
            u_1_1: self.u_1_1.unwrap_or_else(|| "".to_string()),
            u_2_2: self.u_2_2.unwrap_or_else(|| "".to_string()),
            u_0_1: self.u_0_1.unwrap_or_else(|| "".to_string()),
            u_0_2: self.u_0_2.unwrap_or_else(|| "".to_string()),
            u_1_2: self.u_1_2.unwrap_or_else(|| "".to_string()),
        }
    }
}
impl FromStr for CoordinateRecord {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match line[0..5].as_ref() {
            "MODEL" => Ok(createModel(line)),
            "ATOM" => Ok(createAtom(line)),
            "ANISOU" => Ok(createAnisou(line)),
            "TER" => Ok(createTer(line)),
            "HETATM" => Ok(createHetatm(line)),
            "ENDMDL" => Ok(createEndmdl(line)),
            _ => Err(()),
        }
    }
}

fn createModel(line: &str) -> CoordinateRecord {
    CoordinateRecordBuilder::new(CoordinateEnum::MODEL, line)
        .serial()
        .build()
}

fn createAtom(line: &str) -> CoordinateRecord {
    CoordinateRecordBuilder::new(CoordinateEnum::ATOM, line)
        .serial()
        .name()
        .altLoc()
        .resName()
        .chainId()
        .resSeq()
        .iCode()
        .x()
        .y()
        .z()
        .occupancy()
        .tempFactor()
        .element()
        .charge()
        .build()
}

// https://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU
// The anisotropic temperature factors (columns 29 - 70) are scaled by a factor of 10**4 (Angstroms**2) and are presented as integers.
// The anisotropic temperature factors are stored in the same coordinate frame as the atomic coordinate records.
// ANISOU values are listed only if they have been provided by the depositor.
// the order would go ATOM, ANISOU, ATOM, ANISOU, etc.

fn createAnisou(line: &str) -> CoordinateRecord {
    CoordinateRecordBuilder::new(CoordinateEnum::ANISOU, line)
        .serial()
        .name()
        .altLoc()
        .resName()
        .chainId()
        .resSeq()
        .iCode()
        .u_0_0()
        .u_1_1()
        .u_2_2()
        .u_0_1()
        .u_0_2()
        .u_1_2()
        .element()
        .charge()
        .build()
}

fn createTer(line: &str) -> CoordinateRecord {
    CoordinateRecordBuilder::new(CoordinateEnum::TER, line)
        .serial()
        .resName()
        .chainId()
        .resSeq()
        .iCode()
        .build()
}

fn createHetatm(line: &str) -> CoordinateRecord {
    CoordinateRecordBuilder::new(CoordinateEnum::HETATM, line)
        .serial()
        .name()
        .altLoc()
        .resName()
        .chainId()
        .resSeq()
        .iCode()
        .x()
        .y()
        .z()
        .occupancy()
        .tempFactor()
        .element()
        .charge()
        .build()
}

fn createEndmdl(line: &str) -> CoordinateRecord {
    CoordinateRecordBuilder::new(CoordinateEnum::ENDMDL, line).build()
}
