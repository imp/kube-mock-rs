use super::*;

#[derive(Clone, Debug)]
pub enum Operation {
    Create(api::DynamicObject),
    Delete(api::DynamicObject),
    Get(api::DynamicObject),
    List(api::DynamicObject),
}

impl Operation {
    pub fn new(
        method: &http::Method,
        item: Item,
        object: api::DynamicObject,
    ) -> Result<Self, metav1::Status> {
        match *method {
            http::Method::GET if item.is_object() => Ok(Self::Get(object)),
            http::Method::GET if item.is_kind() => Ok(Self::List(object)),
            http::Method::PUT if item.is_kind() => Ok(Self::Create(object)),
            http::Method::DELETE if item.is_object() => Ok(Self::Delete(object)),
            _ => Err(metav1::Status::method_not_allowed()),
            //
            // http::Method::POST => todo!(),
            // http::Method::PATCH => todo!(),
            // http::Method::TRACE => todo!(),
            // http::Method::OPTIONS => todo!(),
        }
    }

    pub fn dynamic_object(&self) -> &api::DynamicObject {
        match self {
            Self::Create(dynamic_object) => dynamic_object,
            Self::Delete(dynamic_object) => dynamic_object,
            Self::Get(dynamic_object) => dynamic_object,
            Self::List(dynamic_object) => dynamic_object,
        }
    }

    pub fn type_meta(&self) -> Option<&api::TypeMeta> {
        self.dynamic_object().types.as_ref()
    }
}
