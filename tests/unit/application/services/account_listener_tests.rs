use ig_client::application::services::Listener;
use ig_client::presentation::AccountData;
use lightstreamer_rs::subscription::{ItemUpdate, SubscriptionListener};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[test]
fn test_account_listener_new() {
    // Crear un contador para verificar si la callback es llamada
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    // Crear un listener con una callback que incrementa el contador
    let listener: Listener<AccountData> = Listener::new(move |data: &AccountData| {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
        // Verificar que el objeto AccountData tiene los campos esperados
        assert!(data.to_string().contains("item_name"));
        Ok(())
    });

    // Crear un ItemUpdate simulado para probar el callback
    let mut fields = HashMap::new();
    fields.insert("ACCOUNT_ID".to_string(), Some("ABCDEF".to_string()));
    fields.insert("BALANCE".to_string(), Some("1000.0".to_string()));
    fields.insert("DEPOSIT".to_string(), Some("500.0".to_string()));
    fields.insert("PL".to_string(), Some("100.0".to_string()));
    fields.insert("AVAILABLE".to_string(), Some("1100.0".to_string()));

    let item_update = ItemUpdate {
        item_name: Some("ACCOUNT:ABCDEF".to_string()),
        item_pos: 1,
        is_snapshot: true,
        fields,
        changed_fields: HashMap::new(),
    };

    // Llamar a on_item_update que internamente llama a callback
    <Listener<AccountData> as SubscriptionListener>::on_item_update(&listener, &item_update);

    // Verificar que la callback fue llamada
    assert_eq!(*counter.lock().unwrap(), 1);
}
