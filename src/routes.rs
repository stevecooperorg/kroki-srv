use crate::askama_axum;
use crate::askama_axum::{IntoResponse, Template};
use crate::diags::BlockDiagram;

#[derive(Template)]
#[template(path = "Home.html")]
struct Home {
    title: String,
    blockdiag: BlockDiagram,
}

// basic handler that responds with a static string
pub async fn home() -> impl IntoResponse {
    // https://colorhunt.co/palette/f7ecdee9dac19ed2c654bab9
    const CHART_COL: &str = "#3b918f";
    const DEPLOY_COL: &str = "#54BAB9";
    const POD_COL: &str = "#9ED2C6";
    const NODE_COL: &str = "#E9DAC1";
    const NAMESPACE_COL: &str = "#F7ECDE";
    const DISABLED_COL: &str = "#ECECEC";
    const POD1: &str = "pod/web-svr-5gdtfg";
    const POD2: &str = "pod/web-svr-brgdyn";
    const POD3: &str = "pod/web-svr-jtvbdw";
    const NODE1: &'static str = "node/large-4453fbt";
    const NODE2: &'static str = "node/large-bgdfefw";
    const NOT_SCHED: &'static str = "node/<none>>";
    let diag = BlockDiagram::default()
        .with_colored_block("helm-chart", CHART_COL)
        .with_colored_block(POD1, POD_COL)
        .with_colored_block(POD2, POD_COL)
        .with_colored_block(POD3, DISABLED_COL)
        .with_colored_block(NODE1, NODE_COL)
        .with_colored_block(NODE2, NODE_COL)
        .with_colored_block(NOT_SCHED, DISABLED_COL)
        .with_colored_block("namespace/web", NAMESPACE_COL)
        .with_colored_block("deploy/web-svr", DEPLOY_COL)
        .with_colored_block("replica-set/deploy-bvfbd", DEPLOY_COL)
        .with_colored_block("replica-set/deploy-nrcvr", DEPLOY_COL)
        .with_edge("helm-chart", "deploy/web-svr")
        .with_edge("deploy/web-svr", POD1)
        .with_edge("deploy/web-svr", POD2)
        .with_edge("deploy/web-svr", POD3)
        .with_edge("deploy/web-svr", "replica-set/deploy-bvfbd")
        .with_edge("deploy/web-svr", "replica-set/deploy-nrcvr")
        .with_edge(POD1, NODE1)
        .with_edge(POD1, "namespace/web")
        .with_edge(POD2, NODE2)
        .with_edge(POD2, "namespace/web")
        .with_edge(POD3, NOT_SCHED)
        .with_edge(POD3, "namespace/web");

    let b64 = diag.to_kroki();
    println!("{}", b64);

    askama_axum::into_response(&Home {
        title: "Observability Example".to_string(),
        blockdiag: diag,
    })
}
