use std::{collections::VecDeque, sync::Arc};
use time::UtcDateTime;
use tokio::{
    spawn,
    sync::{
        Mutex,
        mpsc::{Sender, channel},
    },
};
use tracing::{
    Event, Id, Level, Subscriber,
    field::{Field, Visit},
    level_filters::LevelFilter,
    span::Attributes,
};
use tracing_subscriber::{
    Layer, fmt,
    fmt::time::LocalTime,
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
};

pub struct LogService {
    pub buffer: Arc<Mutex<VecDeque<CapturedEvent>>>,
}

impl LogService {
    const CAPACITY: usize = 256;
    pub fn init() -> Arc<Self> {
        let (tx, mut rx) = channel(100);
        tracing_subscriber::registry()
            .with(fmt::layer().with_timer(LocalTime::rfc_3339()))
            .with(MemLayer::new(tx))
            .with(LevelFilter::INFO)
            .init();
        let buffer = Arc::new(Mutex::new(VecDeque::with_capacity(Self::CAPACITY)));
        let svc: Arc<Self> = Self { buffer }.into();
        let svc1 = svc.clone();
        spawn(async move {
            let svc = svc1;
            while let Some(msg) = rx.recv().await {
                svc.push(msg).await;
            }
        });
        svc
    }

    pub async fn push(self: &Arc<Self>, event: CapturedEvent) {
        let mut guard = self.buffer.lock().await;
        if guard.len() >= Self::CAPACITY {
            guard.pop_front();
        }
        guard.push_back(event);
    }
}

pub struct CapturedEvent {
    pub timestamp: UtcDateTime,
    pub config: Option<String>,
    pub level: Level,
    pub target: String,
    pub message: String,
}

#[derive(Default)]
struct FieldVisitor {
    config: Option<String>,
    message: Option<String>,
}

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{:?}", value).trim_matches('"').to_string());
        }
        if field.name() == "config" {
            self.config = Some(format!("{:?}", value).trim_matches('"').to_string());
        }
    }
}
#[derive(Clone)]
pub struct MemLayer {
    sender: Sender<CapturedEvent>,
}

impl MemLayer {
    pub fn new(sender: Sender<CapturedEvent>) -> Self {
        Self { sender }
    }
}

impl<S> Layer<S> for MemLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut visitor = FieldVisitor::default();
        attrs.record(&mut visitor);
        if let Some(span) = ctx.span(id)
            && let Some(config) = visitor.config
        {
            span.extensions_mut().insert(config)
        }
    }

    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let span = _ctx.current_span().id().and_then(|id| _ctx.span(id));
        let config: Option<String> = span.and_then(|s| s.extensions().get().cloned());
        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);
        // 只有当事件中确实包含 message 字段时，我们才捕获它
        if let Some(message) = visitor.message {
            let metadata = event.metadata();
            let captured = CapturedEvent {
                timestamp: UtcDateTime::now(),
                config,
                level: *metadata.level(),
                target: event.metadata().target().to_string(),
                message,
            };
            if let Err(err) = self.sender.try_send(captured) {
                println!("send event error : {:?}", err);
            }
        }
    }
}
