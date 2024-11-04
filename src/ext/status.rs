use super::*;

pub trait StatusExt {
    fn failure(reason: impl ToString) -> Self;
    fn not_found<K>(name: impl ToString) -> Self
    where
        K: kube::Resource<DynamicType = ()>;
    fn already_exists<K>(name: impl ToString) -> Self
    where
        K: kube::Resource<DynamicType = ()>;

    fn no_such_resource() -> Self;
    fn method_not_allowed() -> Self;
    fn bad_request(message: impl ToString) -> Self;
}

impl StatusExt for metav1::Status {
    fn failure(reason: impl ToString) -> Self {
        Self {
            status: Some("Failure".to_string()),
            reason: Some(reason.to_string()),
            ..Self::default()
        }
    }
    fn not_found<K>(name: impl ToString) -> Self
    where
        K: kube::Resource<DynamicType = ()>,
    {
        let name = name.to_string();
        let group = K::group(&());
        let plural = K::plural(&());
        let message = if group.is_empty() {
            format!("{plural} \"{name}\" not found")
        } else {
            format!("{plural}.{group} \"{name}\" not found")
        };

        let details = metav1::StatusDetails {
            causes: None,
            group: Some(group.to_string()),
            kind: Some(plural.to_string()),
            name: Some(name.to_string()),
            retry_after_seconds: None,
            uid: None,
        };

        Self {
            code: Some(404),
            details: Some(details),
            message: Some(message),
            ..Self::failure("NotFound")
        }
    }

    // {
    //     "apiVersion": "v1",
    //     "code": 409,
    //     "details": {
    //         "kind": "configmaps",
    //         "name": "mockcm"
    //     },
    //     "kind": "Status",
    //     "message": "configmaps \"mockcm\" already exists",
    //     "metadata": {},
    //     "reason": "AlreadyExists",
    //     "status": "Failure"
    // }
    fn already_exists<K>(name: impl ToString) -> Self
    where
        K: kube::Resource<DynamicType = ()>,
    {
        let name = name.to_string();
        let message = message::<K>(&name, "already exists");
        Self {
            code: Some(409),
            details: Some(details::<K>(&name)),
            message: Some(message),
            ..Self::failure("AlreadyExists")
        }
    }

    // {
    //   "kind": "Status",
    //   "apiVersion": "v1",
    //   "metadata": {},
    //   "status": "Failure",
    //   "message": "the server could not find the requested resource",
    //   "reason": "NotFound",
    //   "details": {},
    //   "code": 404
    // }

    fn no_such_resource() -> Self {
        Self {
            code: Some(404),
            details: Some(metav1::StatusDetails::default()),
            message: Some("the server could not find the requested resource".to_string()),
            ..Self::failure("NotFound")
        }
    }

    // {
    //   "kind": "Status",
    //   "apiVersion": "v1",
    //   "metadata": {},
    //   "status": "Failure",
    //   "message": "the server does not allow this method on the requested resource",
    //   "reason": "MethodNotAllowed",
    //   "details": {},
    //   "code": 405
    // }
    fn method_not_allowed() -> Self {
        Self {
            code: Some(405),
            details: Some(metav1::StatusDetails::default()),
            message: Some(
                "the server does not allow this method on the requested resource".to_string(),
            ),
            ..Self::failure("MethodNotAllowed")
        }
    }

    // {
    //     "apiVersion": "v1",
    //     "code": 400,
    //     "kind": "Status",
    //     "message": "ConfigMap in version \"v1\" cannot be handled as a ConfigMap: json: cannot unmarshal number into Go struct field ConfigMap.data of type map[string]string",
    //     "metadata": {},
    //     "reason": "BadRequest",
    //     "status": "Failure"
    // }
    fn bad_request(message: impl ToString) -> Self {
        Self {
            code: Some(400),
            message: Some(message.to_string()),
            ..Self::failure("BadRequest")
        }
    }
}

fn message<K>(name: &str, text: &str) -> String
where
    K: kube::Resource<DynamicType = ()>,
{
    let group = K::group(&());
    let plural = K::plural(&());
    if group.is_empty() {
        format!("{plural} \"{name}\" {text}")
    } else {
        format!("{plural}.{group} \"{name}\" {text}")
    }
}

fn details<K>(name: &str) -> metav1::StatusDetails
where
    K: kube::Resource<DynamicType = ()>,
{
    metav1::StatusDetails {
        causes: None,
        group: Some(K::group(&()).to_string()),
        kind: Some(K::plural(&()).to_string()),
        name: Some(name.to_string()),
        retry_after_seconds: None,
        uid: None,
    }
}
