use chrono::NaiveDateTime;
use kdbx_rs::Database as KdbxDatabase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub(crate) enum Value {
//     /// A value using in-memory encryption
//     Protected(String),
//     /// A value that's unencrypted in the database
//     Standard(String),
//     /// A empty value
//     Empty,
//     /// A empty value that should be protected if filled
//     ProtectEmpty,
// }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Audit times for this item
pub struct Times {
    /// Time last edited
    pub last_modification_time: NaiveDateTime,
    /// Time created
    pub creation_time: NaiveDateTime,
    /// Time last accessed
    pub last_access_time: NaiveDateTime,
    /// Time at which this password needs rotation
    pub expiry_time: NaiveDateTime,
    /// Time at which this password was last moved within the database
    pub location_changed: NaiveDateTime,
    /// Whether this password expires
    pub expires: bool,
    /// Count of usages with autofill functions
    pub usage_count: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub uuid: Uuid,
    pub override_url: Option<String>,
    // pub tags: Vec<String>,
    pub times: Times,
    pub fields: Vec<Field>,
    // pub binaries: Vec<Binary>,
    // pub history: Vec<History>,
    // pub auto_type: AutoType,
    // pub custom_icon_uuid: Uuid,
    // pub custom_icon_data: Vec<u8>,
    // pub expires: Times,
    // pub usage_count: u32,
    // pub location_changed: Times,
    // pub string: String,
    // pub parent_group_uuid: Uuid,
    // pub parent_group: Group,
}

impl Entry {
    pub fn new(ref_entry: kdbx_rs::database::Entry) -> Self {
        Self {
            uuid: ref_entry.uuid(),
            override_url: ref_entry
                .url()
                .is_some()
                .then(|| ref_entry.url().unwrap().to_string()),
            // tags: ref_entry..map(|t| t.to_string()).collect(),
            times: Times {
                last_modification_time: ref_entry.times().last_modification_time,
                creation_time: ref_entry.times().creation_time,
                last_access_time: ref_entry.times().last_access_time,
                expiry_time: ref_entry.times().expiry_time,
                location_changed: ref_entry.times().location_changed,
                expires: ref_entry.times().expires,
                usage_count: ref_entry.times().usage_count,
            },
            fields: ref_entry
                .fields()
                .map(|f| Field {
                    key: f.key().to_string(),
                    value: f.value().is_some().then(|| f.value().unwrap().to_string()),
                })
                .collect(),
            // parent_group_uuid: ref_entry.parent_group_uuid(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A group or folder of password entries and child groups
pub struct Group {
    /// Identifier for this group
    uuid: Uuid,
    /// Name of this group
    name: String,
    /// Password items within this group
    entries: Vec<Entry>,
    /// Subfolders of this group
    groups: Vec<Group>,
    /// Access times for this group
    pub(crate) times: Times,
}

impl Group {
    pub fn new(ref_group: kdbx_rs::database::Group) -> Self {
        // let mut groups = vec![];
        // for group in ref_group.groups() {
        //     groups.push(Group::new(group));
        // }
        Self {
            uuid: ref_group.uuid(),
            name: ref_group.name().to_string(),
            entries: ref_group.entries().map(|e| Entry::new(e.clone())).collect(),
            groups: ref_group.groups().map(|g| Group::new(g.clone())).collect(),
            times: Times {
                last_modification_time: ref_group.times().last_modification_time,
                creation_time: ref_group.times().creation_time,
                last_access_time: ref_group.times().last_access_time,
                expiry_time: ref_group.times().expiry_time,
                location_changed: ref_group.times().location_changed,
                expires: ref_group.times().expires,
                usage_count: ref_group.times().usage_count,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub database_name: String,
    // pub memory_protection: MemoryProtection,
    pub custom_data: Vec<Field>,
    pub database_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    meta: Meta,
    groups: Vec<Group>,
}

impl Database {
    pub fn new(ref_database: KdbxDatabase) -> Self {
        let mut groups = vec![];
        for group in ref_database.root().groups() {
            groups.push(Group::new(group.clone()));
        }
        Self {
            meta: Meta {
                database_name: ref_database.name().to_string(),
                custom_data: vec![],
                database_description: ref_database.description().to_string(),
            },
            groups,
        }

        // Self {
        //     meta: Meta {
        //         database_name: "New Database".to_string(),
        //         custom_data: vec![],
        //         database_description: "".to_string(),
        //     },
        //     groups: vec![],
        // }
    }
}
