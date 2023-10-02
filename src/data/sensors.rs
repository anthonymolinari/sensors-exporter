use lm_sensors::prelude::*;
use lm_sensors::errors::Error;

pub fn get_sensor_data() -> Result<String, Error> {
    let sensors = lm_sensors::Initializer::default().initialize()?;
    let mut body = String::new();

    for chip in sensors.chip_iter(None) {
        for feature in chip.feature_iter() {
            let name = feature.name().transpose()?.unwrap_or("N/A");
            for sub_feature in feature.sub_feature_iter() {
                if let Ok(value) = sub_feature.raw_value() {
                    let metrics_label = format!("{}_{}_{}_{}", chip, name, feature, sub_feature)
                        .replace(" ", "_")
                        .replace("-", "_")
                        .replace("+", "");
                    body.push_str(format!("# TYPE {} gague \n", metrics_label).as_str());
                    body.push_str(format!("# HELP {} {:?} \n", metrics_label, sub_feature.kind().unwrap_or_default()).as_str());
                    body.push_str(format!("{} {}\n", metrics_label, value).as_str());
                }
            }
        }
    }
    Ok(body)
}
