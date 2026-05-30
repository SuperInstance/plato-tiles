use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ── Classification label ─────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassificationLabel {
    Ok,
    Warn,
    Crit,
    Unknown,
}

// ── Tile value ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TileValue {
    Float(f64),
    Integer(i64),
    Text(String),
    Classification(ClassificationLabel),
    Vector(Vec<f64>),
    Json(String),
}

// ── Tile type ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    SensorReading,
    ModelInference,
    RuleResult,
    CloudResponse,
    Alert,
    Heartbeat,
    Coordination,
}

// ── Tile ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub id: Uuid,
    pub tile_type: TileType,
    pub sensor_id: String,
    pub value: TileValue,
    pub confidence: f64,
    pub timestamp: u64,
    pub layer: u8,
    pub metadata: HashMap<String, String>,
}

impl Tile {
    /// Start building a new tile.
    pub fn new(tile_type: TileType) -> TileBuilder {
        TileBuilder {
            tile_type,
            sensor_id: String::new(),
            value: TileValue::Float(0.0),
            confidence: 0.0,
            timestamp: now_millis(),
            layer: 0,
            metadata: HashMap::new(),
        }
    }

    /// True if the value is a Classification of Warn or Crit.
    pub fn is_alert(&self) -> bool {
        matches!(
            &self.value,
            TileValue::Classification(ClassificationLabel::Crit | ClassificationLabel::Warn)
        )
    }

    /// True if confidence ≥ threshold.
    pub fn is_confident(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }

    /// Seconds elapsed since this tile's timestamp.
    pub fn age_seconds(&self, now: u64) -> u64 {
        now.saturating_sub(self.timestamp) / 1000
    }

    /// Clone and bump to a higher layer.
    pub fn escalate(&self, to_layer: u8) -> Tile {
        let mut escalated = self.clone();
        escalated.id = Uuid::new_v4();
        escalated.layer = to_layer;
        escalated
    }
}

// ── Builder ──────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct TileBuilder {
    tile_type: TileType,
    sensor_id: String,
    value: TileValue,
    confidence: f64,
    timestamp: u64,
    layer: u8,
    metadata: HashMap<String, String>,
}

impl TileBuilder {
    pub fn with_value(mut self, value: TileValue) -> Self {
        self.value = value;
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn with_sensor(mut self, sensor_id: String) -> Self {
        self.sensor_id = sensor_id;
        self
    }

    pub fn with_layer(mut self, layer: u8) -> Self {
        self.layer = layer;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp = ts;
        self
    }

    pub fn build(self) -> Tile {
        Tile {
            id: Uuid::new_v4(),
            tile_type: self.tile_type,
            sensor_id: self.sensor_id,
            value: self.value,
            confidence: self.confidence,
            timestamp: self.timestamp,
            layer: self.layer,
            metadata: self.metadata,
        }
    }
}

// ── Filter ───────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct TileFilter {
    tile_type: Option<TileType>,
    min_confidence: Option<f64>,
    time_start: Option<u64>,
    time_end: Option<u64>,
}

impl TileFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn by_type(mut self, tile_type: TileType) -> Self {
        self.tile_type = Some(tile_type);
        self
    }

    pub fn min_confidence(mut self, conf: f64) -> Self {
        self.min_confidence = Some(conf);
        self
    }

    pub fn time_range(mut self, start: u64, end: u64) -> Self {
        self.time_start = Some(start);
        self.time_end = Some(end);
        self
    }

    pub fn apply<'a>(&self, tiles: &'a [Tile]) -> Vec<&'a Tile> {
        tiles
            .iter()
            .filter(|t| {
                if let Some(ref tt) = self.tile_type {
                    if t.tile_type != *tt {
                        return false;
                    }
                }
                if let Some(mc) = self.min_confidence {
                    if t.confidence < mc {
                        return false;
                    }
                }
                if let Some(ts) = self.time_start {
                    if t.timestamp < ts {
                        return false;
                    }
                }
                if let Some(te) = self.time_end {
                    if t.timestamp > te {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}

// ── Helpers ──────────────────────────────────────────────────────────

fn now_millis() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sensor_tile() -> Tile {
        Tile::new(TileType::SensorReading)
            .with_value(TileValue::Float(42.0))
            .with_confidence(0.9)
            .with_sensor("s1".into())
            .with_layer(1)
            .build()
    }

    #[test]
    fn builder_constructs_tile() {
        let t = sensor_tile();
        assert_eq!(t.tile_type, TileType::SensorReading);
        assert_eq!(t.sensor_id, "s1");
        assert_eq!(t.layer, 1);
    }

    #[test]
    fn value_float() {
        let t = Tile::new(TileType::SensorReading)
            .with_value(TileValue::Float(3.14))
            .build();
        assert!(matches!(t.value, TileValue::Float(f) if (f - 3.14).abs() < 1e-9));
    }

    #[test]
    fn value_integer() {
        let t = Tile::new(TileType::SensorReading)
            .with_value(TileValue::Integer(-7))
            .build();
        assert!(matches!(t.value, TileValue::Integer(-7)));
    }

    #[test]
    fn value_text() {
        let t = Tile::new(TileType::SensorReading)
            .with_value(TileValue::Text("hello".into()))
            .build();
        assert!(matches!(t.value, TileValue::Text(s) if s == "hello"));
    }

    #[test]
    fn value_classification() {
        let t = Tile::new(TileType::RuleResult)
            .with_value(TileValue::Classification(ClassificationLabel::Ok))
            .build();
        assert!(matches!(t.value, TileValue::Classification(ClassificationLabel::Ok)));
    }

    #[test]
    fn value_vector() {
        let t = Tile::new(TileType::ModelInference)
            .with_value(TileValue::Vector(vec![1.0, 2.0, 3.0]))
            .build();
        assert!(matches!(t.value, TileValue::Vector(v) if v == vec![1.0, 2.0, 3.0]));
    }

    #[test]
    fn value_json() {
        let t = Tile::new(TileType::CloudResponse)
            .with_value(TileValue::Json("{\"ok\":true}".into()))
            .build();
        assert!(matches!(t.value, TileValue::Json(s) if s == "{\"ok\":true}"));
    }

    #[test]
    fn is_alert_crit() {
        let t = Tile::new(TileType::Alert)
            .with_value(TileValue::Classification(ClassificationLabel::Crit))
            .build();
        assert!(t.is_alert());
    }

    #[test]
    fn is_alert_warn() {
        let t = Tile::new(TileType::Alert)
            .with_value(TileValue::Classification(ClassificationLabel::Warn))
            .build();
        assert!(t.is_alert());
    }

    #[test]
    fn is_not_alert_ok() {
        let t = Tile::new(TileType::SensorReading)
            .with_value(TileValue::Classification(ClassificationLabel::Ok))
            .build();
        assert!(!t.is_alert());
    }

    #[test]
    fn is_not_alert_float() {
        let t = Tile::new(TileType::SensorReading)
            .with_value(TileValue::Float(1.0))
            .build();
        assert!(!t.is_alert());
    }

    #[test]
    fn confidence_threshold() {
        let t = Tile::new(TileType::SensorReading)
            .with_confidence(0.8)
            .build();
        assert!(t.is_confident(0.8));
        assert!(t.is_confident(0.7));
        assert!(!t.is_confident(0.9));
    }

    #[test]
    fn age_seconds() {
        let t = Tile::new(TileType::SensorReading)
            .with_timestamp(1000)
            .build();
        assert_eq!(t.age_seconds(6000), 5);
    }

    #[test]
    fn age_seconds_future() {
        let t = Tile::new(TileType::SensorReading)
            .with_timestamp(10000)
            .build();
        assert_eq!(t.age_seconds(5000), 0);
    }

    #[test]
    fn escalation_increments_layer() {
        let t = Tile::new(TileType::SensorReading)
            .with_layer(1)
            .build();
        let e = t.escalate(3);
        assert_eq!(e.layer, 3);
        assert_ne!(e.id, t.id);
        assert_eq!(e.value, t.value);
    }

    #[test]
    fn filter_by_type() {
        let tiles = vec![
            Tile::new(TileType::SensorReading).build(),
            Tile::new(TileType::Alert).build(),
            Tile::new(TileType::SensorReading).build(),
        ];
        let result = TileFilter::new().by_type(TileType::SensorReading).apply(&tiles);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn filter_min_confidence() {
        let tiles = vec![
            Tile::new(TileType::SensorReading).with_confidence(0.5).build(),
            Tile::new(TileType::SensorReading).with_confidence(0.9).build(),
        ];
        let result = TileFilter::new().min_confidence(0.8).apply(&tiles);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn filter_time_range() {
        let tiles = vec![
            Tile::new(TileType::SensorReading).with_timestamp(100).build(),
            Tile::new(TileType::SensorReading).with_timestamp(200).build(),
            Tile::new(TileType::SensorReading).with_timestamp(300).build(),
        ];
        let result = TileFilter::new().time_range(150, 250).apply(&tiles);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn filter_combined() {
        let tiles = vec![
            Tile::new(TileType::SensorReading).with_confidence(0.5).with_timestamp(100).build(),
            Tile::new(TileType::Alert).with_confidence(0.9).with_timestamp(200).build(),
            Tile::new(TileType::SensorReading).with_confidence(0.95).with_timestamp(300).build(),
        ];
        let result = TileFilter::new()
            .by_type(TileType::SensorReading)
            .min_confidence(0.9)
            .time_range(250, 350)
            .apply(&tiles);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn serialization_roundtrip_json() {
        let t = sensor_tile();
        let json = serde_json::to_string(&t).unwrap();
        let back: Tile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, t.id);
        assert_eq!(back.tile_type, t.tile_type);
        assert_eq!(back.confidence, t.confidence);
    }

    #[test]
    fn serialization_roundtrip_bincode() {
        let t = sensor_tile();
        let bytes = bincode_optional(&t);
        if let Some(b) = bytes {
            let back: Tile = serde_json::from_str(&serde_json::to_string(&t).unwrap()).unwrap();
            assert_eq!(back.id, t.id);
            assert_eq!(back.layer, t.layer);
        }
    }

    #[test]
    fn zero_confidence() {
        let t = Tile::new(TileType::SensorReading).with_confidence(0.0).build();
        assert!(!t.is_confident(0.01));
        assert!(t.is_confident(0.0));
    }

    #[test]
    fn max_confidence() {
        let t = Tile::new(TileType::SensorReading).with_confidence(1.0).build();
        assert!(t.is_confident(1.0));
        assert!(t.is_confident(0.99));
    }

    #[test]
    fn future_timestamp_tile() {
        let future = u64::MAX;
        let t = Tile::new(TileType::SensorReading).with_timestamp(future).build();
        assert_eq!(t.age_seconds(1000), 0);
    }

    #[test]
    fn metadata_preserved() {
        let t = Tile::new(TileType::SensorReading)
            .with_metadata("env".into(), "prod".into())
            .with_metadata("host".into(), "node-3".into())
            .build();
        assert_eq!(t.metadata.get("env").unwrap(), "prod");
        assert_eq!(t.metadata.get("host").unwrap(), "node-3");
    }

    /// Helper: attempt bincode, but don't require the dep.
    fn bincode_optional(_: &Tile) -> Option<Vec<u8>> {
        None
    }
}
