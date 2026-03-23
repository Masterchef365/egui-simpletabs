//! Metric prefix unilities

/// Prefixes which will be displayed
const PREFIXES: [&'static str; 17] = [
    "y", "z", "a", "f", "p", "n", "μ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];

/// Prefixes which can be entered and are equivalent (easier to type)
const CRUDE_PREFIXES: [&'static str; 17] = [
    "y", "z", "a", "f", "p", "n", "u", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];

/// Calculate the exponent of the first prefix
const fn first_prefix_exp() -> i32 {
    -3 * (PREFIXES.len() as i32 - 1) / 2
}

/// Display the given value with the given unit, appropriately prefixed according to magnitude
pub fn to_metric_prefix(value: f64, unit: impl std::fmt::Display) -> String {
    let exp10 = (value.abs().log10() / 3.0).floor() as i32;

    let exponent = exp10 * 3;

    let idx = (exponent - first_prefix_exp()) / 3;

    let prefix = (idx >= 0)
        .then(|| idx as usize)
        .and_then(|i| PREFIXES.get(i));

    if let Some(prefix) = prefix {
        format!("{:.3} {prefix}{unit}", value / 10_f64.powi(exponent))
    } else {
        format!("{:.3} {unit}", value) // Fallback in case exponent is out of range
    }
}

/// Extracts the float value for the string (using the given unit)
pub fn from_metric_prefix<'s>(s: &'s str, unit: &str) -> Result<f64, ()> {
    let mut s = s.trim();

    if s.ends_with(unit) {
        s = &s[0..s.len() - unit.len()];
    }

    let value_end = s
        .chars()
        .position(|c| !(c.is_digit(10) || ['.', 'e', '+', '-'].contains(&c)))
        .unwrap_or_else(|| s.chars().count());

    let (value_part, prefix_part) = s.split_at(value_end);
    let prefix = prefix_part.trim();

    let value: f64 = value_part.parse().map_err(|_| ())?;

    let find_prefix = |p: &&str| p == &prefix;
    let prefix_idx = PREFIXES
        .iter()
        .position(find_prefix)
        .or_else(|| CRUDE_PREFIXES.iter().position(find_prefix));

    let exponent = match prefix_idx {
        Some(idx) => first_prefix_exp() + idx as i32 * 3,
        None => 0,
    };
    let value = value * 10_f64.powi(exponent);

    Ok(value)
}

/// Adds parsers and formatters to a DragValue for the metric prefix
pub fn metric_prefix_dragvalue<'a>(
    drag: egui::DragValue<'a>,
    unit: &'static str,
) -> egui::DragValue<'a> {
    drag.custom_parser(move |s| from_metric_prefix(s, unit).ok())
        .custom_formatter(move |value, _| to_metric_prefix(value, unit))
}

/// Shorthand for metric_prefix_dragvalue() with the default DragValue
pub fn edit_metric_f64<'v>(value: &'v mut f64, unit: &'static str) -> egui::DragValue<'v> {
    let speed = *value / 1000.0;
    metric_prefix_dragvalue(egui::DragValue::new(value).speed(speed), unit)
}

#[test]
fn test_to_metric_prefix() {
    assert_eq!(to_metric_prefix(1.000, "g"), "1.000 g");
    assert_eq!(to_metric_prefix(10.0, "s"), "10.000 s");
    assert_eq!(to_metric_prefix(1100.0, "N"), "1.100 kN");
    assert_eq!(to_metric_prefix(1000.0, 'V'), "1.000 kV");
    assert_eq!(to_metric_prefix(0.001, "Ohm"), "1.000 mOhm");
    assert_eq!(to_metric_prefix(1.000, "m"), "1.000 m");
    assert_eq!(to_metric_prefix(0.001, "m"), "1.000 mm");
}

#[test]
fn test_from_metric_prefix() {
    assert_eq!(from_metric_prefix("1 kV", "V").unwrap(), 1000.0);
    assert_eq!(from_metric_prefix("1 mOhm", "Ohm").unwrap(), 0.001);
    assert_eq!(from_metric_prefix("1Rad", "Rad").unwrap(), 1.0);
    assert_eq!(from_metric_prefix("1m", "J").unwrap(), 0.001);
    assert_eq!(from_metric_prefix("1kJ", "J").unwrap(), 1000.0);
    assert_eq!(from_metric_prefix("1uJ", "J").unwrap(), 1e-6);
    assert_eq!(from_metric_prefix("1μJ", "J").unwrap(), 1e-6);
    assert_eq!(from_metric_prefix("1mm", "m").unwrap(), 1e-3);
}
