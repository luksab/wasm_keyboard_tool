use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, KeyEvent};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{collections::HashSet, fmt, str::FromStr};
use strum::{EnumCount, VariantNames};
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumString, EnumVariantNames};

use defines::*;

// [L/R] [Pinkie, Ring, Middle, Index, thumbL, thumbU, thumbD]
#[derive(
    EnumString,
    Display,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    TryFromPrimitive,
    EnumVariantNames,
    EnumCountMacro,
)]
#[repr(u16)]
#[wasm_bindgen]
pub enum Finger {
    LP = 0b0100000000000000,
    LR = 0b0010000000000000,
    LM = 0b0001000000000000,
    LI = 0b0000100000000000,
    LU = 0b0000010000000000,
    LD = 0b0000001000000000,
    LL = 0b0000000100000000,

    RP = 0b0000000001000000,
    RR = 0b0000000000100000,
    RM = 0b0000000000010000,
    RI = 0b0000000000001000,
    RU = 0b0000000000000100,
    RD = 0b0000000000000010,
    RL = 0b0000000000000001,
}

// #[wasm_bindgen]
// impl Finger {
//     pub fn from_strinsg(s: &str) -> Finger {
//         match s {
//             "LP" => Finger::LP,
//             "LR" => Finger::LR,
//             "LM" => Finger::LM,
//             "LI" => Finger::LI,
//             "RI" => Finger::RI,
//             "RM" => Finger::RM,
//             "RR" => Finger::RR,
//             "RP" => Finger::RP,
//             "LU" => Finger::LU,
//             "LD" => Finger::LD,
//             "LL" => Finger::LL,
//             "RU" => Finger::RU,
//             "RD" => Finger::RD,
//             "RL" => Finger::RL,
//             _ => panic!("Invalid finger name"),
//         }
//     }
// }

impl fmt::Debug for Finger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[wasm_bindgen]
struct Combo {
    fingers: Vec<Finger>,
    key: String,
}

impl fmt::Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, finger) in self.fingers.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", finger)?;
            } else {
                write!(f, " {}", finger)?;
            }
        }
        write!(f, " {}", self.key)
    }
}

// implement ord based on the number of fingers
impl Ord for Combo {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.fingers.len().cmp(&other.fingers.len())
    }
}

impl PartialOrd for Combo {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Config {
    combos: Vec<Combo>,
}

#[wasm_bindgen]
impl Config {
    /// format: `([L/R][Pinkie, Ring, Middle, Index, thumbL, thumbU, thumbD]+ [key]\n)+`
    pub fn from_str(config: &str) -> Result<Config, String> {
        let mut combos = Vec::new();
        let mut lines = config.lines();
        let mut line_num = 0;
        while let Some(line) = lines.next() {
            line_num += 1;
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let mut words = line.split_whitespace();
            let mut fingers = Vec::new();
            let mut key = None;
            while let Some(finger) = words.next() {
                let finger = match Finger::from_str(finger.to_uppercase().as_str()) {
                    Ok(f) => f,
                    Err(_) => {
                        // return Err(format!(
                        //     "Invalid finger name at line {}: {}",
                        //     line_num, finger
                        // ))
                        key = Some(finger.to_uppercase());
                        break;
                    }
                };
                fingers.push(finger);
            }
            if fingers.len() == 0 {
                return Err(format!("Invalid finger on line {}", line_num));
            }
            let key = key.ok_or(format!("Missing Key: line {}", line_num))?;
            // if key.len() != 1 {
            //     return Err(format!(
            //         "Key on line {} must be a single character: {}",
            //         line_num, key
            //     ));
            // }
            combos.push(Combo {
                fingers,
                key: key.to_string(),
            });
        }
        Ok(Config { combos })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // don't put a newline at the last line
        for (i, combo) in self.combos.iter().enumerate() {
            if i == self.combos.len() - 1 {
                write!(f, "{}", combo)?;
            } else {
                write!(f, "{}\n", combo)?;
            }
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Config {
    fn check_dup(&self) -> Result<(), String> {
        let mut finger_set = HashSet::new();
        for combo in &self.combos {
            if !finger_set.insert(combo.fingers.clone()) {
                return Err(format!("Duplicate combo: {}", combo));
            }
        }
        Ok(())
    }

    fn check_dup_in_combo(&self) -> Result<(), String> {
        for combo in &self.combos {
            let mut finger_set = HashSet::new();
            for finger in &combo.fingers {
                if !finger_set.insert(finger.clone()) {
                    return Err(format!("Duplicate finger in combo: {}", combo));
                }
            }
        }
        Ok(())
    }

    fn check_all_single(&self) -> Result<(), String> {
        match self.get_finger_lookup() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn check(&self) -> Result<(), String> {
        self.check_dup()?;
        self.check_dup_in_combo()?;
        // self.check_all_single()?;
        Ok(())
    }
}

#[wasm_bindgen]
impl Config {
    /// get what pressing each finger will do
    pub fn get_key_with_fingers(&self, fingers: Array) -> JsValue {
        let fingers = fingers
            .iter()
            .map(|f| f.as_string().unwrap_or_default())
            .map(|f| Finger::from_str(f.to_uppercase().as_str()).unwrap_or(Finger::LP))
            .collect::<Vec<Finger>>();
        // let str = format!("{:?}", fingers);
        // console::log_1(&str.into());
        let values = self.get_key_with_fingers_rs(fingers);
        return JsValue::from(
            values
                .into_iter()
                .map(|x| JsValue::from_str(&x))
                .collect::<Array>(),
        );
    }

    /// gets called by `get_key_with_fingers`
    fn get_key_with_fingers_rs(&self, fingers: Vec<Finger>) -> Vec<String> {
        let mut key_with_finger = vec!["".to_string(); 15];
        for combo in &self.combos {
            if fingers.iter().all(|f| combo.fingers.contains(f)) {
                let leftover = combo
                    .fingers
                    .iter()
                    .filter(|f| !fingers.contains(f))
                    .collect::<Vec<&Finger>>();

                if leftover.len() == 0 {
                    key_with_finger[Finger::COUNT] = combo.key.clone();
                    // console::log_2(&"This outputs:".into(), &combo.key.clone().into());
                } else if leftover.len() == 1 {
                    key_with_finger[*leftover[0] as usize] = combo.key.clone();
                } else {
                    // for finger in leftover {
                    //     key_with_finger[*finger as usize] = combo.key.clone();
                    // }
                    // key_with_finger = key_with_finger_tmp;
                }
                // key_with_finger.push(combo.key.clone());
            }
        }
        key_with_finger
    }
}

#[wasm_bindgen]
impl Config {
    fn get_option_finger_lookup(&self) -> Result<Vec<Option<String>>, String> {
        let mut single_key_lookup = Finger::VARIANTS.iter().map(|_| None).collect::<Vec<_>>();

        for combo in &self.combos {
            if combo.fingers.len() != 1 {
                continue;
            }
            let finger = combo.fingers[0];
            match &single_key_lookup[finger as usize] {
                Some(other_key) => {
                    return Err(format!(
                        "Finger {} is already mapped to {}",
                        finger, other_key
                    ));
                }
                None => {
                    single_key_lookup[finger as usize] = Some(combo.key.clone());
                }
            }
        }

        // single_key_lookup.iter().map(|x| x.unwrap_or(' ')).collect()
        Ok(single_key_lookup)
    }

    fn get_finger_lookup(&self) -> Result<Vec<String>, String> {
        let mut finger_lookup = self.get_option_finger_lookup()?;
        for (i, key) in finger_lookup.iter_mut().enumerate() {
            if key.is_none() {
                return Err(format!(
                    "Finger {} is not mapped",
                    Finger::try_from(i as u16).unwrap()
                ));
            }
        }
        Ok(finger_lookup.into_iter().map(|x| x.unwrap()).collect())
    }

    pub fn to_keymap(&self) -> Result<String, String> {
        let mut out = String::new();
        out += "[0] = LAYOUT_keychordz(\n";

        let single_key_lookup = self.get_finger_lookup()?;

        out += "   ";
        for (i, key) in single_key_lookup.iter().enumerate() {
            if i == 3 {
                out += &format!("    {},                  ", key);
            } else if i == 7 {
                out += &format!("    {}, \\\n                            ", key);
            } else if i == 10 {
                out += &format!(" {},           ", key);
            } else if i == Finger::COUNT - 1 {
                out += &format!(" {}", key);
            } else if i > 7 {
                out += &format!(" {},", key);
            } else {
                out += &format!("    {},", key);
            }
        }
        out += " \\\n       )";

        Ok(out)
    }

    /// const uint16_t PROGMEM test_combo1[] = {A, S, COMBO_END};
    /// const uint16_t PROGMEM test_combo2[] = {F, H, COMBO_END};
    /// combo_t key_combos[COMBO_COUNT] = {
    ///     COMBO(test_combo1, ESC),
    ///     COMBO(test_combo2, LCTL(Y)), // keycodes with modifiers are possible too!
    /// };
    pub fn to_qmk_combos(&self) -> Result<String, String> {
        let single_key_lookup = self.get_finger_lookup()?;
        let mut progmem_out = String::new();

        for (i, combo) in self.combos.iter().enumerate() {
            if combo.fingers.len() < 2 {
                continue;
            }
            let mut out = String::new();
            out += &format!("const uint16_t PROGMEM combo_{}[] = {{", i);
            for finger in &combo.fingers {
                out += &format!(" {},", single_key_lookup[*finger as usize]);
            }
            out += " COMBO_END};\n";
            progmem_out += &out;
        }

        let mut key_combos_out = String::new();
        key_combos_out += "combo_t key_combos[COMBO_COUNT] = {\n";
        for (i, combo) in self.combos.iter().enumerate() {
            if combo.fingers.len() < 2 {
                continue;
            }
            let mut out = String::new();
            out += &format!("    COMBO(combo_{}, ", i);
            out += &format!("{}),\n", combo.key);

            key_combos_out += &out;
        }
        key_combos_out += "};\n";

        Ok(format!(
            "{}\n\n{}\n\nComboCount = {}",
            progmem_out,
            key_combos_out,
            self.combos.len() - Finger::COUNT,
        ))
    }

    pub fn to_keychordz(&self) -> Result<String, String> {
        let mut out = String::new();

        // sort self.combos by key reversed
        let mut combos = self.combos.clone();
        combos.sort_by(|a, b| b.cmp(&a));
        
        for combo in combos.iter() {
            // let mut finger_combo = 0;
            // for finger in &combo.fingers {
            //     finger_combo |= *finger as u16;
            // }
            let finger_combo = combo
                .fingers
                .iter()
                .map(|finger| format!("Finger::{} as u16", finger))
                .collect::<Vec<String>>()
                .join(" | ");
            let key = combo.key.clone();
            let key = match Key::from_str(&key) {
                Ok(key) => {
                    // out += &format!("Chord::new(0b{:016b}, Key::{:?}),\n", finger_combo, key)
                    out += &format!("Chord::new({}, Key::{:?}),\n", finger_combo, key)
                }
                Err(_) => {
                    console::log_1(&format!("could not parse key: {}", key).into());
                    continue;
                }
            };
        }

        Ok(out)
    }
}
