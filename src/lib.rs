#[derive(Debug, PartialEq)]
struct Biterator<T> {
    index: usize,
    items: Vec<T>,
}

impl<'a, T: 'a> Biterator<T> {
    pub fn from_vec(vec: &[T]) -> Self
    where
        T: Clone,
    {
        Biterator {
            index: 0,
            items: vec.to_vec(),
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        T: Clone,
        I: Iterator<Item = &'a T>,
    {
        Biterator {
            index: 0,
            items: iter.cloned().collect::<Vec<T>>(),
        }
    }

    pub fn next(&mut self) -> Option<&T> {
        if self.index < self.items.len() {
            self.index += 1;
            return self.items.get(self.index - 1);
        }
        None
    }

    pub fn prev(&mut self) -> Option<&T> {
        if self.index > 1 {
            self.index -= 1;
            return self.items.get(self.index - 1);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::Biterator;

    #[test]
    fn test_from_vec() {
        let band_members = vec!["Ayase", "ikura", "AssH", "Honogumo", "Hikaru", "Zakuro"];
        let members_biter = Biterator::from_vec(&band_members);
        assert_eq!(
            members_biter,
            Biterator {
                index: 0,
                items: vec!["Ayase", "ikura", "AssH", "Honogumo", "Hikaru", "Zakuro"]
            }
        );
    }

    #[test]
    fn test_from_iter() {
        let setlist = vec!["Tsubame", "Adventure", "Seventeen", "Shukufuku", "Aidoru"];
        let setlist_iter = setlist.iter();
        let setlist_biter = Biterator::from_iter(setlist_iter);
        assert_eq!(
            setlist_biter,
            Biterator {
                index: 0,
                items: vec!["Tsubame", "Adventure", "Seventeen", "Shukufuku", "Aidoru"]
            }
        );
    }

    #[test]
    fn test_prev_next() {
        let anisong = vec!["Beastars Season 2 Op", "Beastars Season 2 Ed", "Mobile Suit Gundam: The Witch from Mercury Op", "Oshi no Ko Op"];
        let mut anisong_biter = Biterator::from_vec(&anisong);

        assert_eq!(anisong_biter.prev(), None);
        assert_eq!(anisong_biter.prev(), None);
        assert_eq!(anisong_biter.next(), Some(&"Beastars Season 2 Op"));
        assert_eq!(anisong_biter.next(), Some(&"Beastars Season 2 Ed"));
        assert_eq!(anisong_biter.prev(), Some(&"Beastars Season 2 Op"));
        assert_eq!(anisong_biter.next(), Some(&"Beastars Season 2 Ed"));
        assert_eq!(anisong_biter.next(), Some(&"Mobile Suit Gundam: The Witch from Mercury Op"));
        assert_eq!(anisong_biter.next(), Some(&"Oshi no Ko Op"));
        assert_eq!(anisong_biter.next(), None);
    }
}
