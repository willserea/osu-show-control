use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub enum OsuEventValueType {
    String,
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct OsuEventDefinition {
    pub id: &'static str,
    pub label: &'static str,
    pub json_pointer: &'static str,
    pub osc_address: &'static str,
    pub value_type: OsuEventValueType,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShowEvent {
    pub address: String,
    pub value: String,
    pub event_id: String,
    pub label: String,
}

pub struct ShowLogic {
    last_values: HashMap<String, String>,
    event_definitions: Vec<OsuEventDefinition>,
}

impl ShowLogic {
    pub fn new() -> Self {
        Self {
            last_values: HashMap::new(),
            event_definitions: osu_events(),
        }
    }

    pub fn event_definitions(&self) -> &[OsuEventDefinition] {
        &self.event_definitions
    }

    pub fn handle_tosu_update(&mut self, json: &Value) -> Vec<ShowEvent> {
        let mut events = Vec::new();

        for definition in &self.event_definitions {
            let Some(current_value) = value_as_string(json, definition.json_pointer) else {
                continue;
            };

            let last_value = self.last_values.get(definition.id);

            if last_value != Some(&current_value) {
                self.last_values
                    .insert(definition.id.to_string(), current_value.clone());

                events.push(ShowEvent {
                    address: definition.osc_address.to_string(),
                    value: current_value,
                    event_id: definition.id.to_string(),
                    label: definition.label.to_string(),
                });
            }
        }

        events
    }
}

pub fn osu_events() -> Vec<OsuEventDefinition> {
    vec![
        // -----------------------------
        // Allgemeiner Spielstatus
        // -----------------------------
        OsuEventDefinition {
            id: "state_name",
            label: "Spielstatus",
            json_pointer: "/state/name",
            osc_address: "/osu/state",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "settings_mode_name",
            label: "Gewählter Spielmodus",
            json_pointer: "/settings/mode/name",
            osc_address: "/osu/settings/mode",
            value_type: OsuEventValueType::String,
        },
        // -----------------------------
        // Profil
        // -----------------------------
        OsuEventDefinition {
            id: "profile_name",
            label: "Profilname",
            json_pointer: "/profile/name",
            osc_address: "/osu/profile/name",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "profile_pp",
            label: "Profil PP",
            json_pointer: "/profile/pp",
            osc_address: "/osu/profile/pp",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "profile_rank_global",
            label: "Globaler Rang",
            json_pointer: "/profile/rank/global",
            osc_address: "/osu/profile/rank/global",
            value_type: OsuEventValueType::Int,
        },
        // -----------------------------
        // Beatmap Metadaten
        // -----------------------------
        OsuEventDefinition {
            id: "beatmap_artist",
            label: "Beatmap Artist",
            json_pointer: "/beatmap/artist",
            osc_address: "/osu/beatmap/artist",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "beatmap_title",
            label: "Beatmap Titel",
            json_pointer: "/beatmap/title",
            osc_address: "/osu/beatmap/title",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "beatmap_mapper",
            label: "Beatmap Mapper",
            json_pointer: "/beatmap/mapper",
            osc_address: "/osu/beatmap/mapper",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "beatmap_version",
            label: "Beatmap Difficulty",
            json_pointer: "/beatmap/version",
            osc_address: "/osu/beatmap/version",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "beatmap_mode_name",
            label: "Beatmap Modus",
            json_pointer: "/beatmap/mode/name",
            osc_address: "/osu/beatmap/mode",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "beatmap_status_name",
            label: "Beatmap Status",
            json_pointer: "/beatmap/status/name",
            osc_address: "/osu/beatmap/status",
            value_type: OsuEventValueType::String,
        },
        // -----------------------------
        // Beatmap Werte
        // -----------------------------
        OsuEventDefinition {
            id: "beatmap_time_live",
            label: "Beatmap Zeit live",
            json_pointer: "/beatmap/time/live",
            osc_address: "/osu/beatmap/time/live",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "beatmap_time_total",
            label: "Beatmap Zeit total",
            json_pointer: "/beatmap/time/total",
            osc_address: "/osu/beatmap/time/total",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "beatmap_bpm_common",
            label: "Beatmap BPM",
            json_pointer: "/beatmap/stats/bpm/common",
            osc_address: "/osu/beatmap/bpm",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "beatmap_stars_live",
            label: "Beatmap Stars live",
            json_pointer: "/beatmap/stats/stars/live",
            osc_address: "/osu/beatmap/stars/live",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "beatmap_ar_live",
            label: "Beatmap AR live",
            json_pointer: "/beatmap/stats/ar/live",
            osc_address: "/osu/beatmap/ar/live",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "beatmap_cs_live",
            label: "Beatmap CS live",
            json_pointer: "/beatmap/stats/cs/live",
            osc_address: "/osu/beatmap/cs/live",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "beatmap_od_live",
            label: "Beatmap OD live",
            json_pointer: "/beatmap/stats/od/live",
            osc_address: "/osu/beatmap/od/live",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "beatmap_hp_live",
            label: "Beatmap HP live",
            json_pointer: "/beatmap/stats/hp/live",
            osc_address: "/osu/beatmap/hp/live",
            value_type: OsuEventValueType::Float,
        },
        // -----------------------------
        // Gameplay
        // -----------------------------
        OsuEventDefinition {
            id: "play_player_name",
            label: "Spielername im Play",
            json_pointer: "/play/playerName",
            osc_address: "/osu/play/player",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "play_mode_name",
            label: "Aktiver Play-Modus",
            json_pointer: "/play/mode/name",
            osc_address: "/osu/play/mode",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "play_score",
            label: "Score",
            json_pointer: "/play/score",
            osc_address: "/osu/play/score",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "play_accuracy",
            label: "Accuracy",
            json_pointer: "/play/accuracy",
            osc_address: "/osu/play/accuracy",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "play_health_normal",
            label: "HP normal",
            json_pointer: "/play/healthBar/normal",
            osc_address: "/osu/play/hp",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "play_combo_current",
            label: "Combo aktuell",
            json_pointer: "/play/combo/current",
            osc_address: "/osu/play/combo",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "play_combo_max",
            label: "Combo max",
            json_pointer: "/play/combo/max",
            osc_address: "/osu/play/combo/max",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "play_mods_name",
            label: "Mods",
            json_pointer: "/play/mods/name",
            osc_address: "/osu/play/mods",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "play_rank_current",
            label: "Rank aktuell",
            json_pointer: "/play/rank/current",
            osc_address: "/osu/play/rank",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "play_pp_current",
            label: "PP aktuell",
            json_pointer: "/play/pp/current",
            osc_address: "/osu/play/pp",
            value_type: OsuEventValueType::Float,
        },
        // -----------------------------
        // Hits
        // -----------------------------
        OsuEventDefinition {
            id: "play_hits_300",
            label: "Hits 300",
            json_pointer: "/play/hits/300",
            osc_address: "/osu/play/hits/300",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "play_hits_100",
            label: "Hits 100",
            json_pointer: "/play/hits/100",
            osc_address: "/osu/play/hits/100",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "play_hits_50",
            label: "Hits 50",
            json_pointer: "/play/hits/50",
            osc_address: "/osu/play/hits/50",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "play_hits_miss",
            label: "Misses",
            json_pointer: "/play/hits/miss",
            osc_address: "/osu/play/hits/miss",
            value_type: OsuEventValueType::Int,
        },
        // -----------------------------
        // Timing / Show-relevante Zustände
        // -----------------------------
        OsuEventDefinition {
            id: "play_is_break",
            label: "Break",
            json_pointer: "/play/isBreak",
            osc_address: "/osu/play/is-break",
            value_type: OsuEventValueType::Bool,
        },
        OsuEventDefinition {
            id: "play_is_kiai",
            label: "Kiai",
            json_pointer: "/play/isKiai",
            osc_address: "/osu/play/is-kiai",
            value_type: OsuEventValueType::Bool,
        },
        // -----------------------------
        // Result Screen
        // -----------------------------
        OsuEventDefinition {
            id: "results_rank",
            label: "Result Rank",
            json_pointer: "/resultsScreen/rank",
            osc_address: "/osu/results/rank",
            value_type: OsuEventValueType::String,
        },
        OsuEventDefinition {
            id: "results_score",
            label: "Result Score",
            json_pointer: "/resultsScreen/score",
            osc_address: "/osu/results/score",
            value_type: OsuEventValueType::Int,
        },
        OsuEventDefinition {
            id: "results_accuracy",
            label: "Result Accuracy",
            json_pointer: "/resultsScreen/accuracy",
            osc_address: "/osu/results/accuracy",
            value_type: OsuEventValueType::Float,
        },
        OsuEventDefinition {
            id: "results_pp_current",
            label: "Result PP",
            json_pointer: "/resultsScreen/pp/current",
            osc_address: "/osu/results/pp",
            value_type: OsuEventValueType::Float,
        },
    ]
}

pub fn value_as_string(json: &Value, json_pointer: &str) -> Option<String> {
    let value = json.pointer(json_pointer)?;

    if let Some(text) = value.as_str() {
        return Some(text.to_string());
    }

    if let Some(number) = value.as_i64() {
        return Some(number.to_string());
    }

    if let Some(number) = value.as_f64() {
        return Some(format_float(number));
    }

    if let Some(boolean) = value.as_bool() {
        return Some(boolean.to_string());
    }

    // Falls ein Feld ein Array oder Objekt ist, erstmal kompakt als JSON ausgeben.
    if value.is_array() || value.is_object() {
        return Some(value.to_string());
    }

    None
}

fn format_float(value: f64) -> String {
    let rounded = format!("{:.3}", value);

    rounded
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
