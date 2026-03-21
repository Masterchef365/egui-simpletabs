use egui::emath::Numeric;

type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<f64>) -> f64>;
fn get(get_set_value: &mut GetSetValue<'_>) -> f64 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: f64) {
    (get_set_value)(Some(value));
}

pub struct Dial<'a> {
    get_set_value: GetSetValue<'a>,
}

impl<'a> Dial<'a> {
     pub fn new<Num: Numeric>(value: &'a mut Num) -> Self {
        Self::from_get_set(move |v: Option<f64>| {
            if let Some(v) = v {
                *value = Num::from_f64(v);
            }
            value.to_f64()
        })

    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<f64>) -> f64) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
        }
    }
}
