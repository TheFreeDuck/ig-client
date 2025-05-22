#[cfg(test)]
mod tests {
    use ig_client::application::models::market::{
        Currency, DealingRules, MarketDetails, MarketSnapshot, StepDistance, StepUnit,
    };
    use serde::Deserialize;

    /// Test the complete MarketDetails deserialization with the provided JSON
    #[test]
    fn test_deserialize_complete_market_details() {
        let json_data = r#"
        {
          "instrument": {
            "epic": "DO.D.OTCDDAX.1.IP",
            "expiry": "22-MAY-25",
            "name": "Daily Germany 40 25050 PUT",
            "forceOpenAllowed": true,
            "stopsLimitsAllowed": false,
            "lotSize": 1.0,
            "unit": "CONTRACTS",
            "type": "UNKNOWN",
            "controlledRiskAllowed": true,
            "streamingPricesAvailable": true,
            "marketId": "DE30",
            "currencies": [
              {
                "code": "EUR",
                "symbol": "E",
                "baseExchangeRate": 1.0,
                "exchangeRate": 0.81,
                "isDefault": true
              }
            ],
            "sprintMarketsMinimumExpiryTime": null,
            "sprintMarketsMaximumExpiryTime": null,
            "marginDepositBands": [
              {
                "min": 0,
                "max": 150,
                "margin": 5,
                "currency": "EUR"
              },
              {
                "min": 150,
                "max": 1500,
                "margin": 5,
                "currency": "EUR"
              },
              {
                "min": 1500,
                "max": 2250,
                "margin": 5,
                "currency": "EUR"
              },
              {
                "min": 2250,
                "max": null,
                "margin": 15,
                "currency": "EUR"
              }
            ],
            "marginFactor": 5,
            "marginFactorUnit": "PERCENTAGE",
            "slippageFactor": {
              "unit": "pct",
              "value": 50.0
            },
            "limitedRiskPremium": {
              "value": 0,
              "unit": null
            },
            "openingHours": null,
            "expiryDetails": {
              "lastDealingDate": "2025-05-22T15:29",
              "settlementInfo": "Settles basis official cash close of DAX 40 index"
            },
            "rolloverDetails": null,
            "newsCode": ".GDAX",
            "chartCode": null,
            "country": "DE",
            "valueOfOnePip": "1.00",
            "onePipMeans": "1 Index Point",
            "contractSize": "1",
            "specialInfo": [
              "MAX KNOCK OUT LEVEL DISTANCE",
              "DEFAULT KNOCK OUT LEVEL DISTANCE"
            ]
          },
          "dealingRules": {
            "minStepDistance": {
              "unit": "POINTS",
              "value": 1.0E10
            },
            "minDealSize": {
              "unit": "POINTS",
              "value": 0.1
            },
            "minControlledRiskStopDistance": {
              "unit": "PERCENTAGE",
              "value": 1.0
            },
            "minNormalStopOrLimitDistance": {
              "unit": "POINTS",
              "value": 1.0
            },
            "maxStopOrLimitDistance": {
              "unit": "POINTS",
              "value": 1111.0
            },
            "controlledRiskSpacing": {
              "unit": "POINTS",
              "value": 0.0
            },
            "marketOrderPreference": "NOT_AVAILABLE",
            "trailingStopsPreference": "NOT_AVAILABLE"
          },
          "snapshot": {
            "marketStatus": "TRADEABLE",
            "netChange": 23961.5,
            "percentageChange": -0.66,
            "updateTime": "04:35:47",
            "delayTime": 0,
            "bid": 1086.0,
            "offer": 1091.0,
            "high": 1097.0,
            "low": 1055.0,
            "binaryOdds": null,
            "decimalPlacesFactor": 1,
            "scalingFactor": 1,
            "controlledRiskExtraSpread": null
          }
        }
        "#;

        let result: Result<MarketDetails, _> = serde_json::from_str(json_data);
        assert!(
            result.is_ok(),
            "Failed to deserialize MarketDetails: {:?}",
            result.err()
        );

        let market_details = result.unwrap();

        // Verify instrument details
        let instrument = market_details.instrument;
        assert_eq!(instrument.epic, "DO.D.OTCDDAX.1.IP");
        assert_eq!(instrument.name, "Daily Germany 40 25050 PUT");
        assert_eq!(instrument.expiry, "22-MAY-25");
        assert_eq!(instrument.contract_size, "1");
        assert_eq!(instrument.lot_size, Some(1.0));
        assert_eq!(instrument.margin_factor, Some(5.0));
        assert_eq!(
            instrument.margin_factor_unit,
            Some("PERCENTAGE".to_string())
        );

        // Verify currency information
        let currencies = instrument.currencies.expect("Currencies should be present");
        assert_eq!(currencies.len(), 1);
        let currency = &currencies[0];
        assert_eq!(currency.code, "EUR");
        assert_eq!(currency.symbol, Some("E".to_string()));
        assert_eq!(currency.base_exchange_rate, Some(1.0));
        assert_eq!(currency.exchange_rate, Some(0.81));
        assert_eq!(currency.is_default, Some(true));

        // Verify dealing rules
        let dealing_rules = &market_details.dealing_rules;
        assert_eq!(dealing_rules.min_step_distance.value, Some(1.0e10));
        assert_eq!(dealing_rules.min_deal_size.value, Some(0.1));
        assert_eq!(
            dealing_rules.min_controlled_risk_stop_distance.value,
            Some(1.0)
        );
        assert_eq!(
            dealing_rules.min_normal_stop_or_limit_distance.value,
            Some(1.0)
        );
        assert_eq!(dealing_rules.max_stop_or_limit_distance.value, Some(1111.0));
        assert_eq!(dealing_rules.controlled_risk_spacing.value, Some(0.0));
        assert_eq!(dealing_rules.market_order_preference, "NOT_AVAILABLE");
        assert_eq!(dealing_rules.trailing_stops_preference, "NOT_AVAILABLE");

        // Verify market snapshot
        let snapshot = &market_details.snapshot;
        assert_eq!(snapshot.market_status, "TRADEABLE");
        assert_eq!(snapshot.net_change, Some(23961.5));
        assert_eq!(snapshot.percentage_change, Some(-0.66));
        assert_eq!(snapshot.update_time, Some("04:35:47".to_string()));
        assert_eq!(snapshot.delay_time, Some(0));
        assert_eq!(snapshot.bid, Some(1086.0));
        assert_eq!(snapshot.offer, Some(1091.0));
        assert_eq!(snapshot.high, Some(1097.0));
        assert_eq!(snapshot.low, Some(1055.0));
        assert_eq!(snapshot.binary_odds, None);
        assert_eq!(snapshot.decimal_places_factor, Some(1));
        assert_eq!(snapshot.scaling_factor, Some(1));
        assert_eq!(snapshot.controlled_risk_extra_spread, None);
    }

    #[test]
    fn test_deserialize_complete_market_details_two() {
        let json_data = r#"
        {
          "marketDetails": [
            {
              "instrument": {
                "epic": "DO.D.OTCDDAX.1.IP",
                "expiry": "22-MAY-25",
                "name": "Daily Germany 40 25050 PUT",
                "forceOpenAllowed": true,
                "stopsLimitsAllowed": false,
                "lotSize": 1.0,
                "unit": "CONTRACTS",
                "type": "UNKNOWN",
                "controlledRiskAllowed": true,
                "streamingPricesAvailable": true,
                "marketId": "DE30",
                "currencies": [
                  {
                    "code": "EUR",
                    "symbol": "E",
                    "baseExchangeRate": 1.0,
                    "exchangeRate": 0.81,
                    "isDefault": true
                  }
                ],
                "sprintMarketsMinimumExpiryTime": null,
                "sprintMarketsMaximumExpiryTime": null,
                "marginDepositBands": [
                  {
                    "min": 0,
                    "max": 150,
                    "margin": 5,
                    "currency": "EUR"
                  },
                  {
                    "min": 150,
                    "max": 1500,
                    "margin": 5,
                    "currency": "EUR"
                  },
                  {
                    "min": 1500,
                    "max": 2250,
                    "margin": 5,
                    "currency": "EUR"
                  },
                  {
                    "min": 2250,
                    "max": null,
                    "margin": 15,
                    "currency": "EUR"
                  }
                ],
                "marginFactor": 5,
                "marginFactorUnit": "PERCENTAGE",
                "slippageFactor": {
                  "unit": "pct",
                  "value": 50.0
                },
                "limitedRiskPremium": {
                  "value": 0,
                  "unit": null
                },
                "openingHours": null,
                "expiryDetails": {
                  "lastDealingDate": "2025-05-22T15:29",
                  "settlementInfo": "Settles basis official cash close of DAX 40 index"
                },
                "rolloverDetails": null,
                "newsCode": ".GDAX",
                "chartCode": null,
                "country": "DE",
                "valueOfOnePip": "1.00",
                "onePipMeans": "1 Index Point",
                "contractSize": "1",
                "specialInfo": [
                  "MAX KNOCK OUT LEVEL DISTANCE",
                  "DEFAULT KNOCK OUT LEVEL DISTANCE"
                ]
              },
              "dealingRules": {
                "minStepDistance": {
                  "unit": "POINTS",
                  "value": 1.0E10
                },
                "minDealSize": {
                  "unit": "POINTS",
                  "value": 0.1
                },
                "minControlledRiskStopDistance": {
                  "unit": "PERCENTAGE",
                  "value": 1.0
                },
                "minNormalStopOrLimitDistance": {
                  "unit": "POINTS",
                  "value": 1.0
                },
                "maxStopOrLimitDistance": {
                  "unit": "POINTS",
                  "value": 1111.0
                },
                "controlledRiskSpacing": {
                  "unit": "POINTS",
                  "value": 0.0
                },
                "marketOrderPreference": "NOT_AVAILABLE",
                "trailingStopsPreference": "NOT_AVAILABLE"
              },
              "snapshot": {
                "marketStatus": "TRADEABLE",
                "netChange": 23981,
                "percentageChange": -0.58,
                "updateTime": "06:45:20",
                "delayTime": 0,
                "bid": 1067.6,
                "offer": 1070.4,
                "high": 1125.5,
                "low": 1055.0,
                "binaryOdds": null,
                "decimalPlacesFactor": 1,
                "scalingFactor": 1,
                "controlledRiskExtraSpread": null
              }
            },
            {
              "instrument": {
                "epic": "DO.D.OTCDDAX.2.IP",
                "expiry": "22-MAY-25",
                "name": "Daily Germany 40 25050 CALL",
                "forceOpenAllowed": true,
                "stopsLimitsAllowed": false,
                "lotSize": 1.0,
                "unit": "CONTRACTS",
                "type": "UNKNOWN",
                "controlledRiskAllowed": true,
                "streamingPricesAvailable": true,
                "marketId": "DE30",
                "currencies": [
                  {
                    "code": "EUR",
                    "symbol": "E",
                    "baseExchangeRate": 1.0,
                    "exchangeRate": 0.81,
                    "isDefault": true
                  }
                ],
                "sprintMarketsMinimumExpiryTime": null,
                "sprintMarketsMaximumExpiryTime": null,
                "marginDepositBands": [
                  {
                    "min": 0,
                    "max": 150,
                    "margin": 5,
                    "currency": "EUR"
                  },
                  {
                    "min": 150,
                    "max": 1500,
                    "margin": 5,
                    "currency": "EUR"
                  },
                  {
                    "min": 1500,
                    "max": 2250,
                    "margin": 5,
                    "currency": "EUR"
                  },
                  {
                    "min": 2250,
                    "max": null,
                    "margin": 15,
                    "currency": "EUR"
                  }
                ],
                "marginFactor": 5,
                "marginFactorUnit": "PERCENTAGE",
                "slippageFactor": {
                  "unit": "pct",
                  "value": 50.0
                },
                "limitedRiskPremium": {
                  "value": 0,
                  "unit": null
                },
                "openingHours": null,
                "expiryDetails": {
                  "lastDealingDate": "2025-05-22T15:29",
                  "settlementInfo": "Settles basis official cash close of DAX 40 index"
                },
                "rolloverDetails": null,
                "newsCode": ".GDAX",
                "chartCode": null,
                "country": "DE",
                "valueOfOnePip": "1.00",
                "onePipMeans": "1 Index Point",
                "contractSize": "1",
                "specialInfo": [
                  "MAX KNOCK OUT LEVEL DISTANCE",
                  "DEFAULT KNOCK OUT LEVEL DISTANCE"
                ]
              },
              "dealingRules": {
                "minStepDistance": {
                  "unit": "POINTS",
                  "value": 1.0E10
                },
                "minDealSize": {
                  "unit": "POINTS",
                  "value": 0.1
                },
                "minControlledRiskStopDistance": {
                  "unit": "PERCENTAGE",
                  "value": 1.0
                },
                "minNormalStopOrLimitDistance": {
                  "unit": "POINTS",
                  "value": 1.0
                },
                "maxStopOrLimitDistance": {
                  "unit": "POINTS",
                  "value": 1111.0
                },
                "controlledRiskSpacing": {
                  "unit": "POINTS",
                  "value": 0.0
                },
                "marketOrderPreference": "NOT_AVAILABLE",
                "trailingStopsPreference": "NOT_AVAILABLE"
              },
              "snapshot": {
                "marketStatus": "TRADEABLE",
                "netChange": 23981,
                "percentageChange": -0.58,
                "updateTime": "15:29:30",
                "delayTime": 0,
                "bid": 0.0,
                "offer": 5.0,
                "high": 5.0,
                "low": 0.0,
                "binaryOdds": null,
                "decimalPlacesFactor": 1,
                "scalingFactor": 1,
                "controlledRiskExtraSpread": null
              }
            }
          ]
        }
        "#;

        #[derive(Deserialize)]
        struct MarketDetailsResponse {
            #[serde(rename = "marketDetails")]
            market_details: Vec<MarketDetails>,
        }

        let result: Result<MarketDetailsResponse, _> = serde_json::from_str(json_data);
        assert!(
            result.is_ok(),
            "Failed to deserialize MarketDetailsResponse: {:?}",
            result.err()
        );

        let response = result.unwrap();
        let market_details = response.market_details;

        assert!(
            !market_details.is_empty(),
            "Market details should not be empty"
        );
        assert_eq!(market_details[0].instrument.epic, "DO.D.OTCDDAX.1.IP");
    }

    /// Test StepDistance deserialization with different value types
    #[test]
    fn test_step_distance_deserialization() {
        // Test with regular numeric value
        let json = r#"{"unit": "POINTS", "value": 1.5}"#;
        let result: StepDistance = serde_json::from_str(json).unwrap();
        assert!(matches!(result.unit, Some(StepUnit::Points)));
        assert_eq!(result.value, Some(1.5));

        // Test with scientific notation
        let json = r#"{"unit": "PERCENTAGE", "value": 1.0E10}"#;
        let result: StepDistance = serde_json::from_str(json).unwrap();
        assert!(matches!(result.unit, Some(StepUnit::Percentage)));
        assert_eq!(result.value, Some(1.0e10));

        // Test with null value
        let json = r#"{"unit": "POINTS", "value": null}"#;
        let result: StepDistance = serde_json::from_str(json).unwrap();
        assert!(matches!(result.unit, Some(StepUnit::Points)));
        assert_eq!(result.value, None);

        // Test with missing value (should default to None)
        let json = r#"{"unit": "POINTS"}"#;
        let result: StepDistance = serde_json::from_str(json).unwrap();
        assert!(matches!(result.unit, Some(StepUnit::Points)));
        assert_eq!(result.value, None);
    }

    /// Test DealingRulesV3 deserialization
    #[test]
    fn test_dealing_rules_deserialization() {
        let json = r#"
        {
            "minStepDistance": {"unit": "POINTS", "value": 1.0E10},
            "minDealSize": {"unit": "POINTS", "value": 0.1},
            "minControlledRiskStopDistance": {"unit": "PERCENTAGE", "value": 1.0},
            "minNormalStopOrLimitDistance": {"unit": "POINTS", "value": 1.0},
            "maxStopOrLimitDistance": {"unit": "POINTS", "value": 1111.0},
            "controlledRiskSpacing": {"unit": "POINTS", "value": 0.0},
            "marketOrderPreference": "NOT_AVAILABLE",
            "trailingStopsPreference": "NOT_AVAILABLE"
        }
        "#;

        let result: DealingRules = serde_json::from_str(json).unwrap();

        assert_eq!(result.min_step_distance.value, Some(1.0e10));
        assert_eq!(result.min_deal_size.value, Some(0.1));
        assert_eq!(result.min_controlled_risk_stop_distance.value, Some(1.0));
        assert_eq!(result.min_normal_stop_or_limit_distance.value, Some(1.0));
        assert_eq!(result.max_stop_or_limit_distance.value, Some(1111.0));
        assert_eq!(result.controlled_risk_spacing.value, Some(0.0));
        assert_eq!(result.market_order_preference, "NOT_AVAILABLE");
        assert_eq!(result.trailing_stops_preference, "NOT_AVAILABLE");
    }

    /// Test MarketSnapshotV3 deserialization with various null values
    #[test]
    fn test_market_snapshot_v3_deserialization() {
        let json = r#"
        {
            "marketStatus": "TRADEABLE",
            "netChange": 23961.5,
            "percentageChange": -0.66,
            "updateTime": "04:35:47",
            "delayTime": 0,
            "bid": 1086.0,
            "offer": 1091.0,
            "high": 1097.0,
            "low": 1055.0,
            "binaryOdds": null,
            "decimalPlacesFactor": 1,
            "scalingFactor": 1,
            "controlledRiskExtraSpread": null
        }
        "#;

        let result: MarketSnapshot = serde_json::from_str(json).unwrap();

        assert_eq!(result.market_status, "TRADEABLE");
        assert_eq!(result.net_change, Some(23961.5));
        assert_eq!(result.percentage_change, Some(-0.66));
        assert_eq!(result.update_time, Some("04:35:47".to_string()));
        assert_eq!(result.delay_time, Some(0));
        assert_eq!(result.bid, Some(1086.0));
        assert_eq!(result.offer, Some(1091.0));
        assert_eq!(result.high, Some(1097.0));
        assert_eq!(result.low, Some(1055.0));
        assert_eq!(result.binary_odds, None);
        assert_eq!(result.decimal_places_factor, Some(1));
        assert_eq!(result.scaling_factor, Some(1));
        assert_eq!(result.controlled_risk_extra_spread, None);
    }

    /// Test edge cases and error scenarios
    #[test]
    fn test_edge_cases() {
        // Test with minimal valid JSON
        let minimal_json = r#"
        {
            "instrument": {
                "epic": "TEST.EPIC",
                "name": "Test Instrument",
                "expiry": "DFB",
                "contractSize": "1.0",
                "valueOfOnePip": "10.0"
            },
            "snapshot": {
                "marketStatus": "CLOSED"
            },
            "dealingRules": {
                "minStepDistance": {"unit": "POINTS"},
                "minDealSize": {"unit": "POINTS"},
                "minControlledRiskStopDistance": {"unit": "PERCENTAGE"},
                "minNormalStopOrLimitDistance": {"unit": "POINTS"},
                "maxStopOrLimitDistance": {"unit": "POINTS"},
                "controlledRiskSpacing": {"unit": "POINTS"},
                "marketOrderPreference": "AVAILABLE",
                "trailingStopsPreference": "AVAILABLE"
            }
        }
        "#;

        let result: Result<MarketDetails, _> = serde_json::from_str(minimal_json);
        assert!(result.is_ok());

        let market_details = result.unwrap();
        assert_eq!(market_details.snapshot.market_status, "CLOSED");

        // All StepDistance values should default to None when not provided
        assert_eq!(market_details.dealing_rules.min_step_distance.value, None);
        assert_eq!(market_details.dealing_rules.min_deal_size.value, None);
    }

    /// Test currency deserialization edge cases
    #[test]
    fn test_currency_edge_cases() {
        let json = r#"
        {
            "code": "USD",
            "symbol": null,
            "baseExchangeRate": 1.25,
            "exchangeRate": 1.0,
            "isDefault": false
        }
        "#;

        let result: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(result.code, "USD");
        assert_eq!(result.symbol, None);
        assert_eq!(result.base_exchange_rate, Some(1.25));
        assert_eq!(result.exchange_rate, Some(1.0));
        assert_eq!(result.is_default, Some(false));
    }
}
