use std::collections::BTreeMap;
use crate::db::traits::{crud::CrudSubscribable, table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKey::{Text, Principal}, TableAllocatable, TableData, TableSubs}};
use crate::models::prescription::{PrescriptionId, Prescription};

pub struct PrescriptionTable {
    pub data: TableData<PrescriptionId, Prescription>,
    pub subs: TableSubs,
}

impl TableAllocatable<PrescriptionTable> for PrescriptionTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }
}

impl TableSerializable<PrescriptionId, Prescription> for PrescriptionTable {}

impl TableDeserializable<PrescriptionId, Prescription> for PrescriptionTable {}

impl TableSubscribable for PrescriptionTable {
    fn get_subs(
        &self
    ) -> &TableSubs {
        &self.subs
    }

    fn get_subs_mut(
        &mut self
    ) -> &mut TableSubs {
        &mut self.subs
    }
}

impl CrudSubscribable<PrescriptionId, Prescription> for PrescriptionTable {
    fn get_data(
        &self
    ) -> &TableData<PrescriptionId, Prescription> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PrescriptionId, Prescription> {
        &mut self.data
    }

    fn get_subs(
        &self
    ) -> &TableSubs {
        &self.subs
    }

    fn get_keys(
        k: &PrescriptionId,
        v: &Prescription
    ) -> Vec<crate::db::traits::table::TableEventKey> {
        vec![Text(k.clone()), Principal(v.doctor.clone())]
    }
}