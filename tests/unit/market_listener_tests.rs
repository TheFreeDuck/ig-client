use ig_client::application::services::Listener;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ig_client::presentation::MarketData;

#[test]
fn test_market_listener_new() {
    // Crear un contador para verificar si la callback es llamada
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();
    
    // Crear un listener con una callback que incrementa el contador
    let listener = Listener::new(move |data: &MarketData| {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
        // Verificar que el objeto MarketData tiene los campos esperados
        assert!(data.to_string().contains("item_name"));
        Ok(())
    });
    
    // Crear un ItemUpdate simulado para probar el callback
    let mut fields = HashMap::new();
    fields.insert("BID".to_string(), Some("1.2000".to_string()));
    fields.insert("OFFER".to_string(), Some("1.2010".to_string()));
    fields.insert("HIGH".to_string(), Some("1.2100".to_string()));
    fields.insert("LOW".to_string(), Some("1.1900".to_string()));
    fields.insert("MARKET_STATE".to_string(), Some("tradeable".to_string()));
    
    let item_update = ItemUpdate {
        item_name: Some("CS.D.EURUSD.CFD.IP".to_string()),
        item_pos: 1,
        is_snapshot: true,
        fields,
        changed_fields: HashMap::new(),
    };
    
    // Llamar a on_item_update que internamente llama a callback
    <Listener<MarketData> as SubscriptionListener>::on_item_update(&listener, &item_update);
    
    // Verificar que la callback fue llamada
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[test]
fn test_market_listener_mock() {
    // Verificar que podemos crear un mock del listener
    let listener = Listener::new(|data :&MarketData| {
        // Simplemente verificamos que el objeto MarketData tiene los campos esperados
        assert!(data.to_string().contains("item_name"));
        Ok(())
    });
    
    // Crear un ItemUpdate simulado para probar el callback
    let mut fields = HashMap::new();
    fields.insert("BID".to_string(), Some("1.2000".to_string()));
    fields.insert("OFFER".to_string(), Some("1.2010".to_string()));
    fields.insert("HIGH".to_string(), Some("1.2100".to_string()));
    fields.insert("LOW".to_string(), Some("1.1900".to_string()));
    fields.insert("MARKET_STATE".to_string(), Some("tradeable".to_string()));
    
    let item_update = ItemUpdate {
        item_name: Some("CS.D.EURUSD.CFD.IP".to_string()),
        item_pos: 1,
        is_snapshot: true,
        fields,
        changed_fields: HashMap::new(),
    };
    
    // Llamar a on_item_update que internamente llama a callback
    <Listener<MarketData> as SubscriptionListener>::on_item_update(&listener, &item_update);
    
    // Si llegamos aquí sin errores, la prueba pasa
    assert!(true);
}
