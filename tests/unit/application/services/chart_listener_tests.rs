use ig_client::application::services::Listener;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ig_client::presentation::ChartData;

#[test]
fn test_chart_listener_new() {
    // Crear un contador para verificar si la callback es llamada
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();
    
    // Crear un listener con una callback que incrementa el contador
    let listener = Listener::new(move |data: &ChartData| {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
        // Verificar que el objeto ChartData tiene los campos esperados
        assert!(data.to_string().contains("item_name"));
        Ok(())
    });
    
    // Crear un ItemUpdate simulado para probar el callback
    let mut fields = HashMap::new();
    fields.insert("BID".to_string(), Some("1.2000".to_string()));
    fields.insert("OFR".to_string(), Some("1.2010".to_string()));
    fields.insert("LTV".to_string(), Some("1000".to_string()));
    
    let item_update = ItemUpdate {
        item_name: Some("CHART:CS.D.EURUSD.CFD.IP:TICK".to_string()),
        item_pos: 1,
        is_snapshot: true,
        fields,
        changed_fields: HashMap::new(),
    };
    
    // Llamar a on_item_update que internamente llama a callback
    <Listener<ChartData> as SubscriptionListener>::on_item_update(&listener, &item_update);
    
    // Verificar que la callback fue llamada
    assert_eq!(*counter.lock().unwrap(), 1);
}
