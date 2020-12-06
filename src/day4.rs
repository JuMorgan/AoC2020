use nom::{
    IResult,
    branch::{alt, permutation},
    bytes::complete::{is_not, tag, take_while_m_n, take_while},
    combinator::map_res,
    sequence::{separated_pair, tuple},
    character::complete::{digit1, char},
  };
/*

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

*/
const BYR : &str = "byr";
const IYR : &str = "iyr";
const EYR : &str = "eyr";
const HGT : &str = "hgt";
const HCL : &str = "hcl";
const ECL : &str = "ecl";
const PID : &str = "pid";
const CID : &str = "cid";

enum PassportField {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId
}

fn remove_trailing_whitespace(input: &str) -> IResult<&str, ()> {
    let (remaining, _) = alt((
        take_while_m_n(1, 1, |x| x == '\n'),
        take_while(|x| x == ' '),
    ))(input)?;

    Ok((remaining, ()))
}

fn parse_height(input: &str) -> IResult<&str, PassportField> {
    let (rem, (_,height)) = separated_pair(tag(HGT), char(':'), digit1)(input)?;
    let (rem, unit) = alt((tag("cm"), tag("in")))(rem)?;
    let h : u32 = height.parse::<u32>().unwrap_or(0);
    
    if (unit == "cm" && 150 <= h && h <= 193) || (unit == "in" && 59 <= h && h <= 76)  {
        Ok((remove_trailing_whitespace(rem)?.0, PassportField::Height))
    } else{
        Err(nom::Err::Error(nom::error::Error::new(HGT, nom::error::ErrorKind::Tag)))
    }
}


fn parse_year(input: &str) -> IResult<&str, u32> {
    let (rem, year) = map_res(digit1, |c : &str| c.parse::<u32>() )(input)?;

    Ok((remove_trailing_whitespace(rem)?.0, year))
}


fn parse_expiration_year(input: &str) -> IResult<&str, PassportField> {
    let (rem, (_,year)) = separated_pair(tag(EYR), char(':'), parse_year)(input)?;

    if 2020 <= year && year <= 2030 {
        Ok((remove_trailing_whitespace(rem)?.0, PassportField::ExpirationYear))
    }
    else {
        Err(nom::Err::Error(nom::error::Error::new(EYR, nom::error::ErrorKind::Tag)))
    }
}

fn parse_issue_year(input: &str) -> IResult<&str, PassportField> {
    let (rem, (_,year)) = separated_pair(tag(IYR), char(':'), parse_year)(input)?;

    if 2010 <= year && year <= 2020 {
        Ok((remove_trailing_whitespace(rem)?.0, PassportField::IssueYear))
    }
    else {
        Err(nom::Err::Error(nom::error::Error::new(IYR, nom::error::ErrorKind::Tag)))
    }
}

fn parse_birth_year(input: &str) -> IResult<&str, PassportField> {
    let (rem, (_,year)) = separated_pair(tag(BYR), char(':'), parse_year)(input)?;

    if 1920 <= year && year <= 2002 {
        Ok((remove_trailing_whitespace(rem)?.0, PassportField::BirthYear))
    }
    else {
        Err(nom::Err::Error(nom::error::Error::new(BYR, nom::error::ErrorKind::Tag)))
    }
}

fn parse_passport_id(input: &str) -> IResult<&str, PassportField> {
    let (rem, (_,pid)) = separated_pair(tag(PID), char(':'), digit1)(input)?;

    if pid.len() == 9 {
        Ok((remove_trailing_whitespace(rem)?.0, PassportField::PassportId))
    }
    else {
        Err(nom::Err::Error(nom::error::Error::new(PID, nom::error::ErrorKind::Tag)))
    } 
}

fn parse_country_id(input: &str) -> IResult<&str, PassportField> {
    if input.is_empty() {
        return Ok((input, PassportField::CountryId));
    }

    let (rem, _) = separated_pair(tag(CID), char(':'), is_not(" \t\r\n"))(input)?;
    Ok((remove_trailing_whitespace(rem)?.0, PassportField::CountryId))
}


fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
  map_res(
    take_while_m_n(2, 2, is_hex_digit),
    from_hex
  )(input)
}

fn parse_hex_color(input: &str) -> IResult<&str, ()> {
  let (input, _) = tag("#")(input)?;
  let (input, _) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

  Ok((input, ()))
}


fn parse_eye_color(input: &str) -> IResult<&str, PassportField> {
    let (rem, _) = separated_pair(tag(ECL), char(':'), alt((tag("amb"),
                                                                     tag("blu"),
                                                                     tag("brn"),
                                                                     tag("gry"),
                                                                     tag("hzl"),
                                                                     tag("grn"),
                                                                     tag("oth"))))(input)?;

    Ok((remove_trailing_whitespace(rem)?.0, PassportField::EyeColor))
}

fn parse_hair_color(input: &str) -> IResult<&str, PassportField> {
    let (rem, _) = separated_pair(tag(HCL), char(':'), parse_hex_color)(input)?;

    Ok((remove_trailing_whitespace(rem)?.0, PassportField::HairColor))
}

fn parse_passport(input: &str) -> IResult<&str, (PassportField, PassportField, PassportField, PassportField, PassportField, PassportField, PassportField, PassportField)> {
    // so this is probably real slow
    let (rem, a) = permutation((parse_eye_color, 
                                parse_expiration_year, 
                                parse_issue_year, 
                                parse_birth_year, 
                                parse_hair_color, 
                                parse_height, 
                                parse_passport_id, 
                                parse_country_id))(input)?;

    Ok((rem, a))
  }

pub fn run(data : &Vec<&str>) -> u32 {
    data.iter().fold(0, |acc, x| {
        if let Ok(_) = parse_passport(x) { acc + 1} else {acc}
    })
}