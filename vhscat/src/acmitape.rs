// generated by rust-bindgen 0.57.0, hacked down manually

use std::io;
use std::io::prelude::*;

use anyhow::*;
use serde_derive::*;

use crate::read_primitives::*;

#[derive(Debug, Default, Copy, Clone, Serialize)]
pub struct TapeHeader {
    pub file_id: [u8; 4],
    pub file_size: u32,
    pub entity_count: i32,
    pub feature_count: i32,
    pub entity_offset: u32,
    pub feature_offset: u32,
    pub position_count: i32,
    pub position_offset: u32,
    pub entity_event_offset: u32,
    pub general_event_offset: u32,
    pub general_event_trailer_offset: u32,
    pub text_event_offset: u32,
    pub feature_event_offset: u32,
    pub general_event_count: i32,
    pub entity_event_count: i32,
    pub text_event_count: i32,
    pub feature_event_count: i32,
    pub start_tim: f32,
    pub tot_play_time: f32,
    pub tod_offset: f32,
}

impl TapeHeader {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let mut file_id: [u8; 4] = [0; 4];
        r.read_exact(&mut file_id)?;

        let file_size = read_u32(r)?;
        let entity_count = read_i32(r)?;
        let feature_count = read_i32(r)?;
        let entity_offset = read_u32(r)?;
        let feature_offset = read_u32(r)?;
        let position_count = read_i32(r)?;
        let position_offset = read_u32(r)?;
        let entity_event_offset = read_u32(r)?;
        let general_event_offset = read_u32(r)?;
        let general_event_trailer_offset = read_u32(r)?;
        let text_event_offset = read_u32(r)?;
        let feature_event_offset = read_u32(r)?;
        let general_event_count = read_i32(r)?;
        let entity_event_count = read_i32(r)?;
        let text_event_count = read_i32(r)?;
        let feature_event_count = read_i32(r)?;
        let start_tim = read_f32(r)?;
        let tot_play_time = read_f32(r)?;
        let tod_offset = read_f32(r)?;

        Ok(Self {
            file_id,
            file_size,
            entity_count,
            feature_count,
            entity_offset,
            feature_offset,
            position_count,
            position_offset,
            entity_event_offset,
            general_event_offset,
            general_event_trailer_offset,
            text_event_offset,
            feature_event_offset,
            general_event_count,
            entity_event_count,
            text_event_count,
            feature_event_count,
            start_tim,
            tot_play_time,
            tod_offset,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Entity {
    pub uid: i32,
    pub kind: i32,
    pub count: i32,
    pub flags: u32,
    pub lead_index: i32,
    pub slot: i32,
    pub special_flags: u32,
    pub first_position_offset: u32,
    pub first_event_offset: u32,
}

impl Entity {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let uid = read_i32(r)?;
        let kind = read_i32(r)?;
        let count = read_i32(r)?;
        let flags = read_u32(r)?;
        let lead_index = read_i32(r)?;
        let slot = read_i32(r)?;
        let special_flags = read_u32(r)?;
        let first_position_offset = read_u32(r)?;
        let first_event_offset = read_u32(r)?;

        Ok(Self {
            uid,
            kind,
            count,
            flags,
            lead_index,
            slot,
            special_flags,
            first_position_offset,
            first_event_offset,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct TimelineEntry {
    pub time: f32,
    pub payload: TimelineEntryPayload,
    pub next_update_offset: u32,
    pub previous_update_offset: u32,
}

impl TimelineEntry {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let time = read_f32(r)?;
        let payload = match read_u8(r)? {
            0 => TimelineEntryPayload::Pos(Position::read(r)?),
            1 => TimelineEntryPayload::Switch(Switch::read(r)?),
            2 => TimelineEntryPayload::DOF(DOF::read(r)?),
            wut => bail!("Invalid timeline entry type: {}", wut),
        };
        let next_update_offset = read_u32(r)?;
        let previous_update_offset = read_u32(r)?;

        Ok(Self {
            time,
            payload,
            next_update_offset,
            previous_update_offset,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub enum TimelineEntryPayload {
    Pos(Position),
    Switch(Switch),
    DOF(DOF),
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub radar_target: i32,
}

impl Position {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let x = read_f32(r)?;
        let y = read_f32(r)?;
        let z = read_f32(r)?;
        let pitch = read_f32(r)?;
        let roll = read_f32(r)?;
        let yaw = read_f32(r)?;
        let radar_target = read_i32(r)?;

        Ok(Self {
            x,
            y,
            z,
            pitch,
            roll,
            yaw,
            radar_target,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Switch {
    pub switch_index: i32,
    pub switch_value: i32,
    pub previous_switch_value: i32,
}

impl Switch {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let switch_index = read_i32(r)?;
        let switch_value = read_i32(r)?;
        let previous_switch_value = read_i32(r)?;

        // The other union values have another four 4-byte values
        io::copy(&mut r.take(16), &mut io::sink())?;

        Ok(Self {
            switch_index,
            switch_value,
            previous_switch_value,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct DOF {
    pub dof_index: i32,
    pub dof_value: f32,
    pub previous_dof_value: f32,
}

impl DOF {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let dof_index = read_i32(r)?;
        let dof_value = read_f32(r)?;
        let previous_dof_value = read_f32(r)?;

        // The other union values have another four 4-byte values
        io::copy(&mut r.take(16), &mut io::sink())?;

        Ok(Self {
            dof_index,
            dof_value,
            previous_dof_value,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct GeneralEventHeader {
    pub event_type: u8,
    pub index: i32,
    pub time: f32,
    pub time_end: f32,
    pub kind: i32,
    pub user: i32,
    pub flags: i32,
    pub scale: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl GeneralEventHeader {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let event_type = read_u8(r)?;
        let index = read_i32(r)?;
        let time = read_f32(r)?;
        let time_end = read_f32(r)?;
        let kind = read_i32(r)?;
        let user = read_i32(r)?;
        let flags = read_i32(r)?;
        let scale = read_f32(r)?;
        let x = read_f32(r)?;
        let y = read_f32(r)?;
        let z = read_f32(r)?;
        let dx = read_f32(r)?;
        let dy = read_f32(r)?;
        let dz = read_f32(r)?;
        let roll = read_f32(r)?;
        let pitch = read_f32(r)?;
        let yaw = read_f32(r)?;

        Ok(Self {
            event_type,
            index,
            time,
            time_end,
            kind,
            user,
            flags,
            scale,
            x,
            y,
            z,
            dx,
            dy,
            dz,
            roll,
            pitch,
            yaw,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct GeneralEventTrailer {
    pub time_end: f32,
    pub index: i32,
}

impl GeneralEventTrailer {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let time_end = read_f32(r)?;
        let index = read_i32(r)?;

        Ok(Self { time_end, index })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct FeatureEvent {
    pub time: f32,
    pub index: i32,
    pub new_status: i32,
    pub previous_status: i32,
}

impl FeatureEvent {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let time = read_f32(r)?;
        let index = read_i32(r)?;
        let new_status = read_i32(r)?;
        let previous_status = read_i32(r)?;

        Ok(Self {
            time,
            index,
            new_status,
            previous_status,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CallsignRecord {
    pub label: String,
    pub team_color: i32,
}

impl CallsignRecord {
    pub fn read<R: Read>(r: &mut R) -> Result<Self> {
        let mut label_bytes: [u8; 16] = [0; 16];
        r.read_exact(&mut label_bytes)?;

        let label_len = label_bytes.iter().position(|c| *c == 0).unwrap_or(16);
        let label = String::from_utf8_lossy(&label_bytes[0..label_len]).to_string();
        let team_color = read_i32(r)?;
        Ok(Self { label, team_color })
    }
}
