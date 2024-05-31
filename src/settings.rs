use chrono::NaiveTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AppSettings {
    #[serde(
        deserialize_with = "deserialize_trigger_time",
        serialize_with = "serialize_trigger_time"
    )]
    pub trigger_time: NaiveTime,
    pub countdown_seconds: u32,
    pub allow_snooze: bool,
}

fn serialize_trigger_time<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted_time = time.format("%H:%M").to_string();
    serializer.serialize_str(&formatted_time)
}

fn deserialize_trigger_time<'a, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'a>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveTime::parse_from_str(&s, "%H:%M").map_err(serde::de::Error::custom)
}

impl AppSettings {
    fn generate_settings_file(&self, path: &Path) {
        let mut writer = BufWriter::new(File::create(path).unwrap());
        let settings_string: String = serde_json::to_string_pretty(&self).unwrap();
        writer.write_all(settings_string.as_bytes()).unwrap();
        let _ = writer.flush();
    }

    pub fn load_or_create_settings(path: &Path) -> AppSettings {
        // Open existing settings or create default ones
        let settings_string = fs::read_to_string(path).map_or(
            {
                let settings = AppSettings::default();
                settings.generate_settings_file(path);
                let settings_string = serde_json::to_string(&settings).unwrap();
                settings_string
            },
            |s| s,
        );

        // TODO: validate settings format before
        AppSettings::from(settings_string)
    }
}

impl From<String> for AppSettings {
    fn from(value: String) -> Self {
        serde_json::from_str::<Self>(&value).unwrap()
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            countdown_seconds: 10u32,
            allow_snooze: false,
            trigger_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AppSettings;
    use chrono::NaiveTime;
    use serde_json::{json, Value};
    use std::{
        fs::{self},
        path::Path,
    };

    fn remove_test_file_at(path: &Path) {
        fs::remove_file(path).unwrap()
    }
    #[test]
    fn check_generated_file_settings_are_correct() {
        let mut settings = AppSettings::default();
        let settings_path = Path::new("generated_settings_test.json");
        settings.trigger_time = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
        settings.generate_settings_file(settings_path);

        let result_json_string = fs::read_to_string(settings_path).unwrap();
        let result_json: Value = serde_json::from_str(&result_json_string).unwrap();

        let expected_json = json!({
            "trigger_time": "18:00",
            "countdown_seconds": 10,
            "allow_snooze": false,
        });

        remove_test_file_at(settings_path);
        assert_eq!(expected_json, result_json);
    }

    #[test]
    fn check_settings_file_exist_or_create_default() {
        let mut changed_settings = AppSettings::default();
        changed_settings.countdown_seconds = 100;
        changed_settings.trigger_time = NaiveTime::from_hms_opt(18, 0, 0).unwrap();

        // If no file exists it will create a file with default settings
        let settings_path = Path::new("settings_test.json");
        AppSettings::load_or_create_settings(settings_path);

        let default_settings_string = fs::read_to_string(settings_path).unwrap();
        let default_settings = AppSettings::from(default_settings_string);
        let expected_settings = AppSettings::default();

        assert_eq!(expected_settings, default_settings);

        // Serialize changed settings to then read them
        changed_settings.generate_settings_file(settings_path);
        let loaded_settings = AppSettings::load_or_create_settings(settings_path);

        // Cleanup test setting file
        remove_test_file_at(settings_path);
        assert_eq!(changed_settings, loaded_settings);
    }
}
