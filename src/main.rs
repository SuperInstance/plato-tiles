use plato_tiles::*;

fn main() {
    let tile = Tile::new(TileType::SensorReading)
        .with_value(TileValue::Float(23.5))
        .with_confidence(0.95)
        .with_sensor("temp-sensor-01".into())
        .with_layer(1)
        .with_metadata("location".into(), "engine-room".into())
        .build();

    println!("Tile: {:?}", tile);
    println!("Is alert? {}", tile.is_alert());
    println!("Is confident > 0.9? {}", tile.is_confident(0.9));
}
