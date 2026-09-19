use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use super::{SamClient, SamClientError, SamCredentials, SamStudent, StudentLessonsPage};

pub trait Clock: Send + Sync {
    fn now(&self) -> Instant;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheTtl {
    pub students: Duration,
    pub lessons: Duration,
}

struct Entry<T> {
    value: T,
    stored_at: Instant,
}

impl<T: Clone> Entry<T> {
    fn fresh_value(&self, now: Instant, ttl: Duration) -> Option<T> {
        (now.saturating_duration_since(self.stored_at) < ttl).then(|| self.value.clone())
    }
}

#[derive(Default)]
struct Cache {
    students: Option<Entry<Vec<SamStudent>>>,
    lessons: HashMap<String, Entry<StudentLessonsPage>>,
}

/// Decorates a [`SamClient`] so that repeated reads within a TTL are served from memory.
///
/// Sharing one instance between every gateway also lets them reuse each other's fetches. Only
/// successful reads are cached, and any login discards the cache because the session — and so
/// the visible data — may belong to a different user afterwards.
pub struct SamClientCacheDecorator {
    inner: Arc<dyn SamClient + Send + Sync>,
    clock: Arc<dyn Clock>,
    ttl: CacheTtl,
    cache: Mutex<Cache>,
}

impl SamClientCacheDecorator {
    #[must_use]
    pub fn new(
        inner: Arc<dyn SamClient + Send + Sync>,
        clock: Arc<dyn Clock>,
        ttl: CacheTtl,
    ) -> Self {
        Self {
            inner,
            clock,
            ttl,
            cache: Mutex::new(Cache::default()),
        }
    }

    fn cache(&self) -> MutexGuard<'_, Cache> {
        self.cache.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl SamClient for SamClientCacheDecorator {
    fn login(&self, credentials: &SamCredentials) -> Result<(), SamClientError> {
        *self.cache() = Cache::default();

        self.inner.login(credentials)
    }

    fn students(&self) -> Result<Vec<SamStudent>, SamClientError> {
        let cached: Option<Vec<SamStudent>> = self
            .cache()
            .students
            .as_ref()
            .and_then(|entry| entry.fresh_value(self.clock.now(), self.ttl.students));

        if let Some(students) = cached {
            return Ok(students);
        }

        let students: Vec<SamStudent> = self.inner.students()?;

        self.cache().students = Some(Entry {
            value: students.clone(),
            stored_at: self.clock.now(),
        });

        Ok(students)
    }

    fn student_lessons(&self, student_id: &str) -> Result<StudentLessonsPage, SamClientError> {
        let cached: Option<StudentLessonsPage> = self
            .cache()
            .lessons
            .get(student_id)
            .and_then(|entry| entry.fresh_value(self.clock.now(), self.ttl.lessons));

        if let Some(page) = cached {
            return Ok(page);
        }

        let page: StudentLessonsPage = self.inner.student_lessons(student_id)?;

        self.cache().lessons.insert(
            student_id.to_owned(),
            Entry {
                value: page.clone(),
                stored_at: self.clock.now(),
            },
        );

        Ok(page)
    }
}
