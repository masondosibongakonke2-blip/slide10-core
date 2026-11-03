mod mode;
mod telemetry;
mod filter;
mod integrity;
mod reconcile;
mod persist;
mod crypto;
mod mesh;

use crypto::Keypair;
use filter::{EdgeFilter, EdgePacket};
use integrity::{sha256_hex, ContentAddressedBlob};
use mesh::{InMemoryMesh, MeshMessage, MeshTransport};
use persist::Store;
use reconcile::{OfflineRecord, Reconciler};
use telemetry::LinkTelemetry;

fn main() {
    println!("=== SLIDE 10 CORE v0.3 ===");

    // Telemetry -> adaptive mode
    let link = LinkTelemetry {
        distance_m: 1500.0,
        fade_margin_db: 4.0,
        packet_loss: 0.33,
        latency_ms: 1200.0,
    };
    let mode = link.suggest_mode();
    println!("mode={} threshold={}", mode.label(), mode.quorum_threshold());

    // Edge filter
    let filter = EdgeFilter::new(40);
    for p in [
        EdgePacket { id: "p1".into(), priority: 92, anomalous: false, payload_size: 100 },
        EdgePacket { id: "p2".into(), priority: 15, anomalous: false, payload_size: 900 },
        EdgePacket { id: "p3".into(), priority: 88, anomalous: true, payload_size: 70 },
    ] {
        println!("packet {} admitted={}", p.id, filter.admit(&p));
    }

    // Signatures + hashes
    let kp = Keypair::generate();
    let payload = b"route=A->B;ttl=30".to_vec();
    let digest = sha256_hex(&payload);
    let sig = kp.sign(&payload);
    println!("digest={digest}");
    println!("signature_ok={}", crypto::verify(&kp.verifying, &payload, &sig).is_ok());

    // Tamper detection
    let mut blob = ContentAddressedBlob::new("cfg", payload.clone());
    println!("clean_verify={:?}", blob.verify());
    blob.data[0] ^= 0x01;
    println!("tamper_verify={:?}", blob.verify());

    // Persistence
    let store = Store::open("slide10_demo.db").expect("db");
    let clean = b"state=ok;seq=1".to_vec();
    let clean_digest = sha256_hex(&clean);
    let clean_sig = kp.sign(&clean);
    store.upsert("rec-1", 1, &clean_digest, &clean, Some(&clean_sig)).unwrap();

    let mut bad = b"state=ok;seq=2".to_vec();
    let bad_digest = sha256_hex(&bad);
    bad[0] ^= 0x01;
    store.upsert("rec-2", 2, &bad_digest, &bad, None).unwrap();

    // Mesh test transport
    let mut mesh = InMemoryMesh::new();
    mesh.send(MeshMessage {
        from: "node-a".into(),
        to: "node-b".into(),
        body: b"hello-partition".to_vec(),
    });
    println!("mesh_received={}", mesh.recv("node-b").len());

    // Reconcile pending
    let pending = store.list_pending().unwrap();
    let mut reconciler = Reconciler::default();
    let records = pending
        .into_iter()
        .map(|(id, seq, digest, payload, _sig)| OfflineRecord {
            id,
            sequence: seq as u64,
            blob: ContentAddressedBlob {
                label: "rec".into(),
                data: payload,
                digest_hex: digest,
            },
        })
        .collect();
    reconciler.apply_bundle_tolerant(records);
    for id in &reconciler.applied {
        store.mark_applied(id).ok();
    }
    println!("applied={:?}", reconciler.applied);
    println!("rejected={:?}", reconciler.rejected);
    println!("=== COMPLETE ===");
                    }
