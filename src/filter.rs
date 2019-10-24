use crate::task::Task;

type Toggle<T> = Option<T>;

// each field is an optional "Toggle". If it is Some() it applies to the filter,
// if none it is ignored
pub struct Filter {
    inbox: Toggle<bool>,
    has_project: Toggle<bool>,
}

impl Filter {
    pub fn empty() -> Filter {
        Filter {
            inbox: None,
            has_project: None,
        }
    }

    pub fn new_inbox() -> Filter {
        let mut f = Filter::empty();
        f.inbox = Some(true);
        f
    }

    pub fn new_projects() -> Filter {
        let mut f = Filter::empty();
        f.has_project = Some(true);
        f
    }

    pub fn into_iter<'a, I: Iterator<Item=&'a Task>>(self, iter: I) -> FilterIter<'a, I> {
        FilterIter::new(self, iter)
    }
}

pub struct FilterIter<'a, I> where I: Iterator<Item=&'a Task> {
    filter: Filter,
    tasks: I,
}

impl<'a, I> FilterIter<'a, I> where I: Iterator<Item=&'a Task> {
    fn new(f: Filter, i: I) -> FilterIter<'a, I> {
        FilterIter { filter: f, tasks: i }
    }
}

impl<'a, I> Iterator for FilterIter<'a, I> where I: Iterator<Item=&'a Task> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(task) = self.tasks.next() {
            if let Some(inbox) = self.filter.inbox {
                if task.inbox != inbox {
                    continue;
                }
            }

            if let Some(has_project) = self.filter.has_project {
                if task.parent.is_some() != has_project {
                    continue;
                }
            }

            // TODO: implement more filters

            return Some(task);
        }

        None // tasks iter is None at this point, therefore we are None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_inbox() {
        let mut tasks = vec![
            Task::default(),
            Task::default(),
            Task::default(),
        ];

        tasks[0].id = "foo".into();
        tasks[1].id = "bar".into();
        tasks[2].id = "baz".into();

        tasks[1].inbox = true;
        tasks[2].inbox = true;

        let iter = Filter::new_inbox().into_iter(tasks.iter());
        let filtered: Vec<&Task> = iter.collect();

        assert!(filtered[0] == &tasks[1]);
        assert!(filtered[1] == &tasks[2]);
        assert!(filtered.len() == 2);
    }

    #[test]
    fn test_filter_projects() {
        let mut tasks = vec![
            Task::default(),
            Task::default(),
            Task::default(),
        ];

        tasks[0].id = "foo".into();
        tasks[1].id = "bar".into();
        tasks[2].id = "baz".into();

        tasks[0].parent = Some("someproj".into());
        tasks[2].parent = Some("someproj".into());

        let iter = Filter::new_projects().into_iter(tasks.iter());
        let filtered: Vec<&Task> = iter.collect();

        assert!(filtered[0] == &tasks[0]);
        assert!(filtered[1] == &tasks[2]);
        assert!(filtered.len() == 2);
    }
}
