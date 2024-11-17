use super::*;

#[derive(Debug, Default)]
pub struct Resources {
    objects: HashMap<uuid::Uuid, api::DynamicObject>,
}

impl Resources {
    pub fn get(&self, id: uuid::Uuid) -> Option<&api::DynamicObject> {
        self.objects.get(&id)
    }

    pub fn lookup_by_meta(&self, meta: &metav1::ObjectMeta) -> Option<&api::DynamicObject> {
        None
    }
}
