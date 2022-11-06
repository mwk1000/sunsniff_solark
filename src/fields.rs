/* Copyright 2022 Bruce Merry
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::ops::Range;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldType {
    Charge,
    Current,
    Energy,
    Frequency,
    Power,
    StateOfCharge,
    Temperature,
    Voltage,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub field_type: FieldType,
    pub offset: usize,
    pub group: &'a str,
    pub name: &'a str,
    pub id: &'a str,
    pub scale: f64,
    pub bias: f64,
    pub unit: &'a str,
}

impl<'a> Field<'a> {
    pub const fn power(offset: usize, group: &'a str, id: &'a str) -> Self {
        Field {
            field_type: FieldType::Power,
            offset,
            group,
            name: "Power",
            id,
            scale: 1.0,
            bias: 0.0,
            unit: "W"
        }
    }

    pub const fn voltage(offset: usize, group: &'a str, id: &'a str) -> Self {
        Field {
            field_type: FieldType::Voltage,
            offset,
            group,
            name: "Voltage",
            id,
            scale: 0.1,
            bias: 0.0,
            unit: "V"
        }
    }

    pub const fn current(offset: usize, group: &'a str, id: &'a str) -> Self {
        Field {
            field_type: FieldType::Current,
            offset,
            group,
            name: "Current",
            id,
            scale: 0.01,
            bias: 0.0,
            unit: "A"
        }
    }

    pub const fn temperature_name(
        offset: usize,
        group: &'a str,
        name: &'a str,
        id: &'a str,
    ) -> Self {
        Field {
            field_type: FieldType::Temperature,
            offset,
            group,
            name,
            id,
            scale: 0.1,
            bias: -100.0,
            unit: "°C"
        }
    }

    pub const fn temperature(offset: usize, group: &'a str, id: &'a str) -> Self {
        Field::temperature_name(offset, group, "Temperature", id)
    }

    pub const fn frequency(offset: usize, group: &'a str, id: &'a str) -> Self {
        Field {
            field_type: FieldType::Frequency,
            offset,
            group,
            name: "Frequency",
            id,
            scale: 0.01,
            bias: 0.0,
            unit: "Hz"
        }
    }

    pub const fn energy(offset: usize, group: &'a str, name: &'a str, id: &'a str) -> Self {
        // TODO: these are probably 32-bit values, but more investigation is
        // needed to figure out where the high bits live.
        Field {
            field_type: FieldType::Energy,
            offset,
            group,
            name,
            id,
            scale: 0.1,
            bias: 0.0,
            unit: "kWh"
        }
    }

    pub const fn charge(offset: usize, group: &'a str, name: &'a str, id: &'a str) -> Self {
        Field {
            field_type: FieldType::Charge,
            offset,
            group,
            name,
            id,
            scale: 1.0,
            bias: 0.0,
            unit: "Ah"
        }
    }

    pub const fn state_of_charge(offset: usize, group: &'a str, id: &'a str) -> Self {
        Field {
            field_type: FieldType::StateOfCharge,
            offset,
            group,
            name: "SOC",
            id,
            scale: 1.0,
            bias: 0.0,
            unit: "%"
        }
    }
}

pub const MAGIC_LENGTH: usize = 292;
pub const MAGIC_HEADER: u8 = 0xa5; // First byte in the packet
pub const SERIAL_RANGE: Range<usize> = 11..21;
pub const DATETIME_OFFSET: usize = 37;
pub const FIELDS: &[Field] = &[
    Field::energy(70, "Battery", "Total charge", "battery_charge_total"),
    Field::energy(74, "Battery", "Total discharge", "battery_discharge_total"),
    Field::energy(82, "Grid", "Total import", "grid_import_total"),
    Field::energy(88, "Grid", "Total export", "grid_export_total"),
    Field::frequency(84, "Grid", "grid_frequency"),
    Field::energy(96, "Load", "Total consumption", "load_consumption_total"),
    Field::temperature_name(106, "Inverter", "DC Temperature", "inverter_temperature_dc"),
    Field::temperature_name(108, "Inverter", "AC Temperature", "inverter_temperature_ac"),
    Field::energy(118, "PV", "Total production", "pv_production_total"),
    Field::charge(140, "Battery", "Capacity", "battery_capacity"),
    Field::voltage(176, "Grid", "grid_voltage"),
    Field::voltage(184, "Load", "load_voltage"),
    Field::power(216, "Grid", "grid_power"),
    Field::power(228, "Load", "load_power"),
    Field::temperature(240, "Battery", "battery_temperature"),
    Field::state_of_charge(244, "Battery", "battery_soc"),
    Field::power(248, "PV", "pv_power"),
    Field::power(256, "Battery", "battery_power"),
    Field::current(258, "Battery", "battery_current"),
    Field::frequency(260, "Load", "load_frequency"),
];