use k8s_openapi_ext as k8s;
use kube_mock::KubeMock;

use k8s::NamespaceExt;

use k8s::appsv1;
use k8s::corev1;

#[tokio::main]
async fn main() -> kube::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let (client, mock) = KubeMock::pair();
    // tokio::spawn(async move {
    //     {
    //         mock.run();
    //     }
    //     // // Force the `Rc` to stay in a scope with no `.await`
    //     // {
    //     //     let rc = Rc::new(());
    //     //     use_rc(rc.clone());
    //     // }

    //     tokio::task::yield_now().await;
    // })
    // // .await
    // // .unwrap()
    // ;
    tokio::spawn(mock.run());

    let nodes = kube::Api::<corev1::Node>::all(client.clone());
    let node = nodes.get("node-1").await?;
    println!("{node:?}");

    let namespaces = kube::Api::<corev1::Namespace>::all(client.clone());
    let ns = namespaces.get("default").await?;
    println!("{ns:?}");

    let pods = kube::Api::<corev1::Pod>::all(client.clone());
    let pod = pods.get("aa").await?;
    println!("{pod:?}");

    let deployments = kube::Api::<appsv1::Deployment>::namespaced(client.clone(), "bravo");
    let deployment = deployments.get("engine").await?;
    println!("{deployment:?}");

    let pp = kube::api::PostParams::default();
    let xxx = corev1::Namespace::new("xxx");
    let ns = namespaces.create(&pp, &xxx).await?;

    print!("XXX: {xxx:#?}");
    println!("NS : {ns:#?}");

    Ok(())
}
