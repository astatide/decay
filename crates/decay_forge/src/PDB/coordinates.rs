use std::str::FromStr;

// the ordering here is just the same order from https://www.wwpdb.org/documentation/file-format-content/format33/sect9.html
#[derive(Debug, PartialEq)]
pub enum CoordinateEnum {
    MODEL,  // start of the model!
    ATOM,   // regular atom
    ANISOU, // anisotropic temperature factors.
    TER,    // termination of residue
    HETATM, // atom not in a residue
    ENDMDL, // end of the model
}

pub struct CoordinateRecord {
    record_type: CoordinateEnum, // 1 to 6
    serial: Option<u32>,         // 7 to 11
    name: Option<String>,        // 13 to 16
    altLoc: Option<String>,      // 17
    resName: Option<String>,     // 18 to 20
    chainID: Option<String>,     // 22
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

impl FromStr for CoordinateRecord {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match line[0..5].as_ref() {
            "MODEL" => createModel(line),
            "ATOM" => createAtom(line),
            "ANISOU" => createAnisou(line),
            "TER" => createTer(line),
            "HETATM" => createHetatm(line),
            "ENDMDL" => createEndmdl(line),
            _ => Err(()),
        }
    }
}

type Err = ();

fn getSerial(line: &str) -> u32 {
    u32::from_str(&line[10..14]).unwrap()
}

fn getName(line: &str) -> String {
    line[12..16].to_string()
}

fn getAltLoc(line: &str) -> String {
    line.chars().nth(16).unwrap().to_string()
}

fn getResName(line: &str) -> String {
    line[17..20].to_string()
}

fn getChainId(line: &str) -> String {
    line.chars().nth(22).unwrap().to_string()
}

fn getResSeq(line: &str) -> String {
    line[22..26].to_string()
}

fn getICode(line: &str) -> String {
    line.chars().nth(26).unwrap().to_string()
}

fn getX(line: &str) -> f64 {
    f64::from_str(&line[30..38]).unwrap()
}

fn getY(line: &str) -> f64 {
    f64::from_str(&line[38..46]).unwrap()
}

fn getZ(line: &str) -> f64 {
    f64::from_str(&line[46..54]).unwrap()
}

fn getOccupancy(line: &str) -> f64 {
    f64::from_str(&line[54..60]).unwrap()
}

fn getTempFactor(line: &str) -> f64 {
    f64::from_str(&line[60..66]).unwrap()
}

fn getElement(line: &str) -> String {
    line[76..78].to_string()
}

fn getCharge(line: &str) -> String {
    line[78..80].to_string()
}

fn getU_0_0(line: &str) -> String {
    line[28..35].to_string()
}

fn getU_1_1(line: &str) -> String {
    line[35..42].to_string()
}

fn getU_2_2(line: &str) -> String {
    line[42..49].to_string()
}

fn getU_0_1(line: &str) -> String {
    line[49..56].to_string()
}

fn getU_0_2(line: &str) -> String {
    line[56..63].to_string()
}

fn getU_1_2(line: &str) -> String {
    line[63..70].to_string()
}

fn createModel(line: &str) -> Result<CoordinateRecord, Err> {
    return Ok(CoordinateRecord {
        record_type: CoordinateEnum::MODEL,
        serial: Some(getSerial(line)),
        name: None,
        altLoc: None,
        resName: None,
        chainID: None,
        resSeq: None,
        iCode: None,
        x: None,
        y: None,
        z: None,
        occupancy: None,
        tempFactor: None,
        element: None,
        charge: None,
        u_0_0: None,
        u_1_1: None,
        u_2_2: None,
        u_0_1: None,
        u_0_2: None,
        u_1_2: None,
    });
}

fn createAtom(line: &str) -> Result<CoordinateRecord, Err> {
    return Ok(CoordinateRecord {
        record_type: CoordinateEnum::ATOM,
        serial: Some(getSerial(line)),
        name: None,
        altLoc: None,
        resName: None,
        chainID: None,
        resSeq: None,
        iCode: None,
        x: None,
        y: None,
        z: None,
        occupancy: None,
        tempFactor: None,
        element: None,
        charge: None,
        u_0_0: None,
        u_1_1: None,
        u_2_2: None,
        u_0_1: None,
        u_0_2: None,
        u_1_2: None,
    });
}

// https://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU
// The anisotropic temperature factors (columns 29 - 70) are scaled by a factor of 10**4 (Angstroms**2) and are presented as integers.
// The anisotropic temperature factors are stored in the same coordinate frame as the atomic coordinate records.
// ANISOU values are listed only if they have been provided by the depositor.
// the order would go ATOM, ANISOU, ATOM, ANISOU, etc.

fn createAnisou(line: &str) -> Result<CoordinateRecord, Err> {
    return Ok(CoordinateRecord {
        record_type: CoordinateEnum::ANISOU,
        serial: Some(getSerial(line)),
        name: None,
        altLoc: None,
        resName: None,
        chainID: None,
        resSeq: None,
        iCode: None,
        x: None,
        y: None,
        z: None,
        occupancy: None,
        tempFactor: None,
        element: None,
        charge: None,
        u_0_0: None,
        u_1_1: None,
        u_2_2: None,
        u_0_1: None,
        u_0_2: None,
        u_1_2: None,
    });
}

fn createTer(line: &str) -> Result<CoordinateRecord, Err> {
    return Ok(CoordinateRecord {
        record_type: CoordinateEnum::TER,
        serial: Some(getSerial(line)),
        name: None,
        altLoc: None,
        resName: None,
        chainID: None,
        resSeq: None,
        iCode: None,
        x: None,
        y: None,
        z: None,
        occupancy: None,
        tempFactor: None,
        element: None,
        charge: None,
        u_0_0: None,
        u_1_1: None,
        u_2_2: None,
        u_0_1: None,
        u_0_2: None,
        u_1_2: None,
    });
}

fn createHetatm(line: &str) -> Result<CoordinateRecord, Err> {
    return Ok(CoordinateRecord {
        record_type: CoordinateEnum::HETATM,
        serial: Some(getSerial(line)),
        name: None,
        altLoc: None,
        resName: None,
        chainID: None,
        resSeq: None,
        iCode: None,
        x: None,
        y: None,
        z: None,
        occupancy: None,
        tempFactor: None,
        element: None,
        charge: None,
        u_0_0: None,
        u_1_1: None,
        u_2_2: None,
        u_0_1: None,
        u_0_2: None,
        u_1_2: None,
    });
}

fn createEndmdl(line: &str) -> Result<CoordinateRecord, Err> {
    return Ok(CoordinateRecord {
        record_type: CoordinateEnum::ENDMDL,
        serial: Some(getSerial(line)),
        name: None,
        altLoc: None,
        resName: None,
        chainID: None,
        resSeq: None,
        iCode: None,
        x: None,
        y: None,
        z: None,
        occupancy: None,
        tempFactor: None,
        element: None,
        charge: None,
        u_0_0: None,
        u_1_1: None,
        u_2_2: None,
        u_0_1: None,
        u_0_2: None,
        u_1_2: None,
    });
}
