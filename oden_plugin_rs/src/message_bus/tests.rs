use super::{host, BusClient, BusOwner, RecvFail, SendError};
use crate::{math::Uuid, settings_api::MockSettingsApi, QueryError, QueryMetadata, SettingsApi};
use std::{
    collections::HashMap,
    ffi::c_void,
    future::Future,
    pin::pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Barrier, Mutex,
    },
    task::{Context, Poll, Wake, Waker},
    thread,
    time::{Duration, Instant},
};

fn test_api() -> MockSettingsApi {
    let api = test_api_without_host_factory();
    api.publish_data(host::HOST_FACTORY_KEY, &host::host_factory_record());
    api
}

fn test_api_without_host_factory() -> MockSettingsApi {
    let data = Arc::new(Mutex::new(HashMap::<String, Vec<u8>>::new()));
    let mut api = MockSettingsApi::new();
    let publish_data = Arc::clone(&data);
    api.expect_publish_data().returning(move |data_id, bytes| {
        publish_data
            .lock()
            .unwrap()
            .insert(data_id.to_string(), bytes.to_vec());
    });
    api.expect_query_data()
        .returning(move |data_id, expected_data_size| {
            let map = data.lock().unwrap();
            let bytes = map.get(data_id).ok_or(QueryError::DataIdNotFound)?;
            if expected_data_size >= 0 && bytes.len() != expected_data_size as usize {
                return Err(QueryError::DataIsNotNotExpectedSize);
            }
            Ok((
                bytes.clone(),
                QueryMetadata {
                    timestampPublished: 0,
                    framesSincePublished: 0,
                },
            ))
        });
    api
}

struct ThreadWaker(thread::Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn thread_waker() -> Waker {
    Waker::from(Arc::new(ThreadWaker(thread::current())))
}

struct CountingWaker {
    wakes: AtomicUsize,
    thread: thread::Thread,
}

impl Wake for CountingWaker {
    fn wake(self: Arc<Self>) {
        self.wakes.fetch_add(1, Ordering::SeqCst);
        self.thread.unpark();
    }
}

fn counting_waker() -> (Arc<CountingWaker>, Waker) {
    let counter = Arc::new(CountingWaker {
        wakes: AtomicUsize::new(0),
        thread: thread::current(),
    });
    (counter.clone(), Waker::from(counter))
}

fn block_on_timeout<F: Future>(fut: F, timeout: Duration) -> F::Output {
    let deadline = Instant::now() + timeout;
    let waker = thread_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => {
                let now = Instant::now();
                if now >= deadline {
                    panic!("future did not resolve within {timeout:?}");
                }
                thread::park_timeout(deadline - now);
            }
        }
    }
}

const UUID_A: Uuid = Uuid::parse("11111111-1111-1111-1111-111111111111");

#[test]
fn broadcast_round_trip_with_name_filtering() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub_a = client.subscribe(&["a"]);
    let sub_ab = client.subscribe(&["a", "b"]);

    owner.broadcast("a", b"payload_a").unwrap();
    owner.broadcast("b", b"payload_b").unwrap();
    owner.broadcast("c", b"payload_c").unwrap();

    let msg = sub_a.try_recv().unwrap();
    assert_eq!(msg.name, "a");
    assert_eq!(msg.payload, b"payload_a");
    assert!(sub_a.try_recv().is_none());

    let msg = sub_ab.try_recv().unwrap();
    assert_eq!(msg.name, "a");
    let msg = sub_ab.try_recv().unwrap();
    assert_eq!(msg.name, "b");
    assert_eq!(msg.payload, b"payload_b");
    assert!(sub_ab.try_recv().is_none());
}

#[test]
fn upstream_round_trip() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    client.send("up", b"hello").unwrap();
    let msg = owner.try_recv_upstream().unwrap();
    assert_eq!(msg.name, "up");
    assert_eq!(msg.payload, b"hello");
    assert!(owner.try_recv_upstream().is_none());
}

#[test]
fn large_messages_grow_receive_buffers() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    let long_name = "n".repeat(1024);
    let big_payload = vec![0xABu8; 64 * 1024];
    let sub = client.subscribe(&[long_name.as_str()]);

    owner.broadcast(&long_name, &big_payload).unwrap();
    let msg = sub.try_recv().unwrap();
    assert_eq!(msg.name, long_name);
    assert_eq!(msg.payload, big_payload);

    client.send(&long_name, &big_payload).unwrap();
    let msg = owner.recv_upstream_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(msg.name, long_name);
    assert_eq!(msg.payload, big_payload);
}

#[test]
fn boundary_sized_messages_round_trip() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    for (name_len, payload_len) in [(64, 256), (65, 257), (128, 512)] {
        let name = "x".repeat(name_len);
        let payload = vec![7u8; payload_len];
        let sub = client.subscribe(&[name.as_str()]);
        owner.broadcast(&name, &payload).unwrap();
        let msg = sub.try_recv().unwrap();
        assert_eq!(msg.name.len(), name_len);
        assert_eq!(msg.payload, payload);
    }
}

#[test]
fn recv_timeout_times_out_and_wakes_on_send() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    assert!(sub.recv_timeout(Duration::ZERO).is_none());
    assert!(sub.recv_timeout(Duration::from_millis(20)).is_none());

    let barrier = Arc::new(Barrier::new(2));
    let sender_barrier = Arc::clone(&barrier);
    let sender = thread::spawn(move || {
        sender_barrier.wait();
        owner.broadcast("a", b"wake").unwrap();
        owner
    });

    barrier.wait();
    let start = Instant::now();
    let msg = sub.recv_timeout(Duration::from_secs(10)).unwrap();
    assert_eq!(msg.payload, b"wake");
    assert!(start.elapsed() < Duration::from_secs(5));
    sender.join().unwrap();
}

#[test]
fn blocked_receiver_survives_owner_restart() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let barrier = Arc::new(Barrier::new(2));
    let receiver_barrier = Arc::clone(&barrier);
    let receiver = thread::spawn(move || {
        receiver_barrier.wait();
        sub.recv_timeout(Duration::from_secs(30))
    });

    barrier.wait();
    drop(owner);
    let owner2 = BusOwner::open(&api, UUID_A);
    owner2.broadcast("a", b"second life").unwrap();

    let result = receiver.join().unwrap();
    assert_eq!(result.unwrap().payload, b"second life");
}

#[test]
fn queued_messages_survive_owner_drop() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    owner.broadcast("a", b"one").unwrap();
    owner.broadcast("a", b"two").unwrap();
    drop(owner);

    assert_eq!(sub.try_recv().unwrap().payload, b"one");
    assert_eq!(sub.try_recv().unwrap().payload, b"two");
    assert!(sub.try_recv().is_none());
}

#[test]
fn upstream_sends_buffer_while_no_owner() {
    let api = test_api();
    let client = BusClient::open(&api, UUID_A);

    client.send("up", b"early one").unwrap();
    client.send("up", b"early two").unwrap();

    let owner = BusOwner::open(&api, UUID_A);
    assert_eq!(owner.try_recv_upstream().unwrap().payload, b"early one");
    assert_eq!(owner.try_recv_upstream().unwrap().payload, b"early two");
    assert!(owner.try_recv_upstream().is_none());
}

#[test]
fn subscription_survives_owner_restart() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    owner.broadcast("a", b"before").unwrap();
    drop(owner);

    let owner2 = BusOwner::open(&api, UUID_A);
    owner2.broadcast("a", b"after").unwrap();

    assert_eq!(sub.try_recv().unwrap().payload, b"before");
    assert_eq!(sub.try_recv().unwrap().payload, b"after");

    client.send("up", b"to new owner").unwrap();
    assert_eq!(owner2.try_recv_upstream().unwrap().payload, b"to new owner");
}

#[test]
fn subscribe_while_no_owner_parks_nothing_and_receives_later() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    drop(owner);

    let sub = client.subscribe(&["a"]);
    assert!(sub.try_recv().is_none());

    let owner2 = BusOwner::open(&api, UUID_A);
    owner2.broadcast("a", b"from successor").unwrap();
    assert_eq!(sub.try_recv().unwrap().payload, b"from successor");
}

#[test]
fn cloned_owner_keeps_broadcasting_after_original_drops() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let owner_clone = owner.clone();
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    drop(owner);
    owner_clone.broadcast("a", b"still here").unwrap();
    assert_eq!(sub.try_recv().unwrap().payload, b"still here");
}

#[test]
fn queue_full_drops_newest_and_reports() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe_with_capacity(&["a"], 2);

    owner.broadcast("a", b"one").unwrap();
    owner.broadcast("a", b"two").unwrap();
    assert_eq!(owner.broadcast("a", b"three"), Err(SendError::QueueFull));

    assert_eq!(sub.try_recv().unwrap().payload, b"one");
    assert_eq!(sub.try_recv().unwrap().payload, b"two");
    assert!(sub.try_recv().is_none());
}

#[test]
fn dropped_subscription_stops_delivery() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub_kept = client.subscribe(&["a"]);
    let sub_dropped = client.subscribe(&["a"]);
    drop(sub_dropped);

    owner.broadcast("a", b"still works").unwrap();
    assert_eq!(sub_kept.try_recv().unwrap().payload, b"still works");
}

#[test]
fn broadcast_before_subscribe_is_parked_and_claimed_once() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    owner.broadcast("early", b"one").unwrap();
    owner.broadcast("early", b"two").unwrap();
    owner.broadcast("other", b"three").unwrap();

    let sub = client.subscribe(&["early"]);
    assert_eq!(sub.try_recv().unwrap().payload, b"one");
    assert_eq!(sub.try_recv().unwrap().payload, b"two");
    assert!(sub.try_recv().is_none());

    let late = client.subscribe(&["early"]);
    assert!(late.try_recv().is_none());

    let other = client.subscribe(&["other"]);
    assert_eq!(other.try_recv().unwrap().payload, b"three");
}

#[test]
fn matched_broadcast_is_not_parked_for_later_subscribers() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    let first = client.subscribe(&["a"]);
    owner.broadcast("a", b"seen").unwrap();

    let late = client.subscribe(&["a"]);
    assert!(late.try_recv().is_none());
    assert_eq!(first.try_recv().unwrap().payload, b"seen");
}

#[test]
fn parked_buffer_caps_and_reports() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    for i in 0..128u8 {
        owner.broadcast("unclaimed", &[i]).unwrap();
    }
    assert_eq!(
        owner.broadcast("unclaimed", b"overflow"),
        Err(SendError::QueueFull)
    );

    let sub = client.subscribe(&["unclaimed"]);
    let mut received = 0;
    while let Some(msg) = sub.try_recv() {
        assert_eq!(msg.payload, [received]);
        received += 1;
    }
    assert_eq!(received, 128);
}

#[test]
fn parked_broadcasts_survive_owner_drop() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    owner.broadcast("early", b"parked").unwrap();
    drop(owner);

    let sub = client.subscribe(&["early"]);
    assert_eq!(sub.try_recv().unwrap().payload, b"parked");
}

#[test]
fn async_recv_delivers_queued_message() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    owner.broadcast("a", b"queued").unwrap();
    let msg = block_on_timeout(sub.recv_async(), Duration::from_secs(5)).unwrap();
    assert_eq!(msg.payload, b"queued");
}

#[test]
fn async_recv_wakes_on_send() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let (counter, waker) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(sub.recv_async());
    assert!(fut.as_mut().poll(&mut cx).is_pending());

    owner.broadcast("a", b"async wake").unwrap();
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 1);
    let Poll::Ready(Some(msg)) = fut.as_mut().poll(&mut cx) else {
        panic!("woken future should resolve with the message");
    };
    assert_eq!(msg.payload, b"async wake");
}

#[test]
fn async_upstream_wakes_owner() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    let (counter, waker) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(owner.recv_upstream_async());
    assert!(fut.as_mut().poll(&mut cx).is_pending());

    client.send("up", b"from client").unwrap();
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 1);
    let Poll::Ready(Some(msg)) = fut.as_mut().poll(&mut cx) else {
        panic!("woken future should resolve with the message");
    };
    assert_eq!(msg.payload, b"from client");
}

#[test]
fn future_resolves_when_subscription_dropped_before_poll() {
    let api = test_api();
    let _owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let fut = sub.recv_async();
    drop(sub);
    assert_eq!(block_on_timeout(fut, Duration::from_secs(5)), None);
}

#[test]
fn future_resolves_when_subscription_dropped_while_pending() {
    let api = test_api();
    let _owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let (counter, waker) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(sub.recv_async());
    assert!(fut.as_mut().poll(&mut cx).is_pending());
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 0);

    drop(sub);
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 1);
    assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready(None));
}

#[test]
fn pending_future_survives_owner_restart() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let (counter, waker) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(sub.recv_async());
    assert!(fut.as_mut().poll(&mut cx).is_pending());

    drop(owner);
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 0);
    assert!(fut.as_mut().poll(&mut cx).is_pending());

    let owner2 = BusOwner::open(&api, UUID_A);
    owner2.broadcast("a", b"new owner").unwrap();
    let Poll::Ready(Some(msg)) = fut.as_mut().poll(&mut cx) else {
        panic!("future should resolve with the new owner's message");
    };
    assert_eq!(msg.payload, b"new owner");
}

#[test]
fn upstream_future_outlives_its_owner() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    let (counter, waker) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(owner.recv_upstream_async());
    assert!(fut.as_mut().poll(&mut cx).is_pending());

    drop(owner);
    client.send("up", b"late").unwrap();
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 1);
    let Poll::Ready(Some(msg)) = fut.as_mut().poll(&mut cx) else {
        panic!("upstream future should resolve with the buffered message");
    };
    assert_eq!(msg.payload, b"late");
}

#[test]
fn concurrent_futures_each_receive() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let waker = thread_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut1 = pin!(sub.recv_async());
    let mut fut2 = pin!(sub.recv_async());
    assert!(fut1.as_mut().poll(&mut cx).is_pending());
    assert!(fut2.as_mut().poll(&mut cx).is_pending());

    owner.broadcast("a", b"first").unwrap();
    let Poll::Ready(Some(msg)) = fut1.as_mut().poll(&mut cx) else {
        panic!("fut1 should have received the message");
    };
    assert_eq!(msg.payload, b"first");
    assert!(fut2.as_mut().poll(&mut cx).is_pending());

    owner.broadcast("a", b"second").unwrap();
    let Poll::Ready(Some(msg)) = fut2.as_mut().poll(&mut cx) else {
        panic!("fut2 should have received the message");
    };
    assert_eq!(msg.payload, b"second");
}

#[test]
fn dropping_one_future_does_not_break_sibling() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let (counter, waker) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut2 = pin!(sub.recv_async());
    assert!(fut2.as_mut().poll(&mut cx).is_pending());

    {
        let mut fut1 = pin!(sub.recv_async());
        assert!(fut1.as_mut().poll(&mut cx).is_pending());
    }
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 0);

    owner.broadcast("a", b"sibling survives").unwrap();
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 1);
    let Poll::Ready(Some(msg)) = fut2.as_mut().poll(&mut cx) else {
        panic!("sibling future lost its wakeup");
    };
    assert_eq!(msg.payload, b"sibling survives");
}

#[test]
fn upstream_queue_full_drops_newest_and_reports() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    for i in 0..128u8 {
        client.send("up", &[i]).unwrap();
    }
    assert_eq!(client.send("up", b"overflow"), Err(SendError::QueueFull));

    assert_eq!(owner.try_recv_upstream().unwrap().payload, [0u8]);
}

#[test]
fn second_open_adopts_live_bus_as_co_owner() {
    let api = test_api();
    let owner1 = BusOwner::open(&api, UUID_A);
    let owner2 = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    owner1.broadcast("a", b"from one").unwrap();
    owner2.broadcast("a", b"from two").unwrap();
    assert_eq!(sub.try_recv().unwrap().payload, b"from one");
    assert_eq!(sub.try_recv().unwrap().payload, b"from two");
}

#[test]
fn open_before_owner_creates_the_shared_bus() {
    let api = test_api();
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let owner = BusOwner::open(&api, UUID_A);
    owner.broadcast("a", b"adopted").unwrap();
    assert_eq!(sub.try_recv().unwrap().payload, b"adopted");
}

#[test]
fn corrupt_discovery_record_is_replaced_with_a_fresh_bus() {
    let api = test_api();
    let _owner = BusOwner::open(&api, UUID_A);

    let key = super::discovery_key(&UUID_A);
    api.publish_data(&key, &[0u8; 32]);

    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);
    let owner2 = BusOwner::open(&api, UUID_A);
    owner2.broadcast("a", b"fresh").unwrap();
    assert_eq!(sub.try_recv().unwrap().payload, b"fresh");
}

const UUID_B: Uuid = Uuid::parse("22222222-2222-2222-2222-222222222222");

#[test]
fn cloned_client_shares_bus() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let clone = client.clone();
    drop(client);

    let sub = clone.subscribe(&["a"]);
    owner.broadcast("a", b"via clone").unwrap();
    assert_eq!(sub.try_recv().unwrap().payload, b"via clone");

    clone.send("up", b"from clone").unwrap();
    assert_eq!(owner.try_recv_upstream().unwrap().payload, b"from clone");
}

#[test]
fn distinct_uuids_are_independent_buses() {
    let api = test_api();
    let owner_a = BusOwner::open(&api, UUID_A);
    let owner_b = BusOwner::open(&api, UUID_B);
    let client_a = BusClient::open(&api, UUID_A);
    let client_b = BusClient::open(&api, UUID_B);
    let sub_a = client_a.subscribe(&["m"]);
    let sub_b = client_b.subscribe(&["m"]);

    owner_a.broadcast("m", b"from a").unwrap();
    assert_eq!(sub_a.try_recv().unwrap().payload, b"from a");
    assert!(sub_b.try_recv().is_none());

    client_b.send("up", b"to b").unwrap();
    assert_eq!(owner_b.try_recv_upstream().unwrap().payload, b"to b");
    assert!(owner_a.try_recv_upstream().is_none());
}

#[test]
fn two_clients_share_one_bus() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client1 = BusClient::open(&api, UUID_A);
    let client2 = BusClient::open(&api, UUID_A);
    let sub1 = client1.subscribe(&["m"]);
    let sub2 = client2.subscribe(&["m"]);

    owner.broadcast("m", b"to all").unwrap();
    assert_eq!(sub1.try_recv().unwrap().payload, b"to all");
    assert_eq!(sub2.try_recv().unwrap().payload, b"to all");

    client1.send("up", b"one").unwrap();
    client2.send("up", b"two").unwrap();
    assert_eq!(owner.try_recv_upstream().unwrap().payload, b"one");
    assert_eq!(owner.try_recv_upstream().unwrap().payload, b"two");
}

#[test]
fn empty_and_unicode_names_and_payloads_round_trip() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    let empty_sub = client.subscribe(&[""]);
    owner.broadcast("", b"").unwrap();
    let msg = empty_sub.try_recv().unwrap();
    assert_eq!(msg.name, "");
    assert_eq!(msg.payload, b"");

    client.send("", b"").unwrap();
    let msg = owner.try_recv_upstream().unwrap();
    assert_eq!(msg.name, "");
    assert_eq!(msg.payload, b"");

    let name = "héllo🚀";
    let unicode_sub = client.subscribe(&[name]);
    owner.broadcast(name, "påyload".as_bytes()).unwrap();
    let msg = unicode_sub.try_recv().unwrap();
    assert_eq!(msg.name, name);
    assert_eq!(msg.payload, "påyload".as_bytes());
}

#[test]
fn recv_upstream_timeout_times_out_and_wakes_on_send() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    assert!(owner.recv_upstream_timeout(Duration::ZERO).is_none());
    assert!(owner
        .recv_upstream_timeout(Duration::from_millis(20))
        .is_none());

    let barrier = Arc::new(Barrier::new(2));
    let sender_barrier = Arc::clone(&barrier);
    let sender = thread::spawn(move || {
        sender_barrier.wait();
        client.send("up", b"wake").unwrap();
    });

    barrier.wait();
    let start = Instant::now();
    let msg = owner
        .recv_upstream_timeout(Duration::from_secs(10))
        .unwrap();
    assert_eq!(msg.payload, b"wake");
    assert!(start.elapsed() < Duration::from_secs(5));
    sender.join().unwrap();
}

#[test]
fn zero_timeout_delivers_already_queued_messages() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    owner.broadcast("a", b"down").unwrap();
    client.send("up", b"up").unwrap();

    assert_eq!(sub.recv_timeout(Duration::ZERO).unwrap().payload, b"down");
    assert_eq!(
        owner.recv_upstream_timeout(Duration::ZERO).unwrap().payload,
        b"up"
    );
}

#[test]
fn max_duration_recv_blocks_until_message() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let barrier = Arc::new(Barrier::new(2));

    let sender_barrier = Arc::clone(&barrier);
    let sender = thread::spawn(move || {
        sender_barrier.wait();
        owner.broadcast("a", b"down").unwrap();
        owner
    });
    barrier.wait();
    assert_eq!(sub.recv_timeout(Duration::MAX).unwrap().payload, b"down");
    let owner = sender.join().unwrap();

    let sender_barrier = Arc::clone(&barrier);
    let sender = thread::spawn(move || {
        sender_barrier.wait();
        client.send("up", b"up").unwrap();
    });
    barrier.wait();
    assert_eq!(
        owner.recv_upstream_timeout(Duration::MAX).unwrap().payload,
        b"up"
    );
    sender.join().unwrap();
}

#[test]
fn async_upstream_delivers_queued_message() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    client.send("up", b"queued").unwrap();
    let msg = block_on_timeout(owner.recv_upstream_async(), Duration::from_secs(5)).unwrap();
    assert_eq!(msg.payload, b"queued");
}

#[test]
fn dropped_pending_future_leaves_message_queued() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    let (counter, waker) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    {
        let mut fut = pin!(sub.recv_async());
        assert!(fut.as_mut().poll(&mut cx).is_pending());
    }

    owner.broadcast("a", b"kept").unwrap();
    assert_eq!(counter.wakes.load(Ordering::SeqCst), 0);
    assert_eq!(sub.try_recv().unwrap().payload, b"kept");
}

#[test]
fn default_subscription_queue_capacity_is_128() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);

    for i in 0..128u8 {
        owner.broadcast("a", &[i]).unwrap();
    }
    assert_eq!(owner.broadcast("a", b"overflow"), Err(SendError::QueueFull));

    let mut received = 0;
    while let Some(msg) = sub.try_recv() {
        assert_eq!(msg.payload, [received]);
        received += 1;
    }
    assert_eq!(received, 128);
}

#[test]
fn empty_name_list_subscription_receives_nothing() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&[]);

    owner.broadcast("a", b"unmatched").unwrap();
    assert!(sub.try_recv().is_none());

    let late = client.subscribe(&["a"]);
    assert_eq!(late.try_recv().unwrap().payload, b"unmatched");
}

#[test]
fn multi_name_subscription_claims_parked_for_all_names() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);

    owner.broadcast("a", b"first").unwrap();
    owner.broadcast("b", b"second").unwrap();

    let sub = client.subscribe(&["a", "b"]);
    assert_eq!(sub.try_recv().unwrap().payload, b"first");
    assert_eq!(sub.try_recv().unwrap().payload, b"second");
    assert!(sub.try_recv().is_none());
}

#[test]
fn owner_open_ignores_corrupt_discovery_record() {
    let api = test_api();
    let key = super::discovery_key(&UUID_A);
    api.publish_data(&key, &[0xFFu8; 32]);

    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);
    owner.broadcast("a", b"fresh").unwrap();
    assert_eq!(sub.try_recv().unwrap().payload, b"fresh");
}

#[test]
fn discovery_record_validation_rejects_field_mismatches() {
    let api = test_api();
    let _owner = BusOwner::open(&api, UUID_A);
    let key = super::discovery_key(&UUID_A);
    let (valid, _) = api
        .query_data(&key, super::DISCOVERY_RECORD_SIZE as i32)
        .ok()
        .unwrap();

    let mut wrong_magic = valid.clone();
    wrong_magic[0] ^= 0xFF;
    assert!(super::read_discovery_record(&wrong_magic).is_none());

    let mut wrong_width = valid.clone();
    wrong_width[8] ^= 0xFF;
    assert!(super::read_discovery_record(&wrong_width).is_none());

    let mut null_vtable = valid.clone();
    null_vtable[16..24].fill(0);
    assert!(super::read_discovery_record(&null_vtable).is_none());

    let mut null_this = valid.clone();
    null_this[24..32].fill(0);
    assert!(super::read_discovery_record(&null_this).is_none());

    assert!(super::read_discovery_record(b"short").is_none());
    assert!(super::read_discovery_record(&valid).is_some());
}

use super::{
    abi::{BusHandle, BusStatus, BusStr, BusSubHandle},
    imp,
};

fn raw_subscribe(h: BusHandle, name: &str, queue_cap: usize) -> BusSubHandle {
    let names = [BusStr {
        ptr: name.as_ptr(),
        len: name.len(),
    }];
    let mut sub: BusSubHandle = 0;
    let status = unsafe {
        ((*h.vtable).subscribe)(h.this, names.as_ptr(), names.len(), queue_cap, &mut sub)
    };
    assert_eq!(status, BusStatus::OK);
    sub
}

fn raw_recv(
    h: BusHandle,
    sub: BusSubHandle,
    name_cap: usize,
    payload_cap: usize,
) -> (BusStatus, Vec<u8>, usize) {
    let mut name_buf = vec![0u8; name_cap];
    let mut payload_buf = vec![0u8; payload_cap];
    let mut name_len = 0usize;
    let mut payload_len = 0usize;
    let status = unsafe {
        ((*h.vtable).try_recv_downstream)(
            h.this,
            sub,
            name_buf.as_mut_ptr(),
            name_cap,
            &mut name_len,
            payload_buf.as_mut_ptr(),
            payload_cap,
            &mut payload_len,
        )
    };
    payload_buf.truncate(if status == BusStatus::OK {
        payload_len
    } else {
        0
    });
    (status, payload_buf, payload_len)
}

#[test]
fn broadcast_queue_full_still_delivers_where_there_is_space() {
    let api = test_api();
    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let small = client.subscribe_with_capacity(&["m"], 1);
    let big = client.subscribe(&["m"]);

    assert_eq!(owner.broadcast("m", b"first"), Ok(()));
    assert_eq!(owner.broadcast("m", b"second"), Err(SendError::QueueFull));

    assert_eq!(big.try_recv().expect("big first").payload, b"first");
    assert_eq!(big.try_recv().expect("big second").payload, b"second");
    assert_eq!(small.try_recv().expect("small first").payload, b"first");
    assert!(small.try_recv().is_none());
}

unsafe extern "C" fn noop_ctx(_ctx: *mut c_void) {}

#[test]
fn raw_abi_rejects_unknown_handles_and_tokens() {
    let h = imp::create_bus_handle();

    assert_eq!(
        unsafe { ((*h.vtable).unsubscribe)(h.this, 9999) },
        BusStatus::INVALID_HANDLE
    );
    assert_eq!(raw_recv(h, 9999, 64, 64).0, BusStatus::INVALID_HANDLE);

    let mut name_len = 0usize;
    let mut payload_len = 0usize;
    let mut name_buf = [0u8; 8];
    let mut payload_buf = [0u8; 8];
    let status = unsafe {
        ((*h.vtable).recv_downstream_timeout)(
            h.this,
            9999,
            0,
            name_buf.as_mut_ptr(),
            name_buf.len(),
            &mut name_len,
            payload_buf.as_mut_ptr(),
            payload_buf.len(),
            &mut payload_len,
        )
    };
    assert_eq!(status, BusStatus::INVALID_HANDLE);

    let mut token = 0u64;
    let status = unsafe {
        ((*h.vtable).register_waker)(
            h.this,
            9999,
            noop_ctx,
            noop_ctx,
            std::ptr::null_mut(),
            &mut token,
        )
    };
    assert_eq!(status, BusStatus::INVALID_HANDLE);

    let status = unsafe {
        ((*h.vtable).register_waker)(
            h.this,
            0,
            noop_ctx,
            noop_ctx,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    assert_eq!(status, BusStatus::INVALID_HANDLE);

    assert_eq!(
        unsafe { ((*h.vtable).unregister_waker)(h.this, 0, 424_242) },
        BusStatus::INVALID_HANDLE
    );

    let sub = raw_subscribe(h, "m", 0);
    assert_eq!(
        unsafe { ((*h.vtable).unregister_waker)(h.this, sub, 424_242) },
        BusStatus::INVALID_HANDLE
    );

    let status =
        unsafe { ((*h.vtable).subscribe)(h.this, std::ptr::null(), 0, 0, std::ptr::null_mut()) };
    assert_eq!(status, BusStatus::INVALID_HANDLE);

    unsafe { ((*h.vtable).unsubscribe)(h.this, sub) };
    unsafe { ((*h.vtable).release)(h.this) };
}

#[test]
fn recv_via_treats_unknown_status_as_dead_without_retrying() {
    let calls = AtomicUsize::new(0);
    let result = super::recv_via(|_, _, _, _, _, _| {
        calls.fetch_add(1, Ordering::SeqCst);
        BusStatus(999)
    });
    assert_eq!(result, Err(RecvFail::Dead));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn host_factory_allocates_the_bus() {
    let api = test_api();
    assert!(host::create_bus_via_host(&api).is_some());

    let owner = BusOwner::open(&api, UUID_A);
    let client = BusClient::open(&api, UUID_A);
    let sub = client.subscribe(&["a"]);
    owner.broadcast("a", b"via host").unwrap();
    assert_eq!(sub.try_recv().unwrap().payload, b"via host");

    client.send("up", b"upstream via host").unwrap();
    assert_eq!(
        owner.try_recv_upstream().unwrap().payload,
        b"upstream via host"
    );
}

#[test]
fn host_factory_record_validation_rejects_field_mismatches() {
    let api = test_api_without_host_factory();
    assert!(host::create_bus_via_host(&api).is_none());

    let valid = host::host_factory_record();

    let mut wrong_magic = valid;
    wrong_magic[0] ^= 0xFF;
    api.publish_data(host::HOST_FACTORY_KEY, &wrong_magic);
    assert!(host::create_bus_via_host(&api).is_none());

    let mut wrong_width = valid;
    wrong_width[8] ^= 0xFF;
    api.publish_data(host::HOST_FACTORY_KEY, &wrong_width);
    assert!(host::create_bus_via_host(&api).is_none());

    let mut null_vtable = valid;
    null_vtable[16..24].fill(0);
    api.publish_data(host::HOST_FACTORY_KEY, &null_vtable);
    assert!(host::create_bus_via_host(&api).is_none());

    api.publish_data(host::HOST_FACTORY_KEY, b"short");
    assert!(host::create_bus_via_host(&api).is_none());

    api.publish_data(host::HOST_FACTORY_KEY, &valid);
    assert!(host::create_bus_via_host(&api).is_some());
}

#[test]
#[should_panic(expected = "factory record")]
fn owner_open_panics_without_host_factory() {
    let api = test_api_without_host_factory();
    let _ = BusOwner::open(&api, UUID_A);
}

#[test]
#[should_panic(expected = "factory record")]
fn client_open_panics_without_host_factory() {
    let api = test_api_without_host_factory();
    let _ = BusClient::open(&api, UUID_A);
}

#[test]
#[should_panic(expected = "factory record")]
fn owner_open_panics_on_corrupt_factory_record() {
    let api = test_api_without_host_factory();
    api.publish_data(host::HOST_FACTORY_KEY, &[0xFFu8; 32]);
    let _ = BusOwner::open(&api, UUID_A);
}
