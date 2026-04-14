/// Inventory Valuation Module — MOD-004
/// Rust translation of COBOL inventory_valuation.cob
/// FIFO (First-In, First-Out) inventory costing for ERP/mainframe migration.

use std::collections::VecDeque;

/// A single lot (batch) of inventory purchased at a specific unit cost.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryLot {
    pub quantity: u32,
    pub unit_cost: f64,
}

impl InventoryLot {
    pub fn new(quantity: u32, unit_cost: f64) -> Self {
        InventoryLot { quantity, unit_cost }
    }
}

/// FIFO inventory ledger for a single item.
#[derive(Debug, Clone)]
pub struct InventoryLedger {
    pub item_code: String,
    /// FIFO queue — oldest lots at the front.
    pub lots: VecDeque<InventoryLot>,
    /// Cumulative cost of goods sold.
    pub total_cogs: f64,
    /// Total units sold across all transactions.
    pub units_sold: u32,
}

impl InventoryLedger {
    /// Create an empty ledger for the given item code.
    pub fn new(item_code: &str) -> Self {
        InventoryLedger {
            item_code: item_code.to_string(),
            lots: VecDeque::new(),
            total_cogs: 0.0,
            units_sold: 0,
        }
    }

    /// Record a purchase — appends a new lot to the back of the FIFO queue.
    pub fn purchase(&mut self, quantity: u32, unit_cost: f64) {
        if quantity == 0 {
            return;
        }
        self.lots.push_back(InventoryLot::new(quantity, unit_cost));
    }

    /// Record a sale using FIFO costing.
    /// Consumes oldest lots first.
    /// Returns the COGS (cost of goods sold) for this transaction.
    /// Panics if there is insufficient inventory.
    pub fn sell(&mut self, quantity: u32) -> f64 {
        let on_hand = self.units_on_hand();
        assert!(
            quantity <= on_hand,
            "Insufficient inventory: requested {}, on hand {}",
            quantity,
            on_hand
        );

        let mut remaining = quantity;
        let mut cogs = 0.0;

        while remaining > 0 {
            let lot = self.lots.front_mut().expect("lots not empty — checked above");

            if lot.quantity <= remaining {
                // Consume the entire lot
                cogs += lot.quantity as f64 * lot.unit_cost;
                remaining -= lot.quantity;
                self.lots.pop_front();
            } else {
                // Partially consume this lot
                cogs += remaining as f64 * lot.unit_cost;
                lot.quantity -= remaining;
                remaining = 0;
            }
        }

        self.total_cogs += cogs;
        self.units_sold += quantity;
        cogs
    }

    /// Current market value of on-hand inventory at cost.
    pub fn current_value(&self) -> f64 {
        self.lots.iter().map(|l| l.quantity as f64 * l.unit_cost).sum()
    }

    /// Total units currently on hand.
    pub fn units_on_hand(&self) -> u32 {
        self.lots.iter().map(|l| l.quantity).sum()
    }

    /// Number of distinct lots currently held.
    pub fn lot_count(&self) -> usize {
        self.lots.len()
    }
}

/// Display a formatted inventory report to stdout.
pub fn display_report(ledger: &InventoryLedger) {
    println!("====================================");
    println!("INVENTORY VALUATION REPORT (FIFO)");
    println!("====================================");
    println!("Item Code:         {}", ledger.item_code);
    println!("Units Sold:        {}", ledger.units_sold);
    println!("Total COGS:        ${:.2}", ledger.total_cogs);
    println!("Units on Hand:     {}", ledger.units_on_hand());
    println!("Inventory Value:   ${:.2}", ledger.current_value());
    if ledger.units_sold > 0 {
        let avg_cost = ledger.total_cogs / ledger.units_sold as f64;
        println!("Avg COGS/Unit:     ${:.4}", avg_cost);
    }
    println!("====================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    /// FIFO ordering: oldest lots consumed first.
    #[test]
    fn test_fifo_ordering() {
        let mut ledger = InventoryLedger::new("TEST-001");
        ledger.purchase(100, 10.00); // lot 1 @ $10
        ledger.purchase(100, 20.00); // lot 2 @ $20

        // Sell 50 — should come entirely from lot 1 @ $10
        let cogs = ledger.sell(50);
        assert!(approx(cogs, 500.0, 1e-10), "Expected $500 COGS, got {}", cogs);
        // 50 units remain in lot 1, lot 2 intact
        assert_eq!(ledger.units_on_hand(), 150);
        assert_eq!(ledger.lot_count(), 2);
        assert!(approx(ledger.current_value(), 50.0 * 10.0 + 100.0 * 20.0, 1e-10));
    }

    /// Partial lot consumption: selling more than first lot forces crossing to second.
    #[test]
    fn test_partial_lot_consumption() {
        let mut ledger = InventoryLedger::new("WIDGET-A");
        ledger.purchase(100, 10.00); // lot 1
        ledger.purchase(200, 12.50); // lot 2
        ledger.purchase(150, 15.00); // lot 3

        // Sell 250 — consumes all of lot 1 (100 @ $10) + 150 of lot 2 (@ $12.50)
        // COGS = 100*10 + 150*12.50 = 1000 + 1875 = 2875
        let cogs = ledger.sell(250);
        assert!(approx(cogs, 2_875.0, 1e-10), "Expected $2875, got {}", cogs);
        assert_eq!(ledger.units_sold, 250);
        assert_eq!(ledger.units_on_hand(), 200); // 50 of lot 2 + 150 of lot 3
        // Remaining value: 50*12.50 + 150*15.00 = 625 + 2250 = 2875
        assert!(approx(ledger.current_value(), 2_875.0, 1e-10));
    }

    /// Selling exact lot size removes the lot entirely.
    #[test]
    fn test_exact_lot_consumption() {
        let mut ledger = InventoryLedger::new("ITEM-X");
        ledger.purchase(50, 5.0);
        ledger.purchase(50, 7.0);

        let cogs = ledger.sell(50); // removes entire first lot
        assert!(approx(cogs, 250.0, 1e-10));
        assert_eq!(ledger.lot_count(), 1);
        assert_eq!(ledger.units_on_hand(), 50);
        // Only lot 2 remains
        assert!(approx(ledger.current_value(), 350.0, 1e-10));
    }

    /// Zero inventory check: units_on_hand and current_value return 0.
    #[test]
    fn test_zero_inventory_state() {
        let ledger = InventoryLedger::new("EMPTY");
        assert_eq!(ledger.units_on_hand(), 0);
        assert!(approx(ledger.current_value(), 0.0, 1e-10));
        assert_eq!(ledger.lot_count(), 0);
        assert!(approx(ledger.total_cogs, 0.0, 1e-10));
    }

    /// COGS accumulates correctly across multiple sell transactions.
    #[test]
    fn test_cogs_accumulation() {
        let mut ledger = InventoryLedger::new("MULTI");
        ledger.purchase(200, 10.0);

        let c1 = ledger.sell(50);  // 50 * 10 = 500
        let c2 = ledger.sell(75);  // 75 * 10 = 750
        let c3 = ledger.sell(25);  // 25 * 10 = 250

        assert!(approx(c1, 500.0, 1e-10));
        assert!(approx(c2, 750.0, 1e-10));
        assert!(approx(c3, 250.0, 1e-10));
        assert!(approx(ledger.total_cogs, 1_500.0, 1e-10));
        assert_eq!(ledger.units_sold, 150);
        assert_eq!(ledger.units_on_hand(), 50);
    }

    /// Purchase after partial sell — new lot goes to back of queue.
    #[test]
    fn test_purchase_after_partial_sell() {
        let mut ledger = InventoryLedger::new("RESTOCK");
        ledger.purchase(100, 10.0);
        let _ = ledger.sell(60); // consume 60 from lot 1, 40 remain
        ledger.purchase(100, 15.0); // new lot at back

        // Next sell of 40 should consume remaining lot-1 units @ $10
        let cogs = ledger.sell(40);
        assert!(approx(cogs, 40.0 * 10.0, 1e-10), "Expected $400, got {}", cogs);
        // Only lot 2 remains
        assert_eq!(ledger.units_on_hand(), 100);
        assert!(approx(ledger.current_value(), 100.0 * 15.0, 1e-10));
    }

    /// Sell all inventory — balance goes to zero.
    #[test]
    fn test_sell_all_inventory() {
        let mut ledger = InventoryLedger::new("CLEAR");
        ledger.purchase(100, 8.0);
        ledger.purchase(200, 9.0);

        let cogs = ledger.sell(300);
        assert!(approx(cogs, 100.0 * 8.0 + 200.0 * 9.0, 1e-10));
        assert_eq!(ledger.units_on_hand(), 0);
        assert!(approx(ledger.current_value(), 0.0, 1e-10));
        assert_eq!(ledger.lot_count(), 0);
    }

    /// Parity with COBOL sample: 100@$10 + 200@$12.50 + 150@$15, sell 250.
    /// COBOL FIFO-SELL: COGS = 100*10 + 150*12.50 = 2875.
    #[test]
    fn test_parity_with_cobol_fifo_formula() {
        let mut ledger = InventoryLedger::new("WIDGET-A");
        ledger.purchase(100, 10.00);
        ledger.purchase(200, 12.50);
        ledger.purchase(150, 15.00);

        let cogs = ledger.sell(250);

        // Replicate COBOL FIFO-SELL arithmetic
        let cobol_cogs: f64 = 100.0 * 10.00 + 150.0 * 12.50;
        assert!(
            approx(cogs, cobol_cogs, 1e-10),
            "Rust COGS {:.2} != COBOL COGS {:.2}",
            cogs,
            cobol_cogs
        );

        // Remaining value: 50*12.50 + 150*15.00
        let cobol_remaining = 50.0 * 12.50 + 150.0 * 15.00;
        assert!(approx(ledger.current_value(), cobol_remaining, 1e-10));
    }

    /// Inventory value equals sum of (qty * cost) for all remaining lots.
    #[test]
    fn test_inventory_value_calculation() {
        let mut ledger = InventoryLedger::new("VALUE-CHECK");
        ledger.purchase(10, 5.0);
        ledger.purchase(20, 8.0);
        ledger.purchase(15, 12.0);

        let expected = 10.0 * 5.0 + 20.0 * 8.0 + 15.0 * 12.0;
        assert!(approx(ledger.current_value(), expected, 1e-10));
    }
}
